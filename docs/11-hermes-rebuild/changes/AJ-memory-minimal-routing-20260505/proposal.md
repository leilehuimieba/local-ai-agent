# 变更提案

## 背景

- 第一刀 `AH-agent-loop-minimal-upgrade-20260505` 已把 Runtime 主链升级为受控多步回路。
- 第二刀 `AI-context-profile-upgrade-20260505` 已把上下文装配收口为 `ask / act / repair / learn` 四类 profile。
- 当前记忆能力虽然已经具备 `system views`、`current memory object`、长期记忆摘要等入口，但 `memory_recall.rs` 仍偏向“把能命中的都拼起来”，还没有形成“这一问到底该读哪层、跳过哪层、如何给出最小 digest”的稳定路由。
- 如果第三刀一上来就把记忆分层写回治理、知识 pack、verify 矩阵一起混做，范围会明显失控，也无法验证“记忆最小路由”本身是否成立。

## 目标

- 产出一个只覆盖“记忆最小路由”的实现 change 工作区，作为第三刀代码实现与验收入口。
- 冻结第三刀当前范围：只做记忆读侧路由，解决 `何时读、优先读哪层、如何输出最小 digest`。
- 明确第三刀的 Runtime / Gateway 边界、影响模块、测试证据与回退方式。

## 非目标

- 不在本 change 中重做长期记忆存储结构或 SQLite schema。
- 不在本 change 中推进知识 pack、知识检索混合召回或 citation 输出。
- 不在本 change 中推进 verify 任务矩阵。
- 不在本 change 中引入新的记忆自动写回治理规则，只允许为最小路由补必要 metadata。

## 验收口径

- 通过标准：
  - 文档范围仅覆盖记忆最小路由，没有混入知识 pack 或 verify 设计。
  - 已明确第三刀最小路由应如何结合 `ask / act / repair / learn` profile 决定读层。
  - 已明确最小 digest、事件 metadata、context snapshot 的新增出口。
  - 后续实现型智能体可直接据此进入代码改造，而不需要重新拆第三刀范围。
