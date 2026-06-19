use std::path::Path;
use std::process::Stdio;

use tokio::process::Command;

use super::ToolResult;
use crate::security::command::{assess_risk, RiskLevel};

pub struct CommandTool<'a> {
    project_dir: &'a Path,
}

impl<'a> CommandTool<'a> {
    pub fn new(project_dir: &'a Path) -> Self {
        Self { project_dir }
    }

    /// 判断命令风险等级
    pub fn risk_level(&self, command: &str) -> RiskLevel {
        assess_risk(command)
    }

    /// 执行命令（异步）
    pub async fn execute_command(&self, command: &str) -> ToolResult {
        if !self.project_dir.exists() {
            return ToolResult::error("项目目录不存在");
        }

        let shell = if cfg!(target_os = "windows") { "cmd" } else { "sh" };
        let arg = if cfg!(target_os = "windows") { "/C" } else { "-c" };

        let output = Command::new(shell)
            .arg(arg)
            .arg(command)
            .current_dir(self.project_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await;

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                if output.status.success() {
                    ToolResult::success(if stdout.is_empty() { stderr } else { stdout })
                } else {
                    ToolResult::error(format!(
                        "命令退出码: {}\nstdout:\n{}\nstderr:\n{}",
                        output.status.code().unwrap_or(-1),
                        stdout,
                        stderr
                    ))
                }
            }
            Err(e) => ToolResult::error(format!("执行命令失败: {}", e)),
        }
    }
}
