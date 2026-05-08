# 验证记录

## 验证方式

- 结构检查：
  - 确认 `query_engine.rs` 在拆分前超过 `600` 行热点红线。
  - 确认 follow-up / replan / tests 已按既定模块边界拆出。
  - 确认拆分后 `query_engine.rs` 已降到热点红线以内。
- 状态检查：
  - `current-state.md`
  - `changes/INDEX.md`
  - `AT/status.md`
- 测试验证：
  - `cargo test -p runtime-core query_engine -- --nocapture`

## 证据位置

- 上游裁决依据：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AR-state-realignment-and-modularity-closeout-20260506/status.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AS-verify-structure-split-20260506/status.md`
- 当前实现目标：
  - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs`
- 当前实现证据：
  - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/replan.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/browser_followup.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/knowledge_followup.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/path_followup.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/tests/`

## 当前验证结论

1. `AT` 建档已完成。
2. `AT` 的范围冻结已完成。
3. `AT` 的模块骨架与函数映射已完成。
4. 第一刀结构拆分已完成，已抽出 `browser_followup.rs`、`knowledge_followup.rs`、`path_followup.rs`。
5. 第二刀 `replan.rs` 职责抽离已完成。
6. `query_engine.rs` 已从约 `689` 行下降到 `98` 行，热点红线已解除。
7. `cargo test -p runtime-core query_engine -- --nocapture` 已通过。
8. 第三刀 `bootstrap.rs` 已完成收益/风险评估；当前不继续拆分，不影响本 change 收口。

## 已完成验证证据

1. 文件体量证据
   - 拆分前：`D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs` 约 `689` 行
   - 第一刀后：`D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs` 为 `592` 行
   - 第二刀后：`D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs` 为 `98` 行
2. 第一刀模块证据
   - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/browser_followup.rs`
   - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/knowledge_followup.rs`
   - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/path_followup.rs`
3. 第二刀模块证据
   - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/replan.rs`
4. 测试迁移证据
   - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/tests/mod.rs`
   - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/tests/resume.rs`
   - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/tests/browser_followup.rs`
   - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/tests/knowledge_followup.rs`
5. 定向测试结果
   - `cargo test -p runtime-core query_engine -- --nocapture`：`11 passed; 0 failed`
6. 范围控制证据
   - 当前只抽出 follow-up、path 与 replan 逻辑
   - 当前未改 `bootstrap / execute` 主体行为
   - 未混入 `verify`、knowledge 扩面或 browser 扩能力
7. 收口裁决证据
   - 当前 `query_engine.rs` 仅 `98` 行
   - AT 目标中的热点红线解除已达成
   - 第三刀 `bootstrap.rs` 未再提供足够收益，故默认不继续

## Gate 映射

- 对应阶段 Gate：`Gate-I（已收口；当前处于自由迭代期）`
- 当前覆盖情况：
  - 已完成 `AT` 建档
  - 已完成 `AT` 边界冻结
  - 已完成 `AT` 模块骨架设计
  - 已完成第一刀结构拆分与测试目录迁移
  - 已完成第二刀 `replan.rs` 职责抽离
  - 已完成当前阶段定向验证与证据回写
  - 已完成第三刀是否继续推进的收口裁决
