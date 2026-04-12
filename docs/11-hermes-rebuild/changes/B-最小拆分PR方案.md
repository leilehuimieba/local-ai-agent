# 最小拆分 PR 方案（模块化收口）

更新时间：2026-04-10
状态：建议执行
适用范围：gateway / runtime-core / frontend contracts

## 1. 目标

1. 在不改变业务行为的前提下，降低核心文件体量和耦合度。
2. 每个 PR 可独立评审、独立回滚。
3. 保持与当前线上行为一致，优先做“结构拆分”，后做“逻辑优化”。

## 2. 约束

1. 严格遵守仓库规则：新增或修改函数单函数不超过 30 行。
2. 每个 PR 必须包含可执行验证。
3. 不在同一个 PR 同时做“结构拆分 + 功能新增”。

## 3. PR 分批

### PR-1：前端合同类型拆分（最低风险）

目标：把 `frontend/src/shared/contracts.ts` 按领域拆为多个文件，仅做导出重组。

建议改动：

1. 新增 `frontend/src/shared/contracts/base.ts`
2. 新增 `frontend/src/shared/contracts/settings.ts`
3. 新增 `frontend/src/shared/contracts/runtime.ts`
4. 新增 `frontend/src/shared/contracts/memory.ts`
5. 保留 `frontend/src/shared/contracts.ts` 作为 barrel export（只 `export *`）

验收：

1. `frontend` 下 `npm run build` 通过。
2. 运行时页面与设置页类型提示不退化。

---

### PR-2：Gateway ChatHandler 解析职责拆分

目标：把 provider/workspace/context hint 解析从 `chat.go` 抽离，Handler 只保留 HTTP 入参与调度。

建议改动：

1. 新增 `gateway/internal/api/chat_context_resolver.go`
2. 新增 `gateway/internal/api/chat_provider_resolver.go`
3. 在 `chat.go` 中仅保留：`Run/Confirm/Retry` 入口、`execute` 调度、错误回写。

拆分边界：

1. `buildRunRequest` 只组织调用，不承载细节策略。
2. `resolveRunContext`、`resolveWorkspace`、`runContextHints` 迁入 context resolver。
3. `resolveProviderRef` 及其链路迁入 provider resolver。

验收：

1. `gateway` 下 `go test ./...` 通过。
2. `/api/v1/chat/run` 正常返回 `202 accepted`。

---

### PR-3：Gateway 确认/重试/记忆写入链路拆分

目标：把 confirmation/retry 相关逻辑从 `chat.go` 分离为独立服务文件。

建议改动：

1. 新增 `gateway/internal/api/chat_confirmation_service.go`
2. 新增 `gateway/internal/api/chat_retry_service.go`
3. 新增 `gateway/internal/api/chat_memory_event_service.go`

拆分边界：

1. `Confirm` 只做入参校验与 service 调用。
2. `Retry` 只做入参校验与 service 调用。
3. `publishConfirmationClosure` 及相关 memory event 生成逻辑移出 `chat.go`。

验收：

1. 保持现有测试通过：`go test ./...`。
2. 新增至少 2 个 `testify` 用例：
   - retryable checkpoint 命中
   - confirmation reject/cancel 事件落地

---

### PR-4：runtime-core 执行编排入口瘦身（关键）

目标：把 `simulate_run` 巨函数拆成“流程 + 阶段处理器”。

建议改动：

1. 新增 `crates/runtime-core/src/run_pipeline.rs`
2. 新增 `crates/runtime-core/src/run_stages.rs`
3. `lib.rs` 仅保留外部入口和最薄适配。

拆分边界：

1. `run_pipeline`：状态推进与 sequence 管理。
2. `run_stages`：Analyze/Plan/Execute/Verify/Finish 的阶段函数。
3. `events` 继续由现有事件工厂提供，避免重复建模。

验收：

1. `cargo test -p runtime-core` 通过。
2. `cargo build -p runtime-host` 通过。
3. 对关键路径做快照对比：`run_started -> ... -> run_finished` 事件序列不回退。

## 4. 每个 PR 的统一检查清单

1. 构建检查：`cargo build`、`go build ./...`、`frontend npm run build`。
2. 接口检查：`/api/v1/chat/run`、`/api/v1/chat/retry`、`/api/v1/chat/confirm`。
3. 回归样本：`scripts/run-v1-regression-check.ps1` 可执行。
4. 文档同步：仅更新涉及模块的入口文档，不做额外扩散。

## 5. 执行顺序建议

1. 先做 PR-1（前端类型拆分，低风险）。
2. 再做 PR-2、PR-3（Gateway 行为不变拆分）。
3. 最后做 PR-4（runtime-core 编排瘦身，风险最高）。

## 6. 回滚策略

1. 每个 PR 独立合并。
2. 任一 PR 失败可单独 `revert`，不影响其他拆分批次。
3. PR-4 必须最后合并，避免与 Gateway 改动叠加放大风险。
