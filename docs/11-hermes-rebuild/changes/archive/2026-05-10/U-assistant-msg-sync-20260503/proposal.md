# U-change: Assistant 消息同步到后端

## 背景

T-productization-phase2 引入了后端 SQLite 结构化会话历史存储（`sessions` + `chat_messages` 表）和 REST API。但当前前端仅在 `addMessage` 中同步用户消息到后端；Assistant 消息通过 `applyEvent` 和 `cancelRun` 直接写入前端内存和 localStorage，**未写入后端数据库**。

这导致 `resumeSession` 从后端加载时，只能恢复用户消息，assistant 回复丢失。

## 目标

在 `applyEvent`（run_finished / completion / run_failed / error）和 `cancelRun` 中，将最终定稿的 assistant 消息同步到后端 `chat_messages` 表。

## 范围

- **前端**：`frontend/lib/local-agent/store.ts`
- **后端**：无需修改（`AddMessage` 已支持任意 role）

## 验收标准

1. 完整对话（用户提问 + assistant 回复）后，后端 `chat_messages` 表中包含 role=assistant 的记录
2. `resumeSession` 从后端加载时，assistant 消息完整恢复
3. `cancelRun` 后，已生成的部分 assistant 内容也同步到后端
