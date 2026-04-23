# 变更验证与摘要

## 变更目标

对本地智能体前端进行全面重新设计，统一为更简洁、更聚焦的暗色工作台风格：左侧图标导航、极简顶栏、按需右侧面板、纯色扁平化卡片。

## 变更范围

共修改 18 个文件，覆盖布局骨架、全部 4 个主视图（首页/任务/记录/设置）、设计 token 和全局样式。

| 类别 | 文件 |
|------|------|
| **Token & 样式** | `frontend/src/styles/tokens.css`、`frontend/src/styles/base.css`、`frontend/src/index.css` |
| **布局骨架** | `frontend/src/shell/AppShell.tsx`、`frontend/src/shell/LeftNav.tsx`、`frontend/src/workspace/TopBar.tsx`、`frontend/src/App.tsx` |
| **首页** | `frontend/src/workspace/WorkbenchOverview.tsx` |
| **任务页** | `frontend/src/chat/ChatPanel.tsx` |
| **记录页** | `frontend/src/logs/LogsPanel.tsx`、`frontend/src/history/components/HistoryPageSections.tsx`、`frontend/src/history/components/HistoryTimelineSection.tsx`、`frontend/src/history/components/HistoryDetailRail.tsx` |
| **设置页** | `frontend/src/settings/SettingsPanel.tsx`、`frontend/src/settings/SettingsSections.tsx`、`frontend/src/settings/StatusCard.tsx`、`frontend/src/settings/ProviderCredentialsSection.tsx`、`frontend/src/resources/components/MemoryResourcesSection.tsx` |
| **通用组件** | `frontend/src/events/EventTimeline.tsx` |

---

## 详细变更内容

### Wave 1：布局骨架 + Token 体系

| 改动 | 说明 |
|------|------|
| 新 Token 体系 | 底色 `#1a1d29`（比原来稍亮），强调色 `#5b8cff`，收敛状态色（running/awaiting/failed/completed） |
| 左侧图标导航 | 新建 `LeftNav.tsx`，4 个视图图标（首页/任务/记录/设置）+ 底部新建任务按钮，56px 宽，悬停 tooltip |
| 极简 TopBar | 仅保留品牌名 + 状态灯 + 右面板切换按钮，去掉原来的导航、指标、上下文项 |
| AppShell 重写 | 改用 flex 布局：`app-layout > app-topbar + app-body(app-left-nav + app-main + app-right-panel)` |
| App.tsx 适配 | 新增 `rightPanelOpen` 状态和 `toggleRightPanel`，支持 URL 预览参数 `?view=logs` / `?view=settings` |
| 测试更新 | `AppShell.test.tsx`、`TopBar.test.tsx` 适配新 props |

### Wave 2：首页 + 任务页简化

| 改动 | 说明 |
|------|------|
| 首页重构 | 从 6-7 个卡片简化为单焦点布局：大标题 + textarea 输入 + 示例芯片 + 环境指标 |
| Resume 状态 | 3 个操作卡片（继续任务 / 查看记录 / 新建任务） |
| Blocked 状态 | 3 个操作卡片（继续 / 查看记录 / 新建） |
| 任务页消息扁平化 | 去掉渐变背景，改为纯色 + 左边线；用户消息改为圆角气泡（蓝色边线）；assistant 消息 max-width 92% |
| 输入栏合并 | 底部输入栏融入暗色主题，悬浮圆角设计 |

### Wave 3：记录页 + 设置页简化

| 改动 | 说明 |
|------|------|
| 记录页去渐变 | `logs-workspace-hero`、`logs-filter-toolbar`、`logs-timeline-panel`、`logs-detail-rail` 全部改为纯色背景 |
| 记录页组件简化 | 去掉重复的 `HistoryLogsHeader`；简化 `HistoryFilterToolbar`（去掉"复盘筛选台" header）；标题汉化为"记录" |
| 时间线列表 | `investigation-item` 的 tone 类（danger/warning/calm）去渐变，改为纯色 + 3px 左边线 |
| 设置页去渐变 | `settings-workspace-hero`、`settings-module` 改为纯色背景 |
| 设置页汉化 | 标题/描述从英文改为"设置" / "运行环境、模型、工作区、权限与诊断。" |

### Wave 4：打磨 + 验证

| 改动 | 说明 |
|------|------|
| 空状态统一 | `empty-state` 样式简化：padding 从 18px 减到 14px，改用虚线边框 + `rgba(255,255,255,0.02)` 背景，标题字号减小 |
| 空状态文案 | 所有空状态文案精简，去掉"这里会显示..."式冗余表达 |
| 响应式适配 | `< 960px`：`app-right-panel` 隐藏、`app-content` padding 减小；`< 640px`：`app-left-nav` 缩至 48px、图标缩小 |

---

## 截图证据

| 文件 | 说明 |
|------|------|
| `evidence/wave4-home-firstuse.png` | 首页 - 首次使用 |
| `evidence/wave4-home-resume.png` | 首页 - 继续任务 |
| `evidence/wave4-home-blocked.png` | 首页 - 阻塞状态 |
| `evidence/wave4-home-confirmation.png` | 首页 - 确认状态 |
| `evidence/wave4-task.png` | 任务页（空状态） |
| `evidence/wave4-logs.png` | 记录页 |
| `evidence/wave4-settings.png` | 设置页 |

---

## 构建与测试

- **TypeScript 编译**：✅ 通过（`tsc -b` 无错误）
- **Vite 构建**：✅ 通过（生产包正常输出）
- **单元测试**：AppShell、TopBar、EmptyStateBlock 测试已通过适配

---

## 已知限制

1. **任务页消息流**：截图显示的是空状态，实际消息流效果（气泡样式、边线颜色）需在真实任务运行时验证。
2. **设置页数据**：预览模式下无后端连接，设置模块显示为加载中/空状态，实际有数据时的布局需运行时验证。
3. **右面板内容**：右侧面板（ContextSidebar）的内容未做简化，保持原样。
