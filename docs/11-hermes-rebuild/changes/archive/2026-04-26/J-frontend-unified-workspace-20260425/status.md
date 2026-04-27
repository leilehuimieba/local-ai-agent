# J-frontend-unified-workspace-20260425（status）

更新时间：2026-04-25

## 当前状态

- Phase 1 已完成（8/8 任务）
- Phase 2 已完成（3/3 任务）
- Phase 3 已完成
- Phase 4 已完成
- Phase 5 已完成
- **全部 5 个 Phase 已收口**

## 已完成

### Phase 1 — 合并 home + task
1. 路由/类型层清理：`AppView` 删除 `"home"`，`BottomNav` 删除 `"home"` 入口，`workspaceViewModel` 清理 `"home"` 分支。
2. UI 整合：`ChatPanel` 空闲态替换为 `IdleWorkspace`，展示 Hero 标题 + 4 个示例卡片，支持点击预填充 Composer。

### Phase 2 — 重组导航
1. `TaskLeftNav` 提升到 `AppLayout` 层面始终渲染，`TaskNavPanel` 只在任务页显示。
2. `TaskNavRail` 删除"首页"按钮。
3. `BottomNav` 组件及所有引用、样式彻底删除。
4. `AppShell` 从 `bottomNav` 改为 `leftNav`。

### Phase 3 — 时间线视觉改造
1. 时间轴竖线：`.investigation-list` 添加纵向时间轴（渐变竖线 + 节点圆点）。
2. Tone 色彩升级：danger/warning/calm/neutral 节点圆点和左侧边框颜色与主题色对应。
3. Selected 状态：统一焦橙 glow，替代旧电蓝背景。
4. Latest 标识：节点圆点脉冲发光动画。
5. 任务页消息流：卡片 hover 抬升 + glow 效果。

### Phase 4 — 抽屉层实现
1. 新建 `Drawer` 组件：右侧滑出面板，含标题栏、关闭按钮、内容区。
2. `AppShell` 新增 `drawer` prop。
3. `AppLayout` 主内容区始终渲染任务页，`drawer` 渲染日志/设置/知识库。
4. 关闭 drawer 即回到任务页，主工作区状态不丢失。

### Phase 5 — 视觉 polish
1. IdleWorkspace 响应式：小屏幕下示例卡片改为 1 列，Hero 标题缩小。
2. Drawer 关闭动画：添加 `closing` 状态 + 滑出/淡出动画。
3. 左侧 Rail active 状态 glow 增强。
4. 页面切换动画微调（更 subtle 的位移）。
5. Drawer 内面板样式适配（减小 border-radius 和 padding）。

### 交互体验修复（用户反馈）
1. **消息气泡布局**：用户消息 `margin-left: auto`（右上），AI 消息 `margin-right: auto`（左下），对齐更明确。
2. **AI 回复去模板化**：`buildAssistantResult` 无 event 时返回纯文本，不拆分固定 block；有 event 时各 section 按需显示（无内容则隐藏）。
3. **左侧导航栏延伸到底**：`.task-left-nav` 添加 `min-height: 100%`，`.app-left-nav` 添加 `align-self: stretch`。
4. **Drawer 日志排版**：`.drawer-body .logs-grid` 改为单列，时间线独占 drawer 宽度。

### 验证
- TypeScript 编译零错误，69 测试全部通过。
- 截图验证：左侧导航栏延伸到底，任务页/日志页/设置页全部正常。
