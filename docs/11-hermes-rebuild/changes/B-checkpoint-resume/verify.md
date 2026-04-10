# 验证记录

## 验证方式

- 单元测试：已补一部分，覆盖 retry checkpoint 查询与 retry request 重建；`runtime-core` 侧新增了确认恢复与失败重试两类短期状态恢复测试。
- 集成测试：已完成一条 `retryable_failure` 恢复闭环样本；`after_confirmation` 路径仍待补。
- 人工验证：已确认 retry 路径不是单纯插入恢复事件，而是会继续进入统一主循环并产生后续执行事件。

## 证据位置

- 测试记录：
  - `gateway/internal/state/runtime_checkpoint_store_test.go`
  - `gateway/internal/api/chat_retry_test.go`
  - `crates/runtime-core/src/query_engine.rs`
  - `cargo test -p runtime-core`
  - `go test ./internal/api ./internal/state`
- 日志或截图：
  - `scripts/run-stage-b-retry-acceptance.ps1`
  - `tmp/stage-b-retry-acceptance/latest.json`
  - `tmp/stage-b-retry-acceptance/logs/runtime.log`
  - `tmp/stage-b-retry-acceptance/logs/gateway.log`

## 联调样本

- 时间：2026-04-10 23:17（Asia/Shanghai）
- 会话：`stage-b-retry-acceptance-1775834258001`
- 初始 run：`run-1775834266276-2`
- 初始 checkpoint：`run-1775834266276-2-1775834266891`
- retry checkpoint：`run-1775834266276-2-1775834269201`
- 关键事实：
  - 初始 run 日志包含 `checkpoint_written`。
  - retry run 与初始 run 保持同一个 `run_id`。
  - retry 日志包含 `checkpoint_resumed`。
  - retry 日志未出现 `checkpoint_resume_skipped`。
  - retry 恢复后继续出现 `analysis_ready`、`plan_ready`、`action_requested`、`action_completed`、`verification_completed`、`checkpoint_written` 等后续事件，说明恢复后回到了统一主循环，而不是只插入恢复提示事件。
  - retry 最终落到 `run_failed`，失败原因符合故意构造的命令失败样本，证明当前脚本已经拿到可重复的失败恢复闭环证据。

## Gate 映射

- 对应阶段 Gate：Gate-B
- 当前覆盖情况：
  - `50 轮任务无致命崩溃`：当前没有直接证据。
  - `中断恢复成功率 >= 95%`：当前已有 checkpoint 写入、retry 查询、retry request 重建、保留原 `run_id` 约束验证、短期状态恢复测试，以及 1 条 `retryable_failure` 接口级闭环样本；仍缺批量成功率统计和 `after_confirmation` 路径样本。
  - `关键事件链路完整可追溯`：当前已拿到 `checkpoint_written -> checkpoint_resumed -> 后续执行事件 -> terminal event` 的接口级证据，且本次 retry 未出现 `checkpoint_resume_skipped`。
