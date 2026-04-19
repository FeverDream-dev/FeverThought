//! Safety and permissioning system for agent actions.
//!
//! Implements the permission classes, default policies, and risk tiers
//! from the plan specification.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Permission classes that agents may request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermissionClass {
    /// Read workspace files.
    ReadWorkspace,
    /// Write workspace files.
    WriteWorkspace,
    /// Run a shell command.
    RunShellCommand,
    /// Run a browser MCP tool.
    RunBrowserMcpTool,
    /// Send data to a cloud AI provider.
    SendToCloudProvider,
    /// Upload a raw screenshot to the cloud.
    UploadRawScreenshot,
    /// Commit changes to git.
    CommitChanges,
    /// Push changes to remote.
    PushChanges,
}

impl PermissionClass {
    /// All permission class variants.
    pub fn all() -> Vec<Self> {
        vec![
            Self::ReadWorkspace,
            Self::WriteWorkspace,
            Self::RunShellCommand,
            Self::RunBrowserMcpTool,
            Self::SendToCloudProvider,
            Self::UploadRawScreenshot,
            Self::CommitChanges,
            Self::PushChanges,
        ]
    }
}

/// Whether a permission is allowed, denied, or requires user approval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionPolicy {
    /// Allowed without prompting the user.
    Allowed,
    /// Denied outright; the action cannot proceed.
    Denied,
    /// Requires user review and explicit approval before proceeding.
    RequiresApproval,
}

/// Risk tier for a proposed agent action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskTier {
    /// Single-file edit, local context only.
    Low,
    /// Multi-file refactor, shell test commands.
    Medium,
    /// Deletions, dependency changes, git operations, external uploads.
    High,
}

impl RiskTier {
    /// Determine risk tier from action characteristics.
    pub fn from_action(
        file_count: usize,
        has_deletions: bool,
        has_shell: bool,
        has_external: bool,
        has_git: bool,
    ) -> Self {
        if has_deletions || has_external || has_git {
            return Self::High;
        }
        if file_count > 1 || has_shell {
            return Self::Medium;
        }
        Self::Low
    }
}

/// Manages permission policies for agent actions.
#[derive(Debug, Clone)]
pub struct PermissionManager {
    policies: HashMap<PermissionClass, PermissionPolicy>,
}

impl PermissionManager {
    /// Create a new manager with default policies from the plan:
    /// - read: allowed within workspace
    /// - write: allowed after plan/approval
    /// - shell: require review
    /// - raw screenshot cloud: denied by default
    /// - git push: always requires explicit user action
    pub fn new() -> Self {
        let mut policies = HashMap::new();
        policies.insert(PermissionClass::ReadWorkspace, PermissionPolicy::Allowed);
        policies.insert(
            PermissionClass::WriteWorkspace,
            PermissionPolicy::RequiresApproval,
        );
        policies.insert(
            PermissionClass::RunShellCommand,
            PermissionPolicy::RequiresApproval,
        );
        policies.insert(
            PermissionClass::RunBrowserMcpTool,
            PermissionPolicy::RequiresApproval,
        );
        policies.insert(
            PermissionClass::SendToCloudProvider,
            PermissionPolicy::RequiresApproval,
        );
        policies.insert(
            PermissionClass::UploadRawScreenshot,
            PermissionPolicy::Denied,
        );
        policies.insert(
            PermissionClass::CommitChanges,
            PermissionPolicy::RequiresApproval,
        );
        policies.insert(
            PermissionClass::PushChanges,
            PermissionPolicy::RequiresApproval,
        );
        Self { policies }
    }

    /// Check whether a permission is allowed.
    pub fn check(&self, class: PermissionClass) -> PermissionPolicy {
        self.policies
            .get(&class)
            .copied()
            .unwrap_or(PermissionPolicy::RequiresApproval)
    }

    /// Grant a permission (e.g. after user approval).
    pub fn grant(&mut self, class: PermissionClass) {
        self.policies.insert(class, PermissionPolicy::Allowed);
    }

    /// Deny a permission.
    pub fn deny(&mut self, class: PermissionClass) {
        self.policies.insert(class, PermissionPolicy::Denied);
    }

    /// Override a specific policy.
    pub fn set_policy(&mut self, class: PermissionClass, policy: PermissionPolicy) {
        self.policies.insert(class, policy);
    }

    /// Build a human-readable description of a proposed action for the UX.
    ///
    /// The user must know:
    /// - what action is proposed
    /// - why it is needed
    /// - what data leaves the machine, if any
    pub fn describe_action(
        class: PermissionClass,
        description: &str,
        reason: &str,
        data_leaves_machine: bool,
    ) -> ActionDescription {
        ActionDescription {
            action: format!("{:?}", class),
            description: description.to_string(),
            reason: reason.to_string(),
            data_leaves_machine,
        }
    }
}

impl Default for PermissionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Human-readable description of a proposed agent action for the permissions UX.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionDescription {
    /// What action is proposed.
    pub action: String,
    /// Human-readable description.
    pub description: String,
    /// Why the action is needed.
    pub reason: String,
    /// Whether data will leave the machine.
    pub data_leaves_machine: bool,
}
