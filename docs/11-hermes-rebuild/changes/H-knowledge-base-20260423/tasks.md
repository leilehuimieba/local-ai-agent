# Knowledge Base 任务列表（tasks）

更新时间：2026-04-23

| ID | 任务 | 类型 | 状态 | 验收标准 | 证据 |
|---|---|---|---|---|---|
| KB-01 | 创建 knowledge-base 类型定义 | 设计 | done | `types.ts` 包含 KnowledgeItem 完整类型 | `frontend/src/knowledge-base/types.ts` |
| KB-02 | 实现 localStorage 数据层 | 实现 | done | store.ts 支持 CRUD + 搜索 + 持久化 | `frontend/src/knowledge-base/store.ts` |
| KB-03 | 实现知识库主面板 | 实现 | done | KnowledgeBasePanel 包含顶部操作栏、筛选栏、内容区 | `frontend/src/knowledge-base/KnowledgeBasePanel.tsx` |
| KB-04 | 实现条目卡片 | 实现 | done | KnowledgeItemCard 展示标题/摘要/标签/时间 | `frontend/src/knowledge-base/KnowledgeItemCard.tsx` |
| KB-05 | 实现详情/编辑面板 | 实现 | done | KnowledgeItemDetail 支持查看和编辑 Markdown | `frontend/src/knowledge-base/KnowledgeItemDetail.tsx` |
| KB-06 | 实现添加条目模态框 | 实现 | done | AddItemModal 支持填写标题/分类/标签/内容/来源 | `frontend/src/knowledge-base/AddItemModal.tsx` |
| KB-07 | 集成导航 | 实现 | done | LeftNav、TaskNavRail、AppView 均增加 knowledge | `frontend/src/shell/LeftNav.tsx` 等 |
| KB-08 | 集成视图渲染 | 实现 | done | workspaceViewModel 增加 renderKnowledgeBaseView | `frontend/src/shell/workspaceViewModel.tsx` |
| KB-09 | 验证构建通过 | 验证 | done | `npm run build` 无错误 | 构建日志 |
| KB-10 | 更新 change 索引 | 文档 | done | INDEX.md 已添加新 change | `docs/11-hermes-rebuild/changes/INDEX.md` |
| KB-11 | 后端 API 设计与实现 | 实现 | done | Go gateway 新增 `/api/v1/knowledge/*` 路由，支持 CRUD + 搜索 | `gateway/internal/knowledge/` |
| KB-12 | 前端 store 切换为 API 调用 | 实现 | done | store.ts 从 localStorage 切换到 fetch API，api.ts 封装后端调用 | `frontend/src/knowledge-base/api.ts`, `frontend/src/knowledge-base/store.ts` |
| KB-13 | 前后端联调构建验证 | 验证 | done | Go build + npm run build 均通过 | 构建日志 |
| KB-14 | 文件上传支持（TXT/PDF） | 实现 | done | 后端支持上传+文本提取，前端支持切换手动/上传模式 | `gateway/internal/knowledge/extract.go`, `frontend/src/knowledge-base/AddItemModal.tsx` |
| KB-15 | 文件上传构建验证 | 验证 | done | Go build + npm run build 均通过 | 构建日志 |
| KB-16 | 任务页知识库引用选择器 | 实现 | done | ChatPanel 新增 KnowledgeBaseSelector，支持自动/不引用/具体条目 | `frontend/src/chat/ChatPanel.tsx` |
| KB-17 | 知识库 ID 透传至后端 Chat | 实现 | done | App.tsx -> api.ts -> gateway chat handler -> context hints | `frontend/src/App.tsx`, `frontend/src/chat/api.ts`, `gateway/internal/api/chat_context_resolver.go` |
| KB-18 | 引用选择器构建验证 | 验证 | done | Go build + npm run build 均通过 | 构建日志 |
| KB-19 | 知识库内直接提问（RAG）后端 | 实现 | done | `/api/v1/knowledge/ask` 端点，支持检索 + LLM 调用 | `gateway/internal/knowledge/ask.go` |
| KB-20 | 知识库内直接提问（RAG）前端 | 实现 | done | KnowledgeBasePanel 新增提问区域，展示回答和引用来源 | `frontend/src/knowledge-base/KnowledgeBasePanel.tsx` |
| KB-21 | RAG 构建验证 | 验证 | done | Go build + npm run build 均通过 | 构建日志 |
| KB-22 | 后端存储迁移至 SQLite（与 Memory 同库） | 实现 | done | `store.go` 改用 SQLite，`migrate.go` 自动导入旧 JSON 数据 | `gateway/internal/knowledge/store.go`, `gateway/internal/knowledge/migrate.go` |
| KB-23 | SQLite 迁移单元测试 | 测试 | done | `store_test.go` 覆盖 CRUD + 迁移 | `gateway/internal/knowledge/store_test.go` |
| KB-24 | SQLite 迁移构建验证 | 验证 | done | `go build` + `go test` + `npm run build` 均通过 | 构建日志 |
| KB-25 | 支持 DOCX 文件上传提取 | 实现 | done | `extract.go` 新增 `extractDocx`，前端 accept 扩展 .docx，单元测试通过 | `gateway/internal/knowledge/extract.go`, `frontend/src/knowledge-base/AddItemModal.tsx`, `gateway/internal/knowledge/extract_test.go` |
| KB-26 | RAG 向量检索优化 | 实现 | done | `ask.go` 优先使用 Embedding 余弦相似度排序，失败 fallback 关键词匹配；Create/Update/Upload 后异步生成 Embedding | `gateway/internal/knowledge/embedding.go`, `gateway/internal/knowledge/ask.go`, `gateway/internal/knowledge/handler.go`, `gateway/internal/knowledge/store.go`, `gateway/internal/config/config.go` |
| KB-27 | RAG 向量检索单元测试 | 测试 | done | `embedding_test.go` 覆盖 CosineSimilarity + rankItemsKeyword | `gateway/internal/knowledge/embedding_test.go` |
| KB-28 | RAG 向量检索构建验证 | 验证 | done | `go build` + `go test` + `npm run build` 均通过 | 构建日志 |
