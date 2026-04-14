# 任务清单

- [x] T01 新增阶段 G 证据保鲜脚本入口
  完成判据：`run-stage-g-evidence-freshness.ps1` 创建并可输出 `tmp/stage-g-evidence-freshness/latest.json`。
- [x] T02 新增阶段 G G-01 聚合验收脚本
  完成判据：`run-stage-g-gate-acceptance.ps1` 创建并可输出 `tmp/stage-g-gate/latest.json`。
- [x] T03 阶段 G 交付文档落版
  完成判据：四份阶段 G 交付文档创建完成。
- [x] T04 首轮执行与证据回写
  完成判据：执行 `run-stage-g-gate-acceptance.ps1 -RefreshEvidence -RequirePass` 通过并回写 `status/verify`。
