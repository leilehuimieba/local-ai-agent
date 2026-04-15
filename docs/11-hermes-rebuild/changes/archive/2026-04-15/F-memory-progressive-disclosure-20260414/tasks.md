# 任务清单

## 执行规则

1. 任务按 `M0 -> M1 -> M2 -> M3 -> M4 -> M5` 顺序推进，未过当前 Gate 不进入下一里程碑。
2. 同一时刻仅允许一个 in-progress 任务。
3. 每个任务必须具备“允许改动文件、完成判据、验证命令、证据路径、回退动作”。
4. 未在 `verify.md` 回写证据前，不得标记完成。

## M0 计划冻结（规划层）

- [x] MEM-01 新建并冻结专项五件套
  完成判据：`proposal/design/tasks/status/verify` 齐全，且里程碑与 Gate 口径一致。
  证据：本目录五件套文件可读且字段完整。
- [x] MEM-02 冻结 Observation Contract v1
  完成判据：字段、类型、必填、降级规则明确。
  证据：`design.md` 对应章节。
- [x] MEM-03 冻结 Retrieval Contract v1
  完成判据：`search/timeline/get_observations` 入参、出参、限制明确。
  证据：`design.md` 对应章节。
- [x] MEM-04 冻结 Context Injection Budget v1
  完成判据：预算配比、裁剪顺序、兜底策略明确。
  证据：`design.md` 对应章节。

## M1 采集与存储

- [x] MEM-05 生命周期事件映射实现
  完成判据：5 类目标事件均可产出 observation。
  证据：`tmp/stage-mem-m1/events-mapping.json`。
- [x] MEM-06 observation 持久化实现（SQLite + JSONL）
  完成判据：主写成功 + 旁路审计写入，且字段可回查。
  证据：`tmp/stage-mem-m1/storage.json`。
- [x] MEM-07 去重策略实现
  完成判据：重复样例仅保留一条有效 observation。
  证据：`tmp/stage-mem-m1/dedupe.json`。
- [x] MEM-08 写入失败降级
  完成判据：写入失败时 run 不中断，失败事件可追溯。
  证据：`tmp/stage-mem-m1/fallback.json`。

## M2 异步处理与恢复

- [x] MEM-09 pending 队列表与状态流转
  完成判据：`pending -> processing -> processed/failed` 闭环可复现。
  证据：`tmp/stage-mem-m2/queue-flow.json`。
- [x] MEM-10 重试策略实现
  完成判据：失败重试符合次数与退避策略。
  证据：`tmp/stage-mem-m2/retry.json`。
- [x] MEM-11 重启恢复实现
  完成判据：重启后未完成任务可恢复处理。
  证据：`tmp/stage-mem-m2/recovery.json`。
- [x] MEM-12 健康与状态接口
  完成判据：可查询队列深度、失败数、处理状态。
  证据：`tmp/stage-mem-m2/health.json`。

## M3 检索与三层披露

- [x] MEM-13 `search` 接口实现
  完成判据：返回轻量索引（不含大文本正文）。
  证据：`tmp/stage-mem-m3/search.json`。
- [x] MEM-14 `timeline` 接口实现
  完成判据：可按 anchor/query 返回时序上下文片段。
  证据：`tmp/stage-mem-m3/timeline.json`。
- [x] MEM-15 `get_observations` 接口实现
  完成判据：支持按 ID 批量拉取详情。
  证据：`tmp/stage-mem-m3/get-observations.json`。
- [x] MEM-16 检索排序融合实现
  完成判据：来源权重 + 新鲜度 + 关键词分生效。
  证据：`tmp/stage-mem-m3/rank.json`。
- [x] MEM-17 固定评测集回归
  完成判据：Top-5 命中率 >= 70%。
  证据：`tmp/stage-mem-eval/latest.json`。

## M4 上下文注入与预算治理

- [x] MEM-18 ContextBuilder 分层注入接入
  完成判据：runtime 可消费 summary/timeline/details 三层输入。
  证据：`tmp/stage-mem-m4/injection.json`。
- [x] MEM-19 预算裁剪实现
  完成判据：超预算稳定裁剪，不破坏上下文结构。
  证据：`tmp/stage-mem-m4/budget.json`。
- [x] MEM-20 引用化输出实现
  完成判据：注入内容带 `observation_id/artifact_ref`。
  证据：`tmp/stage-mem-m4/reference.json`。
- [x] MEM-21 全量 vs 分层对比验证
  完成判据：token 节省 >= 50%，质量不低于基线。
  证据：`tmp/stage-mem-m4/ab-test.json`。

## M5 隐私治理与提审收口

- [x] MEM-22 敏感字段脱敏规则接入
  完成判据：密钥/令牌/敏感字段不入库。
  证据：`tmp/stage-mem-m5/privacy-redact.json`。
- [x] MEM-23 private 片段排除接入
  完成判据：private 内容仅保留审计摘要，不落正文。
  证据：`tmp/stage-mem-m5/private-skip.json`。
- [x] MEM-24 feature flag 回退演练
  完成判据：关闭增强链后回到既有主路径，链路连续可用。
  证据：`tmp/stage-mem-m5/rollback.json`。
- [x] MEM-25 提审包收口
  完成判据：`verify.md/review.md/risk-rollback-register.md` 完整可审计。
  证据：本 change 文档与脚本产物齐全。
