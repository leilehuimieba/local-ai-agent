# 设计：diff apply 最小闭环

## 范围

本 change 只做 Runtime 层的 `workspace_apply_patch` 工具，先形成可测试的代码修改闭环。

## Runtime 设计

1. `capabilities/registry.rs`
   - 新增 `workspace_apply_patch` 工具。
   - 风险级别为 `medium`，category 为 `workspace_write`。
2. `capabilities/spec.rs`
   - 新增 `unified_diff` schema。
   - 入参包含 `diff` 与 `dry_run`。
3. `planner.rs`
   - 新增 `PlannedAction::ApplyPatch`。
4. `action_decode.rs`
   - 解码模型 tool_call 的 `diff` 和 `dry_run`。
5. `executors/patch.rs`
   - 解析多文件 unified diff。
   - 校验目标路径位于 workspace root 内。
   - dry-run 只返回预览。
   - apply 时按上下文匹配替换文件内容。
   - 支持 `/dev/null -> b/path` 形式新增文件。
   - 支持 `a/path -> /dev/null` 形式删除文件。
   - 支持 `rename from` / `rename to` 形式纯 rename。
   - 返回 JSON apply report，记录每个文件的 path、kind、before_chars、after_chars。
   - 失败时返回 JSON failure report，记录 stage、path、reason、rollback_attempted。
6. `tool_trace.rs`
   - 复用现有 artifact 外置机制。

## 安全边界

1. 路径必须通过 `resolve_workspace_path` 校验。
2. 暂不支持二进制 patch。
3. 上下文不匹配时失败，不尝试模糊匹配。
4. apply 写入阶段失败时，会尝试回滚已写入文件。
5. 观察模式下作为修改动作被现有 `risk` 模式护栏拦截。

## 验证

1. 单文件成功 apply。
2. dry-run 不写文件。
3. 路径逃逸失败。
4. 上下文不匹配失败。
5. 多文件 patch 成功 apply。
6. 新增文件 patch 成功 apply。
7. 删除文件 patch 成功 apply。
8. rename patch 成功 apply。
9. 后续写入失败时回滚已写入文件。
10. Runtime 全量测试通过。
