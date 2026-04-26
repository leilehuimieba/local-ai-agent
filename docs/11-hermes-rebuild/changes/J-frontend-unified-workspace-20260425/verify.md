# Verify

## Phase 1 验证证据

### 编译与测试

```bash
cd frontend
npx tsc --noEmit        # 零错误
npm test                # 24 files / 69 tests passed
```

### UI 截图

- 文件：`tmp/frontend-wave1-regression/`（如需要可补具体路径）
- 内容：任务页空闲态展示 Hero "今天想让本地智能体帮你完成什么？" + 4 个示例卡片（修改项目文件 / 执行命令并排错 / 整理项目说明 / 检索本地知识）
- 底部 Dock 在任务页已隐藏

### Phase 1 代码变更文件清单

| 文件 | 改动 |
|------|------|
| `frontend/src/App.tsx` | `AppView` 删除 `"home"`，`useViewState` 清理，`AppLayout` 调整 |
| `frontend/src/shell/BottomNav.tsx` | `ViewId` 删除 `"home"`，`NAV_ITEMS` 删除首页项 |
| `frontend/src/shell/workspaceViewModel.tsx` | 删除 `"home"` 渲染分支，`getChatPanelProps` 新增 `onExampleClick` |
| `frontend/src/chat/ChatPanel.tsx` | `EmptyWorkbench` → `IdleWorkspace`，新增 `IDLE_EXAMPLES`，`ChatPanelProps` 扩展 |
| `frontend/src/chat/ChatPanel.test.tsx` | 测试断言更新 |
| `frontend/src/styles/app-components.css` | 新增 `.idle-workspace` / `.idle-hero` / `.idle-examples` / `.idle-example-chip` 样式 |

### Phase 2 代码变更文件清单

| 文件 | 改动 |
|------|------|
| `frontend/src/shell/AppShell.tsx` | `bottomNav` → `leftNav`，新增 `.app-left-nav` 包装 |
| `frontend/src/shell/AppShell.test.tsx` | prop 名更新 |
| `frontend/src/App.tsx` | 导入 `TaskLeftNav`，传递 `leftNav`，删除 `BottomNav` 引用 |
| `frontend/src/shell/workspaceViewModel.tsx` | 导出 `TaskLeftNav`，`TaskNavPanel` 条件渲染，`TaskNavRail` 删除首页按钮，`renderTaskView` 删除 `TaskLeftNav` |
| `frontend/src/shell/BottomNav.tsx` | **已删除** |
| `frontend/src/styles/app-layout.css` | 删除所有 `.bottom-nav` 样式，`.app-layout` 移除 `padding-bottom` |
| `frontend/src/styles/app-components.css` | `.task-left-nav` 移除 `position: sticky` 和 `border-right` |

### Phase 3 代码变更文件清单

| 文件 | 改动 |
|------|------|
| `frontend/src/styles/app-views.css` | 时间线卡片全面改造：时间轴竖线、节点圆点、tone 颜色、selected glow、latest 脉冲动画、summary/detail 颜色主题化 |
| `frontend/src/styles/app-components.css` | 任务页消息流 hover 效果、用户消息/AI 消息/确认卡片 glow 增强 |

### Phase 4 代码变更文件清单

| 文件 | 改动 |
|------|------|
| `frontend/src/shell/Drawer.tsx` | **新建** 抽屉组件（标题栏 + 关闭按钮 + 内容区） |
| `frontend/src/shell/AppShell.tsx` | 新增 `drawer` prop |
| `frontend/src/shell/AppShell.test.tsx` | 补充 `drawer` prop |
| `frontend/src/App.tsx` | `AppLayout` 主内容区固定 `renderTaskView`，`drawer` 根据 `currentView` 渲染；新增 `readDrawerTitle` |
| `frontend/src/shell/workspaceViewModel.tsx` | `renderWorkspaceContent` 固定返回 task；新增 `renderDrawerContent`；导出 `renderTaskView` |
| `frontend/src/styles/app-layout.css` | 新增 `.drawer-overlay` / `.drawer-panel` / `.drawer-header` / `.drawer-body` 样式及动画 |

### Phase 5 + 交互体验修复 代码变更文件清单

| 文件 | 改动 |
|------|------|
| `frontend/src/styles/app-components.css` | IdleWorkspace 响应式、Drawer 内面板适配、task-nav-button active glow 增强、AI 消息 `margin-right: auto`、task-left-nav `min-height: 100%` |
| `frontend/src/styles/app-layout.css` | 页面切换动画微调、Drawer 关闭动画、app-left-nav `align-self: stretch` |
| `frontend/src/shell/Drawer.tsx` | 添加 `closing` 状态，关闭时先播放退出动画再卸载 |
| `frontend/src/chat/chatResultModel.ts` | `buildAssistantResult` 无 event 时返回纯文本（空 sections）；`buildAnswerSections`/`buildStatusSections` section 按需显示 |
| `frontend/src/chat/ChatPanel.tsx` | `AssistantRecord` 当 sections 为空时直接渲染纯文本，不使用 `ResultBlockStack` |
| `frontend/src/styles/app-views.css` | `.drawer-body .logs-grid` 单列适配、`.drawer-body .logs-detail-stack` 取消 sticky |
