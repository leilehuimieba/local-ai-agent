# 验证记录

## 验证方式

- 单元测试：
  - 紧急且重要事项可通过复核并切换生效
  - 未通过复核但用户坚持时可记账后切换
  - 临时主线结束后可恢复原主线优先级结构
- 人工验证：
  - 从 `MainlineShell` 点击“临时切主线”进入录入面板
  - 切换后收缩态主目标发生变化
  - 恢复后重新显示原主目标

## 证据位置

- 测试记录：
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 关键实现：
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/mainline-switch-rules.ts`
- 命令输出：
  - `npm test -- mainline-shell`
  - `npm test`

## Gate 映射

- 对应阶段 Gate：
  - 主线总控 Agent 第三闭环：临时切主线 -> 记账 / 复核 -> 自动恢复
- 当前覆盖情况：
  - 已完成临时切主线申请、复核/坚持记账、原主线自动恢复的最小前端闭环
  - 后续如需更强证据，可补真实日历时间块联动与跨天恢复验证
