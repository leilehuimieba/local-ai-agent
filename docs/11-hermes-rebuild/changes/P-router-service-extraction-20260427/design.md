# 设计文档

## 分层原则

- api 层：路由注册 + HTTP 适配（解码请求、编码响应、写状态码）
- service 层：业务聚合、数据计算、外部调用

## 移动清单

### 从 router.go → service/settings_aggregate.go

1. `BuildSettingsResponse(repoRoot, cfg, store) SettingsResponse`
2. `BuildMemoryPolicy(repoRoot, workspace) MemoryPolicyStatus`
3. `BuildDiagnostics(repoRoot, cfg, runtimeStatus, ...) DiagnosticsStatus`
4. `BuildExternalConnections(repoRoot, cfg, workspace) []ExternalConnectionSlot`
5. `ExecuteExternalConnectionAction(...)` 及全部依赖
6. `BuildSystemInfo(repoRoot, runtimePort) map[string]any`

### router.go 保留

1. `NewRouter` — 路由组装
2. `registerCoreRoutes` / `registerProvidersRoutes` 等
3. `settingsHandler` — 调用 `service.BuildSettingsResponse`
4. `externalConnectionActionHandler` — 调用 `service.ExecuteExternalConnectionAction`
5. 类型定义（SettingsResponse、MemoryPolicyStatus、DiagnosticsStatus、ExternalConnectionSlot 等）

## 类型归属

SettingsResponse 等类型保留在 api 包（作为 API 合同）。service 层引用 api 包类型。

这是过渡方案。后续可将类型统一移入 contracts 包。
