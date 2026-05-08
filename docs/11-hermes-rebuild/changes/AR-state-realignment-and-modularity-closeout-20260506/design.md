# 技术方案

## 影响范围

- 状态入口：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
- 本 change 工作区：
  - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AR-state-realignment-and-modularity-closeout-20260506/`
- 本轮主要观察对象：
  - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs`
  - `D:/newwork/本地智能体/crates/runtime-core/src/verify.rs`
  - `D:/newwork/本地智能体/frontend/scripts/browser-mcp-server.mjs`
  - `D:/newwork/本地智能体/gateway/internal/api/browser_mcp_runtime_bridge_e2e_test.go`
  - `D:/newwork/本地智能体/scripts/README.md`
  - `D:/newwork/本地智能体/docs/07-test/evidence/20260506-an-browser-recovery-governance/`
  - `D:/newwork/本地智能体/docs/07-test/evidence/20260506-ap-knowledge-answer-eval-pack/`
  - `D:/newwork/本地智能体/tmp/knowledge-answer-evals/`

## 方案

### 1. 状态口径对齐

- 由 `AR-state-realignment-and-modularity-closeout-20260506` 接管当前主推进口径。
- `current-state.md` 不再写“无当前活跃 change”，而是明确：当前仍有未提交代码、未入库 change 材料与待收口证据目录，必须先由 AR 归口。
- `changes/INDEX.md` 只保留一个当前活跃 change：AR；`AP-knowledge-answer-eval-pack-20260506` 继续作为输入 / 回归 change 保留。

### 2. 未收口改动归属策略

- 当前工作区中的 runtime / browser / gateway / docs / scripts / evidence 改动，统一先归到 AR 的“待收口改动归属矩阵”。
- 这些改动的历史来源可以追溯到 `AL / AM / AO / AQ / AP`，但 AR 的职责不是重开这些 change，而是决定：
  - 哪些属于已完成待入库的既有成果；
  - 哪些需要补文档或补证据；
  - 哪些必须再拆出新的实现 change。
- 在 AR 收口前，不再继续扩老 change 的范围描述。

### 3. 未提交改动归属矩阵

