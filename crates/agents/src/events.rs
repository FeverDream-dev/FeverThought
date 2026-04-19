//! Pipeline event model — 12 structured event types emitted during agent execution.

use serde::{Deserialize, Serialize};

/// All event types in the agent pipeline lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "payload")]
pub enum PipelineEvent {
    /// Pipeline has started processing a user request.
    PipelineStarted,

    /// Intake analysis completed — ambiguity, scope, and risk detected.
    IntakeCompleted {
        ambiguity_detected: bool,
        repo_scope: String,
        risk_tier: String,
        destructive_potential: bool,
    },

    /// Pipeline paused to ask the user clarification questions.
    ClarificationNeeded {
        questions: Vec<ClarificationEventQuestion>,
    },

    /// User has responded to clarification questions.
    ClarificationReceived { answers: Vec<String> },

    /// Context gathering completed — relevant files identified.
    ContextGathered {
        files: Vec<String>,
        architecture_notes: Vec<String>,
        probable_change_surface: Vec<String>,
    },

    /// Planner has produced a structured plan.
    PlanCreated {
        plan_id: String,
        title: String,
        step_count: usize,
    },

    /// Plan has been approved (by user or auto-approved for low risk).
    PlanApproved { plan_id: String },

    /// Execution progress — a step has been updated.
    ExecutionProgress {
        step_id: String,
        status: String,
        description: String,
    },

    /// Reviewer has completed analysis of the changes.
    ReviewCompleted {
        findings: Vec<ReviewFinding>,
        approved: bool,
    },

    /// Git Summarizer has generated a commit message and summary.
    SummaryGenerated {
        commit_message: String,
        change_summary: String,
    },

    /// Pipeline finished successfully.
    PipelineCompleted,

    /// Pipeline failed with an error.
    PipelineFailed { error: String, step: String },
}

/// A clarification question included in a ClarificationNeeded event.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClarificationEventQuestion {
    pub id: String,
    pub question: String,
    pub options: Vec<String>,
    pub allow_custom: bool,
}

/// A finding from the Reviewer agent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReviewFinding {
    pub severity: FindingSeverity,
    pub description: String,
    pub file: Option<String>,
    pub line: Option<u32>,
}

/// Severity of a review finding.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FindingSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Wrapper for a timed event tied to an execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub timestamp: String,
    pub execution_id: String,
    pub event: PipelineEvent,
}

/// Trait for receiving pipeline events. Implementations can log, broadcast, or persist events.
pub trait EventSubscriber: Send + Sync {
    fn on_event(&self, envelope: &EventEnvelope);
}
