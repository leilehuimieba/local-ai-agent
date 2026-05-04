# 任务清单

- [x] 建立 change 工作区并同步当前活跃项
  完成判据：`current-state.md` 与 `changes/INDEX.md` 指向本 change。

- [x] 实现 MCP 添加/删除持久化与 Manager 热重载
  完成判据：`POST /api/v1/settings` 支持 `add_mcp_name/add_mcp_url/remove_mcp_id`，并写回 `config/app.json` 后热替换 Manager。

- [x] 补齐设置页添加后刷新与删除入口
  完成判据：添加 MCP 后 settings 自动刷新；展开服务器卡片可通过确认弹窗删除。

- [x] 补后端单元测试
  完成判据：覆盖新增 HTTP Server、重复 URL 拒绝、配置写回和删除。

- [x] 验证 Gateway 与 Frontend 构建与相关测试
  完成判据：执行本次相关 Go 测试、Gateway 构建、前端类型检查、单测、构建和 E2E，并在 `verify.md` 记录结果。

- [x] 收敛全量 Go 测试的既有外部依赖失败
  完成判据：`go test ./...` 不再因 learning evidence 外部网络样本失败或超时。

- [x] 实现 Runtime 可见 MCP 工具预览注入
  完成判据：Gateway 将 ready MCP tools 写入 `RunRequest.context_hints`，Runtime 在工具预览和 AgentResolve prompt 中展示 MCP 工具摘要。

- [x] 实现 Runtime MCP tool registry 调用闭环
  完成判据：allowlisted 且无需确认的 MCP tools 进入模型 tool schema；Runtime 能解码 MCP tool_call 并通过 Gateway `/api/v1/mcp/call` 调用。

- [x] 设计 MCP allowlist / risk level / 审计字段
  完成判据：补策略字段、默认安全规则、日志映射和前端最小展示方案。

- [x] 设计 P1 diff apply 与 Windows doctor 切片
  完成判据：将代码 diff apply、doctor/启动器拆为后续独立 change 候选。
