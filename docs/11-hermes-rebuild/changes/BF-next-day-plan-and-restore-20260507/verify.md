# 验证记录

## 验证方式

- 单元测试：
  - 明日计划可生成
  - 生成后主壳会更新明日核心任务、补充任务与依据说明
- 人工验证：
  - 展开主线总控壳时，可看到“明日计划”摘要位
  - 若存在恢复原主线的上下文，摘要会明确标出恢复说明

## 证据位置

- 测试记录：
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 关键实现：
  - `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/next-day-plan-rules.ts`
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
- 命令输出：
  - `npm test -- mainline-shell`
  - `npm test`

## Gate 映射

- 对应自由迭代期目标：
  - 让主线总控 Agent 从“记录与判断”走到“次日行动方案输出”
- 当前覆盖情况：
  - 已覆盖状态、规则、前端可见层和定向测试
  - 已完成前端全量回归：6 个测试文件、36 个测试全部通过
  - 如需更强证据，后续可补跨天恢复快照、固定晚间查看与补交分流回放
