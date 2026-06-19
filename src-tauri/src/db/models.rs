use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 项目类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectType {
    Desktop,
    Website,
}

impl std::fmt::Display for ProjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectType::Desktop => write!(f, "desktop"),
            ProjectType::Website => write!(f, "website"),
        }
    }
}

impl std::str::FromStr for ProjectType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "desktop" => Ok(ProjectType::Desktop),
            "website" => Ok(ProjectType::Website),
            _ => Err(format!("未知的项目类型: {}", s)),
        }
    }
}

/// 计划状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanStatus {
    Draft,
    Confirmed,
    Rejected,
}

impl std::fmt::Display for PlanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanStatus::Draft => write!(f, "draft"),
            PlanStatus::Confirmed => write!(f, "confirmed"),
            PlanStatus::Rejected => write!(f, "rejected"),
        }
    }
}

impl std::str::FromStr for PlanStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "draft" => Ok(PlanStatus::Draft),
            "confirmed" => Ok(PlanStatus::Confirmed),
            "rejected" => Ok(PlanStatus::Rejected),
            _ => Err(format!("未知的计划状态: {}", s)),
        }
    }
}

/// 项目记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub project_type: ProjectType,
    pub path: String,
    pub plan_json: Option<String>,
    pub plan_status: Option<PlanStatus>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 会话记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub project_id: String,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 消息角色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

impl std::fmt::Display for MessageRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageRole::System => write!(f, "system"),
            MessageRole::User => write!(f, "user"),
            MessageRole::Assistant => write!(f, "assistant"),
            MessageRole::Tool => write!(f, "tool"),
        }
    }
}

impl std::str::FromStr for MessageRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "system" => Ok(MessageRole::System),
            "user" => Ok(MessageRole::User),
            "assistant" => Ok(MessageRole::Assistant),
            "tool" => Ok(MessageRole::Tool),
            _ => Err(format!("未知的消息角色: {}", s)),
        }
    }
}

/// 消息记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub session_id: String,
    pub role: MessageRole,
    pub content: Option<String>,
    pub tool_calls: Option<String>,
    pub tool_result: Option<String>,
    pub metadata: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// 创建项目请求
#[derive(Debug, Clone, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub project_type: ProjectType,
}

/// 更新项目计划请求
#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePlanRequest {
    pub project_id: String,
    pub plan_json: String,
    pub plan_status: PlanStatus,
}
