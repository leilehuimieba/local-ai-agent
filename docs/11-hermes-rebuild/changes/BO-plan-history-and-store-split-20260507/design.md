# 设计说明

## 影响范围

### 文档

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
3. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/BO-plan-history-and-store-split-20260507/*`

### 前端状态

1. `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`
2. `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
3. `D:/newwork/本地智能体/frontend/lib/local-agent/plan-history-rules.ts`

### 前端展示

1. `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
2. `D:/newwork/本地智能体/frontend/components/local-agent/today-plan-history-card.tsx`
3. `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`

## 状态设计

新增：

1. `PlanHistoryItem`
2. `recentPlanHistory`

## 核心判断

1. 仅在计划进入 `taken_over / completed / closed` 时写入或更新最近历史。
2. 同一天同一目标日期优先更新，不重复追加多条。
3. 历史卡只展示最近 3 条。

## 热点治理

1. 把 today-plan summary / reconcile / history 的派生逻辑从 `store.ts` 抽到 `plan-history-rules.ts`。
2. 本刀不追求彻底拆散 store，只做计划簇第一步外提。

## 回退方式

删除计划历史记录与独立规则文件，恢复到 BN 的当日态展示。
