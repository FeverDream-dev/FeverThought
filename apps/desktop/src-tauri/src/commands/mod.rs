use feverthoth_core::app_info as get_app_info;
use feverthoth_core::state::AppState;
use tauri::State;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Welcome to FeverThoth IDE, {}!", name)
}

#[tauri::command]
pub fn app_info() -> feverthoth_core::AppInfo {
    get_app_info()
}

#[tauri::command]
pub async fn workspace_open(
    state: State<'_, AppState>,
    path: String,
) -> Result<feverthoth_workspace::Workspace, String> {
    let mut ws = state.workspace.write();
    ws.open(std::path::PathBuf::from(&path))
        .cloned()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn workspace_close(state: State<'_, AppState>) -> Result<(), String> {
    state.workspace.write().close();
    Ok(())
}

#[tauri::command]
pub async fn workspace_current(
    state: State<'_, AppState>,
) -> Result<Option<feverthoth_workspace::Workspace>, String> {
    Ok(state.workspace.read().current().cloned())
}

#[tauri::command]
pub async fn workspace_read_dir(
    state: State<'_, AppState>,
    path: String,
) -> Result<Vec<feverthoth_workspace::FileEntry>, String> {
    let ws = state.workspace.read();
    ws.read_dir(std::path::Path::new(&path))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn workspace_read_file(
    state: State<'_, AppState>,
    path: String,
) -> Result<String, String> {
    let ws = state.workspace.read();
    ws.read_file(std::path::Path::new(&path))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn workspace_write_file(
    state: State<'_, AppState>,
    path: String,
    content: String,
) -> Result<(), String> {
    let ws = state.workspace.read();
    ws.write_file(std::path::Path::new(&path), &content)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn terminal_spawn(
    _state: State<'_, AppState>,
    _id: String,
    _cols: u16,
    _rows: u16,
) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn terminal_write(
    _state: State<'_, AppState>,
    _id: String,
    _data: String,
) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn terminal_resize(
    _state: State<'_, AppState>,
    _id: String,
    _cols: u16,
    _rows: u16,
) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn terminal_kill(_state: State<'_, AppState>, _id: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn ai_chat(_state: State<'_, AppState>, _message: String) -> Result<String, String> {
    Ok("AI chat not yet connected".to_string())
}

#[tauri::command]
pub fn ai_list_models(state: State<'_, AppState>) -> Vec<feverthoth_providers::ModelInfo> {
    let providers = state.providers.read();
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(providers.all_models())
    })
}

#[tauri::command]
pub fn ai_check_ollama(state: State<'_, AppState>) -> bool {
    let providers = state.providers.read();
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            if let Some(provider) =
                providers.get(&feverthoth_providers::ProviderId("ollama".to_string()))
            {
                provider.is_available().await
            } else {
                false
            }
        })
    })
}

#[tauri::command]
pub async fn settings_get(
    state: State<'_, AppState>,
) -> Result<feverthoth_settings::Settings, String> {
    let settings = state.settings.read();
    Ok(settings.get().clone())
}

#[tauri::command]
pub async fn settings_update(
    state: State<'_, AppState>,
    settings: feverthoth_settings::Settings,
) -> Result<(), String> {
    let mut mgr = state.settings.write();
    mgr.update(settings).map_err(|e| e.to_string())
}
