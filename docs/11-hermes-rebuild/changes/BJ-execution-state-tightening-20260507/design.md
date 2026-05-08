# 设计说明

## 影响范围

### 文档

1. `D:/newwork/本地智能体/docs/11-hermes-rebuild/current-state.md`
2. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/INDEX.md`
3. `D:/newwork/本地智能体/docs/11-hermes-rebuild/changes/BJ-execution-state-tightening-20260507/*`

### 前端状态

1. `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`
2. `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
3. `D:/newwork/本地智能体/frontend/lib/local-agent/mainline-entry-rules.ts`
4. `D:/newwork/本地智能体/frontend/lib/local-agent/followthrough-rules.ts`
5. `D:/newwork/本地智能体/frontend/lib/local-agent/execution-state-rules.ts`

### 前端展示

1. `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
2. `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`

## 状态设计

新增 `execution` 状态块：

1. `status`：`idle / executing / completed / closed`
2. `currentTaskLabel`：当前唯一核心任务
3. `todayCoreTaskCompleted`
4. `todayClosed`
5. `statusText`
6. `helperText`

## 状态流转

1. 初始：`idle`
2. 时间预算接管后：`executing`
3. 标记今天核心任务完成后：`completed`
4. 进入今天收尾后：`closed`

## UI 设计

1. 新增“执行态”卡片，明确展示当前执行状态与当前核心任务。
2. 主按钮区根据 `buildMainlineEntryRecommendation`：
   - 当前动作高亮
   - 非当前动作弱化
3. 当当前动作不是现有固定按钮之一时，插入一条执行态动作按钮：
   - `标记今天核心任务完成`
   - `进入今天收尾`
   - `今天已收尾`

## 风险与回退

1. 风险：执行态优先级过高，可能压住补证动作。
2. 控制方式：只在用户已经明确提交“时间预算接管”后才进入执行态。
3. 回退方式：删除 `execution-state-rules.ts` 与执行态相关字段，恢复 `BI` 的静态联动逻辑。
