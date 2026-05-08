# 验证记录

## 验证方式

- 单元测试：
  - 计划接管 / 完成 / 收尾后会写入最近计划历史
  - 主壳可展示最近计划历史摘要
- 人工验证：
  - 主壳中可回看最近 3 条计划状态

## 证据位置

- 测试记录：
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 关键实现：
  - `D:/newwork/本地智能体/frontend/lib/local-agent/plan-history-rules.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
  - `D:/newwork/本地智能体/frontend/components/local-agent/today-plan-history-card.tsx`
- 命令输出：
  - `npm test -- mainline-shell` → `28 passed`
  - `npm test` → `54 passed`

## Gate 映射

- 对应自由迭代期目标：
  - 让计划从当天执行对象继续升级为可回看、可对账的最近历史闭环
