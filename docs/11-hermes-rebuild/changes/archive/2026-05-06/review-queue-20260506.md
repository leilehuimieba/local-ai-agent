# AO / AQ 签收与归档准备说明

更新时间：2026-05-06

## 目的

本文件用于把当前自由迭代期里已完成并签收的两把 change 收拢成一个并排归档准备入口，方便主控侧或后续对话直接判断：

1. 这两把分别做了什么
2. 哪些证据已经足够
3. 哪些 warning 只是范围边界，不是阻塞
4. 下一步该归档，还是切下一把主推进项

本文件只服务于：

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AO-memory-writeback-governance-20260506/`
2. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AQ-knowledge-answer-closure-20260506/`

本文件不替代：

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. 两个 change 各自的 `review.md`

## 当前结论

当前可以稳定交接为：

- **AO 与 AQ 都已完成实现、测试、提审与签收；当前最合理动作不是回原 change 扩 scope，而是进入归档准备。**

更短口径可写为：

- **当前无活跃 change；AO / AQ 均处于“已签收、待归档”状态。**

## 并排签收卡片

| change | 主题 | 当前最强结论 | 关键证据 | 当前 warning | 推荐动作 |
|---|---|---|---|---|---|
| `AO-memory-writeback-governance-20260506` | 记忆写回治理 | 最小写回分层、准入/拒绝、治理字段与事件透出已经闭环 | `status.md`、`verify.md`、`review.md`、`cargo test -p runtime-core` | `preference_summary(...)` 的存量超长函数属于工程护栏提醒，不阻塞本刀签收 | 进入归档准备，不回 AO 扩 recall/知识/verify |
| `AQ-knowledge-answer-closure-20260506` | 知识问答收口治理 | `SearchKnowledge -> knowledge_answer` 主链已打通，真实入口 `5/5` 通过 | `tmp/knowledge-answer-evals/latest.json`、`verify.md`、`review.md` | 当前只稳定了 agent 工程问答型问题，不等于更广知识问答整体优化 | 进入归档准备，不回 AQ 扩问答质量或检索引擎 |

## AO 裁决要点

### 已完成

1. 写回不再只是“按来源和 kind 写入”，而是已经具备：
   - `working_only`
   - `episodic_memory`
   - `semantic_or_procedural_memory`
2. 第一批治理字段已进入持久化与事件透出：
   - `memory_write_layer`
   - `memory_write_decision`
   - `memory_write_reason`
   - `memory_duplicate_strategy`
3. 已有最小拒绝留痕与 accepted 留痕。
4. 全量 `runtime-core` 回归已通过。

### 不要误写成

1. “记忆治理已经全面完成”
2. “recall 重排或知识层也已一起签收”
3. “超长函数护栏问题已经一并解决”

### 最合理裁决口径

- **AO 已完成最小写回治理闭环，并已按既定范围签收；后续若要深化复用价值评分、验证等级或工程护栏，应拆下一把独立 change。**

## AQ 裁决要点

### 已完成

1. 知识问答不再停在“本地知识检索结果”列表态。
2. `KA-01 ~ KA-04` 已提升为：
   - `verification_task_type = knowledge_answer`
   - `verification_has_citation = true`
   - 回答含 `事实 / 推断 / 建议`
3. `KA-05` 仍保留低证据 `replan_requested = true` 路径。
4. 真实入口回归 `5/5` 通过。

### 不要误写成

1. “全部知识问答质量已经完成优化”
2. “检索引擎或 verify 矩阵已经一起扩完”
3. “低证据问题已经不再需要保守收口”

### 最合理裁决口径

- **AQ 已完成知识问答主链最小收口，并已按既定范围签收；后续若要扩大到更广问答类型，应拆下一把独立 change。**

## 两把 change 的共同边界

1. 都已经有：
   - `proposal.md`
   - `design.md`
   - `tasks.md`
   - `status.md`
   - `verify.md`
   - `review.md`
2. 都已经完成实现与验证，不再需要回原 change 继续混做。
3. 两把的 warning 都属于“边界提醒”，不是当前签收阻塞。
4. 当前最不应该做的动作是：
   - 把 AO 扩成 recall 或知识治理
   - 把 AQ 扩成大范围问答质量优化

## 当前推荐交接口径

### 对主控侧

建议使用：

- **当前无活跃 change。AO 与 AQ 均已完成最小闭环、证据齐全、范围冻结明确，且已签收；建议进入归档准备，而不是回原 change 扩 scope。**

### 对后续执行侧

建议使用：

- **除非主控明确要求补归档材料，否则当前不要继续修改 AO / AQ 代码；如需继续开发，应新开独立 change。**

## 当前最合理的下一步

建议优先顺序：

1. 主控侧基于：
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AO-memory-writeback-governance-20260506/review.md`
   - `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/AQ-knowledge-answer-closure-20260506/review.md`
   - 本文件
   做归档顺序确认。
2. 若继续沿当前口径推进：
   - 把 AO / AQ 迁入归档目录并补归档索引。
3. 若不立刻归档而要继续开发：
   - 明确切下一把独立 change，不回 AO / AQ 扩散范围。

## 当前交接判断

到当前时点，这个待裁决队列已经具备：

1. 稳定的一致状态口径
2. 两把 change 各自完整的提审材料
3. 足够做并排判断的关键证据
4. 清晰的“不要误写成什么”边界说明

因此当前最稳妥的交接判断是：

- **AO / AQ 现在适合进入“并排归档准备 + 不再扩实现”的状态，而不是重新回到代码推进。**
