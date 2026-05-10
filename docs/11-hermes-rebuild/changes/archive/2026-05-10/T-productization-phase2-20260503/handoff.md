# 工作交接文档（Handoff）

> 生成时间：2026-05-03 11:10
> 当前状态：Phase 2 全部完成，无活跃 change，项目处于自由迭代期

---

## 一、项目概况

**本地智能体（Local Agent）** —— 本地优先的 AI 工作空间

- **前端**：Next.js 16.2.4 + React + TypeScript + Tailwind + shadcn/ui，静态导出由 Gateway 托管
- **Gateway**：Go 1.22+，纯标准库 `net/http`，SQLite（`modernc.org/sqlite`）
- **Runtime**：Rust，`runtime-core` + `runtime-host`
- **工作目录**：`d:\newwork\本地智能体`
- **系统**：Windows（PowerShell）

---

## 二、最近完成的所有改动

### Phase 1（S-change，2026-05-02）— 已归档

| 改动 | 文件 |
|---|---|
| Error Boundary | `frontend/components/error-boundary.tsx` |
| 全局 Toast（sonner@1.7.4） | `frontend/app/layout.tsx` 引入 `<Toaster />` |
| Session 隔离 localStorage | `frontend/lib/local-agent/store.ts` — key 改为 `la:session:${sessionId}` |
| 单元测试 11 项 | `frontend/lib/local-agent/__tests__/store.test.ts`（4 项）+ `frontend/components/local-agent/__tests__/markdown.test.tsx`（7 项）|
| 用户快速入门文档 | `docs/09-user-guide/快速入门.md` |

### Phase 2（T-change，2026-05-03）— 已归档

#### 任务 1：后端结构化会话历史存储

- **Gateway 数据层**：`gateway/internal/session/store.go`
  - 表：`sessions`（id, title, created_at, updated_at）+ `chat_messages`（id, session_id, role, content, blocks_json, timestamp）
  - `SessionStore` 提供 List/Get/Create/Touch/Delete Session 和 Get/Add Message
- **Gateway API**：`gateway/internal/api/router_sessions.go`
  - `GET /api/v1/sessions` → `{"items": [...]}`
  - `GET /api/v1/sessions/:id` → session 详情
  - `DELETE /api/v1/sessions/:id`
  - `GET /api/v1/sessions/:id/messages` → `{"items": [...]}`
  - `POST /api/v1/sessions/:id/messages` → 添加消息（body: `{role, content, timestamp?, blocks?}`）
- **路由注册**：`gateway/internal/api/router.go` 第 43 行 `registerSessionRoutes(mux, sessions)`
- **前端 API**：`frontend/lib/local-agent/api.ts`
  - 新增 `fetchSessions()`、`fetchSessionMessages(sessionId)`、`addSessionMessage(sessionId, payload)`
- **前端 Store**：`frontend/lib/local-agent/store.ts`
  - `resumeSession(sessionId)`：优先从后端 `fetchSessionMessages` 加载，失败 fallback 到 localStorage
  - `addMessage()`：状态更新后异步调用 `addSessionMessage` 同步到后端（忽略网络错误，不阻塞 UI）

#### 任务 2：移动端适配

| 文件 | 改动 |
|---|---|
| `frontend/app/page.tsx` | `h-screen` → `h-dvh` |
| `frontend/components/local-agent/left-sidebar.tsx` | `< md` 隐藏侧边栏，底部固定导航栏（任务/历史/知识/设置 + 新任务），`pb-safe` |
| `frontend/components/local-agent/right-drawer.tsx` | `< lg` 改用 `Sheet` 侧滑面板（`@/components/ui/sheet`） |
| `frontend/components/local-agent/top-bar.tsx` | 视图标签页 `hidden md:flex`，logo 缩小 |
| `frontend/components/local-agent/views/task-view.tsx` | `< sm` 折叠导出/新建按钮，触摸目标 44px+ |
| `frontend/components/local-agent/message-bubble.tsx` | `max-w-[80%]` → `max-w-[92%]`（移动端） |
| `frontend/hooks/use-mobile.ts` | `useIsMobile()` hook 已存在 |

#### 任务 3：Composer 文件上传 UI

- `frontend/components/local-agent/views/task-view.tsx`
  - 隐藏 `<input type="file" accept=".txt,.md,.pdf,.docx">` + Paperclip 按钮
  - `attachments` 状态（`{name, content}[]`），支持删除
  - `handleFileSelect`：读取文本文件内容，限制 5MB，拼接到消息内容
  - **注意**：目前仅前端拼接进消息，未调用后端知识库上传 API

---

## 三、构建与启动命令

### 前端
```powershell
cd frontend
# 安装依赖（如 pnpm store 路径变动，加 --force）
pnpm install --force
# 类型检查
npx tsc --noEmit
# 构建（静态导出到 frontend/dist/）
npm run build
# 测试
npm run test
```

