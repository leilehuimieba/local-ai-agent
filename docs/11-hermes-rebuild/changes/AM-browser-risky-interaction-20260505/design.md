# 技术方案

## 影响范围

- Runtime：
  - `crates/runtime-core/src/query_engine.rs`
  - `crates/runtime-core/src/verify.rs`
  - `crates/runtime-core/src/events.rs`
  - 可能涉及现有 browser tool contract 装配代码
- Gateway：
  - `gateway/internal/api/mcp_context_hints.go`
  - `gateway/internal/api/router_mcp.go`
  - `gateway/internal/mcp/manager.go`
- Browser MCP：
  - `frontend/scripts/browser-mcp-server.mjs`

## Harness 判断

- 浏览器主链已经具备“读页面、低风险交互、失败后单次回读”的最小能力。
- 下一步更适合沿浏览器主链向前推进，而不是回到 `AG` 或 `AL` 扩横向 scope。
- 因此本刀只收紧高风险交互，不把知识、记忆或 verify 通用矩阵重新拉进来。

## 范围冻结

- 本刀只做四件事：
  - 定义 `select / submit / upload` 的最小交互 contract
  - 接入现有 risk / confirmation 主链
  - 为三类交互补最小 verify 信号
  - 让 verify 失败时走受控 recovery / handoff，而不是盲点续跑

## 交互合同

### 最小动作集

- `browser/select`
  - 面向下拉框或等价选项控件
  - 至少包含目标元素定位与选项值摘要
- `browser/submit`
  - 面向表单提交或等价确认动作
  - 至少包含触发表单或提交按钮的目标定位
- `browser/upload`
  - 面向文件选择上传
  - 至少包含目标元素定位与文件摘要

### 风险分层

- `select`
  - `risk=medium`
  - `requires_confirmation=true`
- `submit`
  - `risk=high`
  - `requires_confirmation=true`
- `upload`
  - `risk=high`
  - `requires_confirmation=true`

## Verify / Recovery

### 输入信号

- 优先复用浏览器现有事件与 metadata，不重做复杂 DOM 分析：
  - 页面状态变化信号
  - 目标元素或目标页面标识信号
  - 风险审计与权限边界信号
  - 上传或提交后的结果摘要信号

### 通过标准

- `select`
  - 目标元素已定位
  - 选项变更可见
  - 风险边界已记录
- `submit`
  - 目标定位成立
  - 页面状态出现提交后的变化信号
  - 风险边界已记录
- `upload`
  - 目标定位成立
  - 文件摘要已被记录
  - 页面状态或结果摘要出现上传完成信号

### 失败收口

- verify 失败时优先触发一次 `browser/read_page` 回读。
- 如果当前动作已经是回读后的补救路径且仍失败，则直接进入 handoff。
- 不允许在失败后继续自动尝试第二次高风险交互。

## 验证路径

- Go / Rust / Browser MCP 测试覆盖：
  - request-scoped spec 能暴露三类新动作
  - risk / confirmation 分层正确
  - verify 失败会进入单次回读，再失败则 handoff
- 最小联调：
  - `open_page`
  - `select` 或 `submit` 或 `upload`
  - approval 后执行
  - verify 通过或走单次 recovery
