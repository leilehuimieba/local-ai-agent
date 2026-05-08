# 未提交改动归属矩阵与建议入库顺序

更新时间：2026-05-06
适用范围：`D:/newwork/本地智能体`

## 1. 目的

本文件只服务于 `AW-post-av-closeout-routing-20260506` 的 routing 收口判断：

1. 把当前 `git status --short` 与 `git ls-files --others --exclude-standard` 中的未提交路径，按既有 change 归属重新挂账；
2. 明确哪些是“已完成 change 的代码残余”、哪些是“文档 / 证据 / 归档残余”、哪些是“当前 routing 状态残余”；
3. 给出建议入库顺序，避免后续把不同 change 混在同一批次继续扩大 scope。

本文件不是新的实现提案，不承担新的运行时能力设计。

## 2. 当前总判断

当前未提交工作区已经可以稳定回落到既有 change：

1. `AS / AT / AV`：结构治理与 memory 写回结构收口残余；
2. `AM / AN / AL`：浏览器高风险交互、恢复治理与 verify 透出残余；
3. `AP / AQ`：知识回答评测资产与最小主链收口残余；
4. `AR / AU / AW`：状态校准、归属路由与当前 routing 文档残余；
5. `AG / AL`：蓝图 / 已完成 change 的状态回写残余。

因此当前最稳妥的动作仍然不是切出新的实现型 change，而是先按既有 change 把未提交路径重新收口。

## 3. 归属矩阵

### 3.1 路由 / 状态口径层

| 路径组 | 主归属 | 当前性质 | 建议动作 |
| --- | --- | --- | --- |
| `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md` | `AW` | 当前唯一事实源 | 保持为当前主口径 |
| `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md` | `AW` | 当前活跃 change 索引 | 与 `current-state.md` 同批审阅 |
| `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AW-post-av-closeout-routing-20260506/` | `AW` | 当前 routing change 工作区 | 继续作为本轮唯一推进区 |
| `D:/newwork/本地智能体/docs/11-hermes-rebuild/stage-plans/阶段计划总表.md` | `AW` | 次级入口口径回写 | 修正“无当前活跃 change”的过时描述 |

### 3.2 结构治理 / memory 写回残余

