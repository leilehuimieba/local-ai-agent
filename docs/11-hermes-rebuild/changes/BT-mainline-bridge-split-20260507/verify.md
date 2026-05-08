# 验证记录

## 验证方式

- 单元测试：
  - 主线关键证据、晚间证据、时间预算、执行态与明日计划链路保持通过
- 人工验证：
  - 关键入口行为与 UI 状态更新保持一致

## 本次执行结果

1. 定向回归：
   - 命令：`npm test -- mainline-shell`
   - 结果：`30 passed (30)`
2. 前端全量回归：
   - 命令：`npm test`
   - 结果：`56 passed (56)`

## 关键结果

1. 新增 `mainline-bridge-rules.ts`，承接：
   - `applyProbabilityState(...)`
   - `buildEvidenceSubmitState(...)`
   - `buildKeyEvidenceState(...)`
   - `buildTimeBudgetSubmitState(...)`
   - `buildCoreTaskDoneState(...)`
   - `buildTodayClosedState(...)`
   - `buildNextDayPlanState(...)`
   - `buildTimeBudgetPanelOpenState(...)`
2. `store.ts` 中不再直接内联主线桥接函数实现。
3. `store.ts` 行数进一步下降到 `935`，主线桥接责任边界已独立。
4. 主线关键证据、晚间证据、时间预算、执行态与明日计划相关行为测试保持不变。

## 证据位置

1. `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
2. `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
3. `D:/newwork/本地智能体/frontend/lib/local-agent/mainline-bridge-rules.ts`

## Gate 映射

- 对应自由迭代期目标：
  - 持续压缩 `store.ts` 热点，并把主线桥接逻辑独立成可维护模块
