# A 阶段 Runtime Run Contract v1

更新时间：2026-04-10
阶段：A（冻结）
适用模块：`runtime-core`、`runtime-host`、`gateway`

## 1. 目标

1. 冻结 Runtime 与 Gateway 的 `run` 交互合同，避免 B 阶段改造时协议漂移。
2. 明确当前已实现字段、必填字段、顺序约束、兼容策略。
3. 预留 B 阶段恢复能力扩展字段，不破坏现有调用方。

## 2. 接口范围

1. `POST /v1/runtime/run`
2. `GET /v1/runtime/info`
3. `GET /v1/runtime/capabilities`
4. `GET /v1/runtime/connectors`
5. `GET /health`

## 3. RunRequest v1

来源：

1. Rust 定义：`crates/runtime-core/src/contracts.rs`
2. Go 对齐：`gateway/internal/contracts/contracts.go`

必填字段：

1. `request_id`
2. `run_id`
3. `session_id`
4. `trace_id`
5. `user_input`
6. `mode`
7. `model_ref`
8. `workspace_ref`

可选字段：

1. `provider_ref`
2. `context_hints`
3. `confirmation_decision`

字段约束：

1. `mode` 仅允许：`observe`、`standard`、`full_access`。
2. `workspace_ref.root_path` 必须是绝对路径。
3. `run_id` 与 `trace_id` 在单会话内不得复用。
4. `confirmation_decision` 仅在等待确认后重试时传入。

## 4. RuntimeRunResponse v1

结构：

1. `events: RunEvent[]`
2. `result: RunResult`
3. `confirmation_request?: ConfirmationRequest`

事件序列约束：

1. `sequence` 必须严格递增。
2. 首事件必须是 `run_started`。
3. 终事件必须是 `run_finished` 或 `run_failed`。
4. 若存在 `confirmation_request`，`result.status` 必须为 `awaiting_confirmation`。

状态约束：

1. `result.status` 允许值：`completed`、`failed`、`awaiting_confirmation`。
2. `result.final_stage` 必须与最终事件 `stage` 对齐。
3. `result.error` 仅在非 `completed` 时允许非空。

## 5. ErrorInfo v1

字段：

1. `error_code`
2. `message`
3. `summary`
4. `retryable`
5. `source`
6. `stage`
7. `metadata`

约束：

1. `error_code` 必须稳定，不随文案变化。
2. `retryable=true` 必须可给出下一步建议。
3. `source` 允许：`runtime`、`gateway`、`tool`。

## 6. 兼容策略

1. 新增字段必须保持可选，不得让旧调用方解析失败。
2. 不得重命名或删除 v1 已冻结字段。
3. 若字段语义变化，必须通过 `metadata.contract_version` 标注。

## 7. B 阶段预留字段（冻结但暂不启用）

在 v1 中保留以下预留字段，B 阶段可增量启用：

1. `RunRequest.resume_from_checkpoint_id?: string`
2. `RunRequest.resume_strategy?: string`
3. `RunResult.checkpoint_id?: string`
4. `RunResult.resumable?: bool`
5. `RunEvent.checkpoint_written?: bool`

启用规则：

1. 启用时必须保持旧请求可运行。
2. 未启用前默认忽略，不报错。

## 8. 验收清单

1. Gateway 与 Runtime 的 `RunRequest`/`RuntimeRunResponse` 字段一一对齐。
2. `scripts/run-v1-regression-check.ps1` 增补 contract 对齐校验项。
3. 至少 20 条 `run` 样本通过序列约束检查。

## 9. 责任归属

1. Runtime Contract Owner：`runtime-core`。
2. Gateway Contract Owner：`gateway/internal/contracts`。
3. 联调验收 Owner：`gateway/internal/runtime`。
