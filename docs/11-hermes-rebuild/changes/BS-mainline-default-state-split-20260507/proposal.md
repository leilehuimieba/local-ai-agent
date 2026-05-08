# 变更提案

## 为什么做

`BR` 已完成 `switch / personalized` 两簇外提，但 `store.ts` 里仍保留主线总控默认状态装配热点：

1. `createUIMainlineState(...)` 仍直接负责主线 UI 初始展开态。
2. `createDefaultMainlineShell(...)` 仍直接拼装主线总控默认状态。
3. `createDefaultEvidencePacket(...)` 与 `createDefaultTemporaryState(...)` 仍内联在 `store.ts`。

这部分已经形成稳定的“主线总控默认状态定义”责任边界，继续留在 `store.ts` 会让 UI store 装配层和默认状态定义继续耦合。

## 做什么

1. 新增 `mainline-default-state.ts`。
2. 外提主线总控默认状态装配与其局部默认值定义。
3. 继续收紧 `store.ts`，让其只保留 store 创建、action 装配与少量桥接逻辑。

## 不做什么

1. 不改动主线总控 Agent 的产品规则。
2. 不改默认状态字段语义，不改测试口径。
3. 不扩到 runtime store、settings store 或其他模块。

## 验收标准

1. `store.ts` 不再直接内联主线总控默认状态装配细节。
2. 主线默认状态相关 UI 与行为测试保持通过。
3. 前端定向与全量回归保持通过。
