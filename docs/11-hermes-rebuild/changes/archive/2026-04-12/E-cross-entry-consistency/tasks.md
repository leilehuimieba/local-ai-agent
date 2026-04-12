# 任务清单

- [x] 扩展 `chat/run` 可选身份字段
  完成判据：`request_id/run_id/trace_id` 可选输入，默认行为保持兼容。
- [x] 补齐身份字段选择逻辑单测
  完成判据：输入值复用与默认生成路径均有测试覆盖。
- [x] 交付 E-04 一致性脚本
  完成判据：脚本可自动拉起 runtime/gateway 并输出对比报告。
- [x] 产出 E-04 样本证据
  完成判据：`tmp/stage-e-consistency/latest.json` 状态为 `passed`。
- [x] 后端回归验证
  完成判据：`go test ./...`（`gateway/`）通过。
