# 设计：Assistant 消息同步到后端

## 方案概述

复用已有的 `addSessionMessage` API，在 assistant 消息流式输出结束时（finalize）触发一次异步写入。

## 关键决策

1. **只在 finalize 时同步**：streaming 过程中内容频繁变化，若每帧同步会产生大量请求和数据库写入。只在 `isStreaming: false` 时同步一次。
2. **fire-and-forget**：网络请求使用 `.catch(() => {})` 静默失败，不阻塞前端状态机和用户体验。
3. **后端零改动**：`AddMessage` 已支持 `role="assistant"`，无需新接口。

## 修改点

### `frontend/lib/local-agent/store.ts`

#### `applyEvent`
将 `set((state) => { ... })` 改为先声明 `syncContent` 变量，在 return 前捕获最终 assistant 内容，set 结束后异步调用 `addSessionMessage`。

触发同步的事件类型：
- `run_finished` / `completion` → role=assistant，content=final_answer
- `run_failed` / `error` → role=assistant，content=final_answer（或失败摘要）

#### `cancelRun`
如果最后一条消息是 `role="assistant"` 且 `isStreaming: true`，将其 `isStreaming` 设为 false 后，异步同步该条消息的 content 到后端。

## 时序

```
用户发送消息
  → addMessage(user) ──→ POST /api/v1/sessions/:id/messages (user)
Runtime 返回事件流
  → applyEvent(action_completed) → 创建/更新 streaming assistant 消息（内存，不入库）
  → applyEvent(verification_completed) → 更新 streaming assistant 消息（内存，不入库）
  → applyEvent(run_finished) → finalizeStreamingMessage → 内存状态更新 + 异步 POST assistant
resumeSession
  → GET /api/v1/sessions/:id/messages → 返回 user + assistant 完整历史
```
