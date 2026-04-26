# Verify

更新时间：2026-04-26

## 验证命令

```powershell
cd gateway
go test ./internal/api ./internal/service
cd ..\frontend
npm test -- --run
npx tsc --noEmit
cd ..
scripts/run-full-regression.ps1 -OutFile tmp/m-release-wizard-buttonized-regression-20260426.json
```

## 验证结果

1. Gateway Go 测试：`internal/api` 与 `internal/service` 通过。
2. 前端单元测试：25 文件 / 72 测试通过。
3. TypeScript：`npx tsc --noEmit` 0 错误。
4. 全量回归：`tmp/m-release-wizard-buttonized-regression-20260426.json` 显示 6 项全绿。
5. 新增按钮化交互测试覆盖：点击“运行Doctor 诊断”会请求 `/api/v1/release/run`，body 为 `{"step":"doctor"}`，并展示通过、耗时和产物路径。
6. 后端白名单测试覆盖：未知 step 返回错误，已知 step 只能映射到固定脚本配置。

## 证据文件

- `tmp/m-release-wizard-buttonized-regression-20260426.json`
- `tmp/m-productization-release-wizard-regression-20260426.json`
- `tmp/m-release-wizard-install.json`
- `tmp/m-release-wizard-doctor.json`
- `tmp/stage-f-rc/latest.json`
- `docs/11-hermes-rebuild/changes/M-productization-release-wizard-20260426/user-simulation.md`