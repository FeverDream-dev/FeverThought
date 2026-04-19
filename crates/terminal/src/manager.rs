use crate::session::{SessionInfo, TerminalSession};
use std::collections::HashMap;
use std::path::Path;

pub struct TerminalManager {
    sessions: HashMap<String, TerminalSession>,
}

impl TerminalManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub async fn create_session(&mut self, cwd: &Path) -> anyhow::Result<String> {
        let session = TerminalSession::spawn(cwd, None).await?;
        let id = session.id.clone();
        self.sessions.insert(id.clone(), session);
        Ok(id)
    }

    pub fn list_sessions(&self) -> Vec<SessionInfo> {
        self.sessions.values().map(|s| s.info()).collect()
    }

    pub fn get_session(&mut self, id: &str) -> Option<&mut TerminalSession> {
        self.sessions.get_mut(id)
    }

    pub async fn kill_session(&mut self, id: &str) -> anyhow::Result<()> {
        if let Some(mut session) = self.sessions.remove(id) {
            session.kill().await?;
        }
        Ok(())
    }
}

impl Default for TerminalManager {
    fn default() -> Self {
        Self::new()
    }
}
