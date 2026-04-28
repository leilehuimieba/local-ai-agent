# 交接提示词：Go api 包模块化拆分

## 当前状态快照

- **项目阶段**：自由迭代期，阶段 I（可持续交付与工程治理）已收口
- **总路线**：A~H 已完成并归档，无活跃 change
- **Git 工作区**：`main` 分支，最近的 2 次 commit 是 api 包拆分
- **编译状态**：`gateway/` Go 部分编译通过（`go build ./internal/api/...` 绿），`go test ./internal/api/` 核心测试通过

## 最近完成的工作

### 1. provider_settings.go 拆分（已提交 b295645）
- `provider_settings.go` 601行 → `provider_settings.go`(152) + `provider_models.go`(93) + `provider_helpers.go`(308)
- 边界：models 存类型、helpers 存业务逻辑、settings 存路由入口

### 2. router.go 拆分（已提交 bab9d97）
- `router.go` 627行 → 7 个文件，拆分后文件大小如下：

| 文件 | 行数 | 职责 |
|------|------|------|
| `router.go` | 147 | 路由组装(NewRouter) + SPA handler + 运行时状态探测 |
| `router_types.go` | 135 | 全部类型定义（SettingsResponse/DiagnosticsStatus/ExternalConnectionSlot/memoryRouteDeps 等） |
| `router_external.go` | 129 | 外部连接 action handler + validateLocalFilesProject/validateLocalNotesKnowledge + checkAccessibleDirectory |
| `router_memory.go` | 117 | memory CRUD handler + toContractMemories + currentWorkspaceID |
| `router_logs.go` | 70 | systemInfoHandler + logsHandler + queryLogItems + applyLogsQueryFilter |
| `router_settings.go` | 60 | settingsHandler + applySettingsUpdate |
| `router_diagnostics.go` | 55 | diagnosticsCheckHandler + finalizeDiagnostics + warnings/errors 汇总 + appendIfMissing |
| `router_release.go` | 40 | release 路由（之前已存在） |
| `router_providers.go` | 35 | provider 路由注册（之前已存在） |
| `router_chat.go` | 11 | chat 路由注册（之前已存在） |
| `router_learning.go` | 14 | learning 路由注册（之前已存在） |

- `api` 包已无超 600 行文件

## 未提交改动（不属于当前拆分，但需注意）

```
M  frontend/.vite-dev.err
M  gateway/internal/config/config.go
M  gateway/internal/knowledge/ask.go
M  gateway/internal/knowledge/embedding_test.go
M  gateway/internal/knowledge/handler.go
M  gateway/internal/knowledge/models.go
M  gateway/internal/knowledge/store.go
?? gateway/internal/knowledge/category.go
?? gateway/internal/knowledge/chunk.go
?? gateway/internal/knowledge/chunk_test.go
?? gateway/internal/knowledge/classify.go
?? gateway/internal/knowledge/classify_test.go
?? gateway/cmd/import-bestblogs/
?? gateway/cmd/reindex-embeddings/
```

这些是之前未提交的 knowledge 包重构改动，与 api 包拆分无关。如需继续，需单独评估。

## 待办 / 下一步建议

1. **继续 Go api 包拆分（可选）**：
   - `chat_confirmation_memory.go` 311行 — 接近红线，可按"确认流程"拆出
   - `chat.go` 270行 — 接近红线，可按"聊天生命周期"拆出
   - 策略：参考已完成的拆分模式（类型→helpers→handler）

2. **处理未提交 knowledge 包改动**：
   - 评估是否提交、回退或继续完善
   - `gateway/cmd/reindex-embeddings/` 有编译报错（`undefined: knowledge.FindProviderByID`），不是当前拆分引入的

3. **15条 empty 记录**：
   - 3条词汇PDF + 12条扫描件
   - 待百度 OCR 额度恢复后执行 `go run ./cmd/reimport-ocr/main.go` 重试

## 关键路径

- 项目根目录：`D:/newwork/本地智能体`
- Go 后端：`gateway/`（Go 1.25）
- api 包：`gateway/internal/api/`
- 阶段计划：`docs/11-hermes-rebuild/`
- 当前状态：`docs/11-hermes-rebuild/current-state.md`

## 编译验证命令

```powershell
cd gateway
go build ./internal/api/...
go test ./internal/api/ -run "TestBuildRetry|TestResolve|TestLearningProvider|TestBestblogs|TestLearningExtract" -count=1 -timeout 15s
```

## 注意事项

- 文件写入时曾遇到 VS Code / 外部文件 watcher 恢复旧内容的问题。若大文件写入后行数未变，请用 `git status` 和 `Get-Content | Measure-Object` 双重确认
- 拆分遵循同包（`package api`）原则，不引入新包，避免循环依赖
- 所有拆出的函数保持原有签名，调用方无需修改
