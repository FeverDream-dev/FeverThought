use anyhow::Result;
use async_trait::async_trait;
use crate::{
    AiProvider, ChatRequest, ChatResponse, ModelCapabilities, ModelId, ModelInfo,
    ProviderId, ScreenshotAnalysis,
};

pub struct OpenAiProvider {
    api_key: String,
    base_url: String,
    model: String,
}

impl OpenAiProvider {
    pub fn new(api_key: String, base_url: Option<String>, model: Option<String>) -> Self {
        Self {
            api_key,
            base_url: base_url.unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
            model: model.unwrap_or_else(|| "gpt-4o".to_string()),
        }
    }

    pub fn codex(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.openai.com/v1".to_string(),
            model: "codex".to_string(),
        }
    }
}

#[async_trait]
impl AiProvider for OpenAiProvider {
    fn id(&self) -> &ProviderId {
        static ID: once_cell::sync::Lazy<ProviderId> = once_cell::sync::Lazy::new(|| ProviderId("openai".to_string()));
        &ID
    }

    fn name(&self) -> &str {
        "OpenAI"
    }

    fn is_local(&self) -> bool {
        false
    }

    async fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vec![
            ModelInfo {
                id: ModelId("gpt-4o".to_string()),
                name: "GPT-4o".to_string(),
                provider: self.id().clone(),
                context_window: Some(128000),
                capabilities: ModelCapabilities {
                    supports_text: true,
                    supports_vision: true,
                    supports_tools: true,
                    supports_reasoning: true,
                    supports_json_mode: true,
                    supports_streaming: true,
                    supports_mcp_bridge: false,
                },
            },
            ModelInfo {
                id: ModelId("gpt-4o-mini".to_string()),
                name: "GPT-4o Mini".to_string(),
                provider: self.id().clone(),
                context_window: Some(128000),
                capabilities: ModelCapabilities {
                    supports_text: true,
                    supports_vision: true,
                    supports_tools: true,
                    supports_reasoning: false,
                    supports_json_mode: true,
                    supports_streaming: true,
                    supports_mcp_bridge: false,
                },
            },
        ])
    }

    async fn chat(&self, _request: ChatRequest) -> Result<ChatResponse> {
        anyhow::bail!("Use chat_stream for OpenAI")
    }

    async fn chat_stream(&self, _request: ChatRequest) -> Result<tokio_stream::wrappers::ReceiverStream<ChatResponse>> {
        anyhow::bail!("OpenAI streaming not yet implemented")
    }

    async fn analyze_screenshot(&self, _image_bytes: &[u8], _prompt: &str) -> Result<ScreenshotAnalysis> {
        anyhow::bail!("Use local vision model for screenshot analysis")
    }
}

pub struct GeminiProvider {
    api_key: String,
    model: String,
}

impl GeminiProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self {
            api_key,
            model: model.unwrap_or_else(|| "gemini-2.0-flash".to_string()),
        }
    }
}

#[async_trait]
impl AiProvider for GeminiProvider {
    fn id(&self) -> &ProviderId {
        static ID: once_cell::sync::Lazy<ProviderId> = once_cell::sync::Lazy::new(|| ProviderId("gemini".to_string()));
        &ID
    }

    fn name(&self) -> &str {
        "Google Gemini"
    }

    fn is_local(&self) -> bool {
        false
    }

    async fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vec![ModelInfo {
            id: ModelId(self.model.clone()),
            name: format!("Gemini ({})", self.model),
            provider: self.id().clone(),
            context_window: Some(1000000),
            capabilities: ModelCapabilities {
                supports_text: true,
                supports_vision: true,
                supports_tools: true,
                supports_reasoning: true,
                supports_json_mode: true,
                supports_streaming: true,
                supports_mcp_bridge: false,
            },
        }])
    }

    async fn chat(&self, _request: ChatRequest) -> Result<ChatResponse> {
        anyhow::bail!("Use chat_stream for Gemini")
    }

    async fn chat_stream(&self, _request: ChatRequest) -> Result<tokio_stream::wrappers::ReceiverStream<ChatResponse>> {
        anyhow::bail!("Gemini streaming not yet implemented")
    }

    async fn analyze_screenshot(&self, _image_bytes: &[u8], _prompt: &str) -> Result<ScreenshotAnalysis> {
        anyhow::bail!("Use local vision model for screenshot analysis")
    }
}

pub struct OpenRouterProvider {
    api_key: String,
    model: String,
}

impl OpenRouterProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self {
            api_key,
            model: model.unwrap_or_else(|| "anthropic/claude-3.5-sonnet".to_string()),
        }
    }
}

#[async_trait]
impl AiProvider for OpenRouterProvider {
    fn id(&self) -> &ProviderId {
        static ID: once_cell::sync::Lazy<ProviderId> = once_cell::sync::Lazy::new(|| ProviderId("openrouter".to_string()));
        &ID
    }

    fn name(&self) -> &str {
        "OpenRouter"
    }

    fn is_local(&self) -> bool {
        false
    }

    async fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vec![ModelInfo {
            id: ModelId(self.model.clone()),
            name: self.model.clone(),
            provider: self.id().clone(),
            context_window: Some(200000),
            capabilities: ModelCapabilities {
                supports_text: true,
                supports_vision: false,
                supports_tools: true,
                supports_reasoning: true,
                supports_json_mode: true,
                supports_streaming: true,
                supports_mcp_bridge: false,
            },
        }])
    }

    async fn chat(&self, _request: ChatRequest) -> Result<ChatResponse> {
        anyhow::bail!("Use chat_stream for OpenRouter")
    }

    async fn chat_stream(&self, _request: ChatRequest) -> Result<tokio_stream::wrappers::ReceiverStream<ChatResponse>> {
        anyhow::bail!("OpenRouter streaming not yet implemented")
    }

    async fn analyze_screenshot(&self, _image_bytes: &[u8], _prompt: &str) -> Result<ScreenshotAnalysis> {
        anyhow::bail!("Use local vision model for screenshot analysis")
    }
}

pub struct ZaiCodingProvider {
    api_key: String,
    model: String,
}

impl ZaiCodingProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "zai-coding-plan".to_string(),
        }
    }
}

#[async_trait]
impl AiProvider for ZaiCodingProvider {
    fn id(&self) -> &ProviderId {
        static ID: once_cell::sync::Lazy<ProviderId> = once_cell::sync::Lazy::new(|| ProviderId("zai".to_string()));
        &ID
    }

    fn name(&self) -> &str {
        "Z.AI Coding Plan"
    }

    fn is_local(&self) -> bool {
        false
    }

    async fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vec![ModelInfo {
            id: ModelId(self.model.clone()),
            name: "Z.AI Coding Plan".to_string(),
            provider: self.id().clone(),
            context_window: Some(200000),
            capabilities: ModelCapabilities {
                supports_text: true,
                supports_vision: false,
                supports_tools: true,
                supports_reasoning: true,
                supports_json_mode: true,
                supports_streaming: true,
                supports_mcp_bridge: true,
            },
        }])
    }

    async fn chat(&self, _request: ChatRequest) -> Result<ChatResponse> {
        anyhow::bail!("Use chat_stream for Z.AI")
    }

    async fn chat_stream(&self, _request: ChatRequest) -> Result<tokio_stream::wrappers::ReceiverStream<ChatResponse>> {
        anyhow::bail!("Z.AI streaming not yet implemented")
    }

    async fn analyze_screenshot(&self, _image_bytes: &[u8], _prompt: &str) -> Result<ScreenshotAnalysis> {
        anyhow::bail!("Use local vision model for screenshot analysis")
    }
}
