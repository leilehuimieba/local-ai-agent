# 变更提案

## 背景

- 第一刀 `AH-agent-loop-minimal-upgrade-20260505` 已把 Runtime 主链升级为受控多步回路。
- 第二刀 `AI-context-profile-upgrade-20260505` 已把上下文装配收口为 `ask / act / repair / learn` 四类 profile。
- 第三刀 `AJ-memory-minimal-routing-20260505` 已把记忆读侧收口为最小路由。
- 第四刀 `AK-knowledge-pack-minimal-20260505` 已把知识回答主链升级为 `混合检索 -> knowledge pack -> citation-ready 输出 -> ask/learn 注入` 的最小闭环。
- 当前最薄弱点转移到 verify：`verify.rs` 仍以统一规则为主，无法按任务类型稳定判断“知识回答是否足够可交付”“风险边界是否已被遵守”“是否应该 replan / handoff / finish”。

## 目标

- 产出一个只覆盖“verify 任务类型矩阵”的独立实现 change 工作区。
- 冻结第五刀范围：只做 verification policy 的任务分型、knowledge answer 主链优先验证、最小 metadata 扩展与对应测试。
- 明确本刀先验证已经稳定下来的知识回答主链，再向文件修改、命令执行、记忆写入、浏览器交互逐步扩展。

## 非目标

- 不在本 change 中重做第四刀知识 pack 结构或混合检索。
- 不在本 change 中引入新的知识源、复杂 rerank 或 verify UI 面板。
- 不在本 change 中扩多智能体、恢复机制重写或新的权限系统。
- 不在本 change 中一次性把所有任务类型都做成重型规则引擎。

## 验收口径

- 通过标准：
  - 文档范围只覆盖 verify 矩阵，不回 AK 扩知识 scope。
  - 已明确知识回答类 verify 的最小通过标准、失败标准与 metadata 字段。
  - 已明确后续再补文件修改 / 命令执行 / 记忆写入 / 浏览器交互的顺序和边界。
  - 后续实现型智能体可以直接以本 change 为入口，先做知识回答类 verify，再补其余任务类型。
