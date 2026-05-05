# 变更提案

## 背景

- 第一刀 `AH-agent-loop-minimal-upgrade-20260505` 已把 Runtime 主链升级为受控多步回路。
- 第二刀 `AI-context-profile-upgrade-20260505` 已把上下文装配收口为 `ask / act / repair / learn` 四类 profile。
- 第三刀 `AJ-memory-minimal-routing-20260505` 已把记忆读侧收口为最小路由，不再默认全量拼接长期记忆摘要。
- 当前薄弱点转移到知识回答主链：`knowledge.rs` 已能命中片段，但还没有稳定的 `混合检索 -> knowledge pack -> citation-ready 输出 -> profile 注入` 最小闭环。
- 如果现在直接切到 verify 矩阵，会先验证一条尚未稳定的知识回答链，后续知识输出结构一旦调整，验证矩阵还要返工。

## 目标

- 产出一个只覆盖“知识检索增强与 knowledge pack”的独立实现 change 工作区。
- 冻结第四刀范围：只做混合检索、知识 pack、citation-ready 输出与 ask / learn 场景下的最小注入。
- 明确 Runtime / Gateway 边界、影响模块、验证证据与回退方式。

## 非目标

- 不在本 change 中推进 verify 任务矩阵。
- 不在本 change 中重做第三刀记忆路由或长期记忆写回治理。
- 不在本 change 中引入向量数据库替换、复杂语义召回或外部知识源扩张。
- 不在本 change 中重做 Rust / Go / Frontend 边界。

## 验收口径

- 通过标准：
  - 文档范围仅覆盖知识检索增强与 knowledge pack，不混入 verify 矩阵。
  - 已明确最小混合检索要补哪些信号、knowledge pack 要输出哪些字段。
  - 已明确 ask / learn 场景下的 knowledge pack 注入口径与 citation-ready 输出边界。
  - 后续实现型智能体可以直接据此进入第四刀代码实现。
