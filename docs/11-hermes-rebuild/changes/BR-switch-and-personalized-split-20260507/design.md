# 设计说明

## 影响范围

### 文档

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
3. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/BQ-plan-sync-and-takeover-split-20260507/status.md`
4. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/BR-switch-and-personalized-split-20260507/*`

### 前端状态

1. `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
2. `D:/newwork/本地智能体/frontend/lib/local-agent/switch-flow-rules.ts`
3. `D:/newwork/本地智能体/frontend/lib/local-agent/personalized-flow-rules.ts`

## 责任拆分

### 新文件负责

1. `switch-flow-rules.ts`
   - 临时切主线草稿
   - 复核失败 / 通过后的状态落点
   - 自动恢复原主线
2. `personalized-flow-rules.ts`
   - 个性化记录草稿
   - 提交后的反馈、历史追加与洞察回写

### `store.ts` 保留

1. action 注册
2. 默认状态装配
3. 与 evidence / execution / plan-sync 等其他簇的拼装

## 回退方式

1. 删除 `switch-flow-rules.ts`、`personalized-flow-rules.ts`
2. 将相关函数内联回 `store.ts`
