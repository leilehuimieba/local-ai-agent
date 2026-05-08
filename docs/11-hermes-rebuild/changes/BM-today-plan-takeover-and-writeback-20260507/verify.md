# 验证记录

## 验证方式

- 单元测试：
  - 进入计划目标日后，时间预算接管会承接计划核心任务
  - 接管后计划卡显示已接管到今天
  - 核心任务完成与收尾后，计划卡显示执行回写状态
- 人工验证：
  - 主壳计划卡可见今日接管与执行状态提示

## 证据位置

- 测试记录：
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 关键实现：
  - `D:/newwork/本地智能体/frontend/lib/local-agent/next-day-plan-rules.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
- 命令输出：
  - `npm test -- mainline-shell` → `28 passed`
  - `npm test` → `54 passed`

## Gate 映射

- 对应自由迭代期目标：
  - 让明日计划不只知道“何时失效”，也能在目标日接管“今天”并被执行结果回写
