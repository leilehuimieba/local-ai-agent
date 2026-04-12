# 技术方案

## 影响范围

- 日志查询参数：`gateway/internal/api/logs_query.go`
- 日志接口分支：`gateway/internal/api/router.go`
- 运行历史聚合：`gateway/internal/session/bus.go`
- 单测：
  - `gateway/internal/api/logs_query_test.go`
  - `gateway/internal/api/logs_view_runs_test.go`
  - `gateway/internal/session/bus_recent_test.go`
- 验收脚本：`scripts/run-stage-e-cli-history-acceptance.ps1`

## 方案

### 1. logs 查询增加视图参数

- 新增 `view` 参数，支持：
  - `events`：默认，返回事件明细（原有行为）。
  - `runs`：返回 run 级历史（每个 run 只保留最新一条）。

### 2. EventBus 增加 run 级聚合

- 新增 `RecentRuns(limit, sessionID)`：
  - 读取日志文件。
  - 按 `session_id` 过滤（可选）。
  - 从尾部去重 `session_id + run_id`，保留每个 run 的最新条目。
  - 返回时间顺序结果。

### 3. 验收闭环

- 新增 `run-stage-e-cli-history-acceptance.ps1`：
  - 运行关键单测。
  - 产出 `tmp/stage-e-cli-history/latest.json` 与 `go-test.log`。

## 风险与回退

- 风险：`view=runs` 基于日志聚合，依赖日志完整性。
- 缓解：默认视图仍为 `events`，不影响既有调用方。
- 回退：若出现兼容问题，移除 `view=runs` 分支即可回到原行为。
