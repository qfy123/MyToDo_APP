# MyTodo - 个人待办事项管理软件

## 📝 项目简介

MyTodo 是一个现代化的桌面端待办事项管理应用，使用 Rust + Tauri + Vue3 + TypeScript 技术栈开发。

## ✨ 主要功能

- 📋 **任务管理**: 创建、编辑、删除和完成任务
- 🏷️ **标签系统**: 为任务添加和管理标签
- 🔍 **搜索过滤**: 快速找到需要的任务
- 🌙 **深色模式**: 支持亮色/暗色主题切换
- 📅 **日期管理**: 设置任务截止日期和提醒
- 🔔 **系统通知**: 任务完成和截止时间提醒
- 💾 **系统托盘**: 最小化到托盘运行
- ⚙️ **设置管理**: 个性化配置选项

## 🚀 技术栈

- **后端**: Rust + Tauri 2.0
- **前端**: Vue 3 + TypeScript + Vite
- **样式**: TailwindCSS
- **数据库**: SQLite
- **打包**: Tauri Bundle

## 📦 安装包

项目提供两种安装包格式：

- **MSI安装包**: `MyTodo_0.1.0_x64_en-US.msi`
- **NSIS安装包**: `MyTodo_0.1.0_x64-setup.exe`

## 🛠️ 开发环境设置

### 前置条件

- Node.js (推荐 18+)
- Rust (最新稳定版)
- Tauri CLI

### 安装依赖

```bash
# 安装前端依赖
cd frontend
npm install

# 安装 Tauri CLI
cargo install tauri-cli@^2.0.0
```

### 开发模式运行

```bash
# 启动开发服务器
npm run tauri dev
```

### 构建生产版本

```bash
# 构建安装包
npm run tauri build
```

## 📁 项目结构

```
├── frontend/          # 前端 Vue 3 应用
│   ├── src/
│   │   ├── components/    # Vue 组件
│   │   ├── composables/   # 组合式函数
│   │   ├── types/         # TypeScript 类型
│   │   └── utils/         # 工具函数
│   └── package.json
├── src-tauri/         # Tauri 后端应用
│   ├── src/
│   │   ├── commands.rs    # Tauri 命令
│   │   ├── database.rs    # 数据库操作
│   │   ├── models.rs      # 数据模型
│   │   └── lib.rs         # 主程序
│   └── Cargo.toml
└── README.md
```

## 🎯 核心特性

### 任务管理
- 创建任务，支持标题、描述、截止日期和优先级
- 标记任务完成状态
- 删除不需要的任务

### 标签系统
- 为任务添加彩色标签
- 按标签筛选任务
- 管理标签的名称和颜色

### 智能通知
- 任务截止时间提醒
- 任务完成通知
- 可配置的通知设置

### 系统集成
- 系统托盘支持
- 窗口关闭行为配置
- 开机自启动选项

## 🔧 配置选项

应用支持以下配置：
- 关闭行为（退出/最小化到托盘/询问）
- 通知开关和提醒时间
- 主题模式选择
- 启动行为设置

## 📄 许可证

本项目采用 MIT 许可证。

## 🤝 贡献

欢迎提交 Issue 和 Pull Request 来帮助改进这个项目。

---

**MyTodo** - 让任务管理变得更简单！