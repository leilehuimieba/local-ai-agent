# V-change: 历史会话列表 UI

## 背景

U-change 完成后，后端 `chat_messages` 表已包含完整的 user + assistant 消息。前端已有 `fetchSessions` API 和 `resumeSession` 能力，但缺少一个可视化的入口让用户浏览和恢复历史会话。

当前 `LogsView`（导航标签"历史"）只展示运行日志（runs）。用户需要一个按会话维度浏览历史的界面。

## 目标

在"历史"视图中增加"会话历史"标签页，展示所有会话列表，支持点击恢复任意会话。

## 范围

- **前端**：`frontend/components/local-agent/views/logs-view.tsx`
- **后端**：无需修改

## 验收标准

1. "历史"视图中可切换"运行记录"和"会话历史"两个标签
2. "会话历史"列表展示会话标题和最后更新时间
3. 点击会话项可恢复该会话（调用 `resumeSession` + 切回任务视图）
4. 空状态时显示友好提示
