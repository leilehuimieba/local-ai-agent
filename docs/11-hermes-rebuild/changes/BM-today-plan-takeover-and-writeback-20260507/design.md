# 设计说明

## 影响范围

### 文档

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
3. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/BM-today-plan-takeover-and-writeback-20260507/*`

### 前端状态

1. `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`
2. `D:/newwork/本地智能体/frontend/lib/local-agent/next-day-plan-rules.ts`
3. `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`

### 前端展示

1. `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
2. `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`

## 状态设计

在 `nextDayPlan` 上新增：

1. `todayTaskLabel`
2. `takeoverStatus`
3. `takeoverHint`

## 核心判断

1. 若已进入计划目标日且计划可用，打开今天主线接管时优先承接计划核心任务。
2. 提交时间预算后，若今天的核心任务来自计划，则计划进入“已接管到今天”。
3. 标记今天核心任务完成后，计划进入“计划核心任务已完成”。
4. 今天收尾后，计划进入“今日已按计划收尾”。

## 回退方式

删除计划的接管与回写字段，恢复为仅展示日期和证据归属的静态计划卡。
