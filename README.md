# DeepSeek Code Agent

DeepSeek Code Agent 是一款面向非技术用户的智能编程助手。通过自然语言对话，即使没有编程经验，也能开发出可商用的桌面应用或网站。

你只需描述想要的功能，Agent 会主动澄清需求、制定开发计划、生成代码并执行必要的构建命令，全程无需关心底层实现细节。

> **Note**
> 本项目与 DeepSeek Inc. 无隶属关系。

---

## ✨ 核心特性

- **自然语言开发**：用中文或英文描述需求，Agent 理解并转化为可执行方案。
- **需求澄清对话**：当信息不足时，会以弹窗形式询问关键细节，避免误解。
- **开发计划确认**：生成结构化开发计划，支持确认、拒绝、修改三种操作。
- **自动代码生成**：根据确认的计划自动创建项目文件并写入代码。
- **命令自动执行**：低风险的构建/安装命令自动执行，高风险命令需人工二次确认。
- **两种项目类型**：
  - 桌面应用：`Rust + Tauri + SQLite`
  - 网站：`Rust + Tauri（管理端）+ Axum（Web 后端）+ Vue3 + SQLite`
- **本地安全存储**：DeepSeek API Key 保存在系统钥匙串中，项目文件隔离在用户目录下。
- **明暗主题切换**：支持亮色 / 暗色模式，自动跟随系统设置，也可手动切换。

---

## 🛠 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | Tauri v2 |
| 前端 | React 18 + TypeScript + Vite + Tailwind CSS（Agent 自身 UI） |
| 网站前端 | Vue3 + Vite |
| 后端 | Rust（Axum） |
| 数据库 | SQLite |
| 大模型 | DeepSeek API（`deepseek-v4-pro`） |

---

## 📋 环境要求

在运行项目之前，请确保已安装以下工具：

