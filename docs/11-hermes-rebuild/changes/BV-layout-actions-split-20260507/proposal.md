# 变更提案

## 为什么做

`BU` 已完成主线 action 装配簇外提，但 `store.ts` 里还保留最后一个明显的 UI 装配热点：

1. `createUILayoutActions(...)` 仍在 `store.ts` 内承担布局层 action 注册。
2. 左侧栏、右抽屉、移动端菜单、主线展开态与面板开关仍在 `store.ts` 里直接编排。
3. `setTimeBudgetPanelOpen(...)` 仍在布局 action 内联动主线桥接函数。

这部分已经形成稳定的“布局 action 装配层”责任边界，继续留在 `store.ts` 会让 store 壳仍承担 UI 行为编排细节。

## 做什么

1. 新增 `layout-actions.ts`。
2. 外提布局 action 装配函数。
3. 继续收紧 `store.ts`，让其更接近纯 store 壳。

## 不做什么

1. 不改动任何产品规则或交互语义。
2. 不改基础 state 字段定义。
3. 不扩到 runtime/settings/knowledge/logs/memory store。

## 验收标准

1. `store.ts` 不再直接内联布局 action 装配函数实现。
2. 主线与布局相关交互测试保持通过。
3. 前端定向与全量回归保持通过。
