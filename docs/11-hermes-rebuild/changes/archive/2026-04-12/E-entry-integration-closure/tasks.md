# 任务清单

- [x] 补齐 E-05 失败样本脚本
  完成判据：`run-stage-e-entry-failure-acceptance.ps1` 可落盘 `status=passed` 报告。
- [x] 复核 E-02 与 E-04 样本
  完成判据：`tmp/stage-e-entry1/latest.json` 与 `tmp/stage-e-consistency/latest.json` 均为 `passed`。
- [x] 输出 E-05 入口联调报告
  完成判据：`review.md` 含三类证据映射、失败样本解读与回退路径。
- [x] 回写执行索引与总表状态
  完成判据：`INDEX.md`、`全路线最小任务分解总表.md` 状态与当前推进一致。
- [x] 后端回归验证
  完成判据：`go test ./...`（`gateway/`）通过。
