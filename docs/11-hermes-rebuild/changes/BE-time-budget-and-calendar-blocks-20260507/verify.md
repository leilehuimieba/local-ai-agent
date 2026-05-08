# 验证记录

## 验证方式

- 单元测试：
  - 时间预算面板可打开、录入并提交
  - 提交后主壳会更新今日核心任务、时间块列表与预算状态
- 人工验证：
  - 展开主线总控壳时，可看到“今日主线接管”摘要位
  - 当总可用时间偏低时，可见“先降任务量，保主目标最核心部分”或“只保留 1 个不可协商核心任务”的建议

## 证据位置

- 测试记录：
  - `D:/newwork/本地智能体/frontend/components/local-agent/__tests__/mainline-shell.test.tsx`
- 关键实现：
  - `D:/newwork/本地智能体/frontend/lib/local-agent/types.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`
  - `D:/newwork/本地智能体/frontend/lib/local-agent/time-budget-rules.ts`
  - `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`
- 命令输出：
  - `npm test -- mainline-shell`
  - `npm test`

## Gate 映射

- 对应自由迭代期目标：
  - 让主线总控 Agent 从纯任务督战，进入“任务 + 现实时间预算”的接管模式
- 当前覆盖情况：
  - 已覆盖状态、规则、前端可见层和定向测试
  - 已完成前端全量回归：6 个测试文件、35 个测试全部通过
  - 如需更强证据，后续可补真实日历映射、截图与多日恢复链路回放
