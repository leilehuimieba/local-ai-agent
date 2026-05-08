# 设计说明

## 影响范围

### 文档

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
3. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/BN-today-plan-card-and-evidence-reconciliation-20260507/*`

### 前端状态

1. `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`
2. `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`

### 前端展示

1. `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
2. `D:/newwork/本地智能体/frontend/components/local-agent/today-plan-card.tsx`
3. `D:/newwork/本地智能体/frontend/components/local-agent/next-day-plan-card.tsx`
4. `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`

## 状态设计

在 `mainlineShell` 上新增今日计划对账摘要：

1. `todayPlanStatusText`
2. `todayPlanReconcileText`

## 核心判断

1. 只有当计划已进入目标日并被今天承接后，才显示独立今日计划卡。
2. 晚间证据提交后，若今天计划已接管，则显示“是否按计划完成”的对账结果。
3. 若今天计划未完成就进入晚间固定查看窗口，则给出偏差提示。

## 热点治理

1. `mainline-shell.tsx` 已超过 600 行，本刀把“今日计划卡 / 明日计划卡”拆为独立组件。
2. 只做最小拆分，不借机重写整套主壳。

## 回退方式

删除独立今日计划卡和计划对账摘要，恢复到 BM 的内联展示结构。
