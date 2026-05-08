# 验证记录

## 验证方式

- 单元测试：
  - 概率未知时主按钮优先进入关键证据录入
  - 晚间证据已提交且概率已知时主按钮优先生成明日计划
  - 补交窗口超时且裁决为“继续今天”时主按钮优先进入时间预算接管
- 人工验证：
  - 展开主壳后可见“下一步动作”摘要卡
  - 主按钮文案会随状态变化

## 证据位置

- 测试记录：
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 关键实现：
  - `D:/newwork/本地智能体/frontend/lib/local-agent/mainline-entry-rules.ts`
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
- 命令输出：
  - `npm test -- mainline-shell`
  - `npm test`

## Gate 映射

- 对应自由迭代期目标：
  - 让状态判断收口成默认下一步动作
- 当前覆盖情况：
  - 已覆盖主按钮优先级规则、主壳摘要卡与入口联动
  - 已覆盖未知优先补关键证据、已提交后优先明日计划与超时后优先保今天
  - 已完成前端全量回归：6 个测试文件、41 个测试全部通过
