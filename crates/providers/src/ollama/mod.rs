use anyhow::Result;
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use ollama_rs::generation::chat::{request::ChatMessageRequest, ChatMessage as OllamaChatMessage};
use ollama_rs::generation::images::Image;
use ollama_rs::Ollama;
use tokio_stream::StreamExt;

use crate::{
    AiProvider, ChatMessage, ChatRequest, ChatResponse, MessageRole, ModelCapabilities, ModelId,
    ModelInfo, ProviderId, ScreenshotAnalysis, TokenUsage,
};

const DEFAULT_OLLAMA_URL: &str = "http://localhost";
const DEFAULT_OLLAMA_PORT: u16 = 11434;
const DEFAULT_VISION_MODEL: &str = "qwen2.5vl";
const SCREENSHOT_SYSTEM_PROMPT: &str = r#"You are a screenshot analysis engine for an AI-first IDE.
Analyze the provided screenshot and return a JSON object with this exact structure:
{
  "visible_text": ["list of visible text snippets"],
  "ui_elements": [{"element_type": "button|input|panel|tab|menu|label", "label": "text", "state": "active|inactive|error|loading"}],
  "detected_errors": [{"message": "error text", "severity": "error|warning|info", "source": "source if known"}],
  "likely_context": "description of what the user is doing",
  "user_goal_hypothesis": "what the user likely wants to accomplish",
  "ambiguity_flags": ["list of unclear aspects"],
  "followup_questions": ["questions to ask the user"],
  "relevant_regions": [{"x": 0, "y": 0, "width": 100, "height": 100}]
}
Return ONLY valid JSON."#;

pub struct OllamaProvider {
    client: Ollama,
    vision_model: String,
    #[allow(dead_code)]
    default_model: String,
}

impl OllamaProvider {
    pub fn new(
        url: Option<&str>,
        port: Option<u16>,
        vision_model: Option<&str>,
        default_model: Option<&str>,
    ) -> Self {
        let client = Ollama::new(
            url.unwrap_or(DEFAULT_OLLAMA_URL).to_string(),
            port.unwrap_or(DEFAULT_OLLAMA_PORT),
        );
        Self {
            client,
            vision_model: vision_model.unwrap_or(DEFAULT_VISION_MODEL).to_string(),
            default_model: default_model.unwrap_or("llama3.2").to_string(),
        }
    }

    pub fn with_default() -> Self {
        Self::new(None, None, None, None)
    }
}

fn convert_role(role: &MessageRole) -> ollama_rs::generation::chat::MessageRole {
    match role {
        MessageRole::System => ollama_rs::generation::chat::MessageRole::System,
        MessageRole::User => ollama_rs::generation::chat::MessageRole::User,
        MessageRole::Assistant => ollama_rs::generation::chat::MessageRole::Assistant,
        MessageRole::Tool => ollama_rs::generation::chat::MessageRole::Tool,
    }
}

fn to_ollama_messages(messages: &[ChatMessage]) -> Vec<OllamaChatMessage> {
    messages
        .iter()
        .map(|msg| {
            let mut ollama_msg =
                OllamaChatMessage::new(convert_role(&msg.role), msg.content.clone());
            if let Some(images) = &msg.images {
                let ollama_images: Vec<Image> = images
                    .iter()
                    .map(|img| Image::from_base64(img.clone()))
                    .collect();
                ollama_msg = ollama_msg.with_images(ollama_images);
            }
            ollama_msg
        })
        .collect()
}

#[async_trait]
impl AiProvider for OllamaProvider {
    fn id(&self) -> &ProviderId {
        static ID: once_cell::sync::Lazy<ProviderId> =
            once_cell::sync::Lazy::new(|| ProviderId("ollama".to_string()));
        &ID
    }

    fn name(&self) -> &str {
        "Ollama (Local)"
    }

    fn is_local(&self) -> bool {
        true
    }

