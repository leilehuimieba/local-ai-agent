# 技术方案

## 影响范围

- `crates/runtime-core/src/session.rs`
- `crates/runtime-core/src/execution.rs`
- `crates/runtime-core/src/events.rs`
- `docs/11-hermes-rebuild/stage-plans/`

## 方案

- 先冻结 checkpoint 最小字段集合，再定义 resume 从哪个阶段恢复。
- 恢复链路优先回到统一主循环，不做旁路恢复。
- 验证口径优先覆盖中断、取消、失败后恢复三类路径。

## 待确认点

- checkpoint 是否需要兼容已有草稿结构。
- 事件合同里哪些字段必须携带恢复上下文。

## 风险与回退

- 风险：checkpoint 字段过多会导致实现和兼容成本扩大。
- 风险：resume 边界不清会让失败路径重新分叉。
- 回退方式：如果恢复链路不稳定，保留 checkpoint 数据结构草案，回退到上一个稳定事件合同版本。
