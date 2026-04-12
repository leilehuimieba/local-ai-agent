# 技术方案

## 影响范围

- 验收脚本：`scripts/run-stage-f-windows-acceptance.ps1`
- 证据目录：`tmp/stage-f-windows/`
- 状态文档：`docs/11-hermes-rebuild/changes/*`、`stage-plans/全路线最小任务分解总表.md`

## 方案

### 1. 新机验证链路

脚本在隔离目录执行以下步骤：

1. 调用 `install-local-agent.ps1` 执行 install。
2. 启动 `launcher.exe`，等待 gateway/runtime 健康。
3. 通过 `chat/run` 发起首任务（`cmd: Get-Date`）。
4. 轮询日志直到终态事件出现，校验任务完成状态。

### 2. 时间门槛判定

- 开始时间：安装前。
- 结束时间：首任务终态返回后。
- 阈值：`MaxMinutes * 60 * 1000`，默认 10 分钟。
- 判定字段：
  - `elapsed_ms`
  - `threshold_ms`
  - `within_time_budget`

### 3. 输出物

- `latest.json`：结构化报告。
- `latest.md`：可读摘要。

## 风险与回退

- 风险：当前验证为“同机隔离目录模拟”，不等同真实全新 Windows 系统镜像。
- 缓解：在 F-G1 评审中注明验证边界，并保留后续实机抽检计划。
- 回退：若超时或首任务失败，保留报告并回滚到 F-04 已通过基线重新定位。
