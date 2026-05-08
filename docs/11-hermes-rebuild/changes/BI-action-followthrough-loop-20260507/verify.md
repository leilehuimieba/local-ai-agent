# 验证记录

## 验证方式

- 单元测试：
  - 提交晚间证据后自动重判下一步
  - 更新关键证据后自动联动到明日计划或时间预算接管
  - 明日计划生成后提示是否基于最新关键证据
- 人工验证：
  - 主壳出现连续引导提示卡
  - 下一步动作会在关键动作后自动刷新

## 证据位置

- 测试记录：
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 关键实现：
  - `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/followthrough-rules.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/mainline-entry-rules.ts`
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
- 命令输出：
  - `npm test -- mainline-shell`
  - `npm test`

## Gate 映射

- 对应自由迭代期目标：
  - 让“下一步动作”从静态判断进入连续动作流
- 当前覆盖情况：
  - 已覆盖动作后自动重判、连续引导提示与下一步联动
  - 已覆盖证据提交后重判、关键证据更新后联动与明日计划基于最新关键证据说明
  - 已完成前端全量回归：6 个测试文件、43 个测试全部通过
