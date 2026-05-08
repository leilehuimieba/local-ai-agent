# 变更提案

## 背景

- `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AR-state-realignment-and-modularity-closeout-20260506/` 已完成状态校准、未收口改动归属、热点文件备案与聚合回归入口收口，并已明确唯一下一步最小实现项。
- 当前最优先的结构性债务是 `D:/newwork/本地智能体/crates/runtime-core/src/verify.rs`，该文件当前约 `1325` 行，混合了 verify 入口、policy、evidence、五类任务规则、browser failure metadata 与大量测试。
- 如果继续在单文件上叠加 verify 逻辑，会持续提高维护成本，并放大后续 contract 调整和测试定位难度。
- `AR` 已明确要求：下一把实现 change 只做 `verify.rs` 结构拆分，不把 `query_engine.rs`、知识问答扩面、browser 扩能力等内容一起打包。

## 目标

- 新建独立实现 change `AS-verify-structure-split-20260506`。
- 仅完成 `verify.rs` 的结构拆分，把当前单文件按职责分成更小模块。
- 在不改变 verify 行为、contract 和判定阈值的前提下，降低后续维护与测试定位成本。

## 非目标

- 不修改 verify 判定规则、阈值或输出 contract。
- 不处理 `query_engine.rs` 结构拆分。
- 不扩知识问答、browser 能力、memory 路由或其它 runtime 能力范围。
- 不顺手做与 `verify.rs` 拆分无关的大规模代码整理。

## 验收口径

- `D:/newwork/本地智能体/crates/runtime-core/src/verify.rs` 主文件显著瘦身，不再承载所有 verify 逻辑与大测试块。
- 已按设计拆出模块，例如 policy、evidence、knowledge_answer、browser_interaction、file_command_memory 与 tests 分组。
- 既有 verify 相关测试与 `run_verification_metadata` 相关测试保持通过。
- 全程没有引入新的行为 scope，没有改变原有 contract 语义。
