# 2026-05-10 归档索引

归档日期：2026-05-10

本批次归档 12 个已收口 change，覆盖产品化、MCP 补齐、diff apply、会话同步、历史列表、知识库上传、Playwright E2E、Runtime MCP Tool Registry 接入与 CI 修复、frontend 测试覆盖率补充、Go errcheck 修复等主题。

## 归档项列表

1. [AA-windows-doctor-launcher-20260504](AA-windows-doctor-launcher-20260504/) — P1：Windows doctor、一键启动器与服务状态自检
2. [Z-diff-apply-20260504](Z-diff-apply-20260504/) — P1：Aider/Codex 风格代码 diff apply 最小闭环
3. [Y-shortcomings-mcp-first-20260504](Y-shortcomings-mcp-first-20260504/) — 短板补齐第一阶段：MCP 添加/删除持久化、Runtime MCP 调用闭环与安全治理
4. [S-productization-phase1-20260502](S-productization-phase1-20260502/) — 产品化第一阶段：Error Boundary + Toast + 前端测试 + 用户文档 + Session 隔离
5. [T-productization-phase2-20260503](T-productization-phase2-20260503/) — 产品化第二阶段：后端结构化会话历史存储 + 移动端适配 + Composer 文件上传 UI
6. [U-assistant-msg-sync-20260503](U-assistant-msg-sync-20260503/) — Assistant 消息同步到后端
7. [V-history-session-list-20260503](V-history-session-list-20260503/) — 历史会话列表 UI
8. [W-knowledge-upload-backend-20260503](W-knowledge-upload-backend-20260503/) — 文件上传接入后端知识库
9. [X-playwright-mobile-e2e-20260503](X-playwright-mobile-e2e-20260503/) — Playwright E2E 测试覆盖移动端
10. [AD-runtime-mcp-tool-registry-20260505](AD-runtime-mcp-tool-registry-20260505/) — Runtime MCP Tool Registry 接入与 CI 修复：Runtime MCP 注册接入、request-scoped ToolDefinition、模型 schema 统一出口、capability catalog 化、PR #2 CI 全绿
11. [AE-frontend-test-coverage-20260510](AE-frontend-test-coverage-20260510/) — Frontend 测试覆盖率补充：新增 4 个测试文件，118 项测试全绿，覆盖率 33% → 62% Lines
12. [AF-go-errcheck-cleanup-20260510](AF-go-errcheck-cleanup-20260510/) — Go errcheck 逐个修复：移除 `.golangci.yml` 中 errcheck 排除规则，32 个文件 80+ 处显式忽略，errcheck 清零
