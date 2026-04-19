pub mod client;
pub mod handlers;
pub mod transport;

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub use client::LspClient;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ServerId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspServerConfig {
    pub language_id: String,
    pub command: String,
    pub args: Vec<String>,
    pub extensions: Vec<String>,
    pub detect_files: Vec<String>,
    pub install_hint: String,
}

impl LspServerConfig {
    pub fn builtin_configs() -> HashMap<String, Self> {
        let mut configs = HashMap::new();

        configs.insert(
            "rust".into(),
            Self {
                language_id: "rust".into(),
                command: "rust-analyzer".into(),
                args: vec![],
                extensions: vec!["rs".into()],
                detect_files: vec!["Cargo.toml".into()],
                install_hint: "rustup component add rust-analyzer".into(),
            },
        );

        configs.insert(
            "typescript".into(),
            Self {
                language_id: "typescript".into(),
                command: "typescript-language-server".into(),
                args: vec!["--stdio".into()],
                extensions: vec!["ts".into(), "tsx".into(), "js".into(), "jsx".into()],
                detect_files: vec!["tsconfig.json".into(), "package.json".into()],
                install_hint: "npm install -g typescript-language-server typescript".into(),
            },
        );

        configs.insert(
            "python".into(),
            Self {
                language_id: "python".into(),
                command: "pylsp".into(),
                args: vec![],
                extensions: vec!["py".into()],
                detect_files: vec!["pyproject.toml".into(), "setup.py".into()],
                install_hint: "pip install python-lsp-server".into(),
            },
        );

        configs.insert(
            "go".into(),
            Self {
                language_id: "go".into(),
                command: "gopls".into(),
                args: vec!["serve".into()],
                extensions: vec!["go".into()],
                detect_files: vec!["go.mod".into()],
                install_hint: "go install golang.org/x/tools/gopls@latest".into(),
            },
        );

        configs.insert(
            "c".into(),
            Self {
                language_id: "c".into(),
                command: "clangd".into(),
                args: vec![],
                extensions: vec!["c".into(), "h".into(), "cpp".into(), "hpp".into()],
                detect_files: vec!["compile_commands.json".into(), "Makefile".into()],
                install_hint: "Install clangd from your system package manager".into(),
            },
        );

        configs
    }
}

pub struct LspHost {
    servers: RwLock<HashMap<ServerId, Arc<LspClient>>>,
    configs: HashMap<String, LspServerConfig>,
}

impl Default for LspHost {
    fn default() -> Self {
        Self::new()
    }
}

impl LspHost {
    pub fn new() -> Self {
        Self {
            servers: RwLock::new(HashMap::new()),
            configs: LspServerConfig::builtin_configs(),
        }
    }

    pub fn config_for_extension(&self, ext: &str) -> Option<&LspServerConfig> {
        self.configs
            .values()
            .find(|c| c.extensions.iter().any(|e| e == ext))
    }

    pub fn register_server(&self, client: Arc<LspClient>) -> ServerId {
        let id = ServerId(Uuid::new_v4().to_string());
        self.servers.write().insert(id.clone(), client);
        id
    }

    pub fn get_server(&self, id: &ServerId) -> Option<Arc<LspClient>> {
        self.servers.read().get(id).cloned()
    }

    pub fn list_servers(&self) -> Vec<(ServerId, String)> {
        self.servers
            .read()
            .iter()
            .map(|(id, c)| (id.clone(), c.language_id().to_string()))
            .collect()
    }
}
