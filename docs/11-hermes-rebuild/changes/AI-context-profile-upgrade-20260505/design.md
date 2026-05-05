# 技术方案

## 影响范围

- 涉及模块：
  - `crates/runtime-core/src/context_policy.rs`
  - `crates/runtime-core/src/context_builder.rs`
  - `crates/runtime-core/src/run_context_metadata.rs`
  - `crates/runtime-core/src/events.rs`
  - `gateway/internal/api/chat_context_resolver.go`
- 涉及文档或 contract：
  - `docs/11-hermes-rebuild/changes/AG-agent-loop-memory-knowledge-20260505/design.md`
  - Runtime context metadata 与 Gateway hints 注入口径

## 方案

### 范围冻结

- 本刀只做四件事：
  - 定义 `ask / act / repair / learn` 四种默认上下文 profile
  - 让 `context_policy.rs` 能按当前模式、动作类型、恢复态、问题类型自动选 profile
  - 让 `context_builder.rs` 按不同 profile 装配不同最小包
  - 让 profile 名称、选择理由和注入差异进入事件 metadata / context snapshot

### 四类 profile

- `ask_profile`
  - 适用于解释、比较、方案、复盘
  - 优先注入：`session digest + knowledge digest + 项目文档摘要`
- `act_profile`
  - 适用于执行、修改、工具动作推进
  - 优先注入：`目标文件预览 + 风险边界 + 最近工具结果 + artifact hint`
- `repair_profile`
  - 适用于失败恢复、checkpoint 继续、受控补救
  - 优先注入：`last failure + checkpoint summary + 恢复原因 + 受影响路径`
- `learn_profile`
  - 适用于知识沉淀、学习型追问、写回前整理
  - 优先注入：`knowledge digest + related memory digest + reusable principle`

### Profile 选择逻辑

- `context_policy.rs` 至少应综合下面信号：
  - 当前 `mode`
  - 当前 `PlannedAction` 类型
  - 是否从 checkpoint / confirmation 恢复
  - 是否为知识类 / 解释类问题
  - 是否处于失败恢复上下文
- 第一刀主循环已经引入 iteration metadata，本刀需要保证 profile 选择与 iteration 兼容，但不新增新的计划层状态机。

### ContextBuilder 改造

- `context_builder.rs` 需要把“是否注入 session / memory / knowledge / tool preview / artifact hint”的策略显式挂到 profile 下，而不是只靠零散布尔值拼装。
- 每个 profile 需要输出：
  - `assembly_profile`
  - `selection_reason`
  - `phase_label`
  - 注入项差异
- 本刀不要求引入新的知识摘要结构或记忆层结构，只调整“什么时候看、看多少、优先看什么”。

### Gateway 边界

- `gateway/internal/api/chat_context_resolver.go` 只保留 hints 透传和最小注入位，不承担 profile 选择主逻辑。
- profile 主判断仍应收敛在 Runtime，避免策略分叉到 Go 层。

## 状态流转或调用链变化

- 预期变化不是新增 lifecycle stage，而是让同一条运行链在不同阶段拿到不同最小包：
  - `ask_profile` 偏回答与解释
  - `act_profile` 偏执行与推进
  - `repair_profile` 偏恢复与补救
  - `learn_profile` 偏学习和沉淀
- 第一刀的 `PlanEnvelope` 与 iteration metadata 保持不变，本刀只让每次 planning / execute / recovery 看到更合适的上下文。

## 风险与回退

- 主要风险：
  - profile 选择条件如果过宽，可能导致执行类任务误走回答 profile，或恢复态误丢 checkpoint 信息。
  - `context_builder.rs` 当前已有多种 digest 注入逻辑，若 profile 收敛不当，容易出现注入差异与 metadata 表述不一致。
  - Gateway 与 Runtime 若同时各自判断 profile，会造成策略漂移。
- 回退方式：
  - 保留现有通用装配路径作为 fallback，当 profile 选择失败时退回当前稳定装配。
  - 先冻结 profile 选择测试和 metadata 证据，再决定是否删旧逻辑。
