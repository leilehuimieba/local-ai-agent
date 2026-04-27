# Knowledge Base 验证（verify）

更新时间：2026-04-23

## 验证方式

1. 构建验证：`npm run build` 无 TypeScript 错误
2. 功能验证：导航可切换、CRUD 可用、SQLite 持久化
3. 设计验证：不阻塞 Gate-H，不修改现有 Memory 功能

## 证据位置

- `frontend/src/knowledge-base/`
- `frontend/src/shell/LeftNav.tsx`
- `frontend/src/shell/workspaceViewModel.tsx`
- `frontend/src/App.tsx`
- `frontend/src/knowledge-base/api.test.ts`
- `frontend/src/knowledge-base/store.test.ts`
- `frontend/src/knowledge-base/KnowledgeBasePanel.test.tsx`
- `gateway/internal/knowledge/store.go`
- `gateway/internal/knowledge/migrate.go`
- `gateway/internal/knowledge/store_test.go`
- `gateway/internal/knowledge/extract.go`
- `gateway/internal/knowledge/extract_test.go`
- `gateway/internal/knowledge/embedding.go`
- `gateway/internal/knowledge/embedding_test.go`

## 验收标准

1. 左侧导航出现"知识库"图标
2. 知识库页面展示条目卡片网格
3. 可添加/编辑/删除条目
4. 数据刷新后仍保留（SQLite）
5. 文件上传支持 .txt / .md / .pdf / .docx
6. RAG 问答优先使用向量检索（Embedding 余弦相似度），失败自动 fallback 关键词匹配
7. 前端 API/Store/Panel 均有单元测试，vitest 全绿
5. 空状态友好提示
