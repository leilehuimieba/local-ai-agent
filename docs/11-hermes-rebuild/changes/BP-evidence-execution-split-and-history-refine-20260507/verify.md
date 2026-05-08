# 验证记录

## 验证方式

- 单元测试：
  - 最近计划历史可显示待对账 / 已对账
  - 主壳既有计划接管 / 完成 / 收尾流程保持通过
- 人工验证：
  - 最近计划记录卡可见对账状态

## 本次执行结果

1. 定向回归：
   - 命令：`npm test -- mainline-shell`
   - 结果：`1 passed (1)`，`30 passed (30)`
2. 前端全量回归：
   - 命令：`npm test`
   - 结果：`6 passed (6)`，`56 passed (56)`

## 关键修正点

1. 补回跨天刷新后的 `followthrough` 同步，避免 day reset 后只重置 execution 但丢失连续引导。
2. `refreshMainlineEvidenceState` 追加 `syncPlanState(...)`，保证挂载刷新后同步更新：
   - 计划待重算
   - 今日接管状态
   - 计划依据已变化
3. 最近计划记录卡补充展示 `对账：待对账 / 已对账`。
4. 新增显式回归断言，单独覆盖最近计划记录的“待对账 / 已对账”展示。

## 证据位置

- 测试记录：
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 关键实现：
  - `D:/newwork/本地智能体/frontend/lib/local-agent/evidence-flow-rules.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/execution-flow-rules.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/plan-history-rules.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`

## Gate 映射

- 对应自由迭代期目标：
  - 继续压缩 store 热点，并把最近计划历史从“能看状态”推进到“能看是否已对账”
