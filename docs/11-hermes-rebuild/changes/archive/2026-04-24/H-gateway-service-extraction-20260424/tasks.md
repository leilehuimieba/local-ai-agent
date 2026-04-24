# 任务

- [x] T01 创建 `gateway/internal/service/provider_settings.go`，抽取 provider 查询/测试/保存/应用/移除业务逻辑
- [x] T02 缩减 `gateway/internal/api/provider_settings.go` 至纯 handler 层
- [x] T03 创建 `gateway/internal/service/diagnostics.go`，抽取日志/前端/网关/配置四类诊断修复逻辑
- [x] T04 缩减 `gateway/internal/api/diagnostics_remediation.go` 至纯 handler 层
- [x] T05 迁移 diagnostics 测试文件至 `service` 包，适配类型与函数引用
- [x] T06 编译验证 `go build ./...`
- [x] T07 测试验证 `go test ./internal/service` 9/9 通过
