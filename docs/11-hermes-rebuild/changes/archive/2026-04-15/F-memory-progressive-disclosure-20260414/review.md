# 评审记录

## 评审范围

1. `crates/runtime-core/src/observation.rs`
2. `crates/runtime-core/src/context_builder.rs`
3. `crates/runtime-core/src/run_context_metadata.rs`
4. `crates/runtime-core/src/events.rs`
5. `crates/runtime-core/src/prompt.rs`
6. `crates/runtime-core/src/sensitive_data.rs`
7. `crates/runtime-core/examples/export_observation_*.rs`

## 评审结论

1. M1~M5 目标链路已完整实现：采集、存储、队列、检索、注入、治理、回退均可复现。
2. 证据链完整：`tmp/stage-mem-m1`、`tmp/stage-mem-m2`、`tmp/stage-mem-m3`、`tmp/stage-mem-m4`、`tmp/stage-mem-m5`、`tmp/stage-mem-eval`。
3. 关键阈值通过：
   - Top-5 命中率：100%（阈值 70%）
   - 分层注入节省率：58.37%（阈值 50%）
   - 敏感字段脱敏命中：`redacted_count=3`
   - private 片段排除：`private_marker_count=3`
   - rollback 演练：enabled/disabled 双态均可复现

## 风险备注

1. `events.rs` 与 `lib.rs` 存在历史长函数，非本专项新增，不纳入本轮整改范围。
2. M4 的 budget/ab-test 当前由同一注入示例导出，后续如接入真实 tokenizer 可替换为 token 口径。
