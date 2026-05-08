# 变更提案

## 背景

- `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md` 已在上一轮收口到“无当前活跃 change”，原因是 `AT-query-engine-structure-split-20260506` 已完成并默认不再继续第三刀 `bootstrap.rs`。
- 但真实工作区并未处于“干净可切下一实现项”的状态；`git status --short` 与 `git diff --stat` 显示仍有大批未提交改动，且这些改动横跨：
  - `runtime-core` 多个模块
  - `frontend/scripts/browser-mcp-server.mjs`
  - `gateway/internal/api/browser_mcp_runtime_bridge_e2e_test.go`
  - `scripts/README.md`
  - `docs/07-test/evidence/20260506-*`
  - 多个 `changes/` 目录与 `archive/2026-05-06/`
- 当前未提交改动并不是单一 change 的残余，而是多个已完成/已归档/已收口 change 的工作区混合态：
  - `AO`：记忆写回治理相关改动仍未完全入库
  - `AN / AM / AL`：浏览器恢复、高风险交互、verify 元数据相关改动仍未完全入库
  - `AQ / AP`：知识回答主链收口与回归包脚本/证据仍未完全入库
  - `AS / AT`：结构拆分代码与文档虽然已完成裁决，但仍处于未提交状态
- 如果此时直接新开实现 change 并进入业务代码开发，会进一步放大“状态口径正确，但真实工作区归属混乱”的风险。

## 目标

- 新建独立收口 change `AU-worktree-ownership-and-closeout-routing-20260506`。
- 只处理以下问题：
  1. 未提交改动的 change 归属矩阵；
  2. 文档口径与真实工作区的一致性修正；
  3. 当前超 600 行热点文件的后续备案判断；
  4. `scripts / evidence / tmp` 的收口入口与保留边界；
  5. 在不直接实现新功能的前提下，裁决下一把真正的实现 change 应该切到哪里。

## 非目标

- 不直接修改主链业务行为。
- 不在本 change 内继续推进 `memory`、`browser`、`knowledge` 或其它运行时能力实现。
- 不顺手清空整个 `tmp/`、不批量归档所有历史材料、也不在本轮直接做大规模提交切片。
- 不把未来所有热点文件一次性都拉进来做拆分实现。

## 验收口径

- 已建立 `AU-worktree-ownership-and-closeout-routing-20260506` 的正式五件套工作区。
- `current-state.md` 与 `changes/INDEX.md` 已切换到 `AU` 作为当前活跃 change。
- 已形成未提交改动的归属矩阵，至少覆盖：`AO / AN / AM / AQ / AP / AS / AT` 相关残留。
- 已明确当前热点文件备案结论，并给出下一把最小实现 change 的建议入口。
- 全程没有在未完成归属收口前直接进入新的大规模业务代码实现。
