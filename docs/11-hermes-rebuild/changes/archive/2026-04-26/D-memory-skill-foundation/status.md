# 当前状态

- 最近更新时间：2026-04-12
- 状态：进行中
- 状态口径：当前阶段 / 当前 Gate / 当前活跃 change 统一见 `docs/11-hermes-rebuild/current-state.md`（本文件仅记录该 change 的任务进展）
- 已完成：建立阶段 D 首条主推进 change，补齐五件套并纳入索引。
- 已完成：`D-01` 三层记忆 schema v1 冻结，已在 `design.md` 收口字段表、检索口径、迁移草案与回退策略。
- 已完成：`D-02` 迁移脚本最小实现与执行验证，脚本 `scripts/run-stage-d-migrate-acceptance.ps1` 已产出 `tmp/stage-d-migrate-acceptance/latest.json`（`status=passed`）。
- 已完成：`D-03` 关键词检索主路径验收，接口样本已验证 `memory_recalled` 命中与可解释字段（`reason/governance_reason/includes_memory`）。
- 已完成：`D-04` 写回治理规则固化，去重/优先级/归档规则已由 `memory.rs` 单测覆盖并通过（证据：`tmp/stage-d-writeback-governance/latest.txt`）。
- 已完成：`D-05` 技能加载与版本治理接入，已形成 manifest 加载、版本 pin 校验与 workspace 隔离校验最小闭环（证据：`tmp/stage-d-skill-catalog/latest.txt`）。
- 已完成：`D-06` 跨会话连续性首轮样本，已在隔离沙箱复现“会话 A 写记忆 -> 会话 B 召回命中”链路（证据：`tmp/stage-d-day1/latest.json`）。
- 已完成：`D-G1` 7 天 Gate-D 批量验收准备，批量脚本阈值映射已冻结并通过（证据：`tmp/stage-d-batch/latest.json`，`gate_d.ready=true`）。
- 进行中：无。
- 阻塞点：无。
- 下一步：等待 `current-state.md` 指定新的主推进 change；本条仅作历史记录。
