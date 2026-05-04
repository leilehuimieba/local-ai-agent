# 设计：历史会话列表 UI

## 方案概述

在现有 `LogsView` 中增加 `Tabs`（运行记录 / 会话历史），复用已有的 `fetchSessions` API 和 `resumeSession` action。

## 修改点

### `frontend/components/local-agent/views/logs-view.tsx`

1. **引入**：`fetchSessions`、`MessageSquare`、`useRuntimeStore`、`useUIStore`
2. **状态**：`activeTab: "runs" | "sessions"`，`sessions: SessionItem[]`，`sessionsLoading`
3. **加载**：`useEffect` 中当 `activeTab === "sessions"` 时调用 `fetchSessions`
4. **UI**：
   - 顶部增加 `Tabs` 切换（运行记录 / 会话历史）
   - "会话历史"内容区展示垂直列表
   - 每个会话项显示标题 + 相对时间（如"2小时前"）
   - 点击项 → `resumeSession(session.id)` + `setActiveView("task")`
5. **空状态**："暂无会话历史"提示

## 时序

```
用户点击导航"历史"
  → 默认展示"运行记录"（现有逻辑）
用户点击"会话历史"标签
  → 调用 fetchSessions
  → 渲染会话列表
用户点击某个会话
  → resumeSession(id)
  → setActiveView("task")
  → TaskView 加载该会话的完整消息历史
```
