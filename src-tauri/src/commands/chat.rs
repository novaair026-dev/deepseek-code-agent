use std::sync::{Arc, Mutex};

use tauri::State;

use crate::agent::{
    handle_message, handle_plan_approval, AgentResponse, PlanApprovalRequest, SendMessageRequest,
    SendMessageResponse,
};
use crate::db::AppState;

/// 发送消息给 Agent
#[tauri::command]
pub async fn send_message(
    state: State<'_, Arc<Mutex<AppState>>>,
    req: SendMessageRequest,
) -> Result<SendMessageResponse, String> {
    let state = state.inner().clone();
    handle_message(state, req).await.map_err(|e| e.to_string())
}

/// 确认/拒绝/修改开发计划
#[tauri::command]
pub async fn approve_plan(
    state: State<'_, Arc<Mutex<AppState>>>,
    req: PlanApprovalRequest,
) -> Result<AgentResponse, String> {
    let state = state.inner().clone();
    handle_plan_approval(state, req).await.map_err(|e| e.to_string())
}