| 分类 | 对象 | 当前判断 | 归属依据 | AR 动作 |
|---|---|---|---|---|
| 已完成待入库 | `D:/newwork/本地智能体/crates/runtime-core/src/context_builder.rs`、`contracts.rs`、`events.rs`、`executors/memory.rs`、`executors/project.rs`、`handoff.rs`、`memory.rs`、`memory_object_store.rs`、`memory_recall.rs`、`memory_router/mod.rs`、`memory_router/tests.rs`、`memory_schema.rs`、`query_engine.rs`、`run_failure_metadata.rs`、`run_finish_events.rs`、`run_memory_metadata.rs`、`run_resume_event_testkit.rs`、`run_state_builder.rs`、`run_verification_metadata.rs`、`sqlite_store/memory_object.rs`、`sqlite_store/mod.rs`、`storage_migration.rs`、`verify.rs` | 属于既有实现成果的未入库代码，不应继续按“无活跃 change”处理 | 代码增量对应 `AL` 浏览器 verify、`AO` 写回治理、`AQ` 知识回答收口；现有 `status.md` / `verify.md` / `review.md` 已给出完成结论 | 先在 AR 中标为“既有成果待入库”，不重开原 change |
| 已完成待入库 | `D:/newwork/本地智能体/frontend/scripts/browser-mcp-server.mjs`、`D:/newwork/本地智能体/gateway/internal/api/browser_mcp_runtime_bridge_e2e_test.go`、`D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AM-browser-risky-interaction-20260505/` | 属于 `AM` 已完成但尚未正式入库的浏览器高风险交互增量 | `AM` 状态已写明 `select / submit / upload` contract、桥接测试与端到端证据完成 | 在 AR 中归为“AM 既有成果待入库”，不继续扩 browser scope |
| 已完成待入库 | `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/archive/2026-05-06/`、`D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AG-agent-loop-memory-knowledge-20260505/status.md`、`D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AL-verify-matrix-minimal-20260505/status.md`、`D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AL-verify-matrix-minimal-20260505/verify.md` | 属于已完成实现后的状态回写、归档入口与蓝图口径修正 | 文档内容与当前代码状态一致，作用是把已完成成果写回 change 口径 | 在 AR 中作为“文档回写待入库”处理 |
| 已完成待入库 | `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AP-knowledge-answer-eval-pack-20260506/`、`D:/newwork/本地智能体/scripts/run-knowledge-answer-eval-pack.ps1` | 属于 `AP` 已完成的输入 / 回归资产，不属于新的实现型 change | `AP` 已完成 fixture、脚本、首轮失败留痕，且已明确“不继续扩 AP” | 在 AR 中归为“输入资产待入库”，继续保留只读输入角色 |
| 待补文档 / 待补证据引用 | `D:/newwork/本地智能体/scripts/README.md`、`D:/newwork/本地智能体/docs/07-test/evidence/20260506-an-browser-recovery-governance/README.md`、`D:/newwork/本地智能体/docs/07-test/evidence/20260506-ap-knowledge-answer-eval-pack/README.md` | 实体证据已存在，但还缺 AR 级统一引用口径与收口说明 | 当前 README / evidence 文档只说明各自来源，还没有 AR 的聚合入口 | 进入 AR 任务 7，补“聚合回归与证据入口收口单” |
| 待补文档 / 待补状态同步 | `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`、`D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`、`D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AR-state-realignment-and-modularity-closeout-20260506/` | 入口已切换完成，但还要继续补 AR 的任务 6~8 状态闭环 | 当前已完成建档，尚未补齐拆分备案与收口单 | 继续由 AR 自身推进，不外溢到其它 change |
| 需新开实现 change | `D:/newwork/本地智能体/crates/runtime-core/src/verify.rs`、`D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs` 的正式拆分落地 | 当前不属于“待入库就结束”，而是下一把结构治理实现任务 | 两个文件已分别达到 1325 行、689 行，且本轮仍持续承接新增逻辑 | 在 AR 中只备案，不在本轮实施；后续单开结构治理 change |
| 需新开实现 change | 更广义知识问答扩面、更多热点文件拆分（如 `observation/mod.rs`、`planner.rs`、`knowledge.rs`、`task-view.tsx`） | 不应混进 AR 当前收口 | 当前只确认风险和优先级，未形成单独实现方案 | 作为 AR 后置项，不在本轮展开 |

### 4. 热点文件拆分备案

#### 4.1 `verify.rs` 拆分备案

- 当前问题：
  - `D:/newwork/本地智能体/crates/runtime-core/src/verify.rs` 已达 `1325` 行。
  - 文件同时承载：
    - 通用 `VerificationOutcome / VerificationReport` 数据结构
    - task type 与 policy 判断
    - evidence 提取
    - `knowledge_answer / browser_interaction / file_change / command_execution / memory_write` 五类 verify 规则
    - browser failure metadata 与 next-step 生成
    - 大量内联测试
- 当前职责分布判断：
  - 入口与通用模型：约 `1-263`
  - 知识回答规则：约 `264-318`
  - 浏览器交互规则：约 `319-675`
  - 文件 / 命令 / 记忆规则：约 `676-721`
  - 测试模块：`722` 之后
- 建议拆分方向：
  1. `verify/mod.rs`
     - 保留 `verify_tool_execution` 统一入口
     - 保留 `VerificationOutcome / VerificationReport`
  2. `verify/policy.rs`
     - 放 `verification_task_type`、`verification_policy`、`is_browser_tool_name`
  3. `verify/evidence.rs`
     - 放 `verification_evidence`、`skill_hit_reason`、`guard_*`
  4. `verify/knowledge_answer.rs`
     - 放知识回答校验与 citation / fact-inference 边界判定
  5. `verify/browser_interaction.rs`
     - 放 browser verify、failure type、page/selector、recovery summary
  6. `verify/file_command_memory.rs`
     - 放 file change、command execution、memory write 三类较轻逻辑
  7. `verify/tests/`
     - 把超长测试模块拆开，至少按 `knowledge / browser / file_command_memory` 分组
