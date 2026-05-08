# 技术方案

## 影响范围

- Runtime：
  - `crates/runtime-core/src/query_engine.rs`
  - `crates/runtime-core/src/verify.rs`
  - `crates/runtime-core/src/events.rs`
  - `crates/runtime-core/src/run_failure_metadata.rs`
  - 可能涉及 checkpoint / handoff 相关输出
- Gateway：
  - 当前阶段原则上不做主链改动，除非联调证据证明 handoff 出口缺字段

## Harness 判断

- 浏览器主链的执行能力已经覆盖 `open/read/click/type/select/submit/upload`。
- 下一步最值得补的是失败路径治理，而不是继续横向扩动作种类。
- 因此本刀只收紧 recovery / handoff，不回 `AM` 继续扩 contract，也不回 `AL` 扩通用 verify。

## 范围冻结

- 本刀只做四件事：
  - 细化浏览器高风险交互失败分型
  - 收紧单次回读后的停止条件
  - 补最小 handoff artifact 与失败提示
  - 用测试证明失败路径可读、可交接、可停止

## 失败路径治理

### 最小失败分型

- `target_not_found`
  - 目标元素未定位到
- `state_not_changed`
  - 动作执行后没有观察到预期页面变化
- `permission_or_risk_gap`
  - 审计、确认或权限边界信号不足
- `recovery_exhausted`
  - 已执行一次 `read_page` 回读后仍无法验证

### Recovery 顺序

1. 首次高风险交互 verify 失败：
   - 优先补一次 `browser/read_page`
2. 回读后若仍失败：
   - 停止自动续跑
   - 输出 handoff artifact
   - 明确失败分型与建议的人类下一步

## Handoff Artifact

- 最小交接内容至少包括：
  - 失败动作名
  - `page_id`
  - 目标 selector 或关键参数摘要
  - 最近一次 `read_page` / 页面状态摘要
  - 失败分型
  - 推荐下一步

## 验证路径

- Rust 测试覆盖：
  - 失败分型判断
  - 单次回读后停止续跑
  - handoff artifact / failure metadata 输出
- 最小联调：
  - 任选一条高风险交互失败样例
  - 证明失败后能留下可读的交接信息
