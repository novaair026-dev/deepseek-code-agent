pub mod clarifier;
pub mod executor;
pub mod planner;
pub mod types;

use std::path::Path;
use std::sync::{Arc, Mutex};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use crate::commands::config::get_api_key_inner;
use crate::db::models::MessageRole;
use crate::db::repo::{MessageRepo, ProjectRepo, SessionRepo};
use crate::db::AppState;
use crate::llm::DeepSeekClient;

use self::types::*;

pub use self::types::{
    AgentResponse, DevelopmentPlan, PlanAction, PlanApprovalRequest,
};

/// 发送消息请求
#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub project_id: String,
    pub session_id: Option<String>,
    pub content: String,
    pub referenced_files: Option<Vec<String>>,
}

/// 发送消息响应
#[derive(Debug, Serialize)]
pub struct SendMessageResponse {
    pub session_id: String,
    pub response: AgentResponse,
}

/// 加载 Agent 上下文
fn load_context(
    state: &AppState,
    project_id: &str,
) -> anyhow::Result<AgentContext> {
    let project = ProjectRepo::get_by_id(&state.db, project_id)?
        .context("项目不存在")?;

    Ok(AgentContext {
        project_id: project.id.clone(),
        project_path: project.path.clone(),
        project_type: project.project_type,
        project_name: project.name.clone(),
        description: project.description.clone(),
        messages: Vec::new(),
    })
}

/// 同步准备上下文和会话，避免 MutexGuard 跨 await
fn prepare_message_context(
    state: &Arc<Mutex<AppState>>,
    req: &SendMessageRequest,
) -> anyhow::Result<(AgentContext, String, Option<crate::db::models::Project>)> {
    let guard = state.lock().map_err(|e| anyhow::anyhow!("锁失败: {}", e))?;
    let mut context = load_context(&guard, &req.project_id)?;

    let session_id = match &req.session_id {
        Some(id) => id.clone(),
        None => {
            let session = SessionRepo::create(
                &guard.db,
                &req.project_id,
                Some(&req.content[..req.content.len().min(20)]),
            )?;
            session.id
        }
    };

    context.messages = MessageRepo::list_by_session(&guard.db, &session_id)?;

    let user_content = if let Some(files) = &req.referenced_files {
        format!("{}\n\n引用的文件：{}", req.content, files.join(", "))
    } else {
        req.content.clone()
    };

    MessageRepo::create(
        &guard.db,
        &session_id,
        MessageRole::User,
        Some(&user_content),
        None,
        None,
        None,
    )?;

    context.messages = MessageRepo::list_by_session(&guard.db, &session_id)?;
    let project = ProjectRepo::get_by_id(&guard.db, &req.project_id)?;
    Ok((context, session_id, project))
}

/// 处理用户消息
pub async fn handle_message(
    state: Arc<Mutex<AppState>>,
    req: SendMessageRequest,
) -> anyhow::Result<SendMessageResponse> {
    let api_key = get_api_key_inner()?.context("未配置 DeepSeek API Key")?;
    let client = Arc::new(DeepSeekClient::new(api_key));

    let (context, session_id, project) = prepare_message_context(&state, &req)?;

    let response = if let Some(project) = project {
        match project.plan_status {
            Some(crate::db::models::PlanStatus::Confirmed) => {
                // 计划已确认，进入执行阶段
                let executor = executor::Executor::new(
                    client.clone(),
                    state.clone(),
                    context.clone(),
                    session_id.clone(),
                );
                let plan: DevelopmentPlan = serde_json::from_str(
                    &project.plan_json.unwrap_or_else(|| "{}".to_string()),
                )?;
                let result = executor.execute_plan(&plan).await?;
                result.final_response
            }
            _ => {
                // 未确认计划，先进行澄清和规划
                let clarification = clarifier::clarify(&client, &context, &req.content).await?;
                match clarification {
                    AgentResponse::Clarification { .. } => clarification,
                    _ => {
                        // 需求已足够，生成计划
                        planner::plan(&client, &context).await?
                    }
                }
            }
        }
    } else {
        AgentResponse::Error {
            message: "项目不存在".to_string(),
        }
    };

    // 保存 assistant 消息
    let assistant_content = match &response {
        AgentResponse::Message { content } => Some(content.clone()),
        AgentResponse::Done { summary, .. } => Some(summary.clone()),
        AgentResponse::Error { message } => Some(message.clone()),
        _ => None,
    };

    let metadata = serde_json::to_string(&response).ok();
    {
        let guard = state.lock().map_err(|e| anyhow::anyhow!("锁失败: {}", e))?;
        MessageRepo::create(
            &guard.db,
            &session_id,
            MessageRole::Assistant,
            assistant_content.as_deref(),
            None,
            None,
            metadata.as_deref(),
        )?;
    }

    Ok(SendMessageResponse {
        session_id,
        response,
    })
}

