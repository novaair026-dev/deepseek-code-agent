use serde::{Deserialize, Serialize};

/// OpenAI 兼容聊天消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }
}

/// 工具定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: ToolFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFunction {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// 聊天完成请求
#[derive(Debug, Clone, Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
}

/// 流式响应块
#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChunkChoice>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChunkChoice {
    pub index: i32,
    pub delta: DeltaMessage,
    #[serde(default)]
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct DeltaMessage {
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub tool_calls: Option<Vec<DeltaToolCall>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeltaToolCall {
    pub index: i32,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(rename = "type", default)]
    pub tool_type: Option<String>,
    #[serde(default)]
    pub function: Option<DeltaToolCallFunction>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeltaToolCallFunction {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub arguments: Option<String>,
}

/// 非流式完整响应
#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompletionChoice {
    pub index: i32,
    pub message: ResponseMessage,
    #[serde(default)]
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseMessage {
    pub role: String,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: ToolCallFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String,
}

/// 解析后的流式增量内容
#[derive(Debug, Clone, Default)]
pub struct StreamDelta {
    pub content: String,
    pub tool_calls: Vec<ParsedToolCall>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ParsedToolCall {
    pub index: i32,
    pub id: String,
    pub name: String,
    pub arguments: String,
}

impl ChatCompletionChunk {
    pub fn to_delta(&self) -> StreamDelta {
        let mut delta = StreamDelta::default();
        for choice in &self.choices {
            if let Some(content) = &choice.delta.content {
                delta.content.push_str(content);
            }
            if let Some(calls) = &choice.delta.tool_calls {
                for call in calls {
                    delta.tool_calls.push(ParsedToolCall {
                        index: call.index,
                        id: call.id.clone().unwrap_or_default(),
                        name: call
                            .function
                            .as_ref()
                            .and_then(|f| f.name.clone())
                            .unwrap_or_default(),
                        arguments: call
                            .function
                            .as_ref()
                            .and_then(|f| f.arguments.clone())
                            .unwrap_or_default(),
                    });
                }
            }
            if choice.finish_reason.is_some() {
                delta.finish_reason = choice.finish_reason.clone();
            }
        }
        delta
    }
}
