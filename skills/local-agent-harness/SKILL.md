---
name: local-agent-harness
description: 将 Harness Engineering 落到本地智能体项目的运行时约束、评审清单和实现路径上。用于设计、实现或评审本仓库中与 agent loop、context assembly、tool registry、permission gate、memory router、artifact、verify、event contracts 相关的改动；也用于把长任务、状态交接、独立验证和按需上下文装配纳入默认工作流。
---

# Local Agent Harness

## 概览

用这份 skill 把“模型外壳”设计成项目默认做法，而不是只靠提示词补救。
先收紧运行时边界，再决定要不要改代码、加模块或扩能力。

## 使用流程

1. 先读项目入口文档，避免把历史文档当成当前口径。
   先看 `docs/README.md`。
   如果是运行时任务，再看 `docs/02-architecture/本地适配架构原则_V1.md` 和 `docs/06-development/智能体框架主干开发任务书_V1.md`。

2. 再读 `references/harness-checklist.md`。
   这是主清单，按“目标边界 -> 主循环 -> 上下文 -> 工具 -> 权限 -> 验证 -> 记忆 -> 事件 -> 长任务交接”的顺序判断。

3. 涉及当前仓库落点时，再读 `references/project-mapping.md`。
   只加载和当前改动直接相关的模块，不默认通读全部源码。

4. 需要说明这些规则为什么存在、或要补充外部工程依据时，再读 `references/external-patterns.md`。
   它整理了近期官方与行业材料中对 harness、长任务和验证闭环最有用的信号。

## 默认执行规则

1. 把模型视为 `planner + executor` 的一部分，不把它当成完整系统。
2. 默认要求 `Plan -> Execute -> Observe -> Verify -> Finish` 闭环，不允许“生成后直接结束”。
3. 默认使用按需上下文装配，不做全量注入。
4. 默认把大结果外置为 artifact，只把摘要和引用放回事件流与模型上下文。
5. 默认把高风险动作送入统一确认主线，不做旁路审批。
6. 默认把长任务拆成交接节点，保留 handoff artifact，必要时重建上下文再继续。
7. 默认把验证视为独立环节，尽量不要只依赖同一个生成者自评。
8. 默认优先收紧主干，不提前扩多智能体、复杂启发式或重型自动化链路。

## 输出要求

当你用这个 skill 产出方案或评审意见时，优先给出这三类内容：

1. `Harness 判断`
   当前变更补强了哪一层外壳，遗漏了哪一层。

2. `最小改动路径`
   应先收紧哪些现有模块，哪些能力明确后置。

3. `验证证据`
   需要哪些测试、回归、人工确认或事件观测，才能证明这次改动真的让系统更稳。
