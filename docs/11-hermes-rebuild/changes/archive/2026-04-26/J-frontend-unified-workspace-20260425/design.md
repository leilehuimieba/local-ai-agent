# Design: 前端工作台统一与导航重组

## Phase 1 — 合并 home + task

### 类型层
- `AppView` 删除 `"home"`，默认路由 `"task"`
- `useViewState` 删除 `homeIntent` / `useHomePreview`
- `BottomNav` 删除 `"home"` 入口

### UI 层
- 新增 `IdleWorkspace` 组件：Hero 标题 + 副标题 + 4 个示例卡片
- 替换 `ChatPanel` 中原 `EmptyWorkbench`（简单空状态）
- 示例卡片点击触发 `openTaskPageWithDraft`，自动填充 Composer

### 兼容处理
- `buildHomeViewModel` / `homeModel.ts` 保留（供 confirmationBanner 等数据使用）
- `WorkbenchOverview.tsx` 保留（Phase 2 可能复用组件）

## Phase 2~5（待实施）

- Phase 2：左侧 Rail 添加全局视图切换图标（logs/settings/knowledge），删除 BottomNav
- Phase 3：时间线视觉改造
- Phase 4：设置/知识库/日志改为抽屉层，非全页切换
- Phase 5：动效、响应式、微交互 polish
