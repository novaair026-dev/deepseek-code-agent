pub mod chat;
pub mod config;
pub mod plan;
pub mod project;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
