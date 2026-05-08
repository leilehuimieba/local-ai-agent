# 设计说明

## 影响范围

### 文档

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
3. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/BK-cross-day-reopen-alignment-20260507/*`

### 前端状态

1. `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`
2. `D:/newwork/本地智能体/frontend/lib/local-agent/execution-state-rules.ts`
3. `D:/newwork/本地智能体/frontend/lib/local-agent/followthrough-rules.ts`
4. `D:/newwork/本地智能体/frontend/lib/local-agent/mainline-entry-rules.ts`
5. `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`

### 前端展示

1. `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
2. `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`

## 状态设计

在 `execution` 上新增：

1. `activeDate`
2. `needsReopen`
3. `staleFromDate`

## 核心流转

1. 提交时间预算后，把当天日期写入 `activeDate`
2. 刷新主壳时若发现 `activeDate !== today`，触发跨天复位
3. 复位后进入：
   - `新的一天待重开`
   - 或 `昨日未收尾，今天待重开`
4. 若晚间补交逻辑判断“昨天关键判断仍影响今天”，则主按钮优先补关键证据

## 回退方式

删除新增日期字段与跨天复位逻辑，恢复 `BJ` 的日内执行态即可。
