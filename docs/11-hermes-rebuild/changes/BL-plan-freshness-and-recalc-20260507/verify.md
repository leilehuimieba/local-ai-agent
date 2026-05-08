# 验证记录

## 验证方式

- 单元测试：
  - 生成明日计划后展示目标日期与依据证据日期
  - 到达计划目标日但未重开主线时，提示计划待重算
  - 关键证据更新后旧计划提示不是最新版本
- 人工验证：
  - 主壳计划卡可见计划状态与重算原因
  - 计划失效原因会进入状态 Banner

## 证据位置

- 测试记录：
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 关键实现：
  - `D:/newwork/本地智能体/frontend/lib/local-agent/next-day-plan-rules.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
- 命令输出：
  - `npm test -- mainline-shell` → `25 passed`
  - `npm test` → `51 passed`

## Gate 映射

- 对应自由迭代期目标：
  - 让计划不再是静态摘要，而是带日期归属、证据归属和失效判断的可治理对象
- 当前覆盖情况：
  - 已覆盖计划目标日期与证据日期归属
  - 已覆盖目标日到达后的计划待重算提示
  - 已覆盖关键证据更新后的旧计划失效提示
