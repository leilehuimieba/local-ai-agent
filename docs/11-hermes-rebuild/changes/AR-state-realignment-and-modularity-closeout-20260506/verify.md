# 验证记录

## 验证方式

- 工作区状态检查：
  - `git status --short`
  - `git diff --stat`
  - `git diff --name-status`
- 结构性债务检查：
  - 统计 Rust / Go / TypeScript 源文件行数，筛出超过 600 行红线的文件
  - 对 `verify.rs`、`query_engine.rs` 做职责分布抽样阅读
- 人工核对：
  - 对比 `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md` 与真实工作区状态
  - 核对 `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AP-knowledge-answer-eval-pack-20260506/`、`D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-05-06/`、`D:/newwork/本地智能体/docs/07-test/evidence/20260506-*` 是否已经形成但尚未被新的主推进项接住
  - 核对 `AM`、`AP`、`AL`、`AG` 的 `status.md / verify.md` 是否足以为归属矩阵、拆分备案、入口收口单与任务 8 裁决提供依据
  - 核对 `tmp/knowledge-answer-evals/` 中 `latest.json` 与历史批次 JSON 的关系是否清楚

## 证据位置

- 状态入口：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
- 输入 / 历史 change：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AG-agent-loop-memory-knowledge-20260505/`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AL-verify-matrix-minimal-20260505/`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AM-browser-risky-interaction-20260505/`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AP-knowledge-answer-eval-pack-20260506/`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-05-06/INDEX.md`
- 已识别待入库代码 / 入口：
  - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/verify.rs`
  - `D:/newwork/本地智能体/frontend/scripts/browser-mcp-server.mjs`
  - `D:/newwork/本地智能体/gateway/internal/api/browser_mcp_runtime_bridge_e2e_test.go`
  - `D:/newwork/本地智能体/scripts/run-knowledge-answer-eval-pack.ps1`
- 热点拆分备案依据：
  - `D:/newwork/本地智能体/crates/runtime-core/src/verify.rs`：`40-60` 统一入口、`264-675` 多任务规则、`722+` 测试模块
  - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs`：`44-137` 主入口、`138-301` replan 与 browser / knowledge follow-up、`304+` 测试模块
- 聚合回归与证据入口依据：
  - `D:/newwork/本地智能体/scripts/README.md`
  - `D:/newwork/本地智能体/docs/07-test/evidence/20260506-an-browser-recovery-governance/README.md`
  - `D:/newwork/本地智能体/docs/07-test/evidence/20260506-ap-knowledge-answer-eval-pack/README.md`
  - `D:/newwork/本地智能体/tmp/knowledge-answer-evals/latest.json`
  - `D:/newwork/本地智能体/tmp/knowledge-answer-evals/knowledge-answer-eval-*.json`
- 任务 8 裁决依据：
  - `verify.rs` 已被确认为第一优先热点且具备自然模块边界
  - `query_engine.rs` 已被确认为第二优先热点，但不应与 `verify.rs` 同时打包
  - `AR` 的定位是收口与裁决，不是直接结构搬移实现

## 本轮最终裁决结论

1. AR 已完成状态校准、改动归属、热点备案、证据入口收口与下一步裁决。
2. 唯一下一步最小实现项已经确定为：新开 `AS-verify-structure-split-20260506`，只做 `verify.rs` 结构拆分。
3. `query_engine.rs`、更广义知识问答扩面、其它热点文件拆分都被明确后置，不得在 `AS` 内混做。
4. 因此 AR 当前已经达到“可审阅、可切下一主推进项”的状态，不需要再在本 change 内继续扩实现。

## Gate 映射

- 对应阶段 Gate：`Gate-I（已收口；当前处于阶段 I 自由迭代期）`
- 当前覆盖情况：
  - 已完成当前状态口径与真实工作区不一致的识别
  - 已建立 AR 作为正式收口入口
  - 已完成热点文件与待收口目录的第一轮盘点
  - 已完成未提交改动归属矩阵
  - 已完成 `verify.rs`、`query_engine.rs` 拆分备案
  - 已完成聚合回归与证据入口收口单
  - 已完成唯一下一步最小实现闭环的裁决
