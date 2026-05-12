# 设计文档

## 测试策略

### api.ts（~657 行）

- 使用 `vi.stubGlobal('fetch', vi.fn())` 统一 mock
- 每个 exported 函数至少覆盖：成功路径、HTTP 错误路径、异常路径
- 验证请求头、请求体、URL 拼接正确性
- 验证错误消息包含预期前缀

### store.ts（~772 行）

- `useRuntimeStore`：补充 `startNewRun`、`acceptRun`、`completeRun`、`failRun`、`clearSession`、`addEvent`、`setConnectionState` 等动作
- `useUIStore`、`useSettingsStore`、`useKnowledgeStore`、`useLogsStore`、`useMemoryStore`：逐个创建独立测试，验证初始状态与核心 action
- 辅助函数 `metadataText`、`metadataList`、`providerStatus`、`toProviderSettingsItem`：纯函数直接断言

### use-toast.ts（~192 行）

- 直接 export `reducer`，按 action type 分 case 测试
- 验证 TOAST_LIMIT 截断、REMOVE_DELAY 队列、DISMISS 状态转换

### useSessionEventStream.ts（~103 行）

- mock `EventSource` 为可控类，模拟 `open`、`message`、`error` 事件
- 验证连接状态：`idle` → `connecting` → `connected` → `error` → `reconnecting`
- 验证事件解析与回调触发

## 不测试范围

- `app/page.tsx`、`app/layout.tsx` — 纯 UI 组装
- `views/*.tsx` 中的复杂 DOM 交互 — ROI 低，留待 E2E 覆盖
- `components/ui/*` — shadcn/ui 标准组件