| 路径组 | 主归属 | 次级关联 | 当前性质 | 建议动作 |
| --- | --- | --- | --- | --- |
| `D:/newwork/本地智能体/crates/runtime-core/src/verify.rs` | `AS` | `AL` | 已完成结构拆分后的主文件残余 | 视为 `AS` 代码残余，不单开新项 |
| `D:/newwork/本地智能体/crates/runtime-core/src/verify/` | `AS` | `AL` | 新增模块与测试目录 | 与 `verify.rs` 绑定入库 |
| `D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs` | `AT` | `AQ`、`AL`、`AN` | 主文件已降到 `98` 行，但仍承载 follow-up / replan 收口 | 主归属仍按 `AT`，其它主题只做次级说明 |
| `D:/newwork/本地智能体/crates/runtime-core/src/query_engine/` | `AT` | `AQ`、`AL`、`AN` | 新增 follow-up / replan / tests 目录 | 与 `query_engine.rs` 同批 |
| `D:/newwork/本地智能体/crates/runtime-core/src/memory.rs` | `AV` | `AO` | 已完成第一刀结构收口 | 视为 `AV` 代码残余 |
| `D:/newwork/本地智能体/crates/runtime-core/src/memory/policy.rs` | `AV` | `AO` | 新增 memory policy 模块 | 与 `memory.rs` 绑定入库 |
| `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/mod.rs` | `AV` | `AO` | 已完成三刀结构收口 | 视为 `AV` 主体代码残余 |
| `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/audit.rs` | `AV` | `AO` | 新增治理模块 | 与 `memory_router/mod.rs` 同批 |
| `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/knowledge_write.rs` | `AV` | `AO` | 新增写回分层模块 | 与 `memory_router/mod.rs` 同批 |
| `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/write_policy.rs` | `AV` | `AO` | 新增准入策略模块 | 与 `memory_router/mod.rs` 同批 |
| `D:/newwork/本地智能体/crates/runtime-core/src/memory_router/tests.rs` | `AV` | `AO` | 配套测试残余 | 与 `memory_router` 同批 |
| `D:/newwork/本地智能体/crates/runtime-core/src/sqlite_store/mod.rs` | `AV` | `AO` | 已完成结构收口 | 视为 `AV` 主体代码残余 |
| `D:/newwork/本地智能体/crates/runtime-core/src/sqlite_store/schema.rs` | `AV` | `AO` | 新增 schema 模块 | 与 `sqlite_store/mod.rs` 同批 |
| `D:/newwork/本地智能体/crates/runtime-core/src/sqlite_store/cleanup_rules.rs` | `AV` | `AO` | 新增 cleanup 模块 | 与 `sqlite_store/mod.rs` 同批 |
| `D:/newwork/本地智能体/crates/runtime-core/src/context_builder.rs` | `AV` | `AO` | 测试样本补字段 | 作为 `AV/AO` 配套改动处理 |
| `D:/newwork/本地智能体/crates/runtime-core/src/executors/memory.rs` | `AV` | `AO` | memory entry 默认字段补齐 | 作为 `AV/AO` 配套改动处理 |
| `D:/newwork/本地智能体/crates/runtime-core/src/memory_object_store.rs` | `AV` | `AO` | 测试样本补字段 | 作为 `AV/AO` 配套改动处理 |
| `D:/newwork/本地智能体/crates/runtime-core/src/memory_recall.rs` | `AV` | `AO` | 测试样本补字段 | 作为 `AV/AO` 配套改动处理 |
| `D:/newwork/本地智能体/crates/runtime-core/src/memory_schema.rs` | `AV` | `AO` | 结构字段兼容残余 | 作为 `AV/AO` 配套改动处理 |
| `D:/newwork/本地智能体/crates/runtime-core/src/run_memory_metadata.rs` | `AV` | `AO` | 写回治理 metadata 透出 | 主归属按 `AV` |
| `D:/newwork/本地智能体/crates/runtime-core/src/run_finish_events.rs` | `AV` | `AO` | 记忆治理测试补位 | 主归属按 `AV`，不并入浏览器簇 |
| `D:/newwork/本地智能体/crates/runtime-core/src/sqlite_store/memory_object.rs` | `AV` | `AO` | rollback entry 字段兼容 | 作为 `AV/AO` 配套改动处理 |
| `D:/newwork/本地智能体/crates/runtime-core/src/storage_migration.rs` | `AV` | `AO` | 迁移兼容字段补齐 | 作为 `AV/AO` 配套改动处理 |

### 3.3 浏览器高风险交互 / 恢复治理残余

