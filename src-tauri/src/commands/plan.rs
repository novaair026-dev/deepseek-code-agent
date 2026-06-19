use std::sync::{Arc, Mutex};

use tauri::State;

use crate::agent::{confirm_pending_command, AgentResponse};
use crate::db::AppState;

/// 确认执行高风险命令
#[tauri::command]
pub async fn confirm_command(
    state: State<'_, Arc<Mutex<AppState>>>,
    project_id: String,
    session_id: String,
    command: String,
) -> Result<AgentResponse, String> {
    let state = state.inner().clone();
    confirm_pending_command(state, project_id, session_id, command)
        .await
        .map_err(|e| e.to_string())
}
