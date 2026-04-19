use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::process::{Child, Command};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: String,
    pub shell: String,
    pub cwd: String,
    pub alive: bool,
}

pub struct TerminalSession {
    pub id: String,
    pub shell: String,
    pub cwd: PathBuf,
    child: Option<Child>,
}

impl TerminalSession {
    pub async fn spawn(cwd: &std::path::Path, shell: Option<&str>) -> anyhow::Result<Self> {
        let shell = shell
            .map(String::from)
            .unwrap_or_else(crate::shell::detect_shell);

        let id = Uuid::new_v4().to_string();

        let child = Command::new(&shell)
            .current_dir(cwd)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        Ok(Self {
            id,
            shell,
            cwd: cwd.to_path_buf(),
            child: Some(child),
        })
    }

    pub fn info(&self) -> SessionInfo {
        SessionInfo {
            id: self.id.clone(),
            shell: self.shell.clone(),
            cwd: self.cwd.display().to_string(),
            alive: self.child.is_some(),
        }
    }

    pub async fn write(&mut self, data: &[u8]) -> anyhow::Result<()> {
        if let Some(ref mut child) = self.child {
            use tokio::io::AsyncWriteExt;
            if let Some(ref mut stdin) = child.stdin {
                stdin.write_all(data).await?;
                stdin.flush().await?;
            }
        }
        Ok(())
    }

    pub async fn resize(&self, cols: u16, rows: u16) -> anyhow::Result<()> {
        // Resize requires portable-pty or similar; stub for now
        let _ = (cols, rows);
        Ok(())
    }

    pub async fn kill(&mut self) -> anyhow::Result<()> {
        if let Some(ref mut child) = self.child {
            child.kill().await?;
        }
        self.child = None;
        Ok(())
    }

    pub fn is_alive(&self) -> bool {
        self.child.is_some()
    }
}
