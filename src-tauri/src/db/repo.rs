use anyhow::{Context, Result};
use chrono::Utc;
use rusqlite::{params, OptionalExtension};
use uuid::Uuid;

use super::models::*;
use super::DbConn;

pub struct ProjectRepo;

impl ProjectRepo {
    pub fn create(db: &DbConn, req: &CreateProjectRequest, path: &str) -> Result<Project> {
        let conn = db.conn()?;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        conn.execute(
            "INSERT INTO projects (id, name, description, project_type, path, plan_status, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                &id,
                &req.name,
                req.description.as_ref(),
                req.project_type.to_string(),
                path,
                PlanStatus::Draft.to_string(),
                now.timestamp_millis(),
                now.timestamp_millis(),
            ],
        )?;
        Self::get_by_id(db, &id)?.context("创建项目后无法读取")
    }

    pub fn get_by_id(db: &DbConn, id: &str) -> Result<Option<Project>> {
        let conn = db.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, description, project_type, path, plan_json, plan_status, created_at, updated_at
             FROM projects WHERE id = ?1"
        )?;
        let row = stmt
            .query_row([id], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    project_type: row.get::<_, String>(3)?.parse().map_err(|e| rusqlite::Error::InvalidColumnType(3, e, rusqlite::types::Type::Text))?,
                    path: row.get(4)?,
                    plan_json: row.get(5)?,
                    plan_status: row.get::<_, Option<String>>(6)?.map(|s| s.parse().map_err(|e| rusqlite::Error::InvalidColumnType(6, e, rusqlite::types::Type::Text)).ok()).flatten(),
                    created_at: chrono::DateTime::from_timestamp_millis(row.get::<_, i64>(7)?).unwrap_or_default(),
                    updated_at: chrono::DateTime::from_timestamp_millis(row.get::<_, i64>(8)?).unwrap_or_default(),
                })
            })
            .optional()?;
        Ok(row)
    }

    pub fn list(db: &DbConn) -> Result<Vec<Project>> {
        let conn = db.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, description, project_type, path, plan_json, plan_status, created_at, updated_at
             FROM projects ORDER BY updated_at DESC"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                project_type: row.get::<_, String>(3)?.parse().map_err(|e| rusqlite::Error::InvalidColumnType(3, e, rusqlite::types::Type::Text))?,
                path: row.get(4)?,
                plan_json: row.get(5)?,
                plan_status: row.get::<_, Option<String>>(6)?.map(|s| s.parse().map_err(|e| rusqlite::Error::InvalidColumnType(6, e, rusqlite::types::Type::Text)).ok()).flatten(),
                created_at: chrono::DateTime::from_timestamp_millis(row.get::<_, i64>(7)?).unwrap_or_default(),
                updated_at: chrono::DateTime::from_timestamp_millis(row.get::<_, i64>(8)?).unwrap_or_default(),
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().context("读取项目列表失败")
    }

    pub fn update_plan(db: &DbConn, req: &UpdatePlanRequest) -> Result<()> {
        let conn = db.conn()?;
        let now = Utc::now();
        conn.execute(
            "UPDATE projects SET plan_json = ?1, plan_status = ?2, updated_at = ?3 WHERE id = ?4",
            params![
                &req.plan_json,
                req.plan_status.to_string(),
                now.timestamp_millis(),
                &req.project_id,
            ],
        )?;
        Ok(())
    }

    pub fn delete(db: &DbConn, id: &str) -> Result<()> {
        let conn = db.conn()?;
        conn.execute("DELETE FROM projects WHERE id = ?1", [id])?;
        Ok(())
    }
}

pub struct SessionRepo;

impl SessionRepo {
    pub fn create(db: &DbConn, project_id: &str, title: Option<&str>) -> Result<Session> {
        let conn = db.conn()?;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        conn.execute(
            "INSERT INTO sessions (id, project_id, title, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![&id, project_id, title, now.timestamp_millis(), now.timestamp_millis()],
        )?;
        Self::get_by_id(db, &id)?.context("创建会话后无法读取")
    }

