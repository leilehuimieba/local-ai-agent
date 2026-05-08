# 变更提案

## 背景

- `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AV-memory-writeback-structure-closeout-20260506/` 已完成热点治理主目标：
  - `memory_router/mod.rs` 已降到 `504` 行；
  - `sqlite_store/mod.rs` 已降到 `551` 行；
  - `memory.rs` 已降到 `556` 行。
- `AV` 已完成最小聚合验证，`memory write / recall / object / checkpoint` 主链全绿，已满足“结构收口 + 热点治理准备完成”的收口条件。
- 但真实工作区仍不是“可直接切下一实现项”的干净状态；`git status --short` 与 `git diff --stat` 仍显示：
  - 多个已完成 change 的代码与文档同时处于未提交状态；
  - `changes/` 与 `archive/2026-05-06/` 下仍有本轮新增目录未统一切换到最终状态；
  - 当前更像“后 `AV` 收口路由问题”，而不是一个新的单一实现主簇。
- 如果此时直接新开下一把实现 change，会再次把“状态切换 / 归属收口 / 主推进项裁决”与新实现混做。

## 目标

- 新建 `AW-post-av-closeout-routing-20260506`，作为 `AV` 之后的正式收口路由 change。
- 只处理以下事项：
  1. 确认 `AV` 已正式收口；
  2. 统一 `current-state.md`、`changes/INDEX.md` 与各 change `status.md` 的主推进项口径；
  3. 复核当前工作区剩余未收口改动是否构成新的实现主簇；
  4. 给出下一把正式 change 的入口判断；
  5. 在不扩 scope 的前提下，明确哪些目录与 change 保持只读等待归档/入库。

## 非目标

- 不继续扩 `memory`、`browser`、`knowledge`、`verify` 或 `query_engine` 能力。
- 不在本 change 内再做新的大规模代码结构拆分。
- 不在本轮处理提交流水、切 commit、归档全部历史材料。
- 不把多簇未提交改动强行合并成一个新的实现型大 change。

## 验收口径

- `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md` 已切换到 `AW-post-av-closeout-routing-20260506` 作为当前活跃 change。
- `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md` 已同步切换当前活跃项，并把 `AV` 标为已收口。
- `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AV-memory-writeback-structure-closeout-20260506/status.md` 已更新为“已收口（已切换下一主推进项）”。
- 已明确当前剩余未收口工作区是否存在单一新的实现主簇；若不存在，已明确继续保持 closeout routing，而不是误开实现项。
- 已补齐 `AW` 的 `proposal.md`、`design.md`、`tasks.md`、`status.md`、`verify.md`。
