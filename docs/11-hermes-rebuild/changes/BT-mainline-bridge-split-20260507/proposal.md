# 变更提案

## 为什么做

`BS` 已完成主线默认状态装配簇外提，但 `store.ts` 里还保留一组明显的主线桥接热点：

1. `applyProbabilityState(...)` 仍在 `store.ts` 内处理概率已知态桥接。
2. `buildEvidenceSubmitState(...)`、`buildKeyEvidenceState(...)`、`buildTimeBudgetSubmitState(...)` 仍在 `store.ts` 内把外部规则模块拼成最终 UI state。
3. `buildCoreTaskDoneState(...)`、`buildTodayClosedState(...)`、`buildNextDayPlanState(...)`、`buildTimeBudgetPanelOpenState(...)` 仍在 `store.ts` 内承担薄包装桥接。

这些函数的职责已经明显偏向“主线桥接层”，继续留在 `store.ts` 会让 store 壳与桥接细节继续耦合。

## 做什么

1. 新增 `mainline-bridge-rules.ts`。
2. 外提主线概率桥接、提交桥接与计划/执行桥接函数。
3. 继续收紧 `store.ts`，让其更接近纯 store + action 装配。

## 不做什么

1. 不改动主线总控 Agent 的产品规则。
2. 不改测试口径，不改 UI 文案。
3. 不扩到 action 装配簇或其他 store。

## 验收标准

1. `store.ts` 不再直接内联主线桥接函数实现。
2. 主线关键证据、晚间证据、时间预算、执行态、明日计划相关测试保持通过。
3. 前端定向与全量回归保持通过。
