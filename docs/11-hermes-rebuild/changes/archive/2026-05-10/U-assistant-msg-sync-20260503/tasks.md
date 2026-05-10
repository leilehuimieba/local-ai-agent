# 任务清单

- [x] 读取当前 store.ts / api.ts / router_sessions.go / store.go 确认现状
- [ ] 修改 `applyEvent`：在 run_finished/completion/run_failed/error 时同步 assistant 消息
- [ ] 修改 `cancelRun`：截断 streaming assistant 时同步部分消息
- [ ] TypeScript 类型检查
- [ ] 构建验证
- [ ] 更新 status.md / verify.md
