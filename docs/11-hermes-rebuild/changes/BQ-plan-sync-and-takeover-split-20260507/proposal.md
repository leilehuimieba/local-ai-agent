# 变更提案

## 为什么做

`BP` 已完成 evidence / execution 外提与最近计划历史对账细化，但 `store.ts` 仍保留一簇明显热点：

1. `syncPlanState(...)` 仍留在 `store.ts`，承担计划同步与历史回写。
2. 时间预算打开时的“今日接管默认承接计划任务”仍留在 `store.ts`。
3. 提交 evidence / time budget / core done / close day / next day plan 后，仍由 `store.ts` 手工做计划同步与 followthrough 收口。

这一簇已经形成相对独立的“计划同步 / 今日接管”责任边界，继续留在 `store.ts` 会让热点难以下降。

## 做什么

1. 新增 `plan-sync-flow-rules.ts`。
2. 抽出计划同步、今日接管默认填充、计划相关提交后收口逻辑。
3. 让 `store.ts` 进一步退回到 action 装配层。

## 不做什么

1. 不改产品规则。
2. 不改后端持久化。
3. 不做完整 `store.ts` 大拆分，只拆计划同步 / 今日接管簇。

## 验收标准

1. `store.ts` 不再直接承载计划同步 / 今日接管这簇派生逻辑。
2. 现有明日计划、今日接管、计划待重算、计划依据变化、历史回写测试保持通过。
3. 前端定向与全量回归保持通过。
