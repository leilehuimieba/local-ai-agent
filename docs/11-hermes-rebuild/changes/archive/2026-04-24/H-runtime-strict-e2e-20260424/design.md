# Design

## 影响范围

1. `scripts/run-stage-e-entry1-acceptance.ps1`
   - 默认要求 `protocol_fields_ok`、run/session 匹配、`run_started`、`terminal_completed` 同时成立。
   - 新增 `-AllowAcceptedFallback`，仅在人工明确要求时允许 accepted / protocol 降级通过。
   - 报告中增加 `acceptance_mode`、`strict_runtime_terminal_ok`、`accepted_fallback_ok`。

2. `scripts/doctor.ps1`
   - 保持默认只输出 JSON 的兼容行为。
   - 新增 `-FailOnError`，当检查失败时以非零退出码暴露失败状态。

## 回退方式

1. 如严格口径阻塞历史批量验收，可临时显式传入 `-AllowAcceptedFallback`。
2. 如调用方仍依赖 doctor 默认 exit code，可不传 `-FailOnError`。
