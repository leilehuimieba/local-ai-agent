# T02 Windows 10 分钟验收摘要（2026-04-14）

更新时间：2026-04-14  
范围：`F-windows-10min-verification-20260414` / `F-05`

## 1. 验收命令

1. `powershell -ExecutionPolicy Bypass -File scripts/run-stage-f-windows-acceptance.ps1 -MaxMinutes 10`

## 2. 结果摘要

1. 报告文件：`tmp/stage-f-windows/latest.json`
2. 摘要文件：`tmp/stage-f-windows/latest.md`
3. `checked_at`：`2026-04-14T22:29:27.2119836+08:00`
4. `status`：`passed`
5. 时间预算：
   - `elapsed_ms=20603`
   - `threshold_ms=600000`
   - `within_time_budget=true`

## 3. 关键判定

1. `checks.gateway_ready=true`
2. `checks.runtime_ready=true`
3. `checks.first_task_completed=true`
4. `checks.within_time_budget=true`
5. 首任务终态：
   - `event_type=run_finished`
   - `completion_status=completed`

## 4. 证据路径

1. `tmp/stage-f-windows/latest.json`
2. `tmp/stage-f-windows/latest.md`
3. `tmp/stage-f-windows/sandbox/`（本次安装与运行沙箱）