/// 处理计划确认
pub async fn handle_plan_approval(
    state: Arc<Mutex<AppState>>,
    req: PlanApprovalRequest,
) -> anyhow::Result<AgentResponse> {
    let api_key = get_api_key_inner()?.context("未配置 DeepSeek API Key")?;
    let client = Arc::new(DeepSeekClient::new(api_key));

    let (context, project) = {
        let guard = state.lock().map_err(|e| anyhow::anyhow!("锁失败: {}", e))?;
        let ctx = load_context(&guard, &req.project_id)?;
        let project = ProjectRepo::get_by_id(&guard.db, &req.project_id)?
            .context("项目不存在")?;
        (ctx, project)
    };

    match req.action {
        PlanAction::Confirm => {
            let plan = req.modified_plan.context("确认计划时缺少计划内容")?;
            let plan_json = serde_json::to_string(&plan)?;
            {
                let guard = state.lock().map_err(|e| anyhow::anyhow!("锁失败: {}", e))?;
                crate::db::repo::ProjectRepo::update_plan(
                    &guard.db,
                    &crate::db::models::UpdatePlanRequest {
                        project_id: req.project_id.clone(),
                        plan_json,
                        plan_status: crate::db::models::PlanStatus::Confirmed,
                    },
                )?;
            }

            // 开始执行
            let session = {
                let guard = state.lock().map_err(|e| anyhow::anyhow!("锁失败: {}", e))?;
                SessionRepo::create(&guard.db, &req.project_id, Some("计划执行"))?
            };

            let executor = executor::Executor::new(client, state.clone(), context, session.id.clone());
            let result = executor.execute_plan(&plan).await?;
            Ok(result.final_response)
        }
        PlanAction::Reject => {
            // 更新计划状态为 rejected
            {
                let guard = state.lock().map_err(|e| anyhow::anyhow!("锁失败: {}", e))?;
                crate::db::repo::ProjectRepo::update_plan(
                    &guard.db,
                    &crate::db::models::UpdatePlanRequest {
                        project_id: req.project_id.clone(),
                        plan_json: project.plan_json.unwrap_or_else(|| "{}".to_string()),
                        plan_status: crate::db::models::PlanStatus::Rejected,
                    },
                )?;
            }
            Ok(AgentResponse::Message {
                content: format!(
                    "计划已拒绝。请告诉我你希望我如何调整：{}",
                    req.feedback.unwrap_or_default()
                ),
            })
        }
        PlanAction::Modify => {
            let feedback = req.feedback.unwrap_or_else(|| "请调整计划".to_string());
            let previous_plan = req.modified_plan.context("修改计划时缺少原计划")?;
            planner::replan(&client, &context, &feedback, &previous_plan).await
        }
    }
}

/// 确认并执行待处理的高风险命令
pub async fn confirm_pending_command(
    state: Arc<Mutex<AppState>>,
    project_id: String,
    session_id: String,
    command: String,
) -> anyhow::Result<AgentResponse> {
    let project_path = {
        let guard = state.lock().map_err(|e| anyhow::anyhow!("锁失败: {}", e))?;
        let project = ProjectRepo::get_by_id(&guard.db, &project_id)?
            .context("项目不存在")?;
        project.path
    };

    let project_dir = Path::new(&project_path);
    let cmd_tool = crate::tools::cmd_tool::CommandTool::new(project_dir);
    let result = cmd_tool.execute_command(&command).await;

    // 保存结果
    {
        let guard = state.lock().map_err(|e| anyhow::anyhow!("锁失败: {}", e))?;
        MessageRepo::create(
            &guard.db,
            &session_id,
            MessageRole::Tool,
            None,
            None,
            Some(&serde_json::to_string(&result)?),
            None,
        )?;
    }

    if result.success {
        Ok(AgentResponse::Progress {
            step: command,
            detail: result.output.clone(),
            tool_result: Some(result),
        })
    } else {
        Ok(AgentResponse::Error {
            message: result.error.clone().unwrap_or_else(|| "命令执行失败".to_string()),
        })
    }
}
