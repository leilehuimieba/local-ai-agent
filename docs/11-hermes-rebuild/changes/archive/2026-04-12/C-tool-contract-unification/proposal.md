# 变更提案

## 背景

- 阶段 B 的 checkpoint/resume 主线已收口，`B-checkpoint-resume` 已完成并沉淀了 Gate-B 对应证据。
- 阶段计划要求下一步进入阶段 C，统一工具协议、风险分级确认链与审计字段。

## 目标

- 启动阶段 C 首条主推进 change，冻结本轮范围与验收口径。
- 先收口跨端 Tool Contract 基线，再推进实现，避免 runtime/gateway/frontend 各改各的。
- 把权限分级与确认链的最小闭环纳入同一条执行主线。

## 非目标

- 本轮不推进阶段 D（记忆与技能体系）内容。
- 本轮不一次性做完所有执行后端抽象（WSL/bash 先不作为必做）。
- 本轮不重写现有工具系统。

## 验收口径

- `proposal/design/tasks/status/verify` 五件套齐全并与阶段 C 目标对齐。
- `INDEX.md` 有且仅有一个当前活跃 change，且指向本 change。
- 本 change 的任务清单可直接驱动后续实现和验证，不依赖口头约定。
