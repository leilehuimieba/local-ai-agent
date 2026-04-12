# 验证记录

## 验证方式

- 单元测试：`go test ./...`（gateway）通过。
- 单元测试：`cargo test -p runtime-core` 通过（`64 passed; 0 failed`）。
- 接口验收：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-c-tool-elapsed-acceptance.ps1` 通过（`status=passed`）。
- 接口验收：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-c-risk-audit-acceptance.ps1` 通过（`status=passed`）。
- 批量验收：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-c-gate-batch.ps1 -Rounds 5 -RequireGateC` 通过（`status=passed`，`gate_c.ready=true`）。
- 集成测试：阶段 C 风险确认链最小闭环已通过接口级验收，并完成 Gate-C 指标化批量样本验证。
- 提审材料：`docs/11-hermes-rebuild/changes/C-tool-contract-unification/review.md` 已生成。
- 评审签收：总表任务 `C-G1` 已签收，阶段 C 收口并完成索引切换。
- 人工验证：已完成 runtime/gateway/frontend 三端合同字段盘点，并产出字段对齐表；已验证 `RuntimeContextSnapshot` 缺失字段完成跨端同步。
- 构建验证：`npm run build`（frontend）通过。

## 证据位置

- 测试记录：
  - `docs/11-hermes-rebuild/changes/INDEX.md`
  - `docs/11-hermes-rebuild/changes/C-tool-contract-unification/proposal.md`
  - `docs/11-hermes-rebuild/changes/C-tool-contract-unification/design.md`
  - `docs/11-hermes-rebuild/changes/C-tool-contract-unification/tasks.md`
  - `docs/11-hermes-rebuild/changes/C-tool-contract-unification/status.md`
  - `docs/11-hermes-rebuild/changes/C-tool-contract-unification/review.md`
  - `crates/runtime-core/src/contracts.rs`
  - `crates/runtime-core/src/capabilities/spec.rs`
  - `crates/runtime-core/src/tool_trace.rs`
  - `crates/runtime-core/src/verify.rs`
  - `crates/runtime-core/src/run_verification_metadata.rs`
  - `crates/runtime-core/src/run_failure_metadata.rs`
  - `crates/runtime-core/src/checkpoint.rs`
  - `crates/runtime-core/src/executors/agent_resolve.rs`
  - `gateway/internal/contracts/contracts.go`
  - `gateway/internal/api/chat_confirmation_memory.go`
  - `gateway/internal/api/chat_confirmation_memory_test.go`
  - `frontend/src/shared/contracts/runtime.ts`
  - `scripts/run-stage-c-tool-elapsed-acceptance.ps1`
  - `scripts/run-stage-c-risk-audit-acceptance.ps1`
  - `scripts/run-stage-c-gate-batch.ps1`
  - `cargo test -p runtime-core`
  - `go test ./...`（工作目录：`gateway/`）
  - `npm run build`（工作目录：`frontend/`）
  - `tmp/stage-c-tool-elapsed-acceptance/latest.json`
  - `tmp/stage-c-risk-audit-acceptance/latest.json`
  - `tmp/stage-c-gate-c-batch/latest.json`
- 日志或截图：
  - `tmp/stage-c-tool-elapsed-acceptance/logs/runtime.log`
  - `tmp/stage-c-tool-elapsed-acceptance/logs/gateway.log`
  - `tmp/stage-c-risk-audit-acceptance/logs/runtime.log`
  - `tmp/stage-c-risk-audit-acceptance/logs/gateway.log`

## Gate 映射

- 对应阶段 Gate：Gate-C。
- 当前覆盖情况：
  - 已完成阶段 C 主推进项激活与范围冻结。
  - 已完成三端合同字段盘点，并确认 `RuntimeContextSnapshot` 存在跨端字段缺口。
  - 已补齐 `RuntimeContextSnapshot` 缺口并通过 gateway/frontend 构建验证。
  - 已补齐 `tool_elapsed_ms` 成功/失败双路径，并完成 `chat/run -> logs` 接口级样本验证（`verification_completed` 与 `run_finished` 均可检索 `metadata.tool_elapsed_ms`）。
  - 已补齐风险确认链最小闭环审计字段，并完成 `chat/run -> confirmation_required -> checkpoint_resumed -> run_finished` 接口级样本验证（可检索 `confirmation_decision`、`confirmation_chain_step`、`confirmation_resume_strategy`、`checkpoint_id`）。
  - 已完成 Gate-C 指标化批量验证（5 轮）：高风险动作拦截率 `1.0`（>= `0.99`）、失败可定位率 `1.0`（>= `0.95`）、审计字段完整率 `1.0`（= `1.0`），并通过 `tool_elapsed_ms` 样本校验，`gate_c.ready=true`。
  - 已完成阶段评审签收与索引切换，阶段 C 进入“最近收口”。
