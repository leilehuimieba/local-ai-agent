# 技术方案

## Harness 判断

当前项目已经完成了 `工具协议`、`权限与确认`、`artifact 外置`、`事件合同` 的主骨架，说明“模型外壳”不是空白状态。

当前最明显的薄弱层有 5 个：

1. `主循环`
   - 当前更像“分析后执行一个动作并收口”，而不是“受控多步回路”。
2. `上下文装配`
   - 已有压缩摘要、知识摘要、工具预览，但还缺少任务类型化装配 profile。
3. `记忆与知识路由`
   - 已能写入和召回，但 recall 更偏命中，不够像“决策所需最小包”。
4. `验证闭环`
   - 已有 verify，但还偏统一规则，缺少按任务类型独立验收。
5. `知识回答主链`
   - 已能搜到资料，但还没有稳定的“知识 pack -> 推理回答 -> 引证输出”主路径。

本 change 的目标不是重写 Runtime，而是在现有骨架上继续收紧这 5 层。

## 当前现状判断

### 已有优势

1. 主循环可见
   - `crates/runtime-core/src/lib.rs`
   - `crates/runtime-core/src/query_engine.rs`
2. resume / checkpoint 已成立
   - `crates/runtime-core/src/run_resume*.rs`
   - `crates/runtime-core/src/checkpoint/`
3. 短期记忆已结构化
   - `crates/runtime-core/src/session.rs`
4. 上下文装配已具备 profile 雏形
   - `crates/runtime-core/src/context_builder.rs`
   - `crates/runtime-core/src/context_policy.rs`
5. 本地知识与外部增强 recall 已有主链
   - `crates/runtime-core/src/knowledge.rs`
   - `crates/runtime-core/src/knowledge_store.rs`
   - `gateway/internal/knowledge/store.go`
6. verify / risk / confirmation 已进入主线
   - `crates/runtime-core/src/verify.rs`
   - `crates/runtime-core/src/run_risk_flow.rs`
   - `gateway/internal/api/chat_confirmation_service.go`

### 当前问题

1. `planner.rs` 仍以“选一个 PlannedAction”为主，缺少“多步计划 + 预算 + 停止条件”。
2. `context_builder.rs` 已能注入 session / memory / knowledge，但 ask、act、repair、learn 这四类任务还没有清楚分档。
3. `memory_recall.rs` 的摘要已分层，但长期记忆和知识路由还没有形成“按问题类型重排”的策略。
4. `gateway/internal/api/learning_memory.go` 的长期记忆召回更偏重复识别，还不是“真正可复用知识召回”。
5. `gateway/internal/knowledge/store.go` 的默认搜索仍是 `LIKE`，难支撑更复杂的知识问答。
6. `verify.rs` 已有 policy，但还缺少“文件修改 / 命令执行 / 知识回答 / 记忆写入 / 浏览器交互”这类任务矩阵。

## 实施优先级

按下面顺序推进，避免 scope 蔓延：

1. `主循环升级`
2. `上下文装配 profile`
3. `记忆模型与路由升级`
4. `知识检索与回答 pack`
5. `验证矩阵`

理由：

1. 没有多步主循环，后面的记忆和知识只会停留在“查得到但不会用”。
2. 没有上下文 profile，记忆和知识会退化成堆料。
3. 没有知识 pack，项目无法稳定回答“agent 工程问题”。
4. 没有验证矩阵，复杂任务还是只能依赖执行者自评。

## 方案一：把单动作收口升级为受控多步回路

### 目标

让 Runtime 能稳定处理 2~4 步的复杂任务，而不是每轮只执行一个动作就结束。

### 建议落点

- `crates/runtime-core/src/planner.rs`
- `crates/runtime-core/src/query_engine.rs`
- `crates/runtime-core/src/lib.rs`
- `crates/runtime-core/src/run_state_builder.rs`
- `crates/runtime-core/src/events.rs`

### 具体改造

1. 在 planner 层引入最小 `PlanEnvelope`
   - 建议字段：
     - `goal`
     - `current_step`
     - `remaining_steps`
     - `stop_condition`
     - `max_iterations`
     - `iteration_index`
     - `needs_verification`
2. `PlannedAction` 继续保留，但不再直接等同于整轮计划。
3. `query_engine.rs` 增加 replan trigger：
   - 工具结果成功但信息不足；
   - verify 未通过；
   - 命中 confirmation 恢复后需要继续；
   - 读取结果显示需要追加一个观察动作。