- 为什么优先拆它：
  - 它已经是当前全仓最高风险热点之一。
  - 既有实现还在持续往里叠新规则，继续追加只会抬高后续改造成本。
  - 该文件已经天然具备“按任务类型拆模块”的边界，拆分收益明确。
- 本次明确不做：
  - 不改变 verify contract 字段语义
  - 不改变现有 verify 判定阈值
  - 不在 AR 内直接提交任何 Rust 结构改造

#### 4.2 `query_engine.rs` 拆分备案

- 当前问题：
  - `D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs` 已达 `689` 行。
  - 当前文件同时承载：
    - runtime envelope / run state 数据结构
    - bootstrap / execute stage
    - replan 触发条件
    - next action 推导
    - browser readback 路径
    - knowledge follow-up 路径
    - JSON 参数抽取辅助函数
    - 大量测试样例与测试辅助构造
- 当前职责分布判断：
  - runtime state 与 stage 主入口：约 `1-137`
  - replan / next action 决策：约 `138-234`
  - browser / knowledge 辅助判断：约 `236-301`
  - 测试模块：`304` 之后
- 建议拆分方向：
  1. `query_engine/mod.rs`
     - 保留 `RuntimeEnvelope`、`RuntimeRunState`
     - 保留 `bootstrap_run`、`execute_stage`、`should_replan`、`replan_state`
  2. `query_engine/replan.rs`
     - 放 `next_action_from_trace`、`remaining_steps_for_action`、`verify_failed_for_replan`
  3. `query_engine/browser_recovery.rs`
     - 放 browser verify 失败后的 `read_page` follow-up、`page_id` 提取等逻辑
  4. `query_engine/knowledge_followup.rs`
     - 放 `search_knowledge_followup_action`、`agent_knowledge_question`
  5. `query_engine/path_navigation.rs`
     - 放 `next_read_action`、`next_list_action`、`extract_candidate_path`、`clean_path`、`join_base_path`
  6. `query_engine/tests/`
     - 把 browser、knowledge、resume、path-navigation 测试拆开
- 为什么第二优先拆它：
  - 当前 `query_engine.rs` 已经开始混合“状态结构 + 重规划策略 + 工具特化恢复 + 测试脚手架”。
  - 如果继续在单文件里叠 browser / knowledge / recovery 逻辑，会让主循环边界越来越模糊。
  - 但相较 `verify.rs`，它的拆分压力略低，因此排第二优先。
- 本次明确不做：
  - 不改主循环 contract
  - 不调 iteration budget / replan 行为
  - 不在 AR 内直接做模块搬移

#### 4.3 拆分优先顺序

1. 第一优先：`D:/newwork/本地智能体/crates/runtime-core/src/verify.rs`
2. 第二优先：`D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs`
3. 第三优先：其余 runtime-core 热点与前端热点进入后续治理池

#### 4.4 拆分触发条件

- AR 收口完成后，如要进入实现，必须单开结构治理 change。
- 新 change 至少需要：
  - 明确只做结构拆分，不顺手扩逻辑
  - 为每个拆分目标补最小回归证据
  - 明确回退路径：可以只回退模块搬移，不回退已签收行为本身

### 5. 聚合回归与证据入口收口单

#### 5.1 当前有效入口定义

- AR 口径下，当前与本轮收口直接相关的聚合回归 / 证据入口只保留以下四类：
  1. 脚本入口：`D:/newwork/本地智能体/scripts/run-knowledge-answer-eval-pack.ps1`
  2. 脚本索引：`D:/newwork/本地智能体/scripts/README.md`
  3. 证据目录：
     - `D:/newwork/本地智能体/docs/07-test/evidence/20260506-an-browser-recovery-governance/`
     - `D:/newwork/本地智能体/docs/07-test/evidence/20260506-ap-knowledge-answer-eval-pack/`
  4. 运行产物目录：`D:/newwork/本地智能体/tmp/knowledge-answer-evals/`
