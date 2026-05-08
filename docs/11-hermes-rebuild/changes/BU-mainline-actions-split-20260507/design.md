# 设计说明

## 影响范围

### 文档

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
3. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/BT-mainline-bridge-split-20260507/status.md`
4. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/BU-mainline-actions-split-20260507/*`

### 前端状态

1. `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
2. `D:/newwork/本地智能体/frontend/lib/local-agent/mainline-actions.ts`

## 责任拆分

### 新文件负责

1. `createMainlineActions(...)`
2. `createGoalActions(...)`
3. `createEvidenceActions(...)`
4. `createSwitchActions(...)`
5. `createPersonalizedActions(...)`
6. `createTimeBudgetActions(...)`
7. `createExecutionActions(...)`
8. `createNextDayPlanActions(...)`

### `store.ts` 保留

1. store 创建
2. 基础布局态
3. 布局 action
4. runtime/settings/knowledge/logs/memory store 组织

## 回退方式

1. 删除 `mainline-actions.ts`
2. 将主线 action 装配函数内联回 `store.ts`
