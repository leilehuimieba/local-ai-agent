# B 阶段执行记录 05：retryable failure 恢复入口接入

更新时间：2026-04-10
阶段：B（进行中）

## 1. 本次目标

1. 为 `retryable_failure` 类型 checkpoint 增加 gateway 侧恢复入口。
2. 把“失败后重试”从事件语义升级为正式 API 能力。
3. 保持当前恢复策略仍为“入口重入”，不提前进入阶段级继续执行。

## 2. 本次改动范围

1. Go：`gateway/internal/api/chat.go`
2. Go：`gateway/internal/api/router.go`
3. Go：`gateway/internal/state/runtime_checkpoint_store.go`
4. TS：`frontend/src/shared/contracts.ts`
5. TS：`frontend/src/chat/api.ts`

## 3. 关键改动

### 3.1 Gateway 新增失败重试入口

1. 新增 `POST /api/v1/chat/retry`。
2. 入参最小集合为：
3. `session_id`
4. `run_id`
5. `checkpoint_id` 可选。
6. 若未显式指定 `checkpoint_id`，gateway 会按 `run_id + session_id` 查询最近一条 `retryable_failure` checkpoint。

### 3.2 checkpoint 查询独立模块化

1. 新增 `RuntimeCheckpointStore`。
2. 直接读取 `data/storage/main.db` 中的 `runtime_checkpoints` 表。
3. 只暴露当前 B-05 所需能力：
4. 按 `checkpoint_id` 查询。
5. 按 `run_id + session_id` 查询最新可重试 checkpoint。

### 3.3 失败重试幂等边界

1. retry 仅接受：
2. `resumable=true`
3. `resume_reason=retryable_failure`
4. checkpoint 所属 `run_id/session_id` 必须与请求匹配。
5. 若 checkpoint 不存在，返回 `404`。
6. 若 checkpoint 存在但不属于失败重试场景，返回 `400`。

### 3.4 凭据恢复补齐

1. checkpoint 落盘前会清空 `provider_ref.api_key`。
2. gateway 在重建 retry request 时，会重新按当前 provider 配置解析凭据。
3. 避免出现“checkpoint 可读，但恢复时没有可用 API Key”的伪闭环。

### 3.5 前端合同预接入

1. 共享合同新增 `ChatRetryRequest`。
2. `frontend/src/chat/api.ts` 新增 `submitChatRetry`。
3. 当前先不直接暴露 UI 按钮，只冻结调用面，避免后续再改口径。

## 4. 当前恢复语义

1. `after_confirmation`：
2. 仍用于确认完成后的恢复重入。
3. `retry_failure`：
4. 用于失败重试的恢复重入。
5. 两者当前都属于“入口重入 + checkpoint 作为恢复依据”。
6. 当前仍未实现真正的“从中间阶段继续执行”。

## 5. 验证结果

1. `go build ./...`：通过。
2. 前端合同改动为类型与 API 封装补充，未引入新依赖。

## 6. 当前结论

1. B-05 已完成 gateway 侧失败重试入口最小闭环。
2. retryable failure 现在不再只是 runtime 内部语义，已经可被 host 正式发起恢复。
3. 下一步可以先做接口级联调，再决定是否进入真正阶段级继续执行。

## 7. 下一步（B-06）

1. 为 retry API 增加联调验证与最小自动化测试。
2. 评估前端是否需要显式“重试当前任务”按钮。
3. 决定是否进入基于 `resume_stage` 的阶段级恢复实现。
