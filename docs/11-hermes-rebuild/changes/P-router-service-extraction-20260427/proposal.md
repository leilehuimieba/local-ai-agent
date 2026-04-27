# P-router-service-extraction：router.go 聚合逻辑移入 service 层

## 背景

架构审计发现 `gateway/internal/api/router.go` 712 行，其中约 350 行不是路由注册逻辑，而是 settings / diagnostics / memory / external connections 的聚合计算逻辑。这些逻辑应归属 service 层。

## 目标

1. 将 `buildSettingsResponse` 及其依赖函数移入 `internal/service`
2. 将 `executeExternalConnectionAction` 及其依赖函数移入 `internal/service`
3. 将 `systemInfoHandler` 中的聚合逻辑移入 `internal/service`
4. router.go 只保留路由注册、HTTP 适配、类型定义

## 范围

- `gateway/internal/service/settings_aggregate.go`（新增）
- `gateway/internal/api/router.go`（修改，目标 < 400 行）
- `gateway/internal/api/router_release.go`（无需修改）

## 回退方式

恢复 router.go 中被删除的函数即可。
