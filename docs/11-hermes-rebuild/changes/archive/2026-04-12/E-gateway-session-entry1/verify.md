# 验证记录

## 验证方式

- 单元测试：`go test ./...`（工作目录：`gateway/`）。
- 接口验收：`powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-entry1-acceptance.ps1`。
- 人工复核：检查 `run_accepted` 协议字段和 `logs` 过滤结果一致性。

## 证据位置

- 测试记录：
  - `gateway/internal/api/chat.go`
  - `gateway/internal/api/router.go`
  - `gateway/internal/api/logs_query.go`
  - `gateway/internal/session/bus.go`
  - `gateway/internal/api/chat_entry_protocol_test.go`
  - `gateway/internal/api/logs_query_test.go`
  - `gateway/internal/session/bus_recent_test.go`
  - `scripts/run-stage-e-entry1-acceptance.ps1`
- 日志与样本：
  - `tmp/stage-e-entry1/latest.json`
  - `tmp/stage-e-entry1/logs/runtime.log`
  - `tmp/stage-e-entry1/logs/gateway.log`

## Gate 映射

- 对应阶段 Gate：Gate-E（进行中，不做完成声明）。
- 当前覆盖情况：
  - 已完成 `E-02` 后端首入口协议收口与接口样本验证。
  - 已具备会话级和运行级日志过滤能力，可支撑前端接入联调。
  - 尚未完成跨入口一致性对比与阶段评审收口。
