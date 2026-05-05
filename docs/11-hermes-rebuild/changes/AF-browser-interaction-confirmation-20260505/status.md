# 当前状态

- 最近更新时间：2026-05-05
- 状态：已收口
- 状态口径：当前阶段 / 当前 Gate / 当前活跃 change 统一引用 `docs/11-hermes-rebuild/current-state.md`
- 已完成：已完成 AE，浏览器只读动作 `open_page` / `read_page` 已接入 Runtime 主链。
- 已确认：confirmation 型 MCP 工具当前被 request-scoped hints 与 Runtime MCP bridge 过滤，无法进入模型可调度列表。
- 已确认：Gateway MCP 调用当前会直接拒绝 `requires_confirmation=true` 的工具，审批后执行链尚未打通。
- 已完成：已冻结 `click` / `type` contract；当前口径为 `click=risk:medium + requires_confirmation:true`，`type=risk:high + requires_confirmation:true`。
- 已完成：request-scoped MCP tool spec 已改为包含 confirmation 型 allowlisted 工具，Runtime capability catalog 与 model tools 都可见。
- 已完成：Runtime MCP 风险门已能为确认型 MCP 动作生成 confirmation request，并把 `tool_name` / `tool_arguments_json` 带入确认事件。
- 已完成：Gateway `/api/v1/mcp/call` 已支持在批准后的 confirmation 上下文中放行一次确认型 MCP 调用，同时保留原有 allowlist / audit 记录。
- 已完成：真实 browser MCP server 已接出 `click` / `type`，并通过接口级 E2E 验证 `open_page -> click/type -> read_page`。
- 已完成：AF 当前范围已实现并通过测试，可直接进入提交或归档准备。
- 阻塞点：暂无。
- 下一步：等待下一项 change 决定是否继续扩 `submit/select/upload` 一类更高风险交互。
