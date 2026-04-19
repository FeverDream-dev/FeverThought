//! Context engineering module — assembles the minimum context needed for each agent step.
//!
//! Context sources: user request, clarified answers, workspace summary,
//! relevant file snippets, architecture notes, diagnostics/errors,
//! git diff state, screenshot summary JSON, MCP output, explicit constraints.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single context source with priority metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSource {
    pub kind: ContextKind,
    pub content: String,
    /// Higher priority = included first when truncating.
    pub priority: u8,
}

/// Types of context that can be assembled for an agent step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContextKind {
    UserRequest,
    ClarifiedAnswers,
    WorkspaceSummary,
    FileSnippet,
    ArchitectureNotes,
    Diagnostics,
    GitDiff,
    ScreenshotSummary,
    McpOutput,
    ExplicitConstraints,
}

/// Assembled context ready to be injected into a prompt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssembledContext {
    /// Ordered context sources (highest priority first).
    pub sources: Vec<ContextSource>,
    /// Estimated total token count.
    pub estimated_tokens: usize,
    /// Facts separated from assumptions.
    pub facts: Vec<String>,
    pub assumptions: Vec<String>,
}

/// High-value session memory persisted across interactions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionMemory {
    /// User-stated preferences.
    pub user_preferences: Vec<String>,
    /// Repo conventions discovered during the session.
    pub repo_conventions: Vec<String>,
    /// Confirmed technology stack.
    pub tech_stack: Vec<String>,
    /// Pending unresolved questions.
    pub pending_questions: Vec<String>,
}

impl SessionMemory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a user preference.
    pub fn record_preference(&mut self, preference: String) {
        if !self.user_preferences.contains(&preference) {
            self.user_preferences.push(preference);
        }
    }

    /// Record a discovered repo convention.
    pub fn record_convention(&mut self, convention: String) {
        if !self.repo_conventions.contains(&convention) {
            self.repo_conventions.push(convention);
        }
    }

    /// Record a confirmed tech stack item.
    pub fn record_tech(&mut self, tech: String) {
        if !self.tech_stack.contains(&tech) {
            self.tech_stack.push(tech);
        }
    }
}

/// Context assembler — builds the right context for each pipeline step.
pub struct ContextAssembler {
    sources: HashMap<ContextKind, String>,
    memory: SessionMemory,
}

impl ContextAssembler {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
            memory: SessionMemory::new(),
        }
    }

    /// Add a context source.
    pub fn add_source(&mut self, kind: ContextKind, content: String) {
        self.sources.insert(kind, content);
    }

    /// Set session memory (e.g. loaded from persistence).
    pub fn set_memory(&mut self, memory: SessionMemory) {
        self.memory = memory;
    }

    /// Get a reference to session memory.
    pub fn memory(&self) -> &SessionMemory {
        &self.memory
    }

    /// Get a mutable reference to session memory.
    pub fn memory_mut(&mut self) -> &mut SessionMemory {
        &mut self.memory
    }

    /// Assemble context for a pipeline step, respecting the assembly rules:
    /// - Prioritize relevance over bulk
    /// - Summarize repo structure before dumping file text
    /// - Include current file and nearby dependencies first
    /// - Include explicit "do not change" constraints
    /// - Separate facts from assumptions
    /// - Include test/validation instructions when available
    pub fn assemble(&self, max_tokens: usize) -> AssembledContext {
        let mut all_sources: Vec<ContextSource> = Vec::new();

        // Priority ordering per assembly rules
        let priorities = [
            (ContextKind::UserRequest, 10),
            (ContextKind::ClarifiedAnswers, 9),
            (ContextKind::ExplicitConstraints, 9),
            (ContextKind::WorkspaceSummary, 8),
            (ContextKind::ArchitectureNotes, 7),
            (ContextKind::FileSnippet, 6),
            (ContextKind::Diagnostics, 5),
            (ContextKind::GitDiff, 4),
            (ContextKind::McpOutput, 3),
            (ContextKind::ScreenshotSummary, 2),
        ];

        for (kind, priority) in priorities {
            if let Some(content) = self.sources.get(&kind) {
                all_sources.push(ContextSource {
                    kind,
                    content: content.clone(),
                    priority,
                });
            }
        }

        // Add session memory facts
        let mut facts = Vec::new();
        let mut assumptions = Vec::new();

        for pref in &self.memory.user_preferences {
            facts.push(format!("[user preference] {}", pref));
        }
        for conv in &self.memory.repo_conventions {
            facts.push(format!("[repo convention] {}", conv));
        }
        for tech in &self.memory.tech_stack {
            facts.push(format!("[tech stack] {}", tech));
        }

        // Extract facts vs assumptions from explicit constraints
        if let Some(constraints) = self.sources.get(&ContextKind::ExplicitConstraints) {
            for line in constraints.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("assume:") || trimmed.starts_with("assuming ") {
                    assumptions.push(trimmed.to_string());
                } else if !trimmed.is_empty() {
                    facts.push(trimmed.to_string());
                }
            }
        }

        // Sort by priority (descending) and estimate tokens
        all_sources.sort_by(|a, b| b.priority.cmp(&a.priority));

        // Simple token estimation: ~4 chars per token
        let estimated_tokens: usize = all_sources.iter().map(|s| s.content.len() / 4).sum();

        // Truncate if over budget (remove lowest priority sources)
        let mut sources = all_sources;
        let mut total_est = estimated_tokens;
        while total_est > max_tokens && sources.len() > 1 {
            sources.pop();
            total_est = sources.iter().map(|s| s.content.len() / 4).sum();
        }

        AssembledContext {
            sources,
            estimated_tokens: total_est,
            facts,
            assumptions,
        }
    }
}

impl Default for ContextAssembler {
    fn default() -> Self {
        Self::new()
    }
}
