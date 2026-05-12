# 交接提示词

> 本文件面向接替的对话实例。阅读时间约 2 分钟。

## 1. 项目概况

全栈本地智能体（Full-stack local agent），三件套架构：

| 层 | 技术栈 | 目录 |
|---|---|---|
| Runtime | Rust (edition 2024) | `crates/runtime-core/` |
| Gateway | Go 1.25.0, SQLite (modernc.org/sqlite) | `gateway/` |
| Frontend | Next.js 15, React 19, pnpm 10, Node 24 | `frontend/` |

- **构建系统**：Cargo + Go modules + pnpm
- **CI**：GitHub Actions（`.github/workflows/ci.yml`）
- **测试**：Rust `cargo test` / Go `go test` / Frontend Vitest + RTL + Playwright E2E
- **Lint**：Rust clippy `-D warnings` / Go golangci-lint v6 / Frontend ESLint v9 flat config
- **项目阶段**：自由迭代期（阶段 I 已收口，总路线 A~H 已完成）

## 2. 当前状态（截至 2026-05-10）

### 活跃 change
（无）自由迭代期，无活跃 change。

### 近期已完成（P0→P3 全部交付）

| Change | 内容 | 状态 |
|---|---|---|
| AD | PR #2 CI 修复（ESLint v9 迁移、Go lint、gofmt、Rust clippy、BOM 清理、launcher build tag） | 已归档 |
| AE | Frontend 测试覆盖率补充（api/store/toast/EventStream，118 项测试，33%→62% Lines） | 已归档 |
| AF | Go errcheck 逐个修复（移除 `.golangci.yml` 排除规则，32 文件 80+ 处显式忽略） | 已归档 |
| AG | 补充 api.ts error 分支测试（fetchSystemInfo / runDiagnosticsCheck / fetchMCPTools / callMCPTool / fetchProviderSettings / fetchKnowledgeItems / askKnowledgeBase，+7 项测试） | 已归档 |
| AH | 补充 store.ts `providerStatus` 全分支覆盖（inactive / active / error，+2 项测试） | 已归档 |
| AI | Node.js 20 Actions 升级（checkout@v5 / setup-node@v5 / setup-go@v6 / upload-artifact@v5 / codecov@v6 / pnpm-setup@v5） | 已归档 |
| AJ | 补充 mcp-observability-panel.tsx 测试（+17 项测试，views 覆盖率 13%→29%，整体 62%→65% Lines） | 已归档 |
| AK | 补充 logs-view.tsx 测试（+19 项测试，导出 formatDuration/formatTimestamp/LogCard，75.67% Stmts） | 已归档 |
| AL | 补充 knowledge-view.tsx 测试（+8 项测试，导出 isKnowledgeVisible/KnowledgeEmpty，34.05% Stmts） | 已归档 |
| AM | 补充 store.ts setter 与 loadItems catch 分支（+5 项测试，store 81.93% Stmts） | 已归档 |
| AN | Vitest coverage 配置排除 components/ui/**（避免 shadcn/ui 组件拉低业务代码覆盖率统计） | 已归档 |

### 验证基线（本地全绿）

```bash
# Rust
cargo check --workspace          # ✓
cargo clippy --workspace -- -D warnings  # ✓
cargo test -p runtime-core --lib         # 195 passed

# Go
cd gateway && go test ./...      # 9 个包全绿
go run github.com/kisielk/errcheck@latest ./...  # 零告警

# Frontend
cd frontend && pnpm run lint     # 0 errors, 0 warnings
pnpm run test                    # 183 passed
pnpm run test --coverage         # 62.66% Stmts / 64.71% Lines（components/ui 已排除）
```

## 3. 关键技术决策（不可变项）

### ESLint v9 flat config
- 原有 `.eslintrc` 废弃
- 绕过 `FlatCompat` 循环引用，直接用 `createRequire` 加载 `eslint-config-next` 的 CJS 导出
- 关闭 React 19 严格 hooks 规则：`react-hooks/set-state-in-effect`、`react-hooks/purity`
- 关键文件：`frontend/eslint.config.js`

### Go errcheck 策略
- **不再使用 `.golangci.yml` 排除规则**掩盖 `defer Close()`
- 统一模式：`defer func() { _ = x.Close() }()`
- 例外：`httptest.Server.Close()` 无返回值，保持 `defer server.Close()`
- 关键文件：`.golangci.yml`

### 测试 mock 策略
- api.ts：`global.fetch = vi.fn()`（注意 `process.env.NEXT_PUBLIC_API_BASE` 需在文件顶部设置，模块加载时生效）
- store.ts：zustand 单例，测试间通过 `store.setState({...})` 重置
- use-toast.ts：全局 `memoryState`，测试间会互相影响，目前只做了单组测试
- EventSource：自定义 `MockEventSource` 类，通过 `instances` 数组追踪

## 4. 已知问题 / 陷阱

### Go lint cache warning（非阻塞）
CI 上 `actions/setup-go` 的 cache restore 可能报 `Dependencies file is not found`。已在 CI 配置中补充 `cache-dependency-path: gateway/go.sum`，但 CI runner 环境可能有缓存延迟。

### Node.js 20 deprecation（远期）
部分 GitHub Actions 使用 Node.js 20，将于 **2026-09-16** 被移除。当前是 2026-05-10，还有 4 个月。升级 Actions 版本是低风险事项，可择机处理。

### BOM 遗留
部分 Go 源文件曾存在 UTF-8 BOM，已通过重写文件首行移除。如果未来从 Windows 编辑器保存文件，注意检查首行是否出现 BOM。

### 热点文件红线
- `gateway/internal/knowledge/store.go`：519 行（唯一超 500 行 Go 文件），接近 600 行红线
- Frontend CSS 已完全拆分，仅剩 `app/globals.css`

## 5. 下一步建议

自由迭代期无强制任务，可按以下方向自主推进：

| 优先级 | 方向 | 估计工作量 |
|---|---|---|
| P2 | 前端覆盖率从 65% 提升到 75%（剩余 views：task-view / settings-view，knowledge-view 和 logs-view 已部分覆盖） | 中 |
| P3 | `gateway/internal/knowledge/store.go` 目录化拆分 | 中（需进 changes 工作区） |

## 6. 文件索引

```
docs/11-hermes-rebuild/current-state.md      # 当前状态唯一权威记录
docs/11-hermes-rebuild/changes/INDEX.md      # change 目录导航
docs/11-hermes-rebuild/changes/archive/      # 已归档 change
AGENTS.md                                      # 项目规则（必读）
frontend/eslint.config.js                      # ESLint v9 flat config
frontend/vitest.config.ts                      # Vitest 配置
frontend/vitest.setup.ts                       # jest-dom 引入
gateway/.golangci.yml                          # Go lint 配置
gateway/go.mod                                 # Go 1.25.0
```

## 7. 快速上手命令

```bash
# 全量本地验证
scripts/build-all.ps1           # 构建全栈
cd frontend && pnpm run lint && pnpm run test
cd gateway && go test ./... && gofmt -l .
cargo check --workspace && cargo clippy --workspace -- -D warnings
```

---

*最后更新：2026-05-11*