- [Rust](https://rustup.rs/)（推荐最新稳定版）
- [Node.js](https://nodejs.org/)（>= 18）
- [cargo-tauri](https://tauri.app/start/prerequisites/)：Tauri 的 Cargo 命令行工具

安装 cargo-tauri：

```bash
cargo install tauri-cli
```

---

## 🚀 快速开始

### 1. 克隆项目并安装前端依赖

```bash
git clone <仓库地址>
cd deepseek-code-agent
npm install
```

### 2. 配置 DeepSeek API Key

首次运行应用时，会弹出窗口要求输入 DeepSeek API Key。该密钥将加密存储在系统钥匙串中，不会以明文保存在项目或数据库里。

如果你没有 API Key，可以前往 [DeepSeek 开放平台](https://platform.deepseek.com/) 申请。

### 3. 开发运行

```bash
cargo tauri dev
```

此命令会同时启动前端 Vite 开发服务器和 Tauri 桌面应用窗口。

### 4. 构建生产包

```bash
cargo tauri build
```

构建完成后，安装包位于 `src-tauri/target/release/bundle/` 目录下。

---

## 📖 使用指南

### 创建第一个项目

1. 启动应用并完成 API Key 配置。
2. 点击左侧边栏底部的「+ 新建项目」。
3. 输入项目名称和描述，选择项目类型：
   - **桌面应用**：适合开发本地工具类软件。
   - **网站**：适合开发带 Web 后端的在线服务。
4. 在聊天框中输入需求，例如：
   > "帮我做一个待办事项管理工具，可以添加任务、标记完成、按状态筛选。"

### 与 Agent 协作

Agent 的工作流程如下：

1. **需求接收**：你描述想要的功能。
2. **需求澄清**：如果信息不够具体，Agent 会弹出对话框提问。
3. **制定计划**：生成包含阶段、文件、依赖、命令的开发计划。
4. **计划确认**：你可以在弹窗中确认、拒绝或修改计划。
5. **自动执行**：确认后 Agent 自动创建文件、写入代码、运行命令。
6. **结果查看**：在项目目录 `~/DeepSeekCodeAgentProjects/<项目名>` 中查看生成的代码。

### 高危命令确认

为了安全起见，涉及删除、系统修改等操作会被标记为高危命令，执行前必须手动确认。你可以随时在弹窗中点击「取消」来阻止该命令。

---

## 🏗 项目结构

```
deepseek-code-agent/
├── src/                          # React 前端源码
│   ├── components/               # UI 组件（聊天、弹窗、项目列表等）
│   ├── hooks/                    # React Hooks
│   ├── pages/                    # 页面组件
│   ├── services/                 # 调用 Tauri 后端命令
│   ├── types/                    # TypeScript 类型定义
│   ├── App.tsx                   # 应用主入口
│   └── main.tsx                  # React 渲染入口
├── src-tauri/                    # Rust 后端源码
│   ├── src/
│   │   ├── agent/                # Agent 核心：澄清、规划、执行
│   │   ├── commands/             # 暴露给前端的 Tauri 命令
│   │   ├── db/                   # SQLite 数据库与数据模型
│   │   ├── llm/                  # DeepSeek 大模型客户端
│   │   ├── project/              # 项目目录管理
│   │   ├── security/             # 命令安全评级
│   │   ├── templates/            # 项目初始模板
│   │   └── tools/                # 文件读写、命令执行等工具
│   ├── icons/                    # 应用图标
│   ├── Cargo.toml                # Rust 依赖配置
│   └── tauri.conf.json           # Tauri 应用配置
├── package.json                  # 前端依赖与脚本
├── vite.config.ts                # Vite 配置
├── tailwind.config.js            # Tailwind CSS 配置
└── README.md                     # 项目说明
```

---

## ⚙️ 配置说明

### 项目存储位置

所有生成的项目默认保存在用户主目录下：

```
~/DeepSeekCodeAgentProjects/
```

每个项目会创建一个独立子目录，命名格式为 `<项目名>_<类型>`。

### 应用数据目录

SQLite 数据库和本地配置保存在系统应用数据目录中：

- **macOS**：`~/Library/Application Support/com.deepseek-code-agent.DeepSeekCodeAgent/`
- **Windows**：`%APPDATA%\com.deepseek-code-agent\DeepSeekCodeAgent\`
- **Linux**：`~/.local/share/com.deepseek-code-agent.DeepSeekCodeAgent/`

### API Key 存储

API Key 通过系统钥匙串（macOS Keychain、Windows Credential Manager、Linux Secret Service）安全存储，不会在代码或日志中泄露。

---

## 🔒 安全说明

- **目录隔离**：Agent 只能操作 `~/DeepSeekCodeAgentProjects/<项目名>` 目录内的文件，禁止越界访问用户其他目录。
- **命令分级**：
  - **低风险命令**：如 `cargo build`、`npm install`、`ls`，自动执行。
  - **高风险命令**：如删除、移动、修改系统配置，需弹窗确认。
  - **黑名单命令**：如 `rm -rf /`、`sudo`、管道到 shell 的下载命令，直接拒绝。
- **密钥保护**：前端不接触 API Key 明文，所有与 DeepSeek 的通信都在 Rust 后端完成。

---

## 🧑‍💻 开发说明

### 常用命令

```bash
# 前端开发服务器（单独启动，仅用于前端调试）
npm run dev

# 前端生产构建
npm run build

# Tauri 开发模式（推荐，同时启动前后端）
cargo tauri dev

# Tauri 生产构建
cargo tauri build

# 检查 Rust 代码
cd src-tauri && cargo check
```

### 添加新的 Tauri 命令

1. 在 `src-tauri/src/commands/` 下新建或修改模块。
2. 在 `src-tauri/src/lib.rs` 的 `invoke_handler` 中注册命令。
3. 在 `src/services/tauriCommands.ts` 中添加前端调用封装。

### 修改项目模板

项目初始模板位于：

- `src-tauri/src/templates/tauri_app.rs`：桌面应用模板
- `src-tauri/src/templates/axum_site.rs`：网站模板

---

## 📝 许可证

本项目采用 [GNU General Public License v3.0](LICENSE) 开源许可。

---

## 🤝 贡献

欢迎提交 Issue 和 Pull Request。如果你在使用过程中遇到问题，或有新的功能建议，请随时反馈。
