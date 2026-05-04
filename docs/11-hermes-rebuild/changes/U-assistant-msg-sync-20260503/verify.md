# 验证

## 代码变更

- `frontend/lib/local-agent/store.ts`
  - `applyEvent`：在 `run_finished` / `completion` / `run_failed` / `error` 事件处理分支中捕获 `assistantContent`，set 结束后异步调用 `addSessionMessage` 同步到后端
  - `cancelRun`：截断 streaming assistant 消息时捕获 `assistantContent`，set 结束后异步同步到后端

## 构建验证

- `npx tsc --noEmit`：通过
- `npm run build`：通过

## 逻辑验证

1. streaming 过程中不触发同步（符合预期，只在 finalize 时同步一次）
2. 用户消息继续通过 `addMessage` → `addSessionMessage` 同步，不受影响
3. 后端 `AddMessage` 已支持任意 role，无需改动
