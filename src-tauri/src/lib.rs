mod agent;
mod commands;
mod db;
mod llm;
mod project;
mod security;
mod templates;
mod tools;

use db::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = AppState::new().expect("初始化应用状态失败");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::config::get_api_key_configured,
            commands::config::set_api_key,
            commands::project::create_project,
            commands::project::list_projects,
            commands::project::get_project,
            commands::project::delete_project,
            commands::chat::send_message,
            commands::chat::approve_plan,
            commands::plan::confirm_command,
            commands::greet,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
