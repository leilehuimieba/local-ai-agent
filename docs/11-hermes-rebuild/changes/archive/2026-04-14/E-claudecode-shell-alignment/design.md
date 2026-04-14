# 技术方案

## 影响范围

- 涉及模块：
  1. `crates/runtime-core/src/executors/command.rs`
  2. `crates/runtime-core/src/capabilities/spec.rs`
  3. `crates/runtime-core/src/tool_trace.rs`
  4. `crates/runtime-core/src/artifacts.rs`
  5. `crates/runtime-core/src/risk.rs`
  6. `crates/runtime-core/src/compaction.rs`
  7. `crates/runtime-core/src/session.rs`
  8. `frontend/src/events/EventTimeline.tsx`
  9. `frontend/src/runtime/state.ts`
  10. `frontend/src/shared/contracts/runtime.ts`
- 涉及文档或 contract：
  1. `docs/11-hermes-rebuild/stage-plans/A-Tool-Contract-v1.md`
  2. `docs/ClaudeCode本地智能体框架架构设计文档.md`
  3. `docs/11-hermes-rebuild/changes/E-claudecode-shell-alignment/artifacts/ClaudeCodeRev-shell-gap-matrix-20260413.md`

## 方案

- 核心做法：
  1. 先在文档层冻结四条收口主线：
     - 工具结果合同（`summary + detail_preview + artifact_path + error_code + retryable`）
     - 命令输出双轨（模型侧摘要/预览 + UI 侧可展开原文）
     - 权限决策链（规则层 + ask 流程编排 + 结果可回放）
     - 压缩边界（按条目与按消息聚合预算）
  2. 给每条主线定义“源码现状 -> 对标目标 -> 最小实现任务 -> 验收证据”。
  3. 使用同一份差距矩阵承接后续实现，避免在聊天中重复定义口径。
- 状态流转或调用链变化：
  1. 本次仅新增文档与任务约束，不调整运行态调用链。
  2. 实现阶段将按 `tasks.md` 分刀推进，并在每刀后补 `verify.md`。

## 风险与回退

- 主要风险：
  1. 将 Claude Code 的复杂机制一次性平移，导致过度工程。
  2. 文档口径和代码口径二次漂移，出现“设计已收口、实现未跟上”。
  3. 当前 Gate-E 正在推进，若直接切主推进项，可能打断既有收口节奏。
- 回退方式：
  1. 严格按最小 task 切片推进，每刀只落一个可验收主题。
  2. 若实现偏离，回退到本 change 的差距矩阵与任务判据重新对齐。
  3. 本 change 先作为保留观察项，不切换 `current-state.md` 活跃项。
