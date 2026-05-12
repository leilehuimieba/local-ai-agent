# 任务清单

## 任务 1：api.ts 测试

- [ ] 创建 `frontend/lib/local-agent/__tests__/api.test.ts`
- [ ] 覆盖 `fetchSessions`, `fetchSessionMessages`, `addSessionMessage`
- [ ] 覆盖 `submitChatRun`, `submitChatRetry`, `submitChatCancel`, `submitConfirmationDecision`
- [ ] 覆盖 `fetchKnowledgeItems`, `createKnowledgeItem`, `updateKnowledgeItem`, `deleteKnowledgeItem`
- [ ] 覆盖 `fetchSettings`, `updateSettings`, `fetchProviderSettings`, `updateProviderSettings`
- [ ] 覆盖 `fetchLogs`, `fetchMemories`, `deleteMemory`, `uploadKnowledgeFile`
- [ ] 覆盖辅助函数 `toKnowledgePatch`, `normalizeKnowledgeItem`, `readError`

## 任务 2：store.ts 扩展测试

- [ ] 创建 `frontend/lib/local-agent/__tests__/store-extended.test.ts`
- [ ] 覆盖 `useRuntimeStore` 剩余动作：`startNewRun`, `acceptRun`, `completeRun`, `failRun`, `clearSession`, `addEvent`
- [ ] 覆盖 `useUIStore`
- [ ] 覆盖 `useSettingsStore`
- [ ] 覆盖 `useKnowledgeStore`
- [ ] 覆盖 `useLogsStore`
- [ ] 覆盖 `useMemoryStore`
- [ ] 覆盖辅助函数：`metadataText`, `metadataList`, `providerStatus`, `toProviderSettingsItem`

## 任务 3：use-toast.ts 测试

- [ ] 创建 `frontend/hooks/__tests__/use-toast.test.ts`
- [ ] 覆盖 reducer 所有 action type
- [ ] 覆盖 TOAST_LIMIT 截断逻辑
- [ ] 覆盖 REMOVE_DELAY 队列逻辑

## 任务 4：useSessionEventStream.ts 测试

- [ ] 创建 `frontend/hooks/__tests__/useSessionEventStream.test.ts`
- [ ] 覆盖连接状态流转
- [ ] 覆盖事件解析与回调
- [ ] 覆盖重连逻辑

## 任务 5：验证与收口

- [ ] `pnpm run test --coverage` 达到 statements/lines ≥ 60%
- [ ] `pnpm run lint` 无新增 warning
- [ ] 更新 `status.md` 与 `verify.md`
