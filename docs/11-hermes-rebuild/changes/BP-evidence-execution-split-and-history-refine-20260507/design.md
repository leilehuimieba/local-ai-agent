# 设计说明

## 影响范围

### 文档

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
3. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/BP-evidence-execution-split-and-history-refine-20260507/*`

### 前端状态

1. `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
2. `D:/newwork/本地智能体/frontend/lib/local-agent/plan-history-rules.ts`
3. `D:/newwork/本地智能体/frontend/lib/local-agent/evidence-flow-rules.ts`
4. `D:/newwork/本地智能体/frontend/lib/local-agent/execution-flow-rules.ts`
5. `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`

### 前端展示

1. `D:/newwork/本地智能体/frontend/components/local-agent/today-plan-history-card.tsx`
2. `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`

## 状态设计

在 `PlanHistoryItem` 上新增：

1. `reconciled: boolean`
2. `reconcileText: string`

## 核心判断

1. 若计划已 `closed` 且晚间证据已提交，则最近历史标记为已对账。
2. 若计划已 `closed` 但晚间证据未提交，则历史显示“待对账”。
3. `taken_over / completed / closed` 的最近历史仍按同一目标日覆盖更新。

## 热点治理

1. 外提 evidence draft / submit / key-evidence update 簇。
2. 外提 execution done / close / day-reset 簇。
3. `store.ts` 只保留装配和分发，不再承载整段派生细节。

## 回退方式

删除两个新 flow 规则文件，并恢复最近历史细化字段。
