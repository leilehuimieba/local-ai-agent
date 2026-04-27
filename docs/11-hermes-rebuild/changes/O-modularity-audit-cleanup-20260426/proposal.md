# O-modularity-audit-cleanup：模块化审计后第一轮清理

## 背景

架构审计发现前端 API 层 `readErrorText` 重复定义 4 处，Go gateway `pathExists` 重复定义 2 处。属于低风险、高明确度的冗余清理。

## 目标

1. 统一前端 API 错误读取逻辑到 `shared/apiUtils.ts`
2. 统一 Go 路径检查逻辑到 `internal/util/path.go`
3. 不引入新依赖，不改变业务行为

## 范围

- frontend/src/shared/apiUtils.ts（新增）
- frontend/src/chat/api.ts（移除 readErrorText）
- frontend/src/settings/api.ts（移除 readErrorText）
- frontend/src/release/api.ts（移除 readErrorText）
- frontend/src/knowledge-base/api.ts（移除 readErrorText）
- gateway/internal/util/path.go（新增）
- gateway/internal/api/router.go（移除 pathExists）
- gateway/internal/service/diagnostics.go（移除 pathExists）

## 回退方式

恢复各文件内联函数即可。
