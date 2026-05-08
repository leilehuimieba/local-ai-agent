# 验证记录

## 验证方式

- 单元测试：
  - 个性化记录入口可打开并提交
  - 提交后摘要卡会更新更易完成项、短期收益最高项、阻力偏高项
- 人工验证：
  - 展开主线总控壳时，默认能看到“更适合你的推进方式”摘要位
  - 点击“个性化记录”后可录入任务反馈，并在提交后看到个性化反馈 banner

## 证据位置

- 测试记录：
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 关键实现：
  - `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/personalized-followup-rules.ts`
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
- 命令输出：
  - `npm test -- mainline-shell`
  - `npm test`

## Gate 映射

- 对应自由迭代期目标：
  - 在不引入错误自信的前提下，为主线总控 Agent 增加“先通用、后个性化”的反馈闭环
- 当前覆盖情况：
  - 已覆盖状态、规则、前端可见层和定向测试
  - 已完成前端全量回归：6 个测试文件、34 个测试全部通过
  - 如需更强证据，后续可补浏览器截图与多轮任务记录回放
