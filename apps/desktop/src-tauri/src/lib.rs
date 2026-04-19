mod commands;

use feverthoth_core::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = AppState::new().expect("Failed to initialize app state");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_os::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::app_info,
            commands::workspace_open,
            commands::workspace_close,
            commands::workspace_current,
            commands::workspace_read_dir,
            commands::workspace_read_file,
            commands::workspace_write_file,
            commands::terminal_spawn,
            commands::terminal_write,
            commands::terminal_resize,
            commands::terminal_kill,
            commands::ai_chat,
            commands::ai_list_models,
            commands::ai_check_ollama,
            commands::settings_get,
            commands::settings_update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running FeverThoth IDE");
}
