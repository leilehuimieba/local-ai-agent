# 设计说明

## 影响范围

### 文档

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
3. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/BR-switch-and-personalized-split-20260507/status.md`
4. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/BS-mainline-default-state-split-20260507/*`

### 前端状态

1. `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
2. `D:/newwork/本地智能体/frontend/lib/local-agent/mainline-default-state.ts`

## 责任拆分

### 新文件负责

1. `createDefaultMainlineShell(...)`
2. `createDefaultEvidencePacket(...)`
3. `createDefaultTemporaryState(...)`
4. `createUIMainlineState(...)`

### `store.ts` 保留

1. UI store 创建
2. action 注册
3. 默认 UI 基础布局态
4. 业务桥接与提交流转

## 回退方式

1. 删除 `mainline-default-state.ts`
2. 将默认状态装配函数内联回 `store.ts`
