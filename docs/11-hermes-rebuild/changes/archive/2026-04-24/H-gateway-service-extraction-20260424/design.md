# 设计

## 目标

- handler 层只负责 HTTP 协议解析、参数校验、响应序列化
- 业务逻辑（状态计算、外部调用、修复动作）全部下沉到 `internal/service`
- Response/Request 类型跟随业务逻辑所在包，service 包中导出

## 抽取范围

1. `api/provider_settings.go` → `service/provider_settings.go`
2. `api/diagnostics_remediation.go` → `service/diagnostics.go`

## 共享辅助

`pathExists` 原在 `api/router.go`，抽取到 service 包后改名为 `PathExists`，避免 api 包与 service 包循环依赖。