4. 在 `lib.rs` 中把单次 `execute_stage()` 收口扩展成“小步循环”
   - 每轮最多 2~4 次动作；
   - 每轮都保留 stage metadata；
   - 超出预算时进入 handoff，而不是继续硬跑。
5. 事件新增建议：
   - `plan_iteration_started`
   - `plan_iteration_completed`
   - `replan_requested`
   - `iteration_budget_exhausted`

### 成功标准

系统能稳定完成下面这类最小复杂任务：

1. 读取项目入口文档；
2. 读取目标代码文件；
3. 形成修改建议或执行 dry-run；
4. verify 后决定 finish / handoff / confirmation。

### 不要做的事

1. 不在这一刀引入多智能体。
2. 不做无限循环自动修复。
3. 不做复杂 DAG planner。

## 方案二：把上下文装配从“有注入”升级为“按任务类型装配”

### 目标

让模型拿到的是“下一步决策所需最小包”，而不是默认把 session、memory、knowledge 一起塞满。

### 建议落点

- `crates/runtime-core/src/context_policy.rs`
- `crates/runtime-core/src/context_builder.rs`
- `gateway/internal/api/chat_context_resolver.go`

### 具体改造

定义 4 个默认 profile：

1. `ask_profile`
   - 适用于解释、比较、方案、复盘；
   - 优先：session digest、knowledge digest、项目文档摘要。
2. `act_profile`
   - 适用于执行与修改；
   - 优先：目标文件 preview、风险边界、最近工具结果、artifact hint。
3. `repair_profile`
   - 适用于失败恢复；
   - 优先：last failure、checkpoint summary、恢复原因、受影响路径。
4. `learn_profile`
   - 适用于学习沉淀与知识写回；
   - 优先：knowledge digest、related memory digest、reusable principle。

### 具体要求

1. `context_policy.rs` 需要能够根据：
   - 当前 mode；
   - 当前 action 类型；
   - 是否从 checkpoint 恢复；
   - 是否为知识类问题；
   自动选出 profile。
2. `context_builder.rs` 需要输出 profile 名称和选择理由，继续挂到事件 metadata。
3. `chat_context_resolver.go` 只保留 hints 注入，不把过多策略写回 Go 层，策略主判断应留在 Runtime。

### 成功标准

1. 知识问答不再默认带大段工具 preview。
2. 执行类任务不再默认带大段知识摘要。
3. 失败恢复时能优先看到 checkpoint / last failure，而不是一堆历史摘要。

## 方案三：把记忆升级为 Working / Episodic / Semantic 三层

### 目标

把“能写入长期记忆”升级成“知道什么该写、写去哪一层、怎么被召回”。

### 建议落点

- `crates/runtime-core/src/session.rs`
- `crates/runtime-core/src/memory.rs`
- `crates/runtime-core/src/memory_recall.rs`
- `crates/runtime-core/src/memory_router/mod.rs`
- `crates/runtime-core/src/memory_schema.rs`
- `gateway/internal/api/learning_memory.go`

### 分层建议

1. `Working Memory`
   - 当前 `session.short_term` 继续承担；
   - 生命周期以当前 session / 当前 run 为主；
   - 不参与长期复用。
2. `Episodic Memory`
   - 记录一次任务的过程性经验；
   - 例如：
     - 问题是什么；
     - 做了哪些动作；
     - 哪一步失败；
     - 如何恢复。
3. `Semantic / Procedural Memory`
   - 记录可复用知识与工作流；
   - 例如：
     - 某类错误的修复套路；
     - 某类 agent 问题的推荐架构；
     - 某工具的安全使用模式。

### 写回规则

只有满足下面条件才允许进入长期层：

1. 已验证通过；
2. 摘要长度和信息密度达标；
3. 不是当前项目状态回显；
4. 未来跨任务复用价值明确。

### recall 规则

1. recall 先分“问题类型”：
   - 定义型；
   - 关系型；
   - 工作流型；
   - 风险型；
   - 修复型。
2. 每类问题优先召回不同材料：
   - 定义型优先 semantic；
   - 修复型优先 episodic；
   - 工作流型优先 procedural。
3. `memory_recall.rs` 输出除了 summary 之外，建议补：
   - `match_reason`
   - `memory_layer`
   - `reuse_confidence`

### 对 learning memory 的要求

