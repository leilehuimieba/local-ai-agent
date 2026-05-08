# 验证记录

## 验证方式

- 单元测试：
  - 晚间固定窗口状态展示
  - 次日上午“可补交”状态
  - 次日中午后“补交窗口已过”分流状态
- 人工验证：
  - 展开主线总控壳时，可看到“晚上固定查看”摘要卡
  - 晚间证据入口文案会跟随状态变化

## 证据位置

- 测试记录：
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 关键实现：
  - `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/evening-review-rules.ts`
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
- 命令输出：
  - `npm test -- mainline-shell`
  - `npm test`

## Gate 映射

- 对应自由迭代期目标：
  - 让“晚上固定查看 / 次日补交流转”从产品规则进入可执行插件状态
- 当前覆盖情况：
  - 已覆盖状态结构、规则层与前端可见层
  - 已覆盖晚间固定查看、次日上午补交与中午后分流
  - 已完成前端全量回归：6 个测试文件、38 个测试全部通过
