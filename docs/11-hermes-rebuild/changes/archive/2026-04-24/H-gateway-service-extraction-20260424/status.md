# H-gateway-service-extraction-20260424（status）

最近更新时间：2026-04-24
状态：已完成
状态口径：当前阶段 / 当前 Gate / 当前活跃 change 统一引用 `docs/11-hermes-rebuild/current-state.md`

## 当前状态

1. 已完成（首批）：
   - `gateway/internal/service/provider_settings.go` 新建，原 handler 瘦身至 176 行，编译通过。
   - `gateway/internal/service/diagnostics.go` 新建，原 handler 瘦身至 48 行，编译通过。
   - `gateway/internal/api/diagnostics_remediation_test.go` 随业务逻辑迁移至 `gateway/internal/service/diagnostics_test.go`，9 项测试全部通过。
2. 已完成（扩展批次）：
   - Chat 核心链路下沉：`chat_context.go`、`chat_provider.go`、`chat_retry.go`、`chat_confirmation.go`、`chat_execution.go`、`chat_execute.go`、`chat_events.go` 新建；`api/chat.go` 从 411 行瘦身至 ~200 行；测试迁移适配。
   - Learning 系列下沉：`learning_value_score.go`（205 行评分逻辑全下沉）、`learning_recommend.go`（7 个函数下沉）、`learning_explain.go`（8 个函数下沉）、`learning_translate.go`（19 个函数 + PhrasePair 下沉）；api 层改为类型别名 + 转发；`api/learning_explain_translate.go` 从 350 行瘦身至 ~90 行。
   - 构建与测试：`go build ./...` 通过；`go test ./internal/service` 与 `./internal/api` 相关测试通过。
3. 进行中：无。
4. 阻塞点：无。
5. 下一步：归档。
