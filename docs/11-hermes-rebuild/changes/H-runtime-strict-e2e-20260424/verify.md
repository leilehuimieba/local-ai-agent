# Verify

## 验证命令

```powershell
powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-entry1-acceptance.ps1
```

预期：默认严格口径通过，`strict_runtime_terminal_ok=true`，且 terminal event 的 `completion_status=completed`。

```powershell
powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-entry1-acceptance.ps1 -AllowAcceptedFallback
```

预期：仅在需要兼容 accepted / protocol 降级口径时使用，并在报告中标记 `acceptance_mode=accepted_fallback_allowed`。

```powershell
powershell -ExecutionPolicy Bypass -File scripts/doctor.ps1 -OutFile tmp/e2e-test/doctor-failonerror.json -FailOnError
```

预期：当 runtime health 不可用时返回非零。

## 当前证据

1. 严格默认模式：
   - 命令：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-entry1-acceptance.ps1`
   - 结果：通过，exit code = 0。
   - 证据：`tmp/stage-e-entry1/latest.json`、`tmp/e2e-test/strict-stage-e-passed-20260424-rerun.json`
   - 当前报告：`status=passed`，`acceptance_mode=strict_runtime_terminal`，`strict_runtime_terminal_ok=true`，`terminal_completed=true`。
   - 事件链：`run_started -> runtime_returned -> run_started -> analysis_ready -> plan_ready -> memory_recalled -> action_requested -> action_completed -> verification_completed -> memory_written -> memory_written -> knowledge_write_skipped -> checkpoint_written -> run_finished`。
   - 最新复现（2026-04-24）：`status=passed`，`strict_runtime_terminal_ok=true`，`has_run_started=true`，`terminal_completed=true`，`completion_status=completed`。

2. 回归测试：
   - `cargo test --workspace`：通过，exit code = 0。
   - 证据：`tmp/e2e-test/strict-cargo-test-20260424-rerun.log`。
   - `cd gateway; go test ./...`：通过，exit code = 0。
   - 证据：`tmp/e2e-test/strict-go-test-20260424-rerun.log`。

## 根因与修复证据

1. runtime-host 侧：
   - 断点表现：Gateway / Go HTTP client 调 `/v1/runtime/run` 时连接关闭或超时，Gateway 只能看到自身 `run_started`。
   - 修复：请求处理切到 worker thread，并设置较大栈空间，避免 runtime-core 执行链在主线程栈不足时中断。

2. runtime-core storage migration 侧：
   - 断点表现：`memory_recall -> list_current_memory_object_entries_limited_sqlite -> ensure_workspace_imported` 期间递归重入 SQLite connection。
   - 修复：memory object backfill 不再调用 `sync_memory_object_entry_sqlite(request, entry)` 新开连接，而是复用当前 `conn` 调用 `upsert_memory_object_version(conn, entry)`。

3. 验收脚本侧：
   - 严格口径保留为默认。
   - timeout 时保留最后一次日志快照。
   - 等待窗口延长到 90 次轮询，覆盖 runtime 完整事件发布链。


## 继续端到端测试（2026-04-24 17:05）

1. 严格默认模式复跑：
   - 命令：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-entry1-acceptance.ps1`
   - 结果：通过，exit code = 0。
   - 证据：`tmp/e2e-test/strict-stage-e-continue-final-20260424.json`。
   - 当前报告：`status=passed`，`acceptance_mode=strict_runtime_terminal`，`strict_runtime_terminal_ok=true`，`terminal_completed=true`。
   - 事件链：`run_started -> runtime_returned -> run_started -> analysis_ready -> plan_ready -> memory_recalled -> action_requested -> action_completed -> verification_completed -> memory_written -> memory_write_skipped -> knowledge_write_skipped -> checkpoint_written -> run_finished`。

2. 显式 fallback 模式复跑：
   - 命令：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-entry1-acceptance.ps1 -AllowAcceptedFallback`
   - 结果：通过，exit code = 0。
   - 证据：`tmp/e2e-test/strict-stage-e-fallback-continue-20260424.json`。
   - 当前报告：`acceptance_mode=accepted_fallback_allowed`，同时严格字段仍为 `strict_runtime_terminal_ok=true`。

3. doctor fail-on-error：
   - 命令：`powershell -ExecutionPolicy Bypass -File scripts/doctor.ps1 -OutFile tmp/e2e-test/doctor-continue-20260424.json -FailOnError`
   - 结果：失败，exit code = 1。
   - 证据：`tmp/e2e-test/doctor-continue-20260424.json`。
   - 当前报告：`status=failed`，`runtime_health_ok=false`，符合未启动默认 runtime port 时 `-FailOnError` 应非零退出的预期。

4. Rust / Gateway / Frontend 回归：
   - `cargo test --workspace`：通过，exit code = 0，证据 `tmp/e2e-test/cargo-test-continue-20260424.log`。
   - `cd gateway; go test ./...`：通过，exit code = 0，证据 `tmp/e2e-test/go-test-continue-20260424.log`。
   - `cd frontend; npm test`：通过，exit code = 0，证据 `tmp/e2e-test/frontend-test-continue-20260424.log`。
   - `cd frontend; npm run build`：通过，exit code = 0，证据 `tmp/e2e-test/frontend-build-continue-20260424.log`。

5. 继续测试中追加修复：
   - `backfill_memory_objects_if_needed` 复用当前 SQLite connection，避免递归重入。
   - `upsert_memory_object_version` 对已有 entry/version 直接返回现有版本，避免 backfill 重复 upsert 覆盖 rollback restored metadata 或刷新 current 版本。
