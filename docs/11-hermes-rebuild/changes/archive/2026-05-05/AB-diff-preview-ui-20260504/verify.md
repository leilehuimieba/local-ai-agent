# 验证记录

更新时间：2026-05-05

## 计划执行

1. 前端类型检查。
2. 前端相关单元测试。
3. 如涉及交互流程，补 Playwright 或组件测试。

## 已执行

1. `cd frontend; pnpm test -- components/local-agent/__tests__/diff-preview.test.tsx`
   - 结果：通过，1 个测试文件、6 项测试全绿。
2. `cd frontend; pnpm exec tsc --noEmit`
   - 结果：通过。
3. `cd frontend; pnpm test`
   - 结果：通过，3 个测试文件、17 项测试全绿。
4. `git diff --check`
   - 结果：通过。
5. `cargo test -p runtime-core run_risk_flow_tests -- --nocapture`
   - 结果：通过，4 项风险流相关测试全绿。
6. `cargo test -p runtime-core risk -- --nocapture`
   - 结果：通过，9 项 risk / run_risk_flow 相关测试全绿。
7. `cd gateway; go test ./internal/...`
   - 结果：通过。
8. `cd frontend; pnpm test -- lib/local-agent/__tests__/store.test.ts components/local-agent/__tests__/diff-preview.test.tsx`
   - 结果：通过，2 个测试文件、11 项测试全绿。
9. `cd frontend; pnpm exec tsc --noEmit`
   - 结果：通过。
10. `cd frontend; pnpm test -- components/local-agent/__tests__/diff-preview.test.tsx components/local-agent/__tests__/task-view-confirmation.test.tsx`
   - 结果：通过，2 个测试文件、10 项测试全绿。
11. `cd frontend; pnpm exec tsc --noEmit`
   - 结果：通过。
12. `cd frontend; pnpm test -- components/local-agent/__tests__/diff-preview.test.tsx components/local-agent/__tests__/task-view-confirmation.test.tsx`
   - 结果：通过，2 个测试文件、12 项测试全绿。
13. `cd frontend; pnpm exec tsc --noEmit`
   - 结果：通过。
14. 浏览器联调：打开 `http://localhost:3000/acceptance/confirmation-preview`
   - 结果：通过。页面同时展示 structured preview 与 fallback summary；fallback 能正确渲染 rename + modify 摘要。
   - 证据：`tmp/ab-confirmation-acceptance-browser-check.png`

## 未通过 / 残余风险

1. fallback 摘要基于 unified diff 文本推断，字符数为估算值，不等同于 Runtime dry-run 真实统计。
2. 当前浏览器联调使用专用 debug 页面，不是完整事件流端到端场景。

## 验收映射

1. dry-run 文件摘要：由组件测试或手动截图验证。
2. 失败报告展示：由 `DiffPreview` 组件测试覆盖。
3. confirmation payload 绑定 patch 数据：由 Runtime / Frontend / Gateway 测试联合覆盖。
4. 确认后 apply：由确认卡按钮语义、前端决策回调测试和既有 Runtime 风险流测试联合覆盖。
5. fallback 摘要推断：由 `DiffPreview` 组件测试覆盖 create / modify / rename，并由浏览器联调页面补展示证据。
