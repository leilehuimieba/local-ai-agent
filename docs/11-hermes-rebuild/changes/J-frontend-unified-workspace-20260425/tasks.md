# Tasks

## Phase 1 — 合并 home + task（已完成）

- [x] J-01 `AppView` 删除 `"home"`，默认指向 `"task"`
- [x] J-02 `useViewState` 清理 `homeIntent` 相关逻辑
- [x] J-03 `BottomNav` 删除 `"home"` 入口
- [x] J-04 `workspaceViewModel` 清理 `"home"` 渲染分支
- [x] J-05 `ChatPanel` 空闲态替换为 `IdleWorkspace`（Hero + 示例卡片）
- [x] J-06 `getChatPanelProps` 接入 `onExampleClick`
- [x] J-07 测试更新（`ChatPanel.test.tsx` 断言更新）
- [x] J-08 TS 编译 + 69 测试全绿验证

## Phase 2 — 重组导航（已完成）

- [x] J-09 左侧 Rail 添加 logs/settings/knowledge 切换入口（TaskNavRail 已包含）
- [x] J-10 删除 `BottomNav` 组件及所有引用
- [x] J-11 调整 `app-layout.css` 移除底部 Dock 样式，新增 `.app-left-nav`

## Phase 3 — 时间线视觉改造（已完成）

- [x] J-12 时间线卡片添加时间轴竖线 + 节点圆点
- [x] J-13 Tone 色彩升级（danger/warning/calm/neutral）
- [x] J-14 Selected 状态统一焦橙 glow
- [x] J-15 Latest 标识脉冲发光动画
- [x] J-16 任务页消息流 hover 抬升 + glow 效果

## Phase 4 — 抽屉层实现（已完成）

- [x] J-17 新建 `Drawer` 组件（标题栏 + 关闭按钮 + 内容区）
- [x] J-18 `AppShell` 新增 `drawer` prop
- [x] J-19 `AppLayout` 主内容区固定 task，drawer 渲染 logs/settings/knowledge
- [x] J-20 drawer 关闭即回到 task，主工作区状态不丢失

## Phase 5 — 视觉 polish（已完成）

- [x] J-21 IdleWorkspace 响应式（小屏幕 1 列卡片 + 缩小 Hero）
- [x] J-22 Drawer 关闭动画（closing 状态 + 滑出/淡出）
- [x] J-23 左侧 Rail active glow 增强
- [x] J-24 页面切换动画微调
- [x] J-25 Drawer 内面板样式适配

## 交互体验修复（已完成）

- [x] J-26 消息气泡布局：用户右上、AI 左下，对齐修复
- [x] J-27 AI 回复去模板化：无 event 时纯文本，section 按需显示
- [x] J-28 左侧导航栏延伸到底（min-height: 100% + align-self: stretch）
- [x] J-29 Drawer 日志排版改为单列
