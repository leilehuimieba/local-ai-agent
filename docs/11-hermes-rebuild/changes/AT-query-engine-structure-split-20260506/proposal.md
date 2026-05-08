# 变更提案

## 背景

- `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AS-verify-structure-split-20260506/` 已完成 `verify.rs` 的结构拆分、测试迁移与定向验证。
- `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AR-state-realignment-and-modularity-closeout-20260506/` 已把 `D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs` 备案为下一优先热点文件。
- 当前 `D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs` 约 `689` 行，已超过项目对 Rust 源文件 `600` 行的热点红线。
- 该文件当前同时承载：
  - runtime bootstrap / state 构建入口
  - execute / replan / budget 判定
  - knowledge follow-up / browser readback 判定
  - path candidate 解析
  - 内联测试
- 若继续在同一文件内追加主循环与 replan 逻辑，会放大维护成本，也会提高后续定位行为回归与模块接线风险。

## 目标

- 新建独立实现 change `AT-query-engine-structure-split-20260506`。
- 仅完成 `query_engine.rs` 的结构拆分备案与后续实现入口准备。
- 在不改变主循环行为、replan 语义和对外 contract 的前提下，为下一步最小结构治理提供单独工作区。

## 非目标

- 不修改主循环策略、iteration budget、replan 规则或 verification 语义。
- 不顺手扩知识问答、browser 能力、memory 路由或其它 runtime 能力范围。
- 不把 `verify`、`context_builder`、`events` 等其它热点文件一起并入本 change。
- 当前这一步不直接进入 `query_engine.rs` 大规模实现。

## 验收口径

- 已建立 `AT-query-engine-structure-split-20260506` 的正式五件套工作区。
- 已明确 `query_engine.rs` 的拆分边界、模块草图、验证口径与风险边界。
- `current-state.md` 与 `changes/INDEX.md` 已切换到 `AT` 作为新的主推进项。
- 全程没有在未建档前直接进入 `query_engine.rs` 大规模改写。
