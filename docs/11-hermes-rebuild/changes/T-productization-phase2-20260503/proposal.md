# Phase 2 产品化优化 — 提案

## 背景
Phase 1 完成了 Error Boundary、Toast、单元测试、用户文档和 session 隔离 localStorage。Phase 2 聚焦三个提升用户体验的工程项。

## 目标
1. 后端结构化会话历史存储（替代纯 localStorage）
2. 移动端适配（底部导航、Sheet 侧滑、 Composer 折叠）
3. Composer 文件上传 UI

## 影响范围
- Gateway：新增 SQLite 表 + REST API
- Frontend：UI 布局适配、文件上传交互、API 集成
