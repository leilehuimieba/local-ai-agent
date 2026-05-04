# W-change: 文件上传接入后端知识库

## 背景

Phase 2 已在 Composer 中实现了文件上传 UI，但当前仅将文件文本内容拼接到用户消息中发送，**未调用后端知识库上传 API**。后端 `POST /api/v1/knowledge/upload` 已具备完整的文件保存、文本提取、知识库条目创建和嵌入生成能力。

## 目标

用户选择文件后，在保持"文本拼接到消息"体验的同时，异步将文件上传至后端知识库。

## 范围

- **前端**：`frontend/components/local-agent/views/task-view.tsx`
- **后端**：无需修改

## 验收标准

1. 选择文件后，文件内容继续拼接进用户消息
2. 同一文件异步上传至后端知识库（`uploadKnowledgeFile`）
3. 上传成功/失败给用户 Toast 反馈
4. 前端 accept 扩展为 `.txt,.md,.pdf,.docx`，size 限制扩展为 5MB
