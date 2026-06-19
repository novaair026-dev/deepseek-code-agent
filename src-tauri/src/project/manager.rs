use std::path::Path;

use anyhow::{Context, Result};

use crate::db::models::ProjectType;

pub struct ProjectManager<'a> {
    base_dir: &'a Path,
}

impl<'a> ProjectManager<'a> {
    pub fn new(base_dir: &'a Path) -> Self {
        Self { base_dir }
    }

    /// 为项目创建存储目录
    pub fn create_project_dir(&self, name: &str, project_type: ProjectType) -> Result<String> {
        let sanitized = sanitize_name(name);
        let dir_name = format!("{}_{}", sanitized, project_type.to_string());
        let project_dir = self.base_dir.join(&dir_name);
        std::fs::create_dir_all(&project_dir)
            .with_context(|| format!("创建项目目录失败: {:?}", project_dir))?;
        Ok(project_dir.to_string_lossy().to_string())
    }

    /// 删除项目目录
    pub fn delete_project_dir(&self, path: &str) -> Result<()> {
        let path = Path::new(path);
        if path.exists() && path.starts_with(self.base_dir) {
            std::fs::remove_dir_all(path).context("删除项目目录失败")?;
        }
        Ok(())
    }
}

fn sanitize_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect::<String>()
        .to_lowercase()
}
