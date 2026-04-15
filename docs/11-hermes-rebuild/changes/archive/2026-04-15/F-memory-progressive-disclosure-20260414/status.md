# 当前状态

- 最近更新时间：2026-04-14
- 状态：已收口（M0~M5 全量完成）
- 状态口径：当前阶段 / 当前 Gate / 当前活跃 change 统一见 `docs/11-hermes-rebuild/current-state.md`（本文件仅记录该 change 的专项进展）
- 已完成：
  1. 建立 `F-memory-progressive-disclosure-20260414` 专项工作区并补齐五件套。
  2. 冻结 `M0 -> M5` 里程碑、Gate、风险回退口径。
  3. 产出 `MEM-01 ~ MEM-25` AI 可执行任务分解与证据落点。
  4. `M0` 任务已完成：`MEM-01`、`MEM-02`、`MEM-03`、`MEM-04`。
  5. `MEM-05` 已完成：生命周期事件到 observation 映射最小链路已落地，并输出 `tmp/stage-mem-m1/events-mapping.json`。
  6. `MEM-06` 已完成：observation 已打通 SQLite 主写与 JSONL 审计旁路，并输出 `tmp/stage-mem-m1/storage.json`。
  7. `MEM-07` 已完成：基于 content_hash + time_window 的 observation 去重策略已落地，并输出 `tmp/stage-mem-m1/dedupe.json`。
  8. `MEM-08` 已完成：写入失败降级链路已落地，并输出 `tmp/stage-mem-m1/fallback.json`。
  9. `MEM-09` 已完成：pending 队列表状态流转闭环已落地，并输出 `tmp/stage-mem-m2/queue-flow.json`。
  10. `MEM-10` 已完成：failed 队列重试与退避策略已落地，并输出 `tmp/stage-mem-m2/retry.json`。
  11. `MEM-11` 已完成：重启恢复演练通过，并输出 `tmp/stage-mem-m2/recovery.json`。
  12. `MEM-12` 已完成：队列健康状态可查询，并输出 `tmp/stage-mem-m2/health.json`。
  13. 已补并行执行边界：本专项限定 runtime memory 路径，不改安装脚本主线。
  14. `MEM-13` 已完成：`search` 轻量检索接口已落地，并输出 `tmp/stage-mem-m3/search.json`。
  15. `MEM-14` 已完成：`timeline` 时序片段接口已落地，并输出 `tmp/stage-mem-m3/timeline.json`。
  16. `MEM-15` 已完成：`get_observations` 批量详情接口已落地，并输出 `tmp/stage-mem-m3/get-observations.json`。
  17. `MEM-16` 已完成：检索排序融合已落地（来源权重 + 新鲜度 + 关键词分），并输出 `tmp/stage-mem-m3/rank.json`。
  18. `MEM-17` 已完成：固定评测集回归通过（Top-5 命中率 100%），并输出 `tmp/stage-mem-eval/latest.json`。
  19. `MEM-18` 已完成：ContextBuilder 分层注入接入已落地，并输出 `tmp/stage-mem-m4/injection.json`。
  20. `MEM-19` 已完成：预算裁剪已落地，并输出 `tmp/stage-mem-m4/budget.json`。
  21. `MEM-20` 已完成：引用化输出已落地，并输出 `tmp/stage-mem-m4/reference.json`。
  22. `MEM-21` 已完成：全量 vs 分层对比验证通过（节省率 58.37%），并输出 `tmp/stage-mem-m4/ab-test.json`。
  23. `MEM-22` 已完成：敏感字段脱敏已落地，并输出 `tmp/stage-mem-m5/privacy-redact.json`。
  24. `MEM-23` 已完成：private 片段排除已落地，并输出 `tmp/stage-mem-m5/private-skip.json`。
  25. `MEM-24` 已完成：feature flag 回退演练通过，并输出 `tmp/stage-mem-m5/rollback.json`。
  26. `MEM-25` 已完成：提审文档收口完成（`verify.md` 已补全 M3~M5 验收）。
- 进行中：
  1. 无。
- 阻塞点：
  1. 无硬阻塞；按并行边界执行，避免写入锁文件（`current-state.md`、`changes/INDEX.md`）。
- 下一步：
  1. 等待是否切换主推进；本专项并行目标已完成，建议进入归档或转维护态。
