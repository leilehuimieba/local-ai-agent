# 设计说明

## 影响范围

### 文档

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
3. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/BP-evidence-execution-split-and-history-refine-20260507/status.md`
4. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/BQ-plan-sync-and-takeover-split-20260507/*`

### 前端状态

1. `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
2. `D:/newwork/本地智能体/frontend/lib/local-agent/plan-sync-flow-rules.ts`

## 责任拆分

### 新文件负责

1. `syncPlanState(...)`
2. 计划目标日命中后的今日任务默认承接
3. evidence / time budget / core done / close day / next day plan 提交后的计划同步收口

### `store.ts` 保留

1. action 注册
2. 与其他簇的装配
3. UI Store 暴露

## 回退方式

1. 删除 `plan-sync-flow-rules.ts`
2. 将相关函数内联回 `store.ts`
