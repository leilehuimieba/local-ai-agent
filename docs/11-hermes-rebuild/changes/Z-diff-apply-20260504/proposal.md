# Z-change: diff apply 最小闭环

## 背景

Y-change 已补齐 MCP 生态入口、Runtime MCP tool_call 调用闭环和 Gateway 安全审计。下一块短板是代码修改闭环：当前 Runtime 已有 `workspace_write`，但缺少 Aider/Codex 风格的 diff 预览、边界校验和 patch 应用路径。

## 目标

1. 新增 `workspace_apply_patch` Runtime 工具，支持 unified diff 的最小应用闭环。
2. 支持 dry-run 预览，返回将新增或修改的文件列表。
3. 应用前校验路径必须位于当前 workspace root 内。
4. 冲突或上下文不匹配时拒绝应用，不做静默覆盖。
5. 将结果纳入现有 tool trace、artifact 与 verification 流程。

## 非目标

1. 本 change 不实现复杂三方 merge。
2. 本 change 不接 IDE 编辑器 UI。
3. 本 change 不做自动 git commit。
4. 本 change 不实现二进制文件 patch。

## 验收标准

1. 模型可选择 `workspace_apply_patch` 工具。
2. dry-run 不修改文件，并返回结构化预览。
3. apply 能应用多文件 unified diff，并支持新增、删除和 rename。
4. 路径逃逸、上下文不匹配、重复触达文件等场景会明确失败。
5. 写入阶段失败时尝试回滚已写入文件，并返回结构化失败报告。
5. `cargo test -p runtime-core` 通过，并覆盖成功、dry-run、冲突和路径边界。
