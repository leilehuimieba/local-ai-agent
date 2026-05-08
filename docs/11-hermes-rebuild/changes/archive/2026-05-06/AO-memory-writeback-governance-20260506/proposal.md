# 变更提案

## 背景

- `AJ-memory-minimal-routing-20260505` 已经把记忆读取侧收窄为“何时读、读哪层、如何输出最小 digest”。
- 但 `AG-agent-loop-memory-knowledge-20260505` 方案三里，长期记忆“何时允许写回、写去哪一层、如何避免污染”的治理还没有拆成独立实现项。
- 当前项目已经具备 `run_finished / run_failed / memory_written` 等事件骨架，因此下一步最值得补的不是继续扩 recall，而是先把写回边界做实。

## 目标

- 产出一个只覆盖“记忆写回治理”的独立实现 change。
- 冻结本刀范围：只做 Working / Episodic / Semantic(或 Procedural) 的最小写回分类、准入规则、治理 metadata 与最小验证。
- 明确该 change 不回头混做 recall 重排、知识 pack、verify 矩阵扩面。

## 非目标

- 不在本 change 中重做 `AJ` 已收口的 recall 路由。
- 不在本 change 中引入向量检索或新的外部知识后端。
- 不在本 change 中扩展多智能体反思、长期自动总结或复杂评分系统。
- 不在本 change 中重做 Browser / MCP / UI 链路。

## 验收口径

- 文档范围只覆盖记忆写回治理，与 `AJ`、`AK`、`AL`、`AN` 保持边界清晰。
- 已明确长期记忆最小写回分层、准入条件、拒绝条件和治理字段。
- 已明确后续实现应先盘点哪些模块、先补哪些字段、最小测试证据是什么。
