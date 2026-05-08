# 变更提案

## 背景

- `AH` 到 `AL` 已把 agent 主循环、上下文 profile、记忆读侧、知识检索与 verify 主链打通，`AO-memory-writeback-governance-20260506` 又把记忆写回治理补齐到可控状态。
- 当前最薄弱的已经不是“有没有主链能力”，而是“这条主链怎样稳定回归、怎样快速发现退化、怎样避免看起来能答但证据质量下滑”。
- 按 `AG-agent-loop-memory-knowledge-20260505` 蓝图继续推进，下一步最值得补的不是再扩主链行为，而是先沉淀一把最小可复跑的知识回答评测包。

## 目标

- 产出一个只覆盖“知识回答主链回归包”的独立 change。
- 固定最小问题集、评分口径、citation/verification 断言与复跑入口。
- 让后续对 `AJ / AK / AL / AO` 相关代码的调整，都能快速判断是否破坏知识回答主链。

## 非目标

- 不在本 change 中改 Runtime 主循环、memory router、knowledge pack 或 verify 主行为。
- 不在本 change 中扩新的知识源、复杂 rerank、UI 面板或多智能体流程。
- 不把本 change 做成大而全 benchmark，只做最小可用回归包。

## 验收口径

- 已有独立 change 工作区，且范围只覆盖知识回答主链回归包。
- 已明确固定问题集、场景分层、评分规则、 inspect 信号与复跑方式。
- 已明确至少覆盖成功、证据不足、边界约束三类核心风险，而不是只测 happy path。
- 后续实现型智能体可直接以本 change 为入口补脚本、样例与证据，不需要重新大范围诊断。