    pub fn get_by_id(db: &DbConn, id: &str) -> Result<Option<Session>> {
        let conn = db.conn()?;
        let row = conn
            .query_row(
                "SELECT id, project_id, title, created_at, updated_at FROM sessions WHERE id = ?1",
                [id],
                |row| {
                    Ok(Session {
                        id: row.get(0)?,
                        project_id: row.get(1)?,
                        title: row.get(2)?,
                        created_at: chrono::DateTime::from_timestamp_millis(row.get::<_, i64>(3)?).unwrap_or_default(),
                        updated_at: chrono::DateTime::from_timestamp_millis(row.get::<_, i64>(4)?).unwrap_or_default(),
                    })
                },
            )
            .optional()?;
        Ok(row)
    }

    pub fn list_by_project(db: &DbConn, project_id: &str) -> Result<Vec<Session>> {
        let conn = db.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, project_id, title, created_at, updated_at FROM sessions WHERE project_id = ?1 ORDER BY updated_at DESC"
        )?;
        let rows = stmt.query_map([project_id], |row| {
            Ok(Session {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                created_at: chrono::DateTime::from_timestamp_millis(row.get::<_, i64>(3)?).unwrap_or_default(),
                updated_at: chrono::DateTime::from_timestamp_millis(row.get::<_, i64>(4)?).unwrap_or_default(),
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().context("读取会话列表失败")
    }
}

pub struct MessageRepo;

impl MessageRepo {
    pub fn create(
        db: &DbConn,
        session_id: &str,
        role: MessageRole,
        content: Option<&str>,
        tool_calls: Option<&str>,
        tool_result: Option<&str>,
        metadata: Option<&str>,
    ) -> Result<Message> {
        let conn = db.conn()?;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        conn.execute(
            "INSERT INTO messages (id, session_id, role, content, tool_calls, tool_result, metadata, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                &id,
                session_id,
                role.to_string(),
                content,
                tool_calls,
                tool_result,
                metadata,
                now.timestamp_millis(),
            ],
        )?;
        Self::get_by_id(db, &id)?.context("创建消息后无法读取")
    }

    pub fn get_by_id(db: &DbConn, id: &str) -> Result<Option<Message>> {
        let conn = db.conn()?;
        let row = conn
            .query_row(
                "SELECT id, session_id, role, content, tool_calls, tool_result, metadata, created_at FROM messages WHERE id = ?1",
                [id],
                |row| {
                    Ok(Message {
                        id: row.get(0)?,
                        session_id: row.get(1)?,
                        role: row.get::<_, String>(2)?.parse().map_err(|e| rusqlite::Error::InvalidColumnType(2, e, rusqlite::types::Type::Text))?,
                        content: row.get(3)?,
                        tool_calls: row.get(4)?,
                        tool_result: row.get(5)?,
                        metadata: row.get(6)?,
                        created_at: chrono::DateTime::from_timestamp_millis(row.get::<_, i64>(7)?).unwrap_or_default(),
                    })
                },
            )
            .optional()?;
        Ok(row)
    }

    pub fn list_by_session(db: &DbConn, session_id: &str) -> Result<Vec<Message>> {
        let conn = db.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, session_id, role, content, tool_calls, tool_result, metadata, created_at
             FROM messages WHERE session_id = ?1 ORDER BY created_at ASC"
        )?;
        let rows = stmt.query_map([session_id], |row| {
            Ok(Message {
                id: row.get(0)?,
                session_id: row.get(1)?,
                role: row.get::<_, String>(2)?.parse().map_err(|e| rusqlite::Error::InvalidColumnType(2, e, rusqlite::types::Type::Text))?,
                content: row.get(3)?,
                tool_calls: row.get(4)?,
                tool_result: row.get(5)?,
                metadata: row.get(6)?,
                created_at: chrono::DateTime::from_timestamp_millis(row.get::<_, i64>(7)?).unwrap_or_default(),
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().context("读取消息列表失败")
    }
}

pub struct ConfigRepo;

impl ConfigRepo {
    pub fn get(db: &DbConn, key: &str) -> Result<Option<String>> {
        let conn = db.conn()?;
        let value: Option<String> = conn
            .query_row("SELECT value FROM config WHERE key = ?1", [key], |row| row.get(0))
            .optional()?;
        Ok(value)
    }

    pub fn set(db: &DbConn, key: &str, value: &str) -> Result<()> {
        let conn = db.conn()?;
        conn.execute(
            "INSERT INTO config (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }
}
