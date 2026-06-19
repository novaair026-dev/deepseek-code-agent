use crate::llm::{ChatCompletionRequest, ChatMessage, DeepSeekClient};

use super::types::{AgentContext, AgentResponse};

const CLARIFY_PROMPT: &str = r#"你是 DeepSeek Code Agent 的需求分析师。你的任务是判断用户的需求是否足够清晰，能否直接制定开发计划。

如果信息足够，请回复 JSON: {"sufficient": true, "questions": []}
如果信息不足，请回复 JSON: {"sufficient": false, "questions": ["问题1", "问题2", ...]}

需要澄清的典型情况：
- 用户想要的功能边界不清楚
- 目标用户或使用场景不明确
- 技术偏好未说明（但你可以主动建议）
- 样式、布局、交互细节缺失
- 数据来源或持久化需求不明确

请只输出 JSON，不要输出其他内容。"#;

pub async fn clarify(
    client: &DeepSeekClient,
    ctx: &AgentContext,
    user_input: &str,
) -> anyhow::Result<AgentResponse> {
    let mut messages = ctx.to_chat_messages(CLARIFY_PROMPT);
    messages.push(ChatMessage::user(format!(
        "用户最新输入：{}\n\n请判断需求是否清晰，并按要求输出 JSON。",
        user_input
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
    let result: ClarifyResult = serde_json::from_str(json_str).map_err(|e| {
        anyhow::anyhow!("解析澄清结果失败: {} 内容: {}", e, content)
    })?;

    if result.sufficient || result.questions.is_empty() {
        Ok(AgentResponse::Message {
            content: "需求已明确，接下来为你制定开发计划。".to_string(),
        })
    } else {
        Ok(AgentResponse::Clarification {
            questions: result.questions,
        })
    }
}

#[derive(Debug, serde::Deserialize)]
struct ClarifyResult {
    #[serde(default)]
    sufficient: bool,
    #[serde(default)]
    questions: Vec<String>,
}

fn extract_json(text: &str) -> &str {
    if let Some(start) = text.find('{') {
        if let Some(end) = text.rfind('}') {
            return &text[start..=end];
        }
    }
    text
}