- 其余 `tmp/` 子目录、历史脚本和更早 evidence 目录，本轮不作为 AR 当前收口对象。

#### 5.2 scripts 口径

- `D:/newwork/本地智能体/scripts/README.md` 当前已经把 `run-knowledge-answer-eval-pack.ps1` 列入“当前保留脚本”。
- AR 对它的统一解释为：
  - 它是 `AP` 的真实入口脚本；
  - 在 AR 中扮演“当前有效的聚合回归脚本入口”；
  - 其任务是提供 `knowledge-answer-evals/latest.json` 与历史批次 JSON，不承担新的实现职责。
- 收口要求：
  - 在后续入库时，保留 README 中对该脚本的说明；
  - 不再把同类说明散落在多个 change 里重复维护；
  - 若后续出现新聚合脚本，应优先追加到 `scripts/README.md`，再由新 change 文档引用。

#### 5.3 evidence 口径

- `D:/newwork/本地智能体/docs/07-test/evidence/20260506-an-browser-recovery-governance/README.md`
  - 角色：`AN` 的最小失败链证据说明；
  - 作用：证明 browser recovery failure 路径已具备“可读、可停止、可交接”的收口能力；
  - AR 处理：保留为专项证据，不要求补成新的端到端大证据。
- `D:/newwork/本地智能体/docs/07-test/evidence/20260506-ap-knowledge-answer-eval-pack/README.md`
  - 角色：`AP` 回归包的执行与验收说明；
  - 作用：为 `run-knowledge-answer-eval-pack.ps1` 与 `tmp/knowledge-answer-evals/` 建立文档映射；
  - AR 处理：保留为当前知识回答聚合回归的说明入口。
- 收口要求：
  - 两个 evidence README 保持“各自说明本刀证据”的职责，不在 AR 里重写原始证据；
  - AR 只负责定义它们的引用层级：
    - `AN` 证据 = 浏览器恢复治理专项证据
    - `AP` 证据 = 知识回答聚合回归专项证据

#### 5.4 tmp 口径

- `D:/newwork/本地智能体/tmp/knowledge-answer-evals/` 当前结构包含：
  - `latest.json`
  - 多个历史批次 JSON：`knowledge-answer-eval-*.json`
  - `bin/`、`cargo-target/`、`logs/`
- AR 对该目录的统一解释为：
  - `latest.json` 是当前默认读取的聚合报告入口；
  - `knowledge-answer-eval-*.json` 是历史批次报告，作为追溯样本保留；
  - `bin/`、`cargo-target/`、`logs/` 属于运行期支撑产物，不作为长期文档入口引用。
- 收口要求：
  - 文档默认只引用 `latest.json` 作为“当前结果”；
  - 需要追溯时，再按 run-id 读取具体历史批次文件；
  - `bin/`、`cargo-target/`、`logs/` 当前不清理，但也不在主文档中当成证据主入口。

#### 5.5 引用顺序约定

- 在后续任何状态文档或新 change 中，如需引用本轮聚合回归与证据入口，默认按以下顺序：
  1. `scripts/README.md`：确认脚本是否为当前有效入口
  2. 对应 evidence README：确认该脚本 / 证据的语义和边界
  3. `tmp/.../latest.json`：读取当前结果
  4. 需要追溯时，再读取历史批次 JSON
- 这样可以避免：
  - 直接把 `tmp/` 历史产物误当成文档主记录
  - 把 evidence README 当成实现说明
  - 把脚本说明、证据说明、运行结果三层混写

#### 5.6 后续清理边界

- 本轮不清理：
  - `D:/newwork/本地智能体/tmp/knowledge-answer-evals/bin/`
  - `D:/newwork/本地智能体/tmp/knowledge-answer-evals/cargo-target/`
  - `D:/newwork/本地智能体/tmp/knowledge-answer-evals/logs/`
- 后续如需清理，应单独满足两个条件：
  1. `latest.json` 与历史批次 JSON 已确认保留或归档
  2. 清理动作不影响复跑脚本与证据追溯
