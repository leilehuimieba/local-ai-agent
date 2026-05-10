# Phase 2 设计文档

## 1. 后端结构化会话历史存储

### 数据库 Schema
```sql
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
CREATE INDEX idx_sessions_updated ON sessions(updated_at);

CREATE TABLE chat_messages (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    role TEXT NOT NULL,
    content TEXT NOT NULL DEFAULT '',
    blocks_json TEXT,
    timestamp TEXT NOT NULL
);
CREATE INDEX idx_messages_session ON chat_messages(session_id);
CREATE INDEX idx_messages_timestamp ON chat_messages(timestamp);
```

### API 端点
- `GET /api/v1/sessions` — 会话列表
- `GET /api/v1/sessions/:id` — 单个会话
- `DELETE /api/v1/sessions/:id` — 删除会话
- `GET /api/v1/sessions/:id/messages` — 获取消息
- `POST /api/v1/sessions/:id/messages` — 添加消息

### 前端集成
- `resumeSession(sessionId)` 优先从后端 `fetchSessionMessages` 加载，失败时 fallback 到 localStorage
- `addMessage()` 异步同步到后端 `addSessionMessage`

## 2. 移动端适配
- `h-screen` → `h-dvh`
- `< md`：左侧边栏隐藏，底部固定导航栏
- `< lg`：右侧面板改用 `Sheet` 侧滑
- `< sm`：Composer 折叠导出/新建按钮
- `MessageBubble`：`max-w-[92%]`（移动端）
- 触摸目标保持 44px+

## 3. Composer 文件上传 UI
- Paperclip 按钮 + hidden `<input type="file">`
- 支持 `.txt,.md,.pdf,.docx`，限制 5MB
- 附件列表可删除
- 当前仅前端读取文本内容拼接进消息，未接后端知识库上传 API
