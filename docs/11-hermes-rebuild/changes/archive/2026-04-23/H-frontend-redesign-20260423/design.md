# 技术方案

## 影响范围

- 涉及模块：
  1. `frontend/src/shell/AppShell.tsx` — 布局骨架重构
  2. `frontend/src/workspace/TopBar.tsx` — 极简顶部栏
  3. `frontend/src/workspace/ContextSidebar.tsx` — 改为左侧图标导航
  4. `frontend/src/workspace/BottomPanel.tsx` — 输入条融合主题
  5. `frontend/src/styles/tokens.css` — 新 token 体系
  6. `frontend/src/styles/base.css` — 全局样式调整
  7. `frontend/src/styles/index.css` — 布局样式重写
  8. `frontend/src/App.tsx` — 视图路由适配新布局
  9. `frontend/src/workspace/WorkbenchOverview.tsx` — 首页简化
  10. `frontend/src/chat/ChatPanel.tsx` — 任务流视觉简化
  11. `frontend/src/logs/LogsPanel.tsx` — 记录页精简
  12. `frontend/src/settings/SettingsPanel.tsx` — 设置页清晰分组

- 不涉及后端 contract。

## 方案

### 布局架构

```
┌─────────────────────────────────────────────────────────────┐
│  [●] 本地智能体                                       [≡]   │  <- TopBar
├─────┬──────────────────────────────────────────┬────────────┤
│     │                                          │            │
│ 🏠  │                                          │  [展开]    │
│ 💬  │         主内容区（单焦点）                │            │
│ 📋  │                                          │  右侧面板  │
│ ⚙️  │                                          │  (按需)    │
│     │                                          │            │
├─────┴──────────────────────────────────────────┴────────────┤
│  [输入任务，按回车发送...]                              [->]  │  <- 底部输入条
└─────────────────────────────────────────────────────────────┘
```

- **TopBar**：高度 48px，只保留品牌名和全局状态指示灯（颜色圆点）。右侧放右侧面板切换按钮。
- **左侧导航**：宽度 56px（图标-only）或 180px（图标+文字），可切换。当前项左侧 3px 蓝色指示条 + 背景高亮。
- **主内容区**：单一大块，根据当前视图渲染。大量留白，模块之间用留白而非边框分隔。
- **右侧面板**：宽度 280px，默认收起（任务页/记录页可展开）。展示关键状态摘要，3-4 个指标即可。
- **底部输入条**：高度 56px，融合深色主题，圆角输入框，蓝色发送按钮。

### 配色方案

```css
--bg-base: #1a1d29;
--bg-surface: #212536;
--bg-elevated: #2a2f42;
--bg-overlay: rgba(0,0,0,0.6);

--text-primary: #f0f2f5;
--text-secondary: #9ca3b8;
--text-muted: #5e6580;

--accent: #5b8cff;
--accent-hover: #78a7ff;
--accent-soft: rgba(91, 140, 255, 0.12);
--accent-glow: rgba(91, 140, 255, 0.25);

--status-success: #4ade80;
--status-warning: #fbbf24;
--status-error: #f87171;
--status-idle: #6b7280;
--status-running: #5b8cff;

--border-subtle: rgba(255, 255, 255, 0.05);
--border-default: rgba(255, 255, 255, 0.08);
--border-focus: rgba(91, 140, 255, 0.4);

--space-1: 4px; --space-2: 8px; --space-3: 12px;
--space-4: 16px; --space-6: 24px; --space-8: 32px; --space-12: 48px;

--radius-sm: 6px; --radius-md: 10px; --radius-lg: 16px; --radius-xl: 20px;
```

### 组件规范

#### LeftNav
- 宽度 56px（收起）/ 180px（展开）
- 背景 --bg-surface，右边框 1px --border-default
- 当前项：左侧 3px 蓝色条 + --bg-elevated 背景

#### TopBar
- 高度 48px，背景 --bg-base，底部 1px --border-subtle
- 左侧：状态指示灯（8px 圆点）+ 品牌名
- 右侧：右侧面板切换按钮

#### RightPanel
- 宽度 280px，背景 --bg-surface，左边框 1px --border-default
- 收起态完全隐藏

#### BottomInput
- 高度 56px + padding，背景 --bg-base
- 输入框：--bg-elevated 背景，无边框，--radius-lg 圆角

### 现状模块 -> 目标模块映射

| 现状模块 | 目标模块 | 优先级 |
|----------|----------|--------|
| `workspace/ContextSidebar.tsx`（右侧检查器） | `shell/LeftNav.tsx`（左侧图标导航） | P0 |
| `workspace/TopBar.tsx`（顶部品牌+状态+导航） | 极简 TopBar（仅品牌+状态灯+面板切换） | P0 |
| `shell/AppShell.tsx`（简单布局容器） | 新布局骨架（左nav + 主内容 + 右panel + 底输入） | P0 |
| `styles/tokens.css`（深色 token） | 新 token 体系（稍亮深色 + 收敛状态色） | P0 |
| `workspace/BottomPanel.tsx`（调查层） | 融合主题的输入条 + 可折叠调查层 | P1 |
| `workspace/WorkbenchOverview.tsx`（首页） | 极简首页（大标题 + 输入框 + 快速操作） | P1 |
| `chat/ChatPanel.tsx`（聊天面板） | 简洁任务流（非气泡，带边线区分） | P1 |
| `logs/LogsPanel.tsx`（日志页） | 精简记录页（单行筛选 + 时间线） | P2 |
| `settings/SettingsPanel.tsx`（设置页） | 清晰分组设置页（子导航 + 表单） | P2 |

## 状态流转

- 页面级路由不变：`home / task / logs / settings`。
- `AppShell` 承担新布局骨架，视图组件只负责主内容区渲染。
- 右侧面板展开/收起状态提升到 `AppShell` 级别。

## 风险与回退

- 主要风险：布局重构影响现有视图组件的定位。
- 回退方式：保留旧样式文件副本，若新布局不稳定可快速切回。
