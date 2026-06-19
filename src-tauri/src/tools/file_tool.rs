use std::path::{Path, PathBuf};

use super::ToolResult;

pub struct FileTool<'a> {
    project_dir: &'a Path,
}

impl<'a> FileTool<'a> {
    pub fn new(project_dir: &'a Path) -> Self {
        Self { project_dir }
    }

    /// 解析相对路径，确保不越界
    fn resolve_path(&self, relative: &str) -> Result<PathBuf, ToolResult> {
        let path = self.project_dir.join(relative);
        match path.canonicalize() {
            Ok(canonical) => {
                if !canonical.starts_with(self.project_dir) {
                    return Err(ToolResult::error("禁止访问项目目录外的路径"));
                }
                Ok(canonical)
            }
            Err(_) => {
                // 文件可能不存在，检查逻辑路径
                if !path.starts_with(self.project_dir) {
                    return Err(ToolResult::error("禁止访问项目目录外的路径"));
                }
                Ok(path)
            }
        }
    }

    pub fn read_file(&self, path: &str) -> ToolResult {
        let resolved = match self.resolve_path(path) {
            Ok(p) => p,
            Err(e) => return e,
        };

        match std::fs::read_to_string(&resolved) {
            Ok(content) => ToolResult::success(content),
            Err(e) => ToolResult::error(format!("读取文件失败: {}", e)),
        }
    }

    pub fn write_file(&self, path: &str, content: &str) -> ToolResult {
        let resolved = match self.resolve_path(path) {
            Ok(p) => p,
            Err(e) => return e,
        };

        if let Some(parent) = resolved.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                return ToolResult::error(format!("创建目录失败: {}", e));
            }
        }

        match std::fs::write(&resolved, content) {
            Ok(_) => ToolResult::success("文件写入成功"),
            Err(e) => ToolResult::error(format!("写入文件失败: {}", e)),
        }
    }

    pub fn edit_file(&self, path: &str, old_string: &str, new_string: &str) -> ToolResult {
        let resolved = match self.resolve_path(path) {
            Ok(p) => p,
            Err(e) => return e,
        };

        match std::fs::read_to_string(&resolved) {
            Ok(content) => {
                if !content.contains(old_string) {
                    return ToolResult::error("未找到要替换的内容");
                }
                let new_content = content.replace(old_string, new_string);
                match std::fs::write(&resolved, new_content) {
                    Ok(_) => ToolResult::success("文件修改成功"),
                    Err(e) => ToolResult::error(format!("写入文件失败: {}", e)),
                }
            }
            Err(e) => ToolResult::error(format!("读取文件失败: {}", e)),
        }
    }

    pub fn list_directory(&self, path: &str) -> ToolResult {
        let resolved = match self.resolve_path(path) {
            Ok(p) => p,
            Err(e) => return e,
        };

        match std::fs::read_dir(&resolved) {
            Ok(entries) => {
                let mut lines = Vec::new();
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                    lines.push(format!("{}{}", name, if is_dir { "/" } else { "" }));
                }
                lines.sort();
                ToolResult::success(lines.join("\n"))
            }
            Err(e) => ToolResult::error(format!("列出目录失败: {}", e)),
        }
    }
}
