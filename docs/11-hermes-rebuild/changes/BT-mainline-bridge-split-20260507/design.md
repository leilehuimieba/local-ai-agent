# 设计说明

## 影响范围

### 文档

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
3. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/BS-mainline-default-state-split-20260507/status.md`
4. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/BT-mainline-bridge-split-20260507/*`

### 前端状态

1. `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
2. `D:/newwork/本地智能体/frontend/lib/local-agent/mainline-bridge-rules.ts`

## 责任拆分

### 新文件负责

1. `applyProbabilityState(...)`
2. `buildEvidenceSubmitState(...)`
3. `buildKeyEvidenceState(...)`
4. `buildTimeBudgetSubmitState(...)`
5. `buildCoreTaskDoneState(...)`
6. `buildTodayClosedState(...)`
7. `buildNextDayPlanState(...)`
8. `buildTimeBudgetPanelOpenState(...)`

### `store.ts` 保留

1. store 创建
2. action 注册
3. 基础布局态
4. runtime/settings/knowledge/logs/memory store 组织

## 回退方式

1. 删除 `mainline-bridge-rules.ts`
2. 将桥接函数内联回 `store.ts`