| 路径组 | 主归属 | 次级关联 | 当前性质 | 建议动作 |
| --- | --- | --- | --- | --- |
| `D:/newwork/本地智能体/crates/runtime-core/src/contracts.rs` | `AN` | `AL` | `VerificationSnapshot` 浏览器字段透出 | 作为浏览器失败链配套改动处理 |
| `D:/newwork/本地智能体/crates/runtime-core/src/events.rs` | `AN` | `AL` | verification snapshot 浏览器失败字段透出 | 主归属按 `AN` |
| `D:/newwork/本地智能体/crates/runtime-core/src/handoff.rs` | `AN` | `AL` | handoff artifact 浏览器失败字段透出 | 主归属按 `AN` |
| `D:/newwork/本地智能体/crates/runtime-core/src/run_failure_metadata.rs` | `AN` | `AL` | failure metadata 浏览器 page/selector/hint 透出 | 主归属按 `AN` |
| `D:/newwork/本地智能体/crates/runtime-core/src/run_verification_metadata.rs` | `AN` | `AL` | verification metadata 浏览器字段透出 | 主归属按 `AN` |
| `D:/newwork/本地智能体/crates/runtime-core/src/run_resume_event_testkit.rs` | `AN` | `AL` | 测试样本补浏览器字段 | 作为 `AN/AL` 配套改动处理 |
| `D:/newwork/本地智能体/frontend/scripts/browser-mcp-server.mjs` | `AM` | `AN` | `select / submit / upload` 高风险交互实现 | 主归属按 `AM` |
| `D:/newwork/本地智能体/gateway/internal/api/browser_mcp_runtime_bridge_e2e_test.go` | `AM` | `AN` | Browser MCP 风险交互与桥接联调证据 | 主归属按 `AM` |
| `D:/newwork/本地智能体/docs/07-test/evidence/20260506-an-browser-recovery-governance/README.md` | `AN` | 无 | 最小失败链证据目录 | 作为有效证据保留 |
| `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AM-browser-risky-interaction-20260505/` | `AM` | 无 | 已收口文档，但当前未入库 | 只读等待归档/入库 |
| `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-05-06/AN-browser-recovery-governance-20260505/` | `AN` | 无 | 已归档准备文档 | 保持只读 |

### 3.4 知识回答资产 / 最小收口残余

| 路径组 | 主归属 | 次级关联 | 当前性质 | 建议动作 |
| --- | --- | --- | --- | --- |
| `D:/newwork/本地智能体/crates/runtime-core/src/executors/project.rs` | `AQ` | `AP` | agent 工程问答稳定回答收口 | 主归属按 `AQ` |
| `D:/newwork/本地智能体/crates/runtime-core/src/run_state_builder.rs` | `AQ` | `AT` | replan 后执行态 context rebuild 入口 | 主归属按 `AQ`，保留 `AT` 次级说明 |
| `D:/newwork/本地智能体/scripts/run-knowledge-answer-eval-pack.ps1` | `AP` | `AQ` | 真实入口回归脚本 | 作为 `AP` 资产保留 |
| `D:/newwork/本地智能体/scripts/README.md` | `AP` | 无 | 脚本索引回写 | 与 `AP` 同批 |
| `D:/newwork/本地智能体/docs/07-test/evidence/20260506-ap-knowledge-answer-eval-pack/README.md` | `AP` | 无 | 首轮失败与复跑证据说明 | 作为有效证据保留 |
| `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AP-knowledge-answer-eval-pack-20260506/` | `AP` | 无 | 输入 / 回归资产文档 | 保持只读 |
| `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-05-06/AQ-knowledge-answer-closure-20260506/` | `AQ` | 无 | 已签收待归档实现文档 | 保持只读 |

### 3.5 蓝图 / 历史状态回写残余

| 路径组 | 主归属 | 当前性质 | 建议动作 |
| --- | --- | --- | --- |
| `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AG-agent-loop-memory-knowledge-20260505/status.md` | `AG` | 蓝图状态回写 | 只做状态结论更新，不回 AG 扩实现 |
| `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AL-verify-matrix-minimal-20260505/status.md` | `AL` | 已完成 change 状态回写 | 与 `AL/AN` 证据口径保持一致 |
| `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AL-verify-matrix-minimal-20260505/verify.md` | `AL` | 已完成 change 验证回写 | 保持只读，作为历史证据 |
| `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AR-state-realignment-and-modularity-closeout-20260506/` | `AR` | 已完成归属与备案 change 文档 | 只读保留 |
| `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AU-worktree-ownership-and-closeout-routing-20260506/` | `AU` | 已完成上一轮路由 change 文档 | 只读保留 |
| `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AV-memory-writeback-structure-closeout-20260506/` | `AV` | 已完成当前上游结构治理文档 | 只读保留 |
| `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-05-06/INDEX.md` | `AN/AO/AQ` | 归档入口 | 与对应归档文档同批处理 |
| `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-05-06/review-queue-20260506.md` | `AO/AQ` | 并排签收与归档准备说明 | 保持只读 |

