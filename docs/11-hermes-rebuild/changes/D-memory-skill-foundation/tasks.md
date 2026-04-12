# 任务清单

- [x] D-01 冻结三层记忆 schema v1
  完成判据：形成 schema 字段表与迁移策略，能进入实现。  
  证据：`design.md` 已补齐三层字段表、读写检索口径、迁移与回退草案。
- [x] D-02 记忆迁移脚本最小实现
  完成判据：本地数据可平滑迁移，迁移脚本可独立执行并产出证据。  
  证据：`scripts/run-stage-d-migrate-acceptance.ps1` + `tmp/stage-d-migrate-acceptance/latest.json`（`status=passed`，`sqlite_memory_after=4`，`sqlite_knowledge_after=1`，`legacy_duplicate_removed=true`）。
- [x] D-03 接入关键词检索主路径
  完成判据：至少 1 条检索链路可命中并可解释，且有验证样本。  
  证据：`tmp/stage-d-memory-recall-acceptance/latest.json`（`query=recall: 项目 文档 主入口`，`event_type=memory_recalled`，`includes_memory=true`，`digest` 含 `来源/类型/理由`）。
- [x] D-04 固化记忆写回治理规则
  完成判据：去重、优先级、归档规则形成可执行口径并通过单测。  
  证据：`crates/runtime-core/src/memory.rs` 新增 `memory_priority_bonus` 与 3 条治理单测；`tmp/stage-d-writeback-governance/latest.txt`（`memory::tests::*` 全部 `ok`，`67 passed`）。
- [x] D-05 技能加载与版本治理接入
  完成判据：技能版本可识别与隔离。  
  证据：`crates/runtime-core/src/skill_catalog.rs` 新增 manifest 加载、`skill_version_pins` 版本治理、`resolve_workspace_path` 隔离校验；`tmp/stage-d-skill-catalog/latest.txt`（`skill_catalog::tests::*` 全部 `ok`，`70 passed`）。
- [x] D-06 跨会话连续性样本首轮
  完成判据：1 天样本链路可复现。  
  证据：`scripts/run-stage-d-day1-acceptance.ps1` + `tmp/stage-d-day1/latest.json`（`status=passed`，`cross_session=true`，`write_has_memory_written=true`，`recall_has_memory_recalled=true`，`recall_hit=true`，`token_hits=2`）。
- [x] D-G1 批量验证准备
  完成判据：明确 Gate-D 批量验证脚本输入、输出与阈值映射。  
  证据：`scripts/run-stage-d-gate-batch.ps1 -Days 7 -RequireGateD` + `tmp/stage-d-batch/latest.json`（`status=passed`，`gate_d.ready=true`，`days=7`，`pass_days=7`，`pass_rate=1`，`recall_hit_rate=1`）。
