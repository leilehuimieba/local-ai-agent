# 验证记录

## 验证方式

- 单元测试：如新增前端派生逻辑，补最小单测。
- 集成测试：非必须，除非需要补 gateway 只读字段。
- 人工验证：通过设置页或验收入口检查 MCP 工具风险、allowlist、审计字段和空态展示。

## 证据位置

- 测试记录：
  - `frontend/` 下执行 `npm run build` 通过。
  - `gateway/` 下执行 `go test ./internal/api` 通过。
- 日志或截图：
  - 设置页 MCP 空态联调截图：[ac-mcp-observability-settings-empty-state.png](D:/newwork/本地智能体/tmp/ac-mcp-observability-settings-empty-state.png)
  - MCP 验收页截图：[ac-mcp-observability-acceptance.png](D:/newwork/本地智能体/tmp/ac-mcp-observability-acceptance.png)
  - MCP 验收页含最近动作截图：[ac-mcp-observability-acceptance-with-audits.png](D:/newwork/本地智能体/tmp/ac-mcp-observability-acceptance-with-audits.png)
  - MCP 验收页 HTTP 联调截图：[ac-mcp-observability-acceptance-http.png](D:/newwork/本地智能体/tmp/ac-mcp-observability-acceptance-http.png)
  - MCP 验收页文档服务器筛选截图：[ac-mcp-observability-docs-filter-http.png](D:/newwork/本地智能体/tmp/ac-mcp-observability-docs-filter-http.png)
- 代码盘点：
  - `frontend/components/local-agent/views/settings-view.tsx`
  - `frontend/components/local-agent/views/mcp-observability-panel.tsx`
  - `frontend/lib/local-agent/types.ts`
  - `frontend/lib/local-agent/api.ts`
  - `gateway/internal/api/mcp_audit_read.go`
  - `gateway/internal/api/mcp_audit_read_test.go`
  - `gateway/internal/api/router_types.go`
  - `gateway/internal/api/settings_response.go`
  - `gateway/internal/api/router_mcp.go`
  - `gateway/internal/api/mcp_audit.go`
  - `gateway/internal/mcp/client.go`
  - `gateway/internal/mcp/manager.go`
  - `gateway/internal/mcp/policy.go`

## Gate 映射

- 对应阶段 Gate：阶段 I 自由迭代期。
- 当前覆盖情况：AB 已归档并完成前端验收入口留证；AC 已完成字段盘点、前端只读观测面板、正式验收页接入、只读审计 API、最近动作卡片、按服务器联动的审计筛选、错误码聚合和前后端验证留证。
