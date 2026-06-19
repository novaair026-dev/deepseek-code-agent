use std::path::Path;

use anyhow::Result;

const README_MD: &str = r#"# Axum 网站

本项目由 DeepSeek Code Agent 生成。

## 技术栈
- Rust + Axum（后端）
- Tauri（管理桌面端，可选）
- SQLite（数据库）

## 开发

```bash
cargo run
```

## 构建

```bash
cargo build --release
```
"#;

const CARGO_TOML: &str = r#"[package]
name = "web-app"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
tower = "0.5"
tower-http = { version = "0.6", features = ["fs", "cors"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rusqlite = { version = "0.34", features = ["bundled"] }
anyhow = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
"#;

const MAIN_RS: &str = r#"use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "Hello, DeepSeek Code Agent!"
}
"#;

pub async fn init(project_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(project_dir)?;
    std::fs::create_dir_all(project_dir.join("src"))?;
    std::fs::write(project_dir.join("README.md"), README_MD)?;
    std::fs::write(project_dir.join("Cargo.toml"), CARGO_TOML)?;
    std::fs::write(project_dir.join("src/main.rs"), MAIN_RS)?;
    Ok(())
}
