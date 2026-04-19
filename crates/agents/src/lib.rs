pub mod context;
pub mod events;
pub mod permissions;
pub mod pipeline;
pub mod planner;
pub mod prompts;
pub mod roles;
pub mod routing;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentRole {
    IntentClarifier,
    Planner,
    RepoCartographer,
    ToolRouter,
    Implementer,
    Reviewer,
    UiBrowser,
    GitSummarizer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub role: AgentRole,
    pub content: String,
    pub timestamp: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: String,
    pub title: String,
    pub steps: Vec<PlanStep>,
    pub assumptions: Vec<String>,
    pub questions: Vec<ClarificationQuestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStep {
    pub id: String,
    pub description: String,
    pub files: Vec<String>,
    pub status: StepStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StepStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClarificationQuestion {
    pub id: String,
    pub question: String,
    pub options: Vec<ClarificationOption>,
    pub allow_custom: bool,
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClarificationOption {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClarificationResponse {
    pub question_id: String,
    pub selected: Vec<String>,
    pub custom: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentExecution {
    pub id: String,
    pub plan_id: Option<String>,
    pub pipeline_state: PipelineState,
    pub messages: Vec<AgentMessage>,
    pub status: ExecutionStatus,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PipelineState {
    Intake,
    Clarifying,
    GatheringContext,
    Planning,
    Executing,
    Reviewing,
    Summarizing,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExecutionStatus {
    Idle,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}
