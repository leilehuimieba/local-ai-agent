# T02 doctor 核心检查摘要（2026-04-14）

更新时间：2026-04-14  
范围：`F-doctor-core-checks-20260414` / `F-02`

## 1. 验收命令

1. `powershell -ExecutionPolicy Bypass -File scripts/run-stage-f-doctor-acceptance.ps1`

## 2. 结果摘要

1. 报告文件：`tmp/stage-f-doctor/latest.json`
2. `checked_at`：`2026-04-14T21:59:42.4713329+08:00`
3. `status`：`passed`
4. 端口分配：
   - `gateway=57526`
   - `runtime=57525`

## 3. 核心检查结果

1. `go_available=true`
2. `rust_available=true`
3. `node_available=true`
4. `npm_available=true`
5. `config_exists=true`
6. `ports_valid=true`
7. `frontend_dist_exists=true`
8. `runtime_health_ok=true`
9. `gateway_health_ok=true`
10. `logs_writable=true`

## 4. 证据路径

1. `tmp/stage-f-doctor/latest.json`
2. `tmp/stage-f-doctor/logs/runtime.log`
3. `tmp/stage-f-doctor/logs/gateway.log`
4. `tmp/stage-f-doctor/logs/runtime-build.stdout.log`
5. `tmp/stage-f-doctor/logs/runtime-build.stderr.log`
6. `tmp/stage-f-doctor/logs/gateway-build.stdout.log`
7. `tmp/stage-f-doctor/logs/gateway-build.stderr.log`
