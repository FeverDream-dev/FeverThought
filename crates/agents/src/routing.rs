//! Model routing policy — decides which AI provider/model handles each task.
//!
//! Inputs: task type, privacy policy, tool need, vision need, context size,
//! user-pinned preferences, latency/cost mode.

use serde::{Deserialize, Serialize};

use crate::permissions::{PermissionClass, PermissionManager, PermissionPolicy};

/// Configuration for model routing decisions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRoutingConfig {
    /// User-pinned provider (overrides all routing logic).
    pub pinned_provider: Option<String>,
    /// User-pinned model within the provider.
    pub pinned_model: Option<String>,
    /// Whether to prefer local models for privacy.
    pub prefer_local: bool,
    /// Latency vs quality tradeoff.
    pub latency_mode: LatencyMode,
}

impl Default for ModelRoutingConfig {
    fn default() -> Self {
        Self {
            pinned_provider: None,
            pinned_model: None,
            prefer_local: true,
            latency_mode: LatencyMode::Balanced,
        }
    }
}

/// Latency vs quality tradeoff mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LatencyMode {
    /// Prefer speed, accept lower quality.
    Fast,
    /// Balance speed and quality.
    Balanced,
    /// Prefer quality, accept higher latency.
    Quality,
}

/// The type of task being routed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskType {
    /// Screenshot analysis / vision task.
    VisionSummary,
    /// Browser inspection via MCP.
    BrowserInspection,
    /// Heavy coding plan generation.
    CodingPlan,
    /// Broad multi-model access.
    BroadAccess,
    /// Simple offline/local text request.
    LocalText,
    /// General chat / clarification.
    Chat,
    /// Code review.
    Review,
    /// Context gathering / repo analysis.
    ContextGathering,
}

/// Routing decision output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingDecision {
    /// Recommended provider ID (e.g. "ollama", "openai", "openrouter").
    pub provider_id: String,
    /// Recommended model ID (e.g. "qwen2.5-vl", "gpt-4o").
    pub model_id: String,
    /// Why this routing was chosen.
    pub reason: String,
    /// Whether data will leave the machine.
    pub data_leaves_machine: bool,
}

/// The model router evaluates task characteristics and picks the best provider/model.
pub struct ModelRouter {
    config: ModelRoutingConfig,
    permissions: PermissionManager,
}

impl ModelRouter {
    pub fn new(config: ModelRoutingConfig, permissions: PermissionManager) -> Self {
        Self {
            config,
            permissions,
        }
    }

    /// Route a task to the best available provider/model.
    pub fn route(
        &self,
        task: &TaskType,
        needs_vision: bool,
        _needs_tools: bool,
    ) -> RoutingDecision {
        // Guardrail: never ignore explicit user/provider pin
        if let (Some(provider), Some(model)) =
            (&self.config.pinned_provider, &self.config.pinned_model)
        {
            return RoutingDecision {
                provider_id: provider.clone(),
                model_id: model.clone(),
                reason: "User-pinned provider and model override".to_string(),
                data_leaves_machine: provider != "ollama",
            };
        }

        // Guardrail: never send raw images to cloud without permission
        if needs_vision {
            let cloud_check = self.permissions.check(PermissionClass::SendToCloudProvider);
            if cloud_check == PermissionPolicy::Denied || self.config.prefer_local {
                return RoutingDecision {
                    provider_id: "ollama".to_string(),
                    model_id: "qwen2.5-vl".to_string(),
                    reason: "Vision task routed to local Ollama for privacy".to_string(),
                    data_leaves_machine: false,
                };
            }
        }

        // Task-based routing per plan spec
        match task {
            TaskType::VisionSummary => RoutingDecision {
                provider_id: "ollama".to_string(),
                model_id: "qwen2.5-vl".to_string(),
                reason: "Screenshot summary uses local vision model".to_string(),
                data_leaves_machine: false,
            },
            TaskType::BrowserInspection => RoutingDecision {
                provider_id: "mcp".to_string(),
                model_id: "playwright".to_string(),
                reason: "Browser inspection via Playwright/Chrome MCP".to_string(),
                data_leaves_machine: false,
            },
            TaskType::CodingPlan => {
                if self.config.prefer_local {
                    RoutingDecision {
                        provider_id: "ollama".to_string(),
                        model_id: "llama3.2".to_string(),
                        reason: "Heavy coding plan routed to local Ollama (prefer_local=true)"
                            .to_string(),
                        data_leaves_machine: false,
                    }
                } else {
                    RoutingDecision {
                        provider_id: "zai".to_string(),
                        model_id: "coding-plan".to_string(),
                        reason: "Heavy coding plan routed to Z.AI Coding Plan".to_string(),
                        data_leaves_machine: true,
                    }
                }
            }
            TaskType::BroadAccess => RoutingDecision {
                provider_id: "openrouter".to_string(),
                model_id: "auto".to_string(),
                reason: "Broad model access fallback via OpenRouter".to_string(),
                data_leaves_machine: true,
            },
            TaskType::LocalText => RoutingDecision {
                provider_id: "ollama".to_string(),
                model_id: "llama3.2".to_string(),
                reason: "Offline/local text request uses Ollama".to_string(),
                data_leaves_machine: false,
            },
            TaskType::Chat => {
                if self.config.prefer_local {
                    RoutingDecision {
                        provider_id: "ollama".to_string(),
                        model_id: "llama3.2".to_string(),
                        reason: "Chat routed to local model".to_string(),
                        data_leaves_machine: false,
                    }
                } else {
                    RoutingDecision {
                        provider_id: "openai".to_string(),
                        model_id: "gpt-4o".to_string(),
                        reason: "Chat routed to cloud for quality".to_string(),
                        data_leaves_machine: true,
                    }
                }
            }
            TaskType::Review => match self.config.latency_mode {
                LatencyMode::Quality => RoutingDecision {
                    provider_id: "openai".to_string(),
                    model_id: "gpt-4o".to_string(),
                    reason: "Code review uses high-quality cloud model".to_string(),
                    data_leaves_machine: true,
                },
                _ => RoutingDecision {
                    provider_id: "ollama".to_string(),
                    model_id: "llama3.2".to_string(),
                    reason: "Code review uses local model (fast/balanced mode)".to_string(),
                    data_leaves_machine: false,
                },
            },
            TaskType::ContextGathering => RoutingDecision {
                provider_id: "ollama".to_string(),
                model_id: "llama3.2".to_string(),
                reason: "Context gathering uses local model for speed".to_string(),
                data_leaves_machine: false,
            },
        }
    }
}
