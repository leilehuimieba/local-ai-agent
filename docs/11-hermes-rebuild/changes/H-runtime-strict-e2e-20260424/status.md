# Status

更新时间：2026-04-24

## 当前状态

严格 runtime E2E 已通过，全部任务完成。

已完成：

1. Stage-E Entry1 默认验收已切回严格 runtime terminal 口径。
2. accepted / protocol 降级口径保留为显式 `-AllowAcceptedFallback`。
3. doctor 新增 `-FailOnError`，用于批量 E2E 中避免 JSON failed 被 exit code 0 掩盖。
4. Gateway 侧补充 `run_started` 与 `runtime_returned` 可见性事件。
5. runtime-host 已改为请求 worker 线程执行，避免主线程栈不足导致 Go/Gateway 调用 runtime 时连接被关闭。
6. storage migration 的 memory object backfill 已改为复用当前 SQLite connection，避免 `ensure_workspace_imported -> sync_memory_object_entry_sqlite -> with_connection -> ensure_workspace_imported` 递归重入。
7. Stage-E Entry1 等待窗口已延长，并在 timeout 时保留最后一次 logs 快照，便于失败定位。

8. 继续端到端测试已复跑严格模式、显式 fallback 模式、doctor fail-on-error、Rust/Go/前端测试与前端构建。
9. 继续测试中发现 memory object backfill 对已有版本的重复 upsert 会刷新 current 版本并破坏 rollback 语义；已改为已有 entry/version 时直接返回现有版本，避免重复写入。

## 当前阻塞 / 缺口

1. 无。
2. 遗留观察项：runtime-host worker 线程栈值是否需参数化；runtime 日志可作为后续治理参考，但不纳入 Gate-H 长期校准闭合证据。

## 下一步

1. 归档本 change 或切换下一主推进项。