`gateway/internal/api/learning_memory.go` 当前 recall 更偏同文命中，应升级为：

1. 标题相似；
2. tags overlap；
3. reusable principle overlap；
4. 问题意图 overlap。

### 成功标准

1. 追问“这个项目为什么这样设计”时，优先召回 semantic / procedural；
2. 追问“上次为什么失败”时，优先召回 episodic；
3. 长期记忆增长后，不会退化成全量注入。

## 方案四：把知识检索升级为能支撑回答的混合检索

### 目标

让知识库检索不只是“命中一些条目”，而是能稳定支撑 agent 工程问题回答。

### 建议落点

- `gateway/internal/knowledge/store.go`
- `crates/runtime-core/src/knowledge.rs`
- `crates/runtime-core/src/knowledge_store.rs`
- `crates/runtime-core/src/context_builder.rs`

### 具体改造

1. `gateway/internal/knowledge/store.go`
   - 保留 `knowledge_items` / `knowledge_chunks`；
   - 搜索从 `LIKE` 升级为：
     - title / category / tags 精确命中；
     - FTS 或 chunk 命中；
     - citation count / updated_at 辅助重排。
2. `knowledge.rs`
   - 输出不仅要有 `path / snippet`；
   - 还要补：
     - `match_reason`
     - `citation_ready`
     - `use_for`
       - `definition`
       - `relation`
       - `workflow`
       - `risk`
       - `evidence`
3. 增加 `knowledge pack` 概念
   - 供 Runtime 回答时使用；
   - 结构建议：
     - `question_type`
     - `top_hits`
     - `supporting_hits`
     - `citations`
     - `answer_hints`
4. `context_builder.rs`
   - 对知识问答 profile，优先注入 `knowledge pack`，不要默认注入全文。

### 直接适配目标

让系统稳定回答下面这类问题：

1. `agent 是什么`
2. `为什么 agent 需要 memory 和 context engineering`
3. `如何评估一个 agent 是否可靠`
4. `这个项目的 agent loop 为什么不该直接上多智能体`

### 成功标准

1. 回答里能同时给结论和引证来源；
2. 引证来源来自本地知识库，而不是只靠模型硬编；
3. 同一问题重复问，回答结构波动明显下降。

## 方案五：把 verify 升级为任务类型矩阵

### 目标

从“工具结果摘要看起来合理”升级为“按任务类型独立验收”。

### 建议落点

- `crates/runtime-core/src/verify.rs`
- `crates/runtime-core/src/run_verification_metadata.rs`
- 必要时新增 `verify_*` 子模块

### 任务类型矩阵

1. 文件修改类
   - patch dry-run；
   - 目标文件存在性；
   - 变更摘要；
   - 可选 lint / test / build。
2. 命令执行类
   - exit code；
   - stderr 摘要；
   - 产物是否生成；
   - 是否命中预期路径。
3. 知识回答类
   - 至少 2 条相关命中；
   - 是否带 citation；
   - 是否区分事实 / 推断 / 建议。
4. 记忆写入类
   - write 成功；
   - readback 成功；
   - scope 正确；
   - 去重或覆盖规则可见。
5. 浏览器交互类
   - before / after 状态变化；
   - 目标元素是否出现；
   - 是否保留 confirmation 与 risk trace。

### 额外建议

verify metadata 新增两类字段：

1. `capability_risk_checked`
2. `permission_boundary_respected`

这能把“任务是否完成”和“风险是否被约束住”同时纳入闭环。

## 建议实施顺序

### 第一刀：主循环最小升级

1. 引入 `PlanEnvelope`
2. 支持最多 2~4 步回路
3. 增加 replan 事件

### 第二刀：上下文 profile

1. ask / act / repair / learn
2. 让不同 profile 看到不同最小包

### 第三刀：记忆分层与 recall 重排

1. episodic / semantic / procedural
2. 解决“能存不能用”的问题

### 第四刀：知识 pack

1. 支撑 agent 工程类问答
2. 为后续“知识驱动 agent 回答”打底

### 第五刀：verify 矩阵

1. 把复杂任务真正收口成工程闭环

## 明确后置的高级能力

下面这些能力不要和本 change 混做：

1. 多智能体 orchestration
2. 自动长期自我反思
3. 大规模向量数据库替换
4. 复杂浏览器录制回放
5. 全新的 provider / GUI 扩张

理由很简单：当前最值钱的是把“单 agent 主链”打厚，而不是继续扩能力面。
