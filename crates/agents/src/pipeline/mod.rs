use crate::{
    events::{EventEnvelope, EventSubscriber, PipelineEvent},
    context::ContextAssembler,
    permissions::PermissionManager,
    routing::{ModelRouter, ModelRoutingConfig, RoutingDecision, TaskType},
    AgentExecution, AgentMessage, AgentRole, ClarificationQuestion,
    ExecutionStatus, PipelineState, Plan,
};
use feverthoth_providers::{AiProvider, ChatMessage, ChatRequest, MessageRole, ModelId, ProviderId};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use uuid::Uuid;

/// Complete 7-step agent pipeline:
/// Intake → Clarify → GatherContext → Plan → Execute → Review → Summarize
pub struct AgentPipeline {
    providers: Arc<RwLock<HashMap<ProviderId, Arc<dyn AiProvider>>>>,
    executions: RwLock<HashMap<String, AgentExecution>>,
    event_subscribers: RwLock<Vec<Arc<dyn EventSubscriber>>>,
    permissions: RwLock<PermissionManager>,
    model_router: RwLock<ModelRouter>,
}

impl AgentPipeline {
    pub fn new(providers: Arc<RwLock<HashMap<ProviderId, Arc<dyn AiProvider>>>>) -> Self {
        let permissions = PermissionManager::new();
        let router = ModelRouter::new(
            ModelRoutingConfig::default(),
            PermissionManager::new(),
        );
        Self {
            providers,
            executions: RwLock::new(HashMap::new()),
            event_subscribers: RwLock::new(Vec::new()),
            permissions: RwLock::new(permissions),
            model_router: RwLock::new(router),
        }
    }

    pub fn with_permissions(&mut self, perms: PermissionManager) {
        *self.permissions.write() = perms;
    }

    pub fn with_routing_config(&mut self, config: ModelRoutingConfig) {
        *self.model_router.write() = ModelRouter::new(
            config,
            self.permissions.read().clone(),
        );
    }

    pub fn subscribe(&self, subscriber: Arc<dyn EventSubscriber>) {
        self.event_subscribers.write().push(subscriber);
    }

    fn emit(&self, execution_id: &str, event: PipelineEvent) {
        let envelope = EventEnvelope {
            timestamp: chrono::Utc::now().to_rfc3339(),
            execution_id: execution_id.to_string(),
            event,
        };
        for sub in self.event_subscribers.read().iter() {
            sub.on_event(&envelope);
        }
    }

    pub fn create_execution(&self) -> String {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let execution = AgentExecution {
            id: id.clone(),
            plan_id: None,
            pipeline_state: PipelineState::Intake,
            messages: vec![],
            status: ExecutionStatus::Idle,
            created_at: now,
        };
        self.executions.write().insert(id.clone(), execution);
        self.emit(&id, PipelineEvent::PipelineStarted);
        id
    }

