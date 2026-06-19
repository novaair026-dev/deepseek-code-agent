use std::path::Path;

use anyhow::Result;

const README_MD: &str = r#"# Tauri 桌面应用

本项目由 DeepSeek Code Agent 生成。

## 开发

```bash
npm install
npm run tauri dev
```

## 构建

```bash
npm run tauri build
```
"#;

pub async fn init(project_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(project_dir)?;
    std::fs::write(project_dir.join("README.md"), README_MD)?;
    Ok(())
}
