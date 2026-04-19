use feverthoth_lsp::LspHost;
use feverthoth_mcp::McpManager;
use feverthoth_providers::registry::ProviderRegistry;
use feverthoth_settings::SettingsManager;
use feverthoth_workspace::WorkspaceManager;
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<RwLock<SettingsManager>>,
    pub workspace: Arc<RwLock<WorkspaceManager>>,
    pub providers: Arc<RwLock<ProviderRegistry>>,
    pub lsp_host: Arc<RwLock<LspHost>>,
    pub mcp: Arc<RwLock<McpManager>>,
}

impl AppState {
    pub fn new() -> anyhow::Result<Self> {
        let settings = SettingsManager::load()?;
        let workspace = WorkspaceManager::new();
        let providers = ProviderRegistry::new();
        let lsp_host = LspHost::new();
        let mcp = McpManager::new();

        Ok(Self {
            settings: Arc::new(RwLock::new(settings)),
            workspace: Arc::new(RwLock::new(workspace)),
            providers: Arc::new(RwLock::new(providers)),
            lsp_host: Arc::new(RwLock::new(lsp_host)),
            mcp: Arc::new(RwLock::new(mcp)),
        })
    }
}
