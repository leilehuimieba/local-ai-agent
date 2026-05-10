# 验证

## 代码变更

- `frontend/components/local-agent/views/logs-view.tsx`
  - 增加 `Tabs` 切换（运行记录 / 会话历史）
  - 新增会话列表加载逻辑：`fetchSessions` + `useState/useEffect`
  - 点击会话项调用 `resumeSession` + `setActiveView("task")`
  - 空状态、加载状态 UI

## 构建验证

- `npx tsc --noEmit`：通过
- `npm run build`：通过

## 逻辑验证

1. 默认展示"运行记录"（现有逻辑不受影响）
2. 切换"会话历史"时调用 `fetchSessions`
3. 点击会话恢复后跳转到任务视图
