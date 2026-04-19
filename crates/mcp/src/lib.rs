use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolCall {
    pub tool_name: String,
    pub arguments: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolResult {
    pub content: String,
    pub is_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum McpProviderType {
    Chrome,
    Playwright,
    ThinkIn,
    Agent,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpProviderConfig {
    pub name: String,
    pub provider_type: McpProviderType,
    pub command: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub enabled: bool,
}

pub struct McpManager {
    providers: HashMap<String, McpProviderConfig>,
}

impl Default for McpManager {
    fn default() -> Self {
        Self::new()
    }
}

impl McpManager {
    pub fn new() -> Self {
        let mut providers = HashMap::new();

        providers.insert(
            "chrome".into(),
            McpProviderConfig {
                name: "Chrome DevTools MCP".into(),
                provider_type: McpProviderType::Chrome,
                command: "chrome-devtools-mcp".into(),
                args: vec![],
                env: HashMap::new(),
                enabled: false,
            },
        );

        providers.insert(
            "playwright".into(),
            McpProviderConfig {
                name: "Playwright MCP".into(),
                provider_type: McpProviderType::Playwright,
                command: "playwright-mcp".into(),
                args: vec![],
                env: HashMap::new(),
                enabled: false,
            },
        );

        Self { providers }
    }

    pub fn list_providers(&self) -> Vec<&McpProviderConfig> {
        self.providers.values().collect()
    }

    pub fn enable(&mut self, name: &str) {
        if let Some(p) = self.providers.get_mut(name) {
            p.enabled = true;
        }
    }

    pub fn disable(&mut self, name: &str) {
        if let Some(p) = self.providers.get_mut(name) {
            p.enabled = false;
        }
    }
}