- 因此当前处理方式是：
  - 先收口引用口径
  - 暂不做运行目录清扫实现

### 6. scripts / evidence / tmp 收口边界

- `scripts`：确认 `D:/newwork/本地智能体/scripts/run-knowledge-answer-eval-pack.ps1` 与 `D:/newwork/本地智能体/scripts/README.md` 的入口说明是否与当前文档口径一致。
- `evidence`：把 `20260506-an-browser-recovery-governance` 与 `20260506-ap-knowledge-answer-eval-pack` 作为已生成证据目录记录到 AR，不在本轮重复造新证据。
- `tmp`：把 `D:/newwork/本地智能体/tmp/knowledge-answer-evals/` 视为当前聚合回归产物目录；其余 `tmp/` 目录本轮只做观察清单，不做大规模清理。

### 7. 唯一下一步最小实现闭环裁决

#### 7.1 裁决结论

- AR 收口后的唯一下一步最小实现项，确定为：
  - **新开独立结构治理 change：`AS-verify-structure-split-20260506`**
- 该 change 只做一件事：
  - 把 `D:/newwork/本地智能体/crates/runtime-core/src/verify.rs` 按既定边界拆成多模块
- 当前明确不把 `query_engine.rs` 一并打包进同一把最小实现项。

#### 7.2 为什么不是别的项

- 不是“继续在 AR 内直接拆代码”：
  - 因为 AR 是状态校准与收口 change，不是实现型 change。
- 不是“先拆 `query_engine.rs`”：
  - 因为 `verify.rs` 风险更高、边界更自然、收益更直接，已被备案为第一优先。
- 不是“先做更广义知识问答扩面”：
  - 因为这会扩能力 scope，而不是先清结构债务。
- 不是“同时拆 `verify.rs + query_engine.rs`”：
  - 因为这会把最小实现闭环重新放大，违背 AR 当前“单一最小项”原则。

#### 7.3 前置依赖

- `AR-state-realignment-and-modularity-closeout-20260506` 的八项任务完成并留痕。
- `verify.rs` 的模块边界、不可变 contract、测试入口已在 AR 中备案。
- 当前待入库成果继续按既有成果处理，不要求在 AR 内先完成第二轮逻辑实现。

#### 7.4 `AS` 的范围冻结

- 只允许覆盖：
  - `verify/mod.rs`
  - `verify/policy.rs`
  - `verify/evidence.rs`
  - `verify/knowledge_answer.rs`
  - `verify/browser_interaction.rs`
  - `verify/file_command_memory.rs`
  - `verify/tests/`
  - 以及为模块接线所必需的最小引用改动
- 明确不做：
  - 不调整 verify 通过 / 失败规则
  - 不改 metadata contract
  - 不把 `query_engine.rs`、knowledge 扩面、browser 能力扩面一起塞入

#### 7.5 `AS` 的最小验收要求

- `verify.rs` 主文件行数显著下降，不再保留多类逻辑与大测试块混杂的现状。
- 既有 verify 相关定向测试保持通过。
- `run_verification_metadata` 相关校验保持通过。
- 如需新增测试，只允许围绕结构搬移后的接线安全，不新增能力 scope。

#### 7.6 后置项

- 第一后置项：`query_engine.rs` 结构拆分，建议作为 `AT` 类独立 change 后置处理。
- 第二后置项：其余 runtime-core 热点文件继续进入治理池。
- 第三后置项：更广义知识问答扩面与前端热点拆分继续后置。
- 第四后置项：`tmp/knowledge-answer-evals/bin`、`cargo-target`、`logs` 的清理仅在不影响追溯时再做。

## 风险与回退

- 主要风险：把“已完成但未入库”与“仍需继续实现”的改动混为一谈，导致再次扩 scope。
- 主要风险：一次性把所有热点文件都塞进当前 change，造成任务失控。
- 主要风险：误把历史归档 change 重新激活。
- 回退方式：如 AR 口径判断失真，仅回退 `current-state.md`、`changes/INDEX.md` 与 AR 文档，不回退既有代码改动。
