# 变更提案

## 背景

- `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md` 当前写为“无当前活跃 change”，但真实工作区仍有未提交代码、未入库 change 目录、未归档完成的证据入口与脚本入口。
- `git status --short` 显示 `D:/newwork/本地智能体/crates/runtime-core/`、`D:/newwork/本地智能体/frontend/scripts/browser-mcp-server.mjs`、`D:/newwork/本地智能体/gateway/internal/api/browser_mcp_runtime_bridge_e2e_test.go`、`D:/newwork/本地智能体/scripts/README.md`、`D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`、`D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md` 等仍在变动。
- `git diff --stat` 显示当前工作区累计 `31 files changed, 2252 insertions(+), 147 deletions(-)`，说明“自由迭代期无活跃 change”的文档口径与真实状态不一致。
- `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-05-06/`、`D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AM-browser-risky-interaction-20260505/`、`D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AP-knowledge-answer-eval-pack-20260506/`、相关 `D:/newwork/本地智能体/docs/07-test/evidence/20260506-*` 目录与 `D:/newwork/本地智能体/scripts/run-knowledge-answer-eval-pack.ps1` 已出现在工作区，但尚未通过新的主推进项统一收口。
- 当前还存在热点文件越线问题，其中 `D:/newwork/本地智能体/crates/runtime-core/src/verify.rs`、`D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs` 本轮仍有新增改动，且多个 Rust / TypeScript 文件长期高于 600 行红线，需要先备案而不是继续放大实现面。

## 目标

- 新建一把正式 change，作为当前唯一主推进入口，先解决状态校准与收口问题。
- 把“文档状态 vs. 真实工作区”不一致点写清楚，并把未提交改动归口到可执行清单。
- 对热点文件、`scripts / evidence / tmp` 目录做最小收口清单与后续拆分备案。
- 为后续最小实现顺序提供入口，但本 change 本身不直接扩新功能。

## 非目标

- 不在本 change 内继续扩 `memory / knowledge / browser` 新能力。
- 不把 `AQ / AO / AM / AP` 重新打开成实现型大 change。
- 不在本轮直接做热点文件拆分落地，只做备案、归属和执行顺序收紧。
- 不在本轮补跑大批量回归，只整理已有回归与证据入口。

## 验收口径

- `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md` 与 `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md` 已切到 `AR-state-realignment-and-modularity-closeout-20260506` 作为当前活跃 change。
- `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AR-state-realignment-and-modularity-closeout-20260506/` 下已具备 `proposal.md`、`design.md`、`tasks.md`、`status.md`、`verify.md`。
- 已明确列出当前未收口改动的归属范围、热点文件红线清单、`scripts / evidence / tmp` 需收口目录。
- 已明确下一步先做的最小实现项与后置项，且没有直接扩 scope 到新功能。
