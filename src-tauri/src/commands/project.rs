use std::sync::{Arc, Mutex};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db::models::{CreateProjectRequest, Project, ProjectType};
use crate::db::repo::ProjectRepo;
use crate::db::AppState;
use crate::project::manager::ProjectManager;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectPayload {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub project_type: ProjectType,
}

/// 创建项目
#[tauri::command]
pub fn create_project(
    state: State<Arc<Mutex<AppState>>>,
    payload: CreateProjectPayload,
) -> Result<Project, String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    let manager = ProjectManager::new(&state.projects_base_dir);
    let path = manager
        .create_project_dir(&payload.name, payload.project_type)
        .map_err(|e| e.to_string())?;

    let req = CreateProjectRequest {
        name: payload.name,
        description: payload.description,
        project_type: payload.project_type,
    };

    ProjectRepo::create(&state.db, &req, &path).map_err(|e| e.to_string())
}

/// 列出所有项目
#[tauri::command]
pub fn list_projects(state: State<Arc<Mutex<AppState>>>) -> Result<Vec<Project>, String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    ProjectRepo::list(&state.db).map_err(|e| e.to_string())
}

/// 获取单个项目
#[tauri::command]
pub fn get_project(
    state: State<Arc<Mutex<AppState>>>,
    id: String,
) -> Result<Option<Project>, String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    ProjectRepo::get_by_id(&state.db, &id).map_err(|e| e.to_string())
}

/// 删除项目
#[tauri::command]
pub fn delete_project(state: State<Arc<Mutex<AppState>>>, id: String) -> Result<(), String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    let project = ProjectRepo::get_by_id(&state.db, &id)
        .map_err(|e| e.to_string())?
        .context("项目不存在")
        .map_err(|e| e.to_string())?;

    // 删除数据库记录
    ProjectRepo::delete(&state.db, &id).map_err(|e| e.to_string())?;

    // 删除项目目录
    let manager = ProjectManager::new(&state.projects_base_dir);
    let _ = manager.delete_project_dir(&project.path);

    Ok(())
}
