# Hermes Change 工作区约定

`docs/11-hermes-rebuild/changes/` 用于承接当前执行主线下的进行中变更。
当前阶段、当前 Gate、当前活跃 change 的状态裁决统一以
`docs/11-hermes-rebuild/current-state.md` 为准。

## 1. 何时创建 change

出现下面任一情况时，创建一个新的 change 目录：

1. 跨多个模块的结构性改动。
2. 需要先做方案再做实现的中等以上任务。
3. 需要单独沉淀验证证据或回退方案的任务。

## 2. 目录命名

目录名建议使用 `阶段-主题` 形式，例如：

1. `B-checkpoint-resume`
2. `B-event-contract-alignment`
3. `C-tool-contract-unification`

## 3. 标准文件

仓库级索引文件：

1. `INDEX.md`

每个 change 目录默认包含：

1. `proposal.md`
2. `design.md`
3. `tasks.md`
4. `status.md`
5. `verify.md`

## 4. 使用规则

1. 先补 `proposal.md` 和 `design.md`，再进入实现。
2. 实现推进时同步勾选 `tasks.md`，避免聊天进度与文件进度脱节。
3. 每次阶段性推进后更新 `status.md`。
4. 进入验证或提审前补齐 `verify.md`。
5. 当前主推进项要同步更新到 [INDEX.md](D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md)。
6. 切换主推进项时，先更新 `current-state.md`，再更新 `INDEX.md`。
7. 如与阶段 Gate 冲突，以 [阶段计划总表](D:/newwork/本地智能体/docs/11-hermes-rebuild/stage-plans/阶段计划总表.md) 为准。
