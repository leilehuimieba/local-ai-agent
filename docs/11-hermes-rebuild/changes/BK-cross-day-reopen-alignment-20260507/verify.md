# 验证记录

## 验证方式

- 单元测试：
  - 昨天已收尾跨天后自动重开
  - 昨天未收尾跨天后不直接续跑
  - 补交窗口超时但仍需补昨天关键判断时，主按钮优先补关键证据
- 人工验证：
  - 执行态卡可见日期归属
  - 跨天后出现“请先重开今天主线”的提醒

## 证据位置

- 测试记录：
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 关键实现：
  - `D:/newwork/本地智能体/frontend/lib/local-agent/execution-state-rules.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/mainline-entry-rules.ts`
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
- 命令输出：
  - `npm test -- mainline-shell`
  - `npm test`

## Gate 映射

- 对应自由迭代期目标：
  - 让主线总控 Agent 在跨天后不延续过期执行态，并能把今天主线重新拉起
- 当前覆盖情况：
  - 已覆盖执行态跨天复位
  - 已覆盖已收尾 / 未收尾两类次日重开分流
  - 已覆盖与晚间补交流程的日期边界对齐
