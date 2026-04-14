# 技术方案

## 影响范围

- 涉及模块：
  1. `scripts/run-stage-f-windows-acceptance.ps1`
  2. `scripts/install-local-agent.ps1`（仅被调用，不改动）
- 涉及文档或 contract：
  1. `docs/11-hermes-rebuild/current-state.md`
  2. `docs/11-hermes-rebuild/changes/INDEX.md`
  3. `docs/11-hermes-rebuild/changes/F-windows-10min-verification-20260414/*`
  4. `tmp/stage-f-windows/latest.json`
  5. `tmp/stage-f-windows/latest.md`

## 方案

- 核心做法：
  1. 以 `run-stage-f-windows-acceptance.ps1 -MaxMinutes 10` 作为 `F-05` 唯一验收入口。
  2. 校验四类通过条件：
     - `gateway_ready=true`
     - `runtime_ready=true`
     - `first_task_completed=true`
     - `within_time_budget=true`
  3. 校验首任务终态：`event_type=run_finished` 且 `completion_status=completed`。
- 状态流转或调用链变化：
  1. 本刀只执行既有验收脚本并回写文档，不改 runtime/gateway 逻辑。
  2. `F-05` 完成后可进入 Gate-F 汇总判定（`scripts/run-stage-f-gate-acceptance.ps1`）。

## 风险与回退

- 主要风险：
  1. 本机资源波动导致耗时超过阈值，触发假失败。
  2. 端口占用或临时网络抖动造成首任务终态不稳定。
- 回退方式：
  1. 若失败，保留 `latest.json/latest.md` 作为失败样本，先定位是时间预算还是首任务终态问题。
  2. 未定位根因前不推进 Gate-F 总签收。
