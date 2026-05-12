# AE-change：Frontend 测试覆盖率补充

## 背景

当前 frontend 仅有 4 个测试文件，覆盖率约 33%（statements）。关键业务逻辑模块（api.ts、store 全集、hooks）缺乏测试，回归风险高。

## 目标

将 frontend 单元测试覆盖率从 ~33% 提升到 **60%+**（statements/lines），优先覆盖纯逻辑模块。

## 范围

1. `lib/local-agent/api.ts` — 全部 fetch 包装函数、错误处理、数据规范化
2. `lib/local-agent/store.ts` — 5 个未测 store 及 runtime store 剩余动作
3. `hooks/use-toast.ts` — reducer 纯函数
4. `hooks/useSessionEventStream.ts` — EventSource 生命周期与状态转换

## 方案

- api.ts：mock `global.fetch`，按成功/失败/异常三条路径覆盖
- store.ts：利用 zustand 的 `getState()`/`setState()` 直接调用 action
- use-toast.ts：直接 import reducer，纯函数输入输出断言
- useSessionEventStream.ts：mock `EventSource` 类，验证连接状态流转

## 验收标准

1. 新增测试文件 ≥ 4 个
2. `pnpm run test --coverage` 中 statements/lines ≥ 60%
3. CI frontend-test job 保持通过
4. 不引入新的 lint warning
