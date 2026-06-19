use anyhow::{Context, Result};
use futures::StreamExt;
use reqwest_eventsource::{Event, EventSource};
use serde_json::json;

use super::types::*;

const DEFAULT_MODEL: &str = "deepseek-v4-pro";
const API_BASE: &str = "https://api.deepseek.com";

#[derive(Debug, Clone)]
pub struct DeepSeekClient {
    api_key: String,
    client: reqwest::Client,
    model: String,
}

impl DeepSeekClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: reqwest::Client::new(),
            model: DEFAULT_MODEL.to_string(),
        }
    }

    /// 发送非流式请求，返回完整响应
    pub async fn chat_complete(&self, request: ChatCompletionRequest) -> Result<ChatCompletionResponse> {
        let url = format!("{}/chat/completions", API_BASE);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("发送 DeepSeek 请求失败")?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("DeepSeek API 错误: {}", text);
        }

        let completion = response
            .json::<ChatCompletionResponse>()
            .await
            .context("解析 DeepSeek 响应失败")?;
        Ok(completion)
    }

    /// 发送流式请求，通过回调返回增量内容
    pub async fn chat_complete_stream<F>(
        &self,
        request: ChatCompletionRequest,
        mut on_delta: F,
    ) -> Result<StreamDelta>
    where
        F: FnMut(StreamDelta),
    {
        let url = format!("{}/chat/completions", API_BASE);
        let mut request = request;
        request.stream = true;

        let es = EventSource::new(
            self.client
                .post(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&request),
        )
        .context("创建 EventSource 失败")?;

        let mut aggregated = StreamDelta::default();
        let mut es = es;

        while let Some(event) = es.next().await {
            match event {
                Ok(Event::Open) => {}
                Ok(Event::Message(message)) => {
                    if message.data == "[DONE]" {
                        break;
                    }
                    match serde_json::from_str::<ChatCompletionChunk>(&message.data) {
                        Ok(chunk) => {
                            let delta = chunk.to_delta();
                            aggregated.content.push_str(&delta.content);
                            for call in &delta.tool_calls {
                                if let Some(existing) = aggregated
                                    .tool_calls
                                    .iter_mut()
                                    .find(|c| c.index == call.index)
                                {
                                    existing.id.push_str(&call.id);
                                    existing.name.push_str(&call.name);
                                    existing.arguments.push_str(&call.arguments);
                                } else {
                                    aggregated.tool_calls.push(call.clone());
                                }
                            }
                            if delta.finish_reason.is_some() {
                                aggregated.finish_reason = delta.finish_reason.clone();
                            }
                            on_delta(delta);
                        }
                        Err(e) => {
                            tracing::warn!("解析 SSE chunk 失败: {} 数据: {}", e, message.data);
                        }
                    }
                }
                Err(e) => {
                    es.close();
                    anyhow::bail!("SSE 错误: {}", e);
                }
            }
        }

        Ok(aggregated)
    }
}

/// 构建工具定义
pub fn build_tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "read_file".to_string(),
                description: "读取指定路径的文件内容".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "相对于项目根目录的文件路径" }
                    },
                    "required": ["path"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "write_file".to_string(),
                description: "写入或创建文件".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string" },
                        "content": { "type": "string" }
                    },
                    "required": ["path", "content"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "edit_file".to_string(),
                description: "修改文件中的部分内容".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string" },
                        "old_string": { "type": "string" },
                        "new_string": { "type": "string" }
                    },
                    "required": ["path", "old_string", "new_string"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "list_directory".to_string(),
                description: "列出目录内容".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "相对于项目根目录的目录路径" }
                    },
                    "required": ["path"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "execute_command".to_string(),
                description: "在项目目录下执行 shell 命令".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "command": { "type": "string", "description": "要执行的命令" },
                        "description": { "type": "string", "description": "命令用途说明" }
                    },
                    "required": ["command"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "ask_user".to_string(),
                description: "当信息不足或需要用户确认时提出问题".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "question": { "type": "string" },
                        "options": { "type": "array", "items": { "type": "string" } }
                    },
                    "required": ["question"]
                }),
            },
        },
    ]
}
