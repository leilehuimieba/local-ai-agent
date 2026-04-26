# Verify

更新时间：2026-04-26

## 验证命令

```powershell
cd frontend
npm test -- --run
npx tsc --noEmit
cd ..
cargo check --workspace
scripts/run-full-regression.ps1 -OutFile tmp/l-competitor-onboarding-regression-20260426.json
```

## 验证结果

1. Frontend 单元测试：24 文件 / 69 测试通过。
2. TypeScript：`npx tsc --noEmit` 0 错误。
3. Rust：`cargo check --workspace` 通过。
4. 全量回归：`tmp/l-competitor-onboarding-regression-20260426.json` 显示 6 项全绿。

## 全量回归摘要

- rust_check：passed
- rust_test：passed
- go_build：passed
- go_test：passed
- frontend_build：passed
- e2e_acceptance：passed

## 场景验证

竞品用户迁移体验模拟已记录在：

- `docs/11-hermes-rebuild/changes/L-competitor-onboarding-20260426/user-simulation.md`
