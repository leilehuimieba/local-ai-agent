# 任务清单

- [x] T01 新增 G-02 告警治理脚本入口
  完成判据：`run-stage-g-warning-governance.ps1` 创建并支持 `-RequirePass`。
- [x] T02 产出 warning tracker 与治理报告
  完成判据：`tmp/stage-g-ops/warning-tracker.json` 与 `tmp/stage-g-ops/latest.json` 成功落盘。
- [x] T03 首轮执行验收
  完成判据：执行 `run-stage-g-warning-governance.ps1 -RefreshEvidence -RequirePass` 通过。
- [x] T04 文档回写与状态同步
  完成判据：本 change 五件套补齐，`INDEX/current-state` 切换到本 change。
