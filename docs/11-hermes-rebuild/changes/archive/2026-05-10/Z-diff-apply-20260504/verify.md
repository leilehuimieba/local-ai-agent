# 验证记录

更新时间：2026-05-04

## 已执行

1. `cargo test -p runtime-core patch -- --nocapture`
   - 结果：通过，10 个 patch 相关测试全绿，覆盖 tool_call 解码、单文件 apply、dry-run、上下文不匹配、路径逃逸、多文件 patch、新增文件、删除文件、rename 和写入失败回滚。
2. `cargo test -p runtime-core`
   - 结果：通过，190 个测试全绿；保留既有 `redact_sensitive_text` 未使用 warning。
3. `cd gateway; go test ./internal/api -run "TestMCPRuntimeBridgeE2EUsesGatewayPolicyAndAudit|TestMCP" -count=1`
   - 结果：通过，确认 Y-change MCP 验收夹具仍可运行。
4. `cd gateway; go test ./...`
   - 结果：通过，Gateway 全量测试通过。
5. `cd gateway; go build -o gateway.exe ./cmd/server`
   - 结果：通过。

## 验收结论

1. Z-change 已满足 proposal 中定义的最小闭环验收标准。
2. 删除文件、rename、细化冲突 report 和失败回滚已纳入本次收口范围。
3. 后续模糊 merge、二进制 patch 和 UI 预览不作为本 change 阻塞项。

## 未通过 / 残余风险

1. 暂不支持二进制 patch。
2. 上下文匹配采用精确匹配，当前不做模糊 merge。
3. 写入失败会尝试回滚已写入文件；极端磁盘故障或权限变化仍可能需要人工检查。

## 验收映射

1. `workspace_apply_patch` 工具注册：由 `cargo test -p runtime-core` 编译和 capability schema 路径覆盖。
2. dry-run 不写文件：由 `executors::patch::tests::dry_run_does_not_write_file` 覆盖。
3. 单文件 apply：由 `executors::patch::tests::applies_single_file_unified_diff` 覆盖。
4. 多文件 patch：由 `executors::patch::tests::applies_multi_file_patch` 覆盖。
5. 新增文件 patch：由 `executors::patch::tests::creates_new_file_from_unified_diff` 覆盖。
6. 删除文件 patch：由 `executors::patch::tests::deletes_file_from_unified_diff` 覆盖。
7. rename patch：由 `executors::patch::tests::renames_file_without_content_change` 覆盖。
8. 写入失败回滚：由 `executors::patch::tests::rollback_restores_first_file_when_later_write_fails` 覆盖。
9. 路径边界与冲突失败：由 `rejects_workspace_escape`、`rejects_context_mismatch` 覆盖。
