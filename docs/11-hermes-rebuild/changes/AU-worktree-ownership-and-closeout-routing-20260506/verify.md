# 验证记录

## 验证方式

- 状态入口检查：
  - `D:/newwork/本地智能体/docs/README.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/Hermes重构总路线图_完整计划.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/stage-plans/阶段计划总表.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
- 真实工作区检查：
  - `git status --short`
  - `git diff --stat`
  - `git diff --name-only`
- 归属采样检查：
  - `git diff -- ...verify/query_engine/events/contracts/run_verification_metadata`
  - `git diff -- ...memory/memory_router/sqlite_store/run_memory_metadata`
  - `git diff -- ...browser_mcp_server/gateway_e2e/handoff/run_failure_metadata/run_finish_events`
  - `git diff -- ...executors/project/scripts/README/run-knowledge-answer-eval-pack.ps1`
- 结构债务检查：
  - 统计 `runtime-core/src` 非测试 Rust 文件行数并筛出超 600 行文件
- 目录收口检查：
  - `scripts/`
  - `docs/07-test/evidence/`
  - `tmp/`

## 证据位置

- 当前真实状态：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
- 上游已完成 change：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AR-state-realignment-and-modularity-closeout-20260506/`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AS-verify-structure-split-20260506/`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AT-query-engine-structure-split-20260506/`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AP-knowledge-answer-eval-pack-20260506/`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-05-06/INDEX.md`
- 本轮归属核对重点：
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/mod.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/sqlite_store/mod.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/events.rs`
  - `D:/newwork/本地智能体/frontend/scripts/browser-mcp-server.mjs`
  - `D:/newwork/本地智能体/gateway/internal/api/browser_mcp_runtime_bridge_e2e_test.go`
  - `D:/newwork/本地智能体/crates/runtime-core/src/executors/project.rs`
  - `D:/newwork/本地智能体/scripts/run-knowledge-answer-eval-pack.ps1`

## 当前验证结论

1. 当前工作区不是“无活跃问题的干净状态”，而是“无活跃实现 change，但存在多簇未提交改动”。
2. 这些未提交改动可以按既有 change 主题稳定归入四个主簇，而不是必须新开一个超大实现 change 一次性吞掉。
3. `AS` 与 `AT` 的结构治理本身已完成，不应继续作为下一步实现入口。
4. 当前最适合先切下一把实现 change 的，不是 browser 或 knowledge，而是 memory 写回治理簇。
5. `scripts / evidence / tmp` 当前已有明确有效入口，其中 `tmp/knowledge-answer-evals/` 需要保留，不能因“目录太旧”被误清。

## Gate 映射

- 对应阶段 Gate：`Gate-I（已收口；当前处于自由迭代期）`
- 当前覆盖情况：
  - 已完成真实工作区状态核对
  - 已完成未提交改动归属矩阵建立
  - 已完成热点文件备案判断
  - 已完成脚本/证据/tmp 收口入口确认
  - 已完成下一把最小实现项裁决前的正式 change 建档
