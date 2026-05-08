# 验证记录

## 验证方式

- 单元测试：
  - 当前推荐动作高亮、非当前动作弱化
  - 时间预算接管后进入执行态
  - 标记核心任务完成后切到收尾动作
  - 今天收尾后停止继续强推其他动作
- 人工验证：
  - 主壳可见“执行态”卡
  - 执行态动作可在主按钮区单独出现

## 证据位置

- 测试记录：
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 关键实现：
  - `D:/newwork/本地智能体/frontend/lib/local-agent/execution-state-rules.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/mainline-entry-rules.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
- 命令输出：
  - `npm test -- mainline-shell`
  - `npm test`

## Gate 映射

- 对应自由迭代期目标：
  - 把主线总控 Agent 从“会推荐下一步”推进到“知道今天是否正在执行、是否已经完成、是否应该收尾”
- 当前覆盖情况：
  - 已覆盖执行态状态结构
  - 已覆盖当前动作高亮与非当前动作弱化
  - 已覆盖今日核心任务完成态与今日收尾态
