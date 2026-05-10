# 任务清单

- [x] 建立 Z-change 工作区并切换活跃项
  完成判据：`current-state.md` 与 `changes/INDEX.md` 指向本 change。

- [x] 新增 Runtime `workspace_apply_patch` 工具注册
  完成判据：模型 tool schema 可见 `workspace_apply_patch`，并包含 `diff/dry_run` 入参。

- [x] 实现 unified diff dry-run
  完成判据：dry-run 返回预览，不修改目标文件。

- [x] 实现 unified diff apply
  完成判据：上下文匹配时写回目标文件，失败时保留原文件。

- [x] 支持多文件 patch、新增文件和 apply report
  完成判据：同一个 diff 可修改多个文件，可新增文件，并返回结构化 JSON 报告。

- [x] 支持删除文件、rename 和失败回滚
  完成判据：删除文件和纯 rename 可应用；后续写入失败时恢复已写入文件，并返回 rollback 标记。

- [x] 细化冲突 report
  完成判据：解析、路径、上下文冲突和写入失败都返回 stage/path/reason/rollback_attempted。

- [x] 补路径边界和冲突测试
  完成判据：路径逃逸、上下文不匹配、删除、rename 和回滚场景均有测试覆盖。

- [x] 运行 Runtime 回归并同步验证记录
  完成判据：`cargo test -p runtime-core` 通过，并在 `verify.md` 记录。
