use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::db::models::{MessageRole, ProjectType};
use crate::db::repo::MessageRepo;
use crate::db::AppState;
use crate::llm::{build_tools, ChatCompletionRequest, ChatMessage, DeepSeekClient};
use crate::security::command::RiskLevel;
use crate::tools::cmd_tool::CommandTool;
use crate::tools::file_tool::FileTool;

use super::types::{AgentContext, AgentResponse, DevelopmentPlan, ExecutionResult, ExecutionStep, PendingCommand};

const EXECUTOR_PROMPT: &str = r#"你是 DeepSeek Code Agent 的执行工程师。你已经获得用户确认的开发计划，现在需要逐步执行完成项目开发。

你可以使用以下工具：
- read_file(path): 读取文件
- write_file(path, content): 写入文件（支持多级目录自动创建）
- edit_file(path, old_string, new_string): 局部修改文件
- list_directory(path): 列出目录
- execute_command(command, description): 执行 shell 命令
- ask_user(question, options): 向用户提问

执行原则：
1. 一次调用一个工具，等待结果后再决定下一步。
2. 优先创建项目骨架和关键文件。
3. 命令执行前说明用途。
4. 如果命令失败，分析错误并尝试修复。
5. 完成后回复 DONE: <总结>。

请使用工具调用格式回复。"#;

#[derive(Debug, Clone)]
pub struct Executor {
    client: Arc<DeepSeekClient>,
    state: Arc<Mutex<AppState>>,
    context: AgentContext,
    session_id: String,
}

impl Executor {
    pub fn new(
        client: Arc<DeepSeekClient>,
        state: Arc<Mutex<AppState>>,
        context: AgentContext,
        session_id: String,
    ) -> Self {
        Self {
            client,
            state,
            context,
            session_id,
        }
    }

    pub async fn execute_plan(
        &self,
        plan: &DevelopmentPlan,
    ) -> anyhow::Result<ExecutionResult> {
        let project_dir = Path::new(&self.context.project_path);
        let file_tool = FileTool::new(project_dir);
        let cmd_tool = CommandTool::new(project_dir);

        // 初始化项目模板
        self.init_project_template(project_dir).await?;

        // 保存计划到消息历史
        self.save_message(
            MessageRole::System,
            Some(&format!("已确认的开发计划：{}", serde_json::to_string(plan)?)),
            None,
            None,
            None,
        )?;

        let mut messages = self.context.to_chat_messages(EXECUTOR_PROMPT);
        messages.push(ChatMessage::user(format!(
            "请开始执行以下开发计划：{}\n\n项目路径：{}\n请使用工具调用逐步完成。",
            serde_json::to_string(plan)?,
            self.context.project_path
        )));

        let mut completed_steps: Vec<ExecutionStep> = Vec::new();
        let mut iteration = 0;
        let max_iterations = 30;

        while iteration < max_iterations {
            iteration += 1;

            let request = ChatCompletionRequest {
                model: "deepseek-v4-pro".to_string(),
                messages: messages.clone(),
                tools: Some(build_tools()),
                tool_choice: Some("auto".to_string()),
                stream: false,
                temperature: Some(0.3),
                max_tokens: None,
            };

            let response = self.client.chat_complete(request).await?;
            let choice = response.choices.into_iter().next();
            let message = match choice {
                Some(c) => c.message,
                None => break,
            };

            // 保存 assistant 消息
            self.save_message(
                MessageRole::Assistant,
                message.content.as_deref(),
                message.tool_calls.as_ref().map(|c| serde_json::to_string(c).unwrap_or_default()).as_deref(),
                None,
                None,
            )?;

            // 将 assistant 消息加入上下文
            messages.push(ChatMessage {
                role: "assistant".to_string(),
                content: message.content.clone().unwrap_or_default(),
            });

            // 检查是否完成
            if let Some(content) = &message.content {
                if content.trim().starts_with("DONE:") {
                    let summary = content.trim().strip_prefix("DONE:").unwrap_or(content).trim().to_string();
                    return Ok(ExecutionResult {
                        completed_steps,
                        pending_command: None,
                        final_response: AgentResponse::Done {
                            summary,
                            next_steps: vec!["你可以在项目目录中查看生成的代码".to_string()],
                        },
                    });
                }
            }

            // 处理工具调用
            if let Some(tool_calls) = message.tool_calls {
                for call in tool_calls {
                    let result = self
                        .execute_tool_call(&call, &file_tool, &cmd_tool)
                        .await?;

                    completed_steps.push(ExecutionStep {
                        description: format!("{}: {}", call.function.name, call.function.arguments),
                        tool_result: result.result.clone(),
                    });

                    // 如果需要确认命令，暂停执行
                    if let Some(pending) = result.pending_command {
                        return Ok(ExecutionResult {
                            completed_steps,
                            pending_command: Some(pending.clone()),
                            final_response: AgentResponse::CommandConfirmation {
                                command: pending.command,
                                description: pending.description,
                            },
                        });
                    }

                    // 将工具结果加入上下文
                    messages.push(ChatMessage {
                        role: "tool".to_string(),
                        content: format!(
                            "工具 {} 执行结果：{}\n{}",
                            call.function.name,
                            if result.result.success { "成功" } else { "失败" },
                            result.result.output
                        ),
                    });
                }
            } else {
                // 没有工具调用，可能 LLM 在犹豫，直接结束
                return Ok(ExecutionResult {
                    completed_steps,
                    pending_command: None,
                    final_response: AgentResponse::Done {
                        summary: message.content.unwrap_or_else(|| "执行完成".to_string()),
                        next_steps: vec![],
                    },
                });
            }
        }

        Ok(ExecutionResult {
            completed_steps,
            pending_command: None,
            final_response: AgentResponse::Done {
                summary: "已达到最大执行轮数，项目骨架已生成。".to_string(),
                next_steps: vec!["请手动检查项目目录中的代码".to_string()],
            },
        })
    }

