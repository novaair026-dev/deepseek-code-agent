use crate::db::models::ProjectType;
use crate::llm::{ChatCompletionRequest, ChatMessage, DeepSeekClient};

use super::types::{AgentContext, AgentResponse, DevelopmentPlan};

fn planning_prompt(project_type: ProjectType) -> String {
    let tech_stack = match project_type {
        ProjectType::Desktop => "Rust + Tauri v2 + React + TypeScript + Tailwind CSS + SQLite",
        ProjectType::Website => "Rust + Tauri v2（桌面管理端）+ Axum（Web 后端）+ React/TypeScript/Tailwind（前端）+ SQLite",
    };

    format!(
        r#"你是 DeepSeek Code Agent 的开发规划师。请根据用户需求，为以下技术栈制定详细的开发计划：

技术栈：{}

请输出以下 JSON 格式：
{{
  "summary": "项目一句话描述",
  "project_type": "desktop 或 website",
  "stages": [
    {{
      "name": "阶段名称",
      "description": "阶段说明",
      "tasks": ["具体任务1", "具体任务2"]
    }}
  ],
  "estimated_files": ["src/main.rs", "src/App.tsx", ...],
  "dependencies": ["依赖1", "依赖2"],
  "run_commands": ["cargo tauri dev", ...]
}}

要求：
1. 计划要具体、可执行，每个阶段包含明确的任务。
2. estimated_files 列出预计创建或修改的关键文件。
3. dependencies 列出需要安装的依赖或 crate。
4. run_commands 列出开发阶段需要运行的命令。
5. 只输出 JSON，不要输出其他内容。"#,
        tech_stack
    )
}

pub async fn plan(
    client: &DeepSeekClient,
    ctx: &AgentContext,
) -> anyhow::Result<AgentResponse> {
    let prompt = planning_prompt(ctx.project_type);
    let messages = ctx.to_chat_messages(&prompt);

    let request = ChatCompletionRequest {
        model: "deepseek-v4-pro".to_string(),
        messages,
        tools: None,
        tool_choice: None,
        stream: false,
        temperature: Some(0.3),
        max_tokens: None,
    };

    let response = client.chat_complete(request).await?;
    let content = response
        .choices
        .into_iter()
        .next()
        .and_then(|c| c.message.content)
        .unwrap_or_default();

    let json_str = extract_json(&content);
    let mut plan: DevelopmentPlan = serde_json::from_str(json_str).map_err(|e| {
        anyhow::anyhow!("解析开发计划失败: {} 内容: {}", e, content)
    })?;

    // 确保 project_type 与当前项目一致
    plan.project_type = ctx.project_type;

    Ok(AgentResponse::Plan { plan })
}

fn extract_json(text: &str) -> &str {
    if let Some(start) = text.find('{') {
        if let Some(end) = text.rfind('}') {
            return &text[start..=end];
        }
    }
    text
}

/// 根据用户修改意见重新生成计划
pub async fn replan(
    client: &DeepSeekClient,
    ctx: &AgentContext,
    feedback: &str,
    previous_plan: &DevelopmentPlan,
) -> anyhow::Result<AgentResponse> {
    let prompt = format!(
        "{}\n\n用户对上次的计划有如下反馈，请根据反馈重新制定开发计划，输出相同 JSON 格式。\n反馈：{}",
        planning_prompt(ctx.project_type),
        feedback
    );

    let mut messages = ctx.to_chat_messages(&prompt);
    messages.push(ChatMessage::user(format!(
        "之前的计划：{}\n\n用户反馈：{}\n\n请重新制定计划。",
        serde_json::to_string(previous_plan).unwrap_or_default(),
        feedback
    )));

    let request = ChatCompletionRequest {
        model: "deepseek-v4-pro".to_string(),
        messages,
        tools: None,
        tool_choice: None,
        stream: false,
        temperature: Some(0.3),
        max_tokens: None,
    };

    let response = client.chat_complete(request).await?;
    let content = response
        .choices
        .into_iter()
        .next()
        .and_then(|c| c.message.content)
        .unwrap_or_default();

    let json_str = extract_json(&content);
    let mut plan: DevelopmentPlan = serde_json::from_str(json_str).map_err(|e| {
        anyhow::anyhow!("解析修改后的开发计划失败: {} 内容: {}", e, content)
    })?;
    plan.project_type = ctx.project_type;

    Ok(AgentResponse::Plan { plan })
}
