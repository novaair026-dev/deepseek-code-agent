use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use directories::ProjectDirs;
use rusqlite::Connection;

pub mod models;
pub mod repo;

/// 数据库连接封装，方便跨线程共享
#[derive(Debug, Clone)]
pub struct DbConn {
    path: PathBuf,
}

impl DbConn {
    /// 打开应用数据目录下的数据库
    pub fn open() -> Result<Self> {
        let dirs = ProjectDirs::from("com", "deepseek-code-agent", "DeepSeekCodeAgent")
            .context("无法确定应用数据目录")?;
        let data_dir = dirs.data_dir();
        std::fs::create_dir_all(data_dir)?;
        let db_path = data_dir.join("app.db");
        let conn = Self::connect(&db_path)?;
        Self::init_schema(&conn)?;
        Ok(Self { path: db_path })
    }

    fn connect(path: &PathBuf) -> Result<Connection> {
        Connection::open(path).context("打开 SQLite 数据库失败")
    }

    fn init_schema(conn: &Connection) -> Result<()> {
        conn.execute_batch(
            r#"
            -- 配置表
            CREATE TABLE IF NOT EXISTS config (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            -- 项目表
            CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                project_type TEXT NOT NULL,
                path TEXT NOT NULL UNIQUE,
                plan_json TEXT,
                plan_status TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );

            -- 会话表
            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                title TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );

            -- 消息表
            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                session_id TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
                role TEXT NOT NULL,
                content TEXT,
                tool_calls TEXT,
                tool_result TEXT,
                metadata TEXT,
                created_at INTEGER NOT NULL
            );
            "#,
        )
        .context("初始化数据库表失败")?;
        Ok(())
    }

    /// 获取一个新的数据库连接
    pub fn conn(&self) -> Result<Connection> {
        Self::connect(&self.path)
    }
}

/// 应用全局状态
#[derive(Debug)]
pub struct AppState {
    pub db: DbConn,
    pub projects_base_dir: std::path::PathBuf,
}

impl AppState {
    pub fn new() -> Result<Arc<Mutex<Self>>> {
        let db = DbConn::open()?;
        let dirs = directories::BaseDirs::new().context("无法获取用户目录")?;
        let projects_base_dir = dirs.home_dir().join("DeepSeekCodeAgentProjects");
        std::fs::create_dir_all(&projects_base_dir)?;
        Ok(Arc::new(Mutex::new(Self {
            db,
            projects_base_dir,
        })))
    }
}