    async fn is_available(&self) -> bool {
        reqwest::get(format!("http://localhost:{}/", DEFAULT_OLLAMA_PORT))
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        let models = self.client.list_local_models().await?;
        let provider_id = ProviderId("ollama".to_string());
        Ok(models
            .into_iter()
            .map(|m| {
                let name = m.name.clone();
                let has_vision = name.contains("vl")
                    || name.contains("vision")
                    || name.contains("llava")
                    || name.contains("qwen2.5vl");
                let has_reasoning = name.contains("reasoner") || name.contains("think");
                let id = ModelId(name.clone());
                ModelInfo {
                    id,
                    name,
                    provider: provider_id.clone(),
                    context_window: None,
                    capabilities: ModelCapabilities {
                        supports_text: true,
                        supports_vision: has_vision,
                        supports_tools: true,
                        supports_reasoning: has_reasoning,
                        supports_json_mode: true,
                        supports_streaming: true,
                        supports_mcp_bridge: false,
                    },
                }
            })
            .collect())
    }

    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let model = request.model.0.clone();
        let ollama_messages = to_ollama_messages(&request.messages);
        let ollama_request = ChatMessageRequest::new(model, ollama_messages);

        let response = self.client.send_chat_messages(ollama_request).await?;

        Ok(ChatResponse {
            content: response.message.content,
            model: request.model.clone(),
            provider: ProviderId("ollama".to_string()),
            usage: response.final_data.map(|fd| TokenUsage {
                prompt_tokens: fd.prompt_eval_count as u32,
                completion_tokens: fd.eval_count as u32,
                total_tokens: (fd.prompt_eval_count + fd.eval_count) as u32,
            }),
            done: Some(true),
        })
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> Result<tokio_stream::wrappers::ReceiverStream<ChatResponse>> {
        let model = request.model.0.clone();
        let provider_id = ProviderId("ollama".to_string());
        let ollama_messages = to_ollama_messages(&request.messages);
        let ollama_request = ChatMessageRequest::new(model, ollama_messages);

        let stream = self
            .client
            .send_chat_messages_stream(ollama_request)
            .await?;

        let (tx, rx) = tokio::sync::mpsc::channel(32);

        tokio::spawn(async move {
            let mut stream = std::pin::pin!(stream);
            while let Some(result) = stream.next().await {
                match result {
                    Ok(chunk) => {
                        let chat_response = ChatResponse {
                            content: chunk.message.content,
                            model: ModelId(chunk.model),
                            provider: provider_id.clone(),
                            usage: None,
                            done: Some(chunk.done),
                        };
                        if tx.send(chat_response).await.is_err() {
                            break;
                        }
                    }
                    Err(()) => break,
                }
            }
        });

        Ok(tokio_stream::wrappers::ReceiverStream::new(rx))
    }

    async fn analyze_screenshot(
        &self,
        image_bytes: &[u8],
        prompt: &str,
    ) -> Result<ScreenshotAnalysis> {
        let b64 = BASE64.encode(image_bytes);
        let message =
            OllamaChatMessage::user(prompt.to_string()).with_images(vec![Image::from_base64(b64)]);

        let request = ChatMessageRequest::new(
            self.vision_model.clone(),
            vec![
                OllamaChatMessage::system(SCREENSHOT_SYSTEM_PROMPT.to_string()),
                message,
            ],
        );

        let response = self.client.send_chat_messages(request).await?;
        let content = response.message.content.trim();

        let analysis: ScreenshotAnalysis = if content.starts_with('{') {
            serde_json::from_str(content).unwrap_or_else(|_| ScreenshotAnalysis {
                visible_text: vec![],
                ui_elements: vec![],
                detected_errors: vec![],
                likely_context: content.to_string(),
                user_goal_hypothesis: String::new(),
                ambiguity_flags: vec![],
                followup_questions: vec![],
                relevant_regions: vec![],
            })
        } else {
            ScreenshotAnalysis {
                visible_text: vec![],
                ui_elements: vec![],
                detected_errors: vec![],
                likely_context: content.to_string(),
                user_goal_hypothesis: String::new(),
                ambiguity_flags: vec![],
                followup_questions: vec![],
                relevant_regions: vec![],
            }
        };

        Ok(analysis)
    }
}
