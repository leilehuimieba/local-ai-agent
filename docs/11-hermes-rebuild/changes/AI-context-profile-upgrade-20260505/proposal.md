# 变更提案

## 背景

- 第一刀 `AH-agent-loop-minimal-upgrade-20260505` 已把 Runtime 主链从单步收口升级为受控多步回路，但当前上下文装配仍主要停留在“已有注入能力”，尚未真正按任务类型分档。
- `AG-agent-loop-memory-knowledge-20260505` 已明确第二刀应优先解决 `ask / act / repair / learn` 四类 profile，让模型每一步拿到的是“当前决策所需最小包”，而不是默认把 session、memory、knowledge、tool preview 一起塞满。
- 如果不先做上下文 profile，后续记忆路由和知识 pack 即便增强，也容易退化成堆料，无法稳定支撑主循环的多步决策。

## 目标

- 产出一个只覆盖“上下文 profile 升级”的实现 change 工作区，作为第二刀代码实现与验收入口。
- 冻结第二刀范围：`ask_profile + act_profile + repair_profile + learn_profile`，以及 profile 选择理由和 metadata 出口。
- 明确第二刀的 Runtime / Gateway 边界、影响模块、测试证据与回退方式。

## 非目标

- 不在本 change 中改造记忆分层、recall 重排或长期写回规则。
- 不在本 change 中实现知识检索混合召回或 `knowledge pack`。
- 不在本 change 中推进 verify 任务矩阵。
- 不引入新的多智能体编排、Provider 扩展或前端大改。

## 验收口径

- 通过标准：
  - 文档范围仅覆盖上下文 profile 升级，没有混入记忆、知识检索、verify 设计。
  - 已明确四类 profile 的触发条件、注入差异、落点模块和验证方式。
  - 后续实现型智能体可直接据此进入代码改造，而不需要重新拆第二刀范围。
