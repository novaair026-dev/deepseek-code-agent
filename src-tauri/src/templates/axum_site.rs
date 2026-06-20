use std::path::Path;

use anyhow::Result;

const README_MD: &str = r#"# Axum + Vue3 网站

本项目由 DeepSeek Code Agent 生成。

## 技术栈
- Rust + Axum（后端 API）
- Vue3 + Vite（前端）
- Tauri（管理桌面端，可选）
- SQLite（数据库）

## 目录结构

```
.
├── Cargo.toml          # Rust 后端配置
├── src/main.rs         # Axum 后端入口
└── frontend/           # Vue3 前端
    ├── package.json
    ├── vite.config.js
    ├── index.html
    └── src/
        ├── main.js
        └── App.vue
```

## 开发

```bash
# 启动后端（默认端口 3000）
cargo run

# 启动前端（在 frontend 目录下，默认端口 5173）
cd frontend
npm install
npm run dev
```

## 构建

```bash
# 构建前端
cd frontend
npm run build

# 构建后端（会自动打包 dist 目录）
cd ..
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

const MAIN_RS: &str = r#"use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, services::ServeDir};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/api/hello", get(hello))
        .fallback_service(ServeDir::new("frontend/dist"))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "Hello from DeepSeek Code Agent!"
}
"#;

const FRONTEND_PACKAGE_JSON: &str = r#"{
  "name": "frontend",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "dependencies": {
    "vue": "^3.5.13"
  },
  "devDependencies": {
    "@vitejs/plugin-vue": "^5.2.1",
    "vite": "^6.0.3"
  }
}
"#;

const FRONTEND_VITE_CONFIG: &str = r#"import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()],
})
"#;

const FRONTEND_INDEX_HTML: &str = r#"<!doctype html>
<html lang="zh-CN">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>DeepSeek Code Agent 网站</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/main.js"></script>
  </body>
</html>
"#;

const FRONTEND_MAIN_JS: &str = r#"import { createApp } from 'vue'
import App from './App.vue'

createApp(App).mount('#app')
"#;

const FRONTEND_APP_VUE: &str = r#"<script setup>
import { ref, onMounted } from 'vue'

const message = ref('Loading...')

onMounted(async () => {
  try {
    const res = await fetch('/api/hello')
    message.value = await res.text()
  } catch (e) {
    message.value = '无法连接后端服务'
  }
})
</script>

<template>
  <div style="font-family: sans-serif; text-align: center; padding: 40px;">
    <h1>{{ message }}</h1>
    <p>由 DeepSeek Code Agent 生成的 Vue3 网站</p>
  </div>
</template>

<style scoped>
h1 {
  color: #2563eb;
}
</style>
"#;

pub async fn init(project_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(project_dir)?;
    std::fs::create_dir_all(project_dir.join("src"))?;
    std::fs::create_dir_all(project_dir.join("frontend/src"))?;

    std::fs::write(project_dir.join("README.md"), README_MD)?;
    std::fs::write(project_dir.join("Cargo.toml"), CARGO_TOML)?;
    std::fs::write(project_dir.join("src/main.rs"), MAIN_RS)?;

    std::fs::write(project_dir.join("frontend/package.json"), FRONTEND_PACKAGE_JSON)?;
    std::fs::write(project_dir.join("frontend/vite.config.js"), FRONTEND_VITE_CONFIG)?;
    std::fs::write(project_dir.join("frontend/index.html"), FRONTEND_INDEX_HTML)?;
    std::fs::write(project_dir.join("frontend/src/main.js"), FRONTEND_MAIN_JS)?;
    std::fs::write(project_dir.join("frontend/src/App.vue"), FRONTEND_APP_VUE)?;

    Ok(())
}
