# 验证记录

## 验证方式

- 单元测试：
  - 今日计划接管后会显示独立今日计划卡
  - 晚间证据提交后可看到今天计划对账摘要
- 人工验证：
  - 主壳中今日计划与明日计划分区更清晰

## 证据位置

- 测试记录：
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 关键实现：
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
  - `D:/newwork/本地智能体/frontend/components/local-agent/today-plan-card.tsx`
  - `D:/newwork/本地智能体/frontend/components/local-agent/next-day-plan-card.tsx`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
- 命令输出：
  - `npm test -- mainline-shell` → `28 passed`
  - `npm test` → `54 passed`

## Gate 映射

- 对应自由迭代期目标：
  - 让今天计划成为独立执行对象，并让晚间证据直接对账计划完成情况
