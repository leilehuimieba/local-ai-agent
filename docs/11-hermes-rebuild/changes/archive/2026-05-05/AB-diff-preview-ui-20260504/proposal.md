# AB-change: diff apply UI 预览

## 背景

Z-change 已补齐 Runtime `workspace_apply_patch` 最小闭环，支持 dry-run、多文件、新增、删除、rename、冲突报告和失败回滚。当前缺口是用户确认层：Runtime 能安全应用 patch，但前端还没有 Aider/Codex 风格的 diff 预览、确认和结果展示。

## 目标

1. 前端展示 `workspace_apply_patch` dry-run 结果，区分新增、修改、删除和 rename。
2. 展示冲突、路径边界、写入失败和回滚结果的结构化报告。
3. 用户确认后再执行 apply。
4. apply 后展示成功文件列表和失败原因。
5. 将 diff 预览纳入现有任务事件流和确认体验。

## 非目标

1. 本 change 不实现复杂三方 merge。
2. 本 change 不实现二进制 diff 预览。
3. 本 change 不实现 IDE 编辑器级逐行编辑。
4. 本 change 不自动创建 Git commit。

## 验收标准

1. 前端能展示 dry-run 文件变更摘要。
2. 新增、修改、删除、rename 四类变更有明确视觉区分。
3. 失败报告能展示 stage、path、reason 和 rollback_attempted。
4. 用户确认后才执行 apply。
5. 前端相关测试或类型检查通过，并记录验证证据。
