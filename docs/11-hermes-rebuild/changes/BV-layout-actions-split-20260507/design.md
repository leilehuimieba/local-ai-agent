# 设计说明

## 影响范围

### 文档

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
3. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/BU-mainline-actions-split-20260507/status.md`
4. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/BV-layout-actions-split-20260507/*`

### 前端状态

1. `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
2. `D:/newwork/本地智能体/frontend/lib/local-agent/layout-actions.ts`

## 责任拆分

### 新文件负责

1. `createUILayoutActions(...)`
2. 布局面板开关与主线展开态切换
3. `setTimeBudgetPanelOpen(...)` 的布局层联动入口

### `store.ts` 保留

1. store 创建
2. 基础布局态
3. runtime/settings/knowledge/logs/memory store 组织

## 回退方式

1. 删除 `layout-actions.ts`
2. 将布局 action 装配函数内联回 `store.ts`
