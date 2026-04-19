use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub root_path: PathBuf,
    pub open_files: Vec<String>,
    pub recent_files: Vec<String>,
    pub last_opened: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub children: Option<Vec<FileEntry>>,
    pub size: Option<u64>,
}

pub struct WorkspaceManager {
    current: Option<Workspace>,
    recent: Vec<Workspace>,
}

impl WorkspaceManager {
    pub fn new() -> Self {
        Self {
            current: None,
            recent: Vec::new(),
        }
    }

    pub fn open(&mut self, path: PathBuf) -> anyhow::Result<&Workspace> {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Untitled".to_string());

        let workspace = Workspace {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            root_path: path,
            open_files: Vec::new(),
            recent_files: Vec::new(),
            last_opened: chrono::Utc::now().to_rfc3339(),
        };

        self.current = Some(workspace);
        Ok(self.current.as_ref().unwrap())
    }

    pub fn close(&mut self) {
        self.current = None;
    }

    pub fn current(&self) -> Option<&Workspace> {
        self.current.as_ref()
    }

    pub fn read_dir(&self, path: &Path) -> anyhow::Result<Vec<FileEntry>> {
        let mut entries: Vec<FileEntry> = Vec::new();
        let dir = std::fs::read_dir(path)?;

        for entry in dir {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let name = entry.file_name().to_string_lossy().to_string();
            let path = entry.path();

            if name.starts_with('.') {
                continue;
            }

            entries.push(FileEntry {
                name,
                path,
                is_dir: metadata.is_dir(),
                children: None,
                size: if metadata.is_file() {
                    Some(metadata.len())
                } else {
                    None
                },
            });
        }

        entries.sort_by(|a, b| {
            b.is_dir
                .cmp(&a.is_dir)
                .then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });

        Ok(entries)
    }

    pub fn read_file(&self, path: &Path) -> anyhow::Result<String> {
        Ok(std::fs::read_to_string(path)?)
    }

    pub fn write_file(&self, path: &Path, content: &str) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        Ok(std::fs::write(path, content)?)
    }

    pub fn add_recent(&mut self, workspace: Workspace) {
        self.recent.retain(|w| w.root_path != workspace.root_path);
        self.recent.insert(0, workspace);
        self.recent.truncate(10);
    }

    pub fn recent_workspaces(&self) -> &[Workspace] {
        &self.recent
    }
}
