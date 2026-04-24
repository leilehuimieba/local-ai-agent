# 本地智能体

一个运行在本地环境的智能体框架，支持多模型 Provider、本地记忆与知识沉淀、任务执行与验证。

## 技术栈

| 模块 | 技术 | 职责 |
|------|------|------|
| runtime-core | Rust | 运行时核心：事件流、状态机、记忆路由、checkpoint |
| runtime-host | Rust | 运行时宿主：进程管理、系统交互 |
| gateway | Go 1.25 | API 网关：HTTP 路由、设置管理、诊断、日志 |
| frontend | React 19 + Vite | 用户界面：聊天、任务、设置、知识库 |

## 项目结构

```
├── crates/           # Rust 工作区
│   ├── runtime-core/
│   └── runtime-host/
├── gateway/          # Go API 网关
├── frontend/         # React 前端
├── docs/             # 文档（执行入口见 docs/README.md）
├── scripts/          # 构建与运维脚本
├── data/             # 运行时数据（记忆、日志、配置）
├── tmp/              # 临时文件与实验目录
└── config/           # 应用配置
```

## 快速开始

### 前端

```bash
cd frontend
npm install
npm run dev
```

### Gateway

```bash
cd gateway
go run ./cmd/server
```

### Runtime Core

```bash
cargo check --workspace
cargo test -p runtime-core --lib
```

## 文档

开发执行入口与阶段状态见 [`docs/README.md`](docs/README.md)。

## 许可证

MIT
