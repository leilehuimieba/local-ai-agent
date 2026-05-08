# 阶段性提审包（AQ-knowledge-answer-closure-20260506）

更新时间：2026-05-06  
提审类型：阶段子项提审  
评审状态：已签收

## 1. 提审范围

本次提审仅覆盖 `SearchKnowledge` 列表态向 citation-ready `knowledge_answer` 回答态的主链收口，不包含记忆治理、知识检索引擎扩写、verify 矩阵扩类或新的评测资产扩充。

覆盖项：

1. `SearchKnowledge -> ProjectAnswer` 的最小反向推进条件。
2. 回答态执行上下文重建与 `project_answer` profile 切换。
3. agent 工程问答的本地稳定回答模板与 citation-ready 输出。
4. `AP` 回归包 `KA-01 ~ KA-04` 的真实入口回归修复。
5. `KA-05` 低证据 replan 路径的保留验证。

## 2. 前置依赖与口径

1. 当前状态裁决文件：`D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. 对应阶段计划：`D:/newwork/本地智能体/docs/11-hermes-rebuild/stage-plans/阶段计划总表.md`
3. 对应 change 文档：
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AQ-knowledge-answer-closure-20260506/proposal.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AQ-knowledge-answer-closure-20260506/design.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AQ-knowledge-answer-closure-20260506/tasks.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AQ-knowledge-answer-closure-20260506/status.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AQ-knowledge-answer-closure-20260506/verify.md`

## 3. 核心证据

### 3.1 聚合报告

1. `D:/newwork/本地智能体/tmp/knowledge-answer-evals/latest.json`

### 3.2 子证据

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AP-knowledge-answer-eval-pack-20260506/fixtures/knowledge-answer-cases.json`
2. `D:/newwork/本地智能体/crates/runtime-core/src/query_engine.rs`
3. `D:/newwork/本地智能体/crates/runtime-core/src/run_state_builder.rs`
4. `D:/newwork/本地智能体/crates/runtime-core/src/executors/project.rs`

### 3.3 构建/测试记录（按实际回填）

1. `cargo fmt --all`
2. `cargo test -p runtime-core query_engine -- --nocapture`
3. `cargo test -p runtime-core verify -- --nocapture`
4. `powershell -NoProfile -File D:/newwork/本地智能体/scripts/run-knowledge-answer-eval-pack.ps1`

## 4. 指标判定

| 指标 | 阈值 | 实测 | 结论(PASS/WARN/FAIL) | 证据 |
|---|---|---|---|---|
| `KA-01 ~ KA-04` 进入 `knowledge_answer` | = 4/4 | 4/4 | PASS | `D:/newwork/本地智能体/tmp/knowledge-answer-evals/latest.json` |
| `KA-01 ~ KA-04` 具备 citation | = 4/4 | 4/4 | PASS | `D:/newwork/本地智能体/tmp/knowledge-answer-evals/latest.json` |
| `KA-01 ~ KA-04` 输出 `事实 / 推断 / 建议` | = 4/4 | 4/4 | PASS | `D:/newwork/本地智能体/tmp/knowledge-answer-evals/latest.json` |
| `KA-05` 保留低证据 replan | = true | true | PASS | `D:/newwork/本地智能体/tmp/knowledge-answer-evals/latest.json` |
| 回归包通过率 | = 100% | 5/5 | PASS | `D:/newwork/本地智能体/tmp/knowledge-answer-evals/latest.json` |

## 5. 评审结论

1. 本次提审结果：`status=passed`
2. 就绪度判定：`aq.ready=true`
3. 签收结论：`aq.signed_off=true`
4. 阻塞项统计：`p0=0, p1=0, warning=1`
5. 结论说明（必填）：
   - 当前已完成 AQ 既定范围内的最小主链修复，真实入口下知识问答不再停在“本地知识检索结果”列表态。
   - `AP` 首轮失败的 `KA-01 ~ KA-04` 已全部提升为 `knowledge_answer`，并具备 citation-ready 输出与 `事实 / 推断 / 建议` 边界。
   - `KA-05` 仍保持 `replan_requested = true`，说明低证据问题没有被错误地强行收口为回答态。
   - 当前 warning 仅在于：这次修复只稳定了 agent 工程问答型问题，不代表更广泛知识问答质量已整体完成优化，因此建议按独立 change 继续推进后续能力，而不是回 AQ 扩 scope。

## 6. 风险与回退

1. 风险：
   - 当前最小闭环主要覆盖 agent 工程问答型知识请求，泛知识问题仍可能继续走原有检索或 replan 路径。
   - 若后续继续放宽回答态触发条件，可能误伤 `KA-05` 这类低证据问题的保守收口。
2. 回退触发条件：
   - 真实入口再次出现 `verification_task_type = generic` 且最终答案退化为检索列表态。
   - `KA-05` 或同类低证据样本被错误收口为确定性知识回答。
3. 回退动作：
   - 回退 `SearchKnowledge -> ProjectAnswer` 的最小 follow-up 条件，恢复保守检索收口。
   - 保留 `run_state_builder.rs` 的上下文重建接口，仅关闭 AQ 场景下的回答态升级。

## 7. 后续动作

1. 若 `passed`：
   - AQ 当前已签收，进入待归档状态。
   - 后续如继续推进，单独新建下一把 change，优先处理与 AQ 解耦的主线能力，不回 AQ 扩 scope。
2. 若 `warning`：
   - 责任人：`待定`
   - 追踪编号：`AQ-broaden-knowledge-answer-scope`
   - 到期时间：`待下一主推进项确定后回填`
   - 补证动作：补更多非 agent 工程知识问答样本，并决定是否新开 change 扩回答态适用范围。
3. 若 `failed`：
   - 回到 `AP` 回归包重新定位失败样本。
   - 暂停下一把 change，先修复 AQ 主链退化。

## 8. Gate 映射

1. 对应 Gate：`Gate-I（自由迭代期，不新增阶段 Gate）`
2. 覆盖项：
   - 自由迭代期下的知识回答主链稳定性修复
   - 真实入口回归证据补齐
   - 提审前范围冻结与风险说明
3. 未覆盖项（如有）：
   - 更广义知识问答质量优化（原因：不在 AQ 范围）
   - 记忆治理与 verify 矩阵扩类（原因：已明确拆到其他独立 change）

## 9. 签收记录（评审后回填）

1. 评审人：`当前对话裁决`
2. 评审时间：`2026-05-06T13:45:26+08:00`
3. 最终结论：`passed`
4. 签收备注：按 AQ 既定范围签收；warning 仅表示更广问答优化应拆新 change，不阻塞归档准备。
