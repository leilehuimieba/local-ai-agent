# 任务分解

## Wave 1：布局骨架 + Token 体系

- [x] W1-1：重写 `tokens.css` — 新配色（稍亮深色 #1a1d29、收敛状态色）
- [x] W1-2：重写 `base.css` — 去掉渐变背景，纯色底，简化全局过渡
- [x] W1-3：重写 `index.css` — 新布局样式（grid 骨架、左侧 nav、右侧面板）
- [x] W1-4：新建 `shell/LeftNav.tsx` — 图标导航（首页/任务/记录/设置）
- [x] W1-5：改写 `workspace/TopBar.tsx` — 极简版（品牌+状态灯+面板切换）
- [x] W1-6：改写 `shell/AppShell.tsx` — 新布局骨架
- [x] W1-7：改写 `App.tsx` — 适配新布局（传递 nav/panel 状态）
- [x] W1-8：构建验证 + 截图 ✅

## Wave 2：首页 + 任务页简化

- [x] W2-1：改写 `workspace/WorkbenchOverview.tsx` — 极简首页
- [x] W2-2：简化 `chat/ChatPanel.tsx` 样式（去渐变、消息扁平化）
- [x] W2-3：合并输入栏到暗色主题
- [x] W2-4：构建验证 ✅

## Wave 3：记录页 + 设置页简化

- [x] W3-1：记录页 CSS 简化（去渐变、扁平化）
- [x] W3-2：记录页组件简化（去重复 header、简化筛选栏、标题汉化）
- [x] W3-3：设置页 CSS 简化（去渐变、扁平化）
- [x] W3-4：设置页组件微调（hero 汉化）
- [x] W3-5：构建验证 + 截图 ✅

## Wave 4：打磨 + 验证

- [x] W4-1：空状态统一简化
  - [x] 简化 empty-state CSS（减小 padding、虚线边框、轻量背景）
  - [x] 简化所有空状态文案（ChatPanel、EventTimeline、HistoryTimeline、HistoryDetailRail、Settings、StatusCard、MemoryResources、ProviderCredentials）
- [x] W4-2：响应式检查
  - [x] 新增 960px 以下：app-content padding 减小、app-right-panel 隐藏
  - [x] 新增 640px 以下：app-left-nav 缩至 48px、app-content padding 进一步减小
- [x] W4-3：构建验证 + 全视图截图 ✅