    /// Step 1: Intake — parse user request and detect ambiguity, scope, risk.
    pub async fn intake(
        &self,
        execution_id: &str,
        user_message: &str,
    ) -> anyhow::Result<IntakeResult> {
        let provider = self.get_default_provider()?;

        let routing = self.model_router.read().route(&TaskType::Chat, false, false);

        let messages = vec![
            ChatMessage {
                role: MessageRole::System,
                content: crate::prompts::INTENT_CLARIFIER.to_string(),
                images: None,
            },
            ChatMessage {
                role: MessageRole::User,
                content: format!(
                    "Analyze this request for ambiguity, scope, and risk. Return JSON:\n\
                     {{\"ambiguity_detected\": bool, \"repo_scope\": string, \"risk_tier\": \"low\"|\"medium\"|\"high\", \
                     \"destructive_potential\": bool, \"needs_clarification\": bool, \
                     \"needs_context\": bool, \"needs_vision\": bool}}\n\nRequest:\n{}",
                    user_message
                ),
                images: None,
            },
        ];

        let request = ChatRequest {
            model: ModelId(routing.model_id.clone()),
            messages,
            stream: false,
            temperature: Some(0.3),
            max_tokens: Some(2048),
        };

        let response = provider.chat(request).await?;

        let result = parse_intake(&response.content);

        if let Some(exec) = self.executions.write().get_mut(execution_id) {
            exec.pipeline_state = PipelineState::Intake;
            exec.messages.push(AgentMessage {
                role: AgentRole::IntentClarifier,
                content: response.content.clone(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                metadata: None,
            });
        }

        self.emit(execution_id, PipelineEvent::IntakeCompleted {
            ambiguity_detected: result.ambiguity_detected,
            repo_scope: result.repo_scope.clone(),
            risk_tier: result.risk_tier.clone(),
            destructive_potential: result.destructive_potential,
        });

        Ok(result)
    }

    /// Step 2: Clarify — ask focused questions if ambiguity is material.
    pub async fn clarify(
        &self,
        execution_id: &str,
        user_message: &str,
    ) -> anyhow::Result<Vec<ClarificationQuestion>> {
        let provider = self.get_default_provider()?;

        let routing = self.model_router.read().route(&TaskType::Chat, false, false);

        if let Some(exec) = self.executions.write().get_mut(execution_id) {
            exec.pipeline_state = PipelineState::Clarifying;
        }

        let messages = vec![
            ChatMessage {
                role: MessageRole::System,
                content: crate::prompts::INTENT_CLARIFIER.to_string(),
                images: None,
            },
            ChatMessage {
                role: MessageRole::User,
                content: format!(
                    "Analyze this request and return clarification questions as JSON:\n\
                     {{\"questions\": [{{\"id\": \"q1\", \"question\": \"...\", \
                     \"options\": [{{\"label\": \"...\", \"value\": \"...\"}}], \
                     \"allow_custom\": true}}]}}\n\nRequest:\n{}",
                    user_message
                ),
                images: None,
            },
        ];

        let request = ChatRequest {
            model: ModelId(routing.model_id.clone()),
            messages,
            stream: false,
            temperature: Some(0.3),
            max_tokens: Some(4096),
        };

        let response = provider.chat(request).await?;
        let questions = parse_clarification_questions(&response.content);

        if let Some(exec) = self.executions.write().get_mut(execution_id) {
            exec.messages.push(AgentMessage {
                role: AgentRole::IntentClarifier,
                content: response.content.clone(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                metadata: None,
            });
        }

        let event_questions: Vec<_> = questions.iter().map(|q| crate::events::ClarificationEventQuestion {
            id: q.id.clone(),
            question: q.question.clone(),
            options: q.options.iter().map(|o| o.label.clone()).collect(),
            allow_custom: q.allow_custom,
        }).collect();
        self.emit(execution_id, PipelineEvent::ClarificationNeeded { questions: event_questions });

        Ok(questions)
    }

    /// Step 3: GatherContext — build relevant files list and architecture notes.
    pub async fn gather_context(
        &self,
        execution_id: &str,
        user_message: &str,
    ) -> anyhow::Result<ContextResult> {
        let provider = self.get_default_provider()?;

        let routing = self.model_router.read().route(&TaskType::ContextGathering, false, false);

        if let Some(exec) = self.executions.write().get_mut(execution_id) {
            exec.pipeline_state = PipelineState::GatheringContext;
        }

        let messages = vec![
            ChatMessage {
                role: MessageRole::System,
                content: crate::prompts::REPO_CARTOGRAPHER.to_string(),
                images: None,
            },
            ChatMessage {
                role: MessageRole::User,
                content: format!(
                    "Identify the most relevant files and architecture for this task. Return JSON:\n\
                     {{\"files\": [\"path/to/file\"], \"architecture_notes\": [\"note\"], \
                     \"probable_change_surface\": [\"path\"]}}\n\nTask:\n{}",
                    user_message
                ),
                images: None,
            },
        ];

        let request = ChatRequest {
            model: ModelId(routing.model_id.clone()),
            messages,
            stream: false,
            temperature: Some(0.2),
            max_tokens: Some(4096),
        };

        let response = provider.chat(request).await?;
        let result = parse_context(&response.content);

        if let Some(exec) = self.executions.write().get_mut(execution_id) {
            exec.messages.push(AgentMessage {
                role: AgentRole::RepoCartographer,
                content: response.content.clone(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                metadata: None,
            });
        }

        self.emit(execution_id, PipelineEvent::ContextGathered {
            files: result.files.clone(),
            architecture_notes: result.architecture_notes.clone(),
            probable_change_surface: result.probable_change_surface.clone(),
        });

        Ok(result)
    }

    /// Step 4: Plan — generate structured implementation plan.
    pub async fn plan(&self, execution_id: &str, user_message: &str) -> anyhow::Result<Plan> {
        let provider = self.get_default_provider()?;

        let routing = self.model_router.read().route(&TaskType::CodingPlan, false, false);

        if let Some(exec) = self.executions.write().get_mut(execution_id) {
            exec.pipeline_state = PipelineState::Planning;
        }

        let exec = self.executions.read().get(execution_id).cloned();
        let mut history = vec![
            ChatMessage {
                role: MessageRole::System,
                content: crate::prompts::PLANNER.to_string(),
                images: None,
            },
        ];

        if let Some(exec) = exec {
            for msg in &exec.messages {
                history.push(ChatMessage {
                    role: MessageRole::User,
                    content: msg.content.clone(),
                    images: None,
                });
            }
        }

        history.push(ChatMessage {
            role: MessageRole::User,
            content: format!(
                "Create a complete implementation plan. Return JSON:\n\
                 {{\"title\": \"...\", \"steps\": [{{\"id\": \"1\", \"description\": \"...\", \"files\": [\"...\"]}}], \
                 \"assumptions\": [\"...\"]}}\n\nFor:\n{}",
                user_message
            ),
            images: None,
        });

        let request = ChatRequest {
            model: ModelId(routing.model_id.clone()),
            messages: history,
            stream: false,
            temperature: Some(0.2),
            max_tokens: Some(8192),
        };

        let response = provider.chat(request).await?;
        let plan = parse_plan(&response.content);

        if let Some(exec) = self.executions.write().get_mut(execution_id) {
            exec.plan_id = Some(plan.id.clone());
            exec.messages.push(AgentMessage {
                role: AgentRole::Planner,
                content: response.content,
                timestamp: chrono::Utc::now().to_rfc3339(),
                metadata: None,
            });
        }

        self.emit(execution_id, PipelineEvent::PlanCreated {
            plan_id: plan.id.clone(),
            title: plan.title.clone(),
            step_count: plan.steps.len(),
        });

        Ok(plan)
    }

    /// Step 5: Execute — implementer applies changes.
    pub async fn execute_step(
        &self,
        execution_id: &str,
        step_description: &str,
    ) -> anyhow::Result<String> {
        let provider = self.get_default_provider()?;

        let routing = self.model_router.read().route(&TaskType::CodingPlan, false, false);

        if let Some(exec) = self.executions.write().get_mut(execution_id) {
            exec.pipeline_state = PipelineState::Executing;
            exec.status = ExecutionStatus::Running;
        }

        let messages = vec![
            ChatMessage {
                role: MessageRole::System,
                content: crate::prompts::IMPLEMENTER.to_string(),
                images: None,
            },
            ChatMessage {
                role: MessageRole::User,
                content: step_description.to_string(),
                images: None,
            },
        ];

        let request = ChatRequest {
            model: ModelId(routing.model_id.clone()),
            messages,
            stream: false,
            temperature: Some(0.2),
            max_tokens: Some(8192),
        };

        let response = provider.chat(request).await?;

        if let Some(exec) = self.executions.write().get_mut(execution_id) {
            exec.messages.push(AgentMessage {
                role: AgentRole::Implementer,
                content: response.content.clone(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                metadata: None,
            });
        }

        self.emit(execution_id, PipelineEvent::ExecutionProgress {
            step_id: "current".to_string(),
            status: "completed".to_string(),
            description: step_description.to_string(),
        });

        Ok(response.content)
    }

    /// Step 6: Review — inspect proposed changes.
    pub async fn review(
        &self,
        execution_id: &str,
        changes: &str,
    ) -> anyhow::Result<ReviewResult> {
        let provider = self.get_default_provider()?;

        let routing = self.model_router.read().route(&TaskType::Review, false, false);

        if let Some(exec) = self.executions.write().get_mut(execution_id) {
            exec.pipeline_state = PipelineState::Reviewing;
        }

        let messages = vec![
            ChatMessage {
                role: MessageRole::System,
                content: crate::prompts::REVIEWER.to_string(),
                images: None,
            },
            ChatMessage {
                role: MessageRole::User,
                content: format!(
                    "Review these changes for correctness, drift, edge cases, and unsafe commands. Return JSON:\n\
                     {{\"approved\": bool, \"findings\": [{{\"severity\": \"info\"|\"warning\"|\"error\"|\"critical\", \
                     \"description\": \"...\", \"file\": \"...\", \"line\": null}}]}}\n\nChanges:\n{}",
                    changes
                ),
                images: None,
            },
        ];

        let request = ChatRequest {
            model: ModelId(routing.model_id.clone()),
            messages,
            stream: false,
            temperature: Some(0.2),
            max_tokens: Some(4096),
        };

        let response = provider.chat(request).await?;
        let result = parse_review(&response.content);

        if let Some(exec) = self.executions.write().get_mut(execution_id) {
            exec.messages.push(AgentMessage {
                role: AgentRole::Reviewer,
                content: response.content.clone(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                metadata: None,
            });
        }

        let findings: Vec<crate::events::ReviewFinding> = result.findings.iter().map(|f| {
            crate::events::ReviewFinding {
                severity: match f.severity.as_str() {
                    "critical" => crate::events::FindingSeverity::Critical,
                    "error" => crate::events::FindingSeverity::Error,
                    "warning" => crate::events::FindingSeverity::Warning,
                    _ => crate::events::FindingSeverity::Info,
                },
                description: f.description.clone(),
                file: f.file.clone(),
                line: f.line,
            }
        }).collect();

        self.emit(execution_id, PipelineEvent::ReviewCompleted {
            findings,
            approved: result.approved,
        });

        Ok(result)
    }

    /// Step 7: Summarize — generate commit message and change summary.
    pub async fn summarize(
        &self,
        execution_id: &str,
        diff: &str,
    ) -> anyhow::Result<SummaryResult> {
        let provider = self.get_default_provider()?;

        let routing = self.model_router.read().route(&TaskType::Chat, false, false);

        if let Some(exec) = self.executions.write().get_mut(execution_id) {
            exec.pipeline_state = PipelineState::Summarizing;
        }

        let messages = vec![
            ChatMessage {
                role: MessageRole::System,
                content: crate::prompts::GIT_SUMMARIZER.to_string(),
                images: None,
            },
            ChatMessage {
                role: MessageRole::User,
                content: format!(
                    "Produce a commit message and change summary. Return JSON:\n\
                     {{\"commit_message\": \"...\", \"change_summary\": \"...\"}}\n\nDiff:\n{}",
                    diff
                ),
                images: None,
            },
        ];

        let request = ChatRequest {
            model: ModelId(routing.model_id.clone()),
            messages,
            stream: false,
            temperature: Some(0.3),
            max_tokens: Some(2048),
        };

        let response = provider.chat(request).await?;
        let result = parse_summary(&response.content);

        if let Some(exec) = self.executions.write().get_mut(execution_id) {
            exec.pipeline_state = PipelineState::Completed;
            exec.status = ExecutionStatus::Completed;
            exec.messages.push(AgentMessage {
                role: AgentRole::GitSummarizer,
                content: response.content,
                timestamp: chrono::Utc::now().to_rfc3339(),
                metadata: None,
            });
        }

        self.emit(execution_id, PipelineEvent::SummaryGenerated {
            commit_message: result.commit_message.clone(),
            change_summary: result.change_summary.clone(),
        });
        self.emit(execution_id, PipelineEvent::PipelineCompleted);

        Ok(result)
    }

    fn get_default_provider(&self) -> anyhow::Result<Arc<dyn AiProvider>> {
        let providers = self.providers.read();
        providers.get(&ProviderId("ollama".to_string()))
            .cloned()
            .or_else(|| providers.values().next().cloned())
            .ok_or_else(|| anyhow::anyhow!("No AI provider available"))
    }
}

#[derive(Debug, Clone)]
pub struct IntakeResult {
    pub ambiguity_detected: bool,
    pub repo_scope: String,
    pub risk_tier: String,
    pub destructive_potential: bool,
    pub needs_clarification: bool,
    pub needs_context: bool,
    pub needs_vision: bool,
}

#[derive(Debug, Clone)]
pub struct ContextResult {
    pub files: Vec<String>,
    pub architecture_notes: Vec<String>,
    pub probable_change_surface: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ReviewResult {
    pub approved: bool,
    pub findings: Vec<ReviewFindingParsed>,
}

#[derive(Debug, Clone)]
pub struct ReviewFindingParsed {
    pub severity: String,
    pub description: String,
    pub file: Option<String>,
    pub line: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct SummaryResult {
    pub commit_message: String,
    pub change_summary: String,
}

fn parse_intake(content: &str) -> IntakeResult {
    if let Ok(json) = extract_json(content) {
        IntakeResult {
            ambiguity_detected: json.get("ambiguity_detected").and_then(|v| v.as_bool()).unwrap_or(false),
            repo_scope: json.get("repo_scope").and_then(|v| v.as_str()).unwrap_or("unknown").to_string(),
            risk_tier: json.get("risk_tier").and_then(|v| v.as_str()).unwrap_or("low").to_string(),
            destructive_potential: json.get("destructive_potential").and_then(|v| v.as_bool()).unwrap_or(false),
            needs_clarification: json.get("needs_clarification").and_then(|v| v.as_bool()).unwrap_or(false),
            needs_context: json.get("needs_context").and_then(|v| v.as_bool()).unwrap_or(true),
            needs_vision: json.get("needs_vision").and_then(|v| v.as_bool()).unwrap_or(false),
        }
    } else {
        IntakeResult {
            ambiguity_detected: false,
            repo_scope: "unknown".to_string(),
            risk_tier: "low".to_string(),
            destructive_potential: false,
            needs_clarification: false,
            needs_context: true,
            needs_vision: false,
        }
    }
}

fn parse_context(content: &str) -> ContextResult {
    if let Ok(json) = extract_json(content) {
        ContextResult {
            files: json.get("files")
                .and_then(|v| serde_json::from_value(v.clone()).ok())
                .unwrap_or_default(),
            architecture_notes: json.get("architecture_notes")
                .and_then(|v| serde_json::from_value(v.clone()).ok())
                .unwrap_or_default(),
            probable_change_surface: json.get("probable_change_surface")
                .and_then(|v| serde_json::from_value(v.clone()).ok())
                .unwrap_or_default(),
        }
    } else {
        ContextResult {
            files: vec![],
            architecture_notes: vec![],
            probable_change_surface: vec![],
        }
    }
}

fn parse_review(content: &str) -> ReviewResult {
    if let Ok(json) = extract_json(content) {
        let findings: Vec<ReviewFindingParsed> = json.get("findings")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter().filter_map(|f| {
                    Some(ReviewFindingParsed {
                        severity: f.get("severity").and_then(|v| v.as_str()).unwrap_or("info").to_string(),
                        description: f.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        file: f.get("file").and_then(|v| v.as_str()).map(String::from),
                        line: f.get("line").and_then(|v| v.as_u64()).map(|l| l as u32),
                    })
                }).collect()
            })
            .unwrap_or_default();

        ReviewResult {
            approved: json.get("approved").and_then(|v| v.as_bool()).unwrap_or(true),
            findings,
        }
    } else {
        ReviewResult {
            approved: true,
            findings: vec![],
        }
    }
}

fn parse_summary(content: &str) -> SummaryResult {
    if let Ok(json) = extract_json(content) {
        SummaryResult {
            commit_message: json.get("commit_message").and_then(|v| v.as_str()).unwrap_or("chore: update files").to_string(),
            change_summary: json.get("change_summary").and_then(|v| v.as_str()).unwrap_or("Changes made.").to_string(),
        }
    } else {
        SummaryResult {
            commit_message: "chore: update files".to_string(),
            change_summary: content.to_string(),
        }
    }
}

fn parse_clarification_questions(content: &str) -> Vec<ClarificationQuestion> {
    if let Ok(json) = extract_json(content) {
        if let Some(questions) = json.get("questions").and_then(|q| serde_json::from_value(q.clone()).ok()) {
            return questions;
        }
    }
    vec![]
}

fn parse_plan(content: &str) -> Plan {
    if let Ok(json) = extract_json(content) {
        serde_json::from_value(json).unwrap_or_else(|_| Plan {
            id: Uuid::new_v4().to_string(),
            title: "Auto-generated plan".into(),
            steps: vec![crate::PlanStep {
                id: "1".into(),
                description: content.to_string(),
                files: vec![],
                status: crate::StepStatus::Pending,
            }],
            assumptions: vec![],
            questions: vec![],
        })
    } else {
        Plan {
            id: Uuid::new_v4().to_string(),
            title: "Direct execution".into(),
            steps: vec![crate::PlanStep {
                id: "1".into(),
                description: content.to_string(),
                files: vec![],
                status: crate::StepStatus::Pending,
            }],
            assumptions: vec![],
            questions: vec![],
        }
    }
}

fn extract_json(content: &str) -> anyhow::Result<serde_json::Value> {
    let trimmed = content.trim();
    if trimmed.starts_with('{') {
        Ok(serde_json::from_str(trimmed)?)
    } else if let Some(start) = content.find('{') {
        let end = content.rfind('}').ok_or_else(|| anyhow::anyhow!("No closing brace"))?;
        Ok(serde_json::from_str(&content[start..=end])?)
    } else {
        Err(anyhow::anyhow!("No JSON found"))
    }
}