### Gateway
```powershell
cd gateway
# 编译
go build -o gateway.exe ./cmd/server
# 启动（端口 38471）
$env:LOCAL_AGENT_GATEWAY_PORT=38471
Start-Process -FilePath ".\gateway.exe" -WorkingDirectory "d:\newwork\本地智能体\gateway" -NoNewWindow -RedirectStandardOutput "d:\newwork\本地智能体\logs\gateway.log" -RedirectStandardError "d:\newwork\本地智能体\logs\gateway.err.log"
# 验证
Invoke-RestMethod -Uri "http://127.0.0.1:38471/health"
```

### 重要注意
- **端口 38471 只能被一个进程占用**。如果之前 launcher 启动了 `server.exe`（旧 Gateway），需要先用 `taskkill /F /IM server.exe` 杀掉，再启动新编译的 `gateway.exe`
- 用 `netstat -ano | findstr "38471"` + `tasklist /FI "PID eq <PID>"` 确认占用进程
- Gateway 有 token 鉴权，API 请求需带 `X-Local-Agent-Token` header（值在 `data/.gateway_token`）

### Runtime（如需启动）
```powershell
# runtime-host 在端口 38472
cd target\release
$env:LOCAL_AGENT_RUNTIME_PORT=38472
.\runtime-host.exe
```

---

## 四、关键文件速查

```
# 前端核心
frontend/lib/local-agent/store.ts          # Zustand store（session 隔离、后端同步）
frontend/lib/local-agent/api.ts            # 前端 API 层（新增 session API）
frontend/lib/local-agent/types.ts          # TypeScript 类型定义
frontend/app/page.tsx                      # 根布局（h-dvh）
frontend/app/layout.tsx                    # Toaster + ErrorBoundary
frontend/components/local-agent/views/task-view.tsx      # 主聊天界面（Composer、文件上传）
frontend/components/local-agent/left-sidebar.tsx         # 底部导航栏
frontend/components/local-agent/right-drawer.tsx         # Sheet 侧滑
frontend/components/error-boundary.tsx     # React Error Boundary

# Gateway 核心
gateway/internal/session/store.go          # SQLite 会话存储（新增）
gateway/internal/session/bus.go            # EventBus（内存事件总线 + JSONL 日志）
gateway/internal/api/router.go             # 路由注册中心（新增 session 路由）
gateway/internal/api/router_sessions.go    # Session REST API handler（新增）
gateway/internal/api/router_chat.go        # 聊天路由
gateway/internal/api/chat.go               # ChatHandler
gateway/internal/contracts/contracts.go    # DTO 结构体

# 文档
AGENTS.md                                  # 项目规则（用中文、单函数 30 行、commit 中文 ≤50 字）
docs/11-hermes-rebuild/current-state.md    # 当前状态（单一事实源）
docs/11-hermes-rebuild/changes/INDEX.md    # change 索引
docs/11-hermes-rebuild/changes/T-productization-phase2-20260503/  # Phase 2 归档
```

---

## 五、已知问题与待办

1. ~~文件上传未接后端~~ → **W-change 已完成**：`handleFileSelect` 中异步调用 `uploadKnowledgeFile`，支持 pdf/docx，5MB 限制。
2. ~~后端未存储 assistant 消息~~ → **U-change 已完成**：`applyEvent` / `cancelRun` finalize 时异步写入 assistant 消息到后端。
3. **后端消息存储未存储 `blocks`**：前端 `Message` 有 `blocks?: ResultBlock[]`，后端 `chat_messages` 表有 `blocks_json` 字段，但 `addSessionMessage` 目前未传递 blocks。如需完整支持，需要在前端 `addMessage` 同步时一并发送 blocks。
4. ~~历史会话列表 UI~~ → **V-change 已完成**：LogsView 增加"会话历史"标签页。
5. ~~Playwright E2E 测试覆盖移动端~~ → **X-change 已完成**：4 项移动端布局/交互测试全绿。
6. **Gateway 进程管理**：`gateway.exe` 和 launcher 启动的 `server.exe` 都会监听同一端口，容易冲突。建议统一使用 `gateway.exe`。

5. **pnpm store 路径变动**：如果前端构建出现 `ERR_PNPM_UNEXPECTED_STORE`，用 `pnpm install --force` 解决。

---

## 六、推荐的下一步（供参考）

- **Phase 3 候选**：
  - 文件上传接入后端知识库（复用 `gateway/internal/knowledge/handler.go` 的 upload/extract）
  - 历史会话列表 UI（前端新增"历史"视图，调用 `fetchSessions`）
  - assistant 消息同步到后端（在 event 处理完成后写入 `chat_messages`）
  - 后端 blocks 字段完整支持
  - Playwright E2E 测试覆盖移动端场景

---

## 七、环境信息

- OS：Windows 11，PowerShell
- Node：20.x，pnpm
- Go：1.22+
- Rust：stable
- Gateway URL：`http://127.0.0.1:38471`
- Runtime URL：`http://127.0.0.1:38472`
- 前端 Dev URL：`http://127.0.0.1:38473`
- Token 文件：`data/.gateway_token`
- SQLite 数据库：`data/storage/main.db`
