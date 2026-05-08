# 验证记录

## 验证方式

- 单元测试：
  - 表单组件可渲染、填写、提交
- 集成测试：
  - 从 `MainlineShell` 打开并提交证据包
- 人工验证：
  - 能看到入口
  - 能提交
  - 提交后状态有反馈

## 证据位置

- 测试记录：
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 日志或截图：
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`

## Gate 映射

- 对应阶段 Gate：
  - 主线总控 Agent 第一闭环：晚间证据包
- 当前覆盖情况：
  - 已完成骨架、最小实现与测试验证
  - 待补：如需更强证据，可补人工截图与补交窗口逻辑
