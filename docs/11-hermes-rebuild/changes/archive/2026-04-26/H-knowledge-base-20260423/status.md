# Knowledge Base 状态（status）

最近更新时间：2026-04-23
状态：已完成（前端 UI + 后端 API + 文件上传全部联调通过）

## 当前状态

1. 前端组件全部实现：
   - `types.ts` / `api.ts` / `store.ts`：类型 + API 封装 + 数据层
   - `KnowledgeItemCard.tsx` / `KnowledgeItemDetail.tsx` / `AddItemModal.tsx`：UI 组件
   - `KnowledgeBasePanel.tsx`：主面板（支持 loading / error 状态）
2. 后端 API 全部实现：
   - `gateway/internal/knowledge/models.go`：数据模型
   - `gateway/internal/knowledge/store.go`：SQLite 存储（与 Memory 同库 `data/storage/main.db`）
   - `gateway/internal/knowledge/handler.go`：HTTP handler
   - `gateway/internal/knowledge/extract.go`：文本提取（TXT + PDF + DOCX）
   - `router.go` 已注册路由
3. 文件上传功能：
   - 后端：`/api/v1/knowledge/upload` 接收 multipart/form-data
   - 支持 .txt / .md / .pdf / .docx 文本提取
   - 提取后自动创建知识条目（标题=首行，摘要=前200字）
   - 前端：AddItemModal 支持"手动输入"和"上传文件"两种模式
4. 导航已集成：LeftNav、TaskNavRail、AppView、workspaceViewModel
5. 构建验证通过：
   - `go build -o server.exe ./cmd/server` 通过
   - `npm run build` 通过

## 已交付 API

| 路由 | 方法 | 功能 |
|---|---|---|
| `/api/v1/knowledge/items` | GET | 列出所有条目 + 分类/标签聚合 |
| `/api/v1/knowledge/items` | POST | 创建条目 |
| `/api/v1/knowledge/items/:id` | GET | 获取单条 |
| `/api/v1/knowledge/items/:id` | PUT | 更新条目 |
| `/api/v1/knowledge/items/:id` | DELETE | 删除条目 |
| `/api/v1/knowledge/search?q=` | GET | 搜索条目 |
| `/api/v1/knowledge/upload` | POST | 上传文件并自动提取文本创建条目 |

## 数据存储

- 知识条目：`data/storage/main.db` 表 `knowledge_items`（与 `long_term_memory` 同库）
- 上传文件：`data/knowledge_base/uploads/{workspace_id}/{filename}`
- 旧 JSON 自动迁移：`NewStore` 时自动将 `data/knowledge_base/*.json` 导入 SQLite，原文件重命名为 `.json.migrated`

## 阻塞点

- 无

## 下一步

- 如需任务页引用知识库，作为后续迭代
- 如需支持更多格式（如 .xlsx、.pptx），可进一步扩展 extract.go
- RAG 检索：当前使用 Embedding API 向量相似度，需 provider 配置 `embeddings_path` 和 `embedding_model`
- 向量检索失败或无 embedding 数据时，自动 fallback 到关键词匹配
- 前端测试：API 层、Store 层、组件层均有单元测试覆盖（24 项通过）
- 如需向量检索，可引入 sqlite-vec 或迁移到专用向量数据库
