# AN 浏览器恢复治理最小失败链证据

日期：2026-05-06

## 范围

- 只验证 `AN-browser-recovery-governance-20260505` 的最小失败链收口。
- 不补新的 Browser MCP 端到端动作联调，不回 `AM` 扩 contract。

## 执行命令

1. `cargo test -p runtime-core browser_read_page_failure_stops_replan_and_handoffs -- --nocapture`
2. `cargo test -p runtime-core verification_snapshot_keeps_browser_failure_fields -- --nocapture`
3. `cargo test -p runtime-core handoff_artifact_keeps_browser_failure_fields -- --nocapture`
4. `cargo test -p runtime-core appends_browser_failure_page_and_selector -- --nocapture`

## 结果

- 四条聚焦测试全部通过。
- 这组测试共同覆盖：
  - 浏览器交互 verify 失败后，单次 `read_page` 回读失败即停止自动续跑
  - `verification_snapshot` 已透出：
    - `browser_failure_type = recovery_exhausted`
    - `browser_page_id = page_01`
    - `browser_selector = #submit`
    - `browser_recovery_attempted = true`
    - `browser_recovery_exhausted = true`
  - handoff artifact 已透出：
    - `browser_failure_type = recovery_exhausted`
    - `browser_page_id = page_01`
    - `browser_selector = #submit`
    - `last_readback_summary` 包含“已执行单次恢复”
  - failure metadata 已透出：
    - `browser_page_id = page_01`
    - `browser_selector = #submit`
    - `failure_recovery_hint` 指向 page/selector/页面状态检查

## 关键代码锚点

- `D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs`
  - `browser_read_page_failure_stops_replan_and_handoffs`
- `D:/newwork/本地智能体/crates/runtime-core/src/events.rs`
  - `verification_snapshot_keeps_browser_failure_fields`
- `D:/newwork/本地智能体/crates/runtime-core/src/handoff.rs`
  - `handoff_artifact_keeps_browser_failure_fields`
- `D:/newwork/本地智能体/crates/runtime-core/src/run_failure_metadata.rs`
  - `appends_browser_failure_page_and_selector`

## 结论

- 对 `AN` 本刀目标而言，最小失败链已经可读、可停止、可交接。
- 当前剩余缺口不在运行时主链，而在是否还要额外补一条真实 Browser MCP 外部联调样本。
