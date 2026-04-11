# 任务清单

- [x] 梳理现有 checkpoint/resume 骨架
  完成判据：明确已具备持久化、恢复事件和返回字段，但尚未恢复执行主线。
- [x] 明确受影响的 runtime 模块
  完成判据：`checkpoint.rs`、`query_engine.rs`、`session.rs`、`events.rs`、`contracts.rs` 均已纳入影响范围。
- [x] 明确恢复边界
  完成判据：本轮只覆盖 `after_confirmation` 与 `retryable_failure` 两类恢复路径。
- [x] 冻结 checkpoint 最小字段集合
  完成判据：字段清单可支撑恢复，不依赖未定义隐式上下文，也不提前扩表。
- [x] 设计恢复入口与阶段切换
  完成判据：能说明 checkpoint 读取后如何回到统一主循环，而不是只插入恢复事件。
- [x] 明确 `resume_matches` 的收口策略
  完成判据：说明是否继续绑定 `run_id`，并给出理由。
- [x] 设计验证样本
  完成判据：至少覆盖审批恢复与可重试失败恢复两类路径，并明确各自证据。
- [x] 实现恢复请求对短期状态的最小接入
  完成判据：`bootstrap_run` 能读取匹配 checkpoint，并把恢复原因写回 session 短期状态。
- [x] 对齐 retry 请求的 `run_id` 约束
  完成判据：`gateway` 构建 retry request 时保留原 `run_id`，并通过测试证明否则会与 runtime 恢复约束冲突。
- [x] 补齐 retry 接口级联调脚本
  完成判据：可在本地启动 gateway/runtime 后跑通 `chat/run -> chat/retry -> logs` 最小闭环。
- [x] 补齐实现与测试任务拆分
  完成判据：能按模块列出最小实现顺序和对应测试入口。
- [x] 补齐阶段性状态记录
  完成判据：`status.md` 与当前推进状态一致。
- [x] 建立 Gate-B 映射
  完成判据：`verify.md` 中写明与 Gate-B 的对应关系。
- [x] 补齐已选动作恢复入口
  完成判据：checkpoint 命中后可优先复用最近一条 `tool_call_snapshot` 反解出的动作，而不是只能重新规划。
- [x] 补齐验证快照摘要恢复入口
  完成判据：checkpoint 命中后可优先回填 `verification_snapshot.summary` 与 `artifact_path` 到短期状态，形成验证前快照摘要。
- [x] 补齐执行中间态摘要恢复入口
  完成判据：checkpoint 命中后可从最近执行事件回填“阶段/事件/下一步”边界到恢复计划，形成最小执行中间态摘要。
- [x] 补齐恢复事件验证快照元数据
  完成判据：`checkpoint_resumed` 可写出恢复时可见的 `verification_code/verification_summary/artifact_path`，并可被 acceptance 结构化断言消费。
- [x] 收敛恢复边界事件类型精确匹配断言
  完成判据：confirmation acceptance 断言 `checkpoint_resume_event_type=confirmation_required`，retry acceptance 断言 `checkpoint_resume_event_type=run_failed`，两条样本均为 `event_type_matched=true`。
- [x] 补齐目标 resumed 事件唯一性断言
  完成判据：confirmation 与 retry acceptance 均输出 `target_resumed_unique=true` 且 `target_resumed_count=1`，避免多次 `checkpoint_resumed` 串扰误判。
