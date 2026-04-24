# Tasks

- [x] T1 收紧 Stage-E Entry1 默认通过条件。
  - 完成判据：无 `run_started` 或无 `terminal_completed` 时脚本默认失败。
- [x] T2 保留 accepted fallback 显式开关。
  - 完成判据：传入 `-AllowAcceptedFallback` 时可按 accepted / protocol 口径通过，并在报告标明降级模式。
- [x] T3 增加 doctor 非零退出码模式。
  - 完成判据：传入 `-FailOnError` 且 JSON `status=failed` 时进程返回非零。
- [x] T4 定位 runtime 严格闭环不产生日志事件的根因。
  - 完成判据：`has_run_started=true` 且 `terminal_completed=true`。
  - 验证结果：2026-04-24 复现通过，`strict_runtime_terminal_ok=true`，`terminal_completed=true`。
