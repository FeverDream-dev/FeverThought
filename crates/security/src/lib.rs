use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: String,
    pub action: String,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub details: String,
    pub approved: bool,
}

pub struct SecurityManager {
    audit_log: Vec<AuditEntry>,
}

impl Default for SecurityManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityManager {
    pub fn new() -> Self {
        Self {
            audit_log: Vec::new(),
        }
    }

    pub fn log_action(&mut self, entry: AuditEntry) {
        self.audit_log.push(entry);
    }

    pub fn audit_log(&self) -> &[AuditEntry] {
        &self.audit_log
    }

    pub fn should_prompt_for_upload(&self, privacy_mode: &str, is_screenshot: bool) -> bool {
        match privacy_mode {
            "strict" => true,
            "balanced" => is_screenshot,
            "open" => false,
            _ => true,
        }
    }

    pub fn redact_secrets(text: &str, patterns: &[&str]) -> String {
        let mut result = text.to_string();
        for pattern in patterns {
            result = result.replace(pattern, "[REDACTED]");
        }
        result
    }
}
