use serde::{Deserialize, Serialize};

use crate::db::models::{Message, ProjectType};
use crate::llm::ChatMessage;
use crate::tools::ToolResult;

/// Agent 会话上下文
#[derive(Debug, Clone)]
pub struct AgentContext {
    pub project_id: String,
    pub project_path: String,
    pub project_type: ProjectType,
    pub project_name: String,
    pub description: Option<String>,
    pub messages: Vec<Message>,
}

impl AgentContext {
    pub fn to_chat_messages(&self, system_prompt: &str) -> Vec<ChatMessage> {
        let mut msgs = vec![ChatMessage::system(system_prompt)];
        for m in &self.messages {
            let role = match m.role {
                crate::db::models::MessageRole::System => "system",
                crate::db::models::MessageRole::User => "user",
                crate::db::models::MessageRole::Assistant => "assistant",
                crate::db::models::MessageRole::Tool => "tool",
            };
            let content = m
                .content
                .clone()
                .unwrap_or_else(|| m.tool_result.clone().unwrap_or_default());
            msgs.push(ChatMessage {
                role: role.to_string(),
                content,
            });
        }
        msgs
    }
}

/// 开发计划
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentPlan {
    pub summary: String,
    pub project_type: ProjectType,
    pub stages: Vec<PlanStage>,
    pub estimated_files: Vec<String>,
    pub dependencies: Vec<String>,
    pub run_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStage {
    pub name: String,
    pub description: String,
    pub tasks: Vec<String>,
}

/// Agent 对用户输入的响应
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AgentResponse {
    /// 需要用户澄清需求
    Clarification {
        questions: Vec<String>,
    },
    /// 需要用户确认开发计划
    Plan {
        plan: DevelopmentPlan,
    },
    /// 普通消息
    Message {
        content: String,
    },
    /// 执行状态更新
    Progress {
        step: String,
        detail: String,
        tool_result: Option<ToolResult>,
    },
    /// 执行完成
    Done {
        summary: String,
        next_steps: Vec<String>,
    },
    /// 需要确认命令
    CommandConfirmation {
        command: String,
        description: String,
    },
    /// 错误
    Error {
        message: String,
    },
}

/// 计划确认操作
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanAction {
    Confirm,
    Reject,
    Modify,
}

/// 计划确认请求
#[derive(Debug, Clone, Deserialize)]
pub struct PlanApprovalRequest {
    pub project_id: String,
    pub action: PlanAction,
    pub modified_plan: Option<DevelopmentPlan>,
    pub feedback: Option<String>,
}

/// 执行结果
#[derive(Debug, Clone, Serialize)]
pub struct ExecutionResult {
    pub completed_steps: Vec<ExecutionStep>,
    pub pending_command: Option<PendingCommand>,
    #[serde(flatten)]
    pub final_response: AgentResponse,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExecutionStep {
    pub description: String,
    pub tool_result: ToolResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingCommand {
    pub command: String,
    pub description: String,
}
