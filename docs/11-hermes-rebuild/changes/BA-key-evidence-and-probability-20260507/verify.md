# 验证记录

## 验证方式

- 单元测试：
  - 关键证据录入面板可渲染、填写、提交
  - 完整且可信的结果会更新概率
  - 非完整考试条件结果不会直接改概率
  - 超过 48 小时后收缩态显示“未知”
- 人工验证：
  - 从 `MainlineShell` 点击“补关键证据”进入录入面板
  - 提交后先看到概率变化，再看到模块级调整建议

## 证据位置

- 测试记录：
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 关键实现：
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/mainline-rules.ts`
- 命令输出：
  - `npm test -- mainline-shell`
  - `npm test`

## Gate 映射

- 对应阶段 Gate：
  - 主线总控 Agent 第二闭环：关键证据 -> 概率更新 -> 模块级调整
- 当前覆盖情况：
  - 已完成关键证据录入、弱参考降级、48 小时未知降级与模块级建议的最小前端闭环
  - 后续如需更强证据，可补浏览器截图与真实日历时间块联动验证