    async fn init_project_template(&self, project_dir: &Path) -> anyhow::Result<()> {
        match self.context.project_type {
            ProjectType::Desktop => crate::templates::tauri_app::init(project_dir).await,
            ProjectType::Website => crate::templates::axum_site::init(project_dir).await,
        }
    }

    async fn execute_tool_call<'b>(
        &self,
        call: &crate::llm::ToolCall,
        file_tool: &FileTool<'b>,
        cmd_tool: &CommandTool<'b>,
    ) -> anyhow::Result<ToolExecutionOutcome> {
        let args: serde_json::Value = serde_json::from_str(&call.function.arguments)
            .unwrap_or_else(|_| serde_json::json!({}));

        let result = match call.function.name.as_str() {
            "read_file" => {
                let path = args["path"].as_str().unwrap_or("");
                file_tool.read_file(path)
            }
            "write_file" => {
                let path = args["path"].as_str().unwrap_or("");
                let content = args["content"].as_str().unwrap_or("");
                file_tool.write_file(path, content)
            }
            "edit_file" => {
                let path = args["path"].as_str().unwrap_or("");
                let old_string = args["old_string"].as_str().unwrap_or("");
                let new_string = args["new_string"].as_str().unwrap_or("");
                file_tool.edit_file(path, old_string, new_string)
            }
            "list_directory" => {
                let path = args["path"].as_str().unwrap_or(".");
                file_tool.list_directory(path)
            }
            "execute_command" => {
                let command = args["command"].as_str().unwrap_or("");
                let description = args["description"].as_str().unwrap_or("执行命令");

                match cmd_tool.risk_level(command) {
                    RiskLevel::Blocked => {
                        return Ok(ToolExecutionOutcome {
                            result: crate::tools::ToolResult::error("该命令已被安全策略阻止"),
                            pending_command: None,
                        });
                    }
                    RiskLevel::High => {
                        return Ok(ToolExecutionOutcome {
                            result: crate::tools::ToolResult::error("等待用户确认"),
                            pending_command: Some(PendingCommand {
                                command: command.to_string(),
                                description: description.to_string(),
                            }),
                        });
                    }
                    RiskLevel::Low => cmd_tool.execute_command(command).await,
                }
            }
            "ask_user" => {
                let question = args["question"].as_str().unwrap_or("");
                crate::tools::ToolResult::success(format!("已向用户提问：{}", question))
            }
            _ => crate::tools::ToolResult::error(format!("未知工具: {}", call.function.name)),
        };

        Ok(ToolExecutionOutcome {
            result,
            pending_command: None,
        })
    }

    fn save_message(
        &self,
        role: MessageRole,
        content: Option<&str>,
        tool_calls: Option<&str>,
        tool_result: Option<&str>,
        metadata: Option<&str>,
    ) -> anyhow::Result<()> {
        let state = self.state.lock().map_err(|e| anyhow::anyhow!("锁失败: {}", e))?;
        MessageRepo::create(
            &state.db,
            &self.session_id,
            role,
            content,
            tool_calls,
            tool_result,
            metadata,
        )?;
        Ok(())
    }
}

struct ToolExecutionOutcome {
    result: crate::tools::ToolResult,
    pending_command: Option<PendingCommand>,
}
