# 验证记录

## 验证方式

- E-01 验收脚本：
  - `powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-cli-history-acceptance.ps1`
- 后端回归：
  - `go test ./...`（`gateway/`）

## 证据位置

- 验收报告：
  - `tmp/stage-e-cli-history/latest.json`
  - `tmp/stage-e-cli-history/go-test.log`
- 关键代码：
  - `gateway/internal/api/logs_query.go`
  - `gateway/internal/api/router.go`
  - `gateway/internal/session/bus.go`

## Gate 映射

- 对应阶段 Gate：Gate-E（已签收）。
- 当前覆盖情况：
  - `E-01` 历史切片已补齐并通过测试。
  - 既有 `E-02`、`E-04`、`E-05`、`E-G1` 保持不回退。
