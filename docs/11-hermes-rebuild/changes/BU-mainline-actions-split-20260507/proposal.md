# 变更提案

## 为什么做

`BT` 已完成主线桥接簇外提，但 `store.ts` 里还保留最后一组明显的主线热点：

1. `createMainlineActions(...)` 仍在 `store.ts` 内聚合主线 action。
2. `createGoalActions(...)`、`createEvidenceActions(...)`、`createSwitchActions(...)`、`createPersonalizedActions(...)` 仍在 `store.ts` 内定义。
3. `createTimeBudgetActions(...)`、`createExecutionActions(...)`、`createNextDayPlanActions(...)` 仍在 `store.ts` 内承担主线 action 注册。

这部分已经形成稳定的“主线 action 装配层”责任边界，继续留在 `store.ts` 会让 store 壳仍然承担主线行为编排细节。

## 做什么

1. 新增 `mainline-actions.ts`。
2. 外提主线 action 装配函数。
3. 继续收紧 `store.ts`，让其更接近纯 store 壳。

## 不做什么

1. 不改动主线总控 Agent 的产品规则。
2. 不改动 action 行为语义，不改测试口径。
3. 不扩到 runtime/settings/knowledge/logs/memory store。

## 验收标准

1. `store.ts` 不再直接内联主线 action 装配函数实现。
2. 主线相关交互测试保持通过。
3. 前端定向与全量回归保持通过。