## 4. 建议入库顺序

当前建议顺序不是“继续实现顺序”，而是“把未提交工作区拆回既有 change 的最稳妥顺序”。

### 批次 0：状态口径先对齐

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
3. `D:/newwork/本地智能体/docs/11-hermes-rebuild/stage-plans/阶段计划总表.md`
4. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AW-post-av-closeout-routing-20260506/`

目的：

- 先保证所有执行入口都承认 `AW` 是当前活跃 change；
- 避免后续在“无活跃 change / AW 进行中”两种口径间来回跳。

### 批次 1：已完成 change 文档 / 证据先入库

1. `AM/`
2. `AP/`
3. `AR/`
4. `AS/`
5. `AT/`
6. `AU/`
7. `AV/`
8. `archive/2026-05-06/`
9. `docs/07-test/evidence/20260506-an-browser-recovery-governance/`
10. `docs/07-test/evidence/20260506-ap-knowledge-answer-eval-pack/`

目的：

- 先把“为什么有这些代码残余”讲清楚；
- 让后续代码入库不再缺 change 文档承接。

### 批次 2：结构治理代码残余

建议顺序：

1. `AS`：`verify.rs` 与 `verify/`
2. `AT`：`query_engine.rs`、`query_engine/`
3. `AV`：`memory.rs`、`memory_router/*`、`sqlite_store/*` 及其配套兼容文件

原因：

1. `AS`、`AT` 已确定为已收口结构治理，边界最单纯；
2. `AV` 体量最大、配套字段兼容文件最多，放在结构层最后更稳。

### 批次 3：浏览器高风险交互 / 恢复治理代码残余

1. `AM`：`frontend/scripts/browser-mcp-server.mjs`、`gateway/internal/api/browser_mcp_runtime_bridge_e2e_test.go`
2. `AN`：`contracts.rs`、`events.rs`、`handoff.rs`、`run_failure_metadata.rs`、`run_verification_metadata.rs`、`run_resume_event_testkit.rs`
3. `AL` 历史状态 / 验证回写

原因：

- 先把 Browser MCP 高风险交互实现与桥接联调证据归到 `AM`；
- 再把失败分型、handoff、metadata 透出归到 `AN`；
- 最后才是 `AL` 的历史验证口径回写。

### 批次 4：知识回答资产与收口代码残余

1. `AQ`：`executors/project.rs`、`run_state_builder.rs`
2. `AP`：`scripts/run-knowledge-answer-eval-pack.ps1`、`scripts/README.md`、`docs/07-test/evidence/20260506-ap-knowledge-answer-eval-pack/`

原因：

- 先入库最小主链收口代码；
- 再入库真实入口回归资产与证据说明。

### 批次 5：蓝图 / 历史状态回写

1. `AG/status.md`
2. `AL/status.md`
3. `AL/verify.md`

原因：

- 这些文件主要是历史口径修正，不应抢在主体代码 / 证据之前。

## 5. 当前不建议做的事

1. 不把当前所有未提交改动重新合并成一把新的大 change。
2. 不在 `AW` 内直接回到 runtime-core 继续扩 browser / knowledge / verify / memory 行为。
3. 不把 `tmp/knowledge-answer-evals/` 或 `docs/07-test/evidence/*` 当作“临时垃圾目录”直接清掉。
4. 不把 `AS / AT / AV / AM / AN / AQ` 这些已完成项重新改写为“仍在实现中”。

## 6. 当前最小下一步

若继续沿 `AW` 推进，最小动作应是：

1. 先按本文件确认“哪些 change 可以正式保持只读等待归档 / 入库”；
2. 再决定是否需要为“分批入库 / 归档顺序”单开一把新的 closeout change；
3. 在没有完成顺序裁决前，不直接进入下一轮业务实现。
