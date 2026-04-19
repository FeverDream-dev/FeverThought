pub mod ollama;
pub mod cloud;
pub mod registry;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProviderId(pub String);

impl fmt::Display for ProviderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelId(pub String);

impl fmt::Display for ModelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: ModelId,
    pub name: String,
    pub provider: ProviderId,
    pub context_window: Option<usize>,
    pub capabilities: ModelCapabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelCapabilities {
    pub supports_text: bool,
    pub supports_vision: bool,
    pub supports_tools: bool,
    pub supports_reasoning: bool,
    pub supports_json_mode: bool,
    pub supports_streaming: bool,
    pub supports_mcp_bridge: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: ModelId,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub content: String,
    pub model: ModelId,
    pub provider: ProviderId,
    pub usage: Option<TokenUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotAnalysis {
    pub visible_text: Vec<String>,
    pub ui_elements: Vec<UiElement>,
    pub detected_errors: Vec<DetectedError>,
    pub likely_context: String,
    pub user_goal_hypothesis: String,
    pub ambiguity_flags: Vec<String>,
    pub followup_questions: Vec<String>,
    pub relevant_regions: Vec<ScreenRegion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiElement {
    pub element_type: String,
    pub label: Option<String>,
    pub bounds: Option<ScreenRegion>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedError {
    pub message: String,
    pub severity: String,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenRegion {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[async_trait::async_trait]
pub trait AiProvider: Send + Sync {
    fn id(&self) -> &ProviderId;
    fn name(&self) -> &str;
    fn is_local(&self) -> bool;
    async fn is_available(&self) -> bool;
    async fn list_models(&self) -> anyhow::Result<Vec<ModelInfo>>;
    async fn chat(&self, request: ChatRequest) -> anyhow::Result<ChatResponse>;
    async fn chat_stream(&self, request: ChatRequest) -> anyhow::Result<tokio_stream::wrappers::ReceiverStream<ChatResponse>>;
    async fn analyze_screenshot(&self, image_bytes: &[u8], prompt: &str) -> anyhow::Result<ScreenshotAnalysis>;
}
