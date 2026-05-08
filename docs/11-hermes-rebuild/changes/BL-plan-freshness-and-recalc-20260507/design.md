# 设计说明

## 影响范围

### 文档

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
3. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/BL-plan-freshness-and-recalc-20260507/*`

### 前端状态

1. `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`
2. `D:/newwork/本地智能体/frontend/lib/local-agent/next-day-plan-rules.ts`
3. `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`

### 前端展示

1. `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
2. `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`

## 状态设计

在 `nextDayPlan` 上新增：

1. `targetDate`
2. `basedOnEvidenceDate`
3. `needsRefresh`
4. `statusText`
5. `refreshReason`

## 核心判断

1. 生成计划时，记录：
   - 目标日期 = 明天
   - 依据证据日期 = 当前关键证据日期
2. 刷新主壳时同步校验：
   - 计划目标日是否已过
   - 是否已到目标日但今天还没重开主线
   - 关键证据是否已更新
   - 原计划依据是否已过期

## 回退方式

删除新增计划状态字段与 `syncNextDayPlan`，恢复原来的静态计划展示即可。
