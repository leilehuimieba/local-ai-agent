# 深色 Token 草案（Wave 1）

## 目标

本草案只服务 Wave 1 的工作台壳层重构。
优先统一背景、文字、边框、状态和间距，不一次性细化所有组件 token。

## 建议 token 分层

## 1. 背景层级

```css
--bg-app: #0f1115;
--bg-app-elevated: #151922;
--bg-panel: #1a1f2b;
--bg-panel-alt: #202635;
--bg-card: #1c2330;
--bg-card-hover: #232b3a;
--bg-soft: rgba(255,255,255,0.04);
--bg-active: rgba(91,140,255,0.14);
```

用途建议：

- `--bg-app`：页面总背景
- `--bg-app-elevated`：shell 外层、顶栏或固定区域
- `--bg-panel`：侧栏、底部面板
- `--bg-card`：卡片、消息块、事件块
- `--bg-active`：选中态、当前项高亮

## 2. 文本层级

```css
--text-primary: #eef2f7;
--text-secondary: #b8c0cc;
--text-muted: #8b96a8;
--text-disabled: #657084;
--text-inverse: #0f1115;
```

用途建议：

- 一级正文、标题用 `--text-primary`
- 辅助说明用 `--text-secondary`
- 标签、描述、弱提示用 `--text-muted`

## 3. 边框层级

```css
--border-subtle: rgba(255,255,255,0.06);
--border-default: rgba(255,255,255,0.10);
--border-strong: rgba(255,255,255,0.16);
--border-accent: rgba(91,140,255,0.32);
```

用途建议：

- 面板分隔用 `subtle`
- 卡片边框用 `default`
- 选中态或聚焦态可用 `accent`

## 4. 强调色与状态色

```css
--accent-blue: #5b8cff;
--accent-blue-hover: #78a7ff;
--accent-blue-soft: rgba(91,140,255,0.14);

--state-running-bg: rgba(76,163,127,0.18);
--state-running-text: #7fe0ae;
--state-running-border: rgba(76,163,127,0.28);

--state-waiting-bg: rgba(245,165,36,0.16);
--state-waiting-text: #ffd27a;
--state-waiting-border: rgba(245,165,36,0.26);

--state-failed-bg: rgba(239,91,91,0.16);
--state-failed-text: #ff9b9b;
--state-failed-border: rgba(239,91,91,0.28);

--state-completed-bg: rgba(91,140,255,0.14);
--state-completed-text: #9fc3ff;
--state-completed-border: rgba(91,140,255,0.26);
```

## 5. 阴影与圆角

```css
--shadow-panel: 0 8px 24px rgba(0, 0, 0, 0.22);
--shadow-soft: 0 4px 12px rgba(0, 0, 0, 0.16);

--radius-sm: 8px;
--radius-md: 12px;
--radius-lg: 16px;
--radius-pill: 999px;
```

建议：

- 阴影尽量轻，更多依赖背景层级和边框
- 圆角统一，不要同时出现太多风格

## 6. 间距与字号

```css
--space-1: 4px;
--space-2: 8px;
--space-3: 12px;
--space-4: 16px;
--space-5: 20px;
--space-6: 24px;
--space-8: 32px;

--text-xs: 12px;
--text-sm: 13px;
--text-base: 14px;
--text-md: 15px;
--text-lg: 18px;
--text-xl: 24px;
```

## 与当前 token 的处理建议

1. 不建议直接删除现有 token，先做映射替换。
2. 第一轮优先替换全局背景、文字、边框和状态 badge 使用的 token。
3. 等 Wave 2 再清理旧亮色 token 残留。

## Wave 1 最小落地要求

1. `AppShell`、`TopBar`、`ContextSidebar`、`BottomPanel` 全部切到深色 token。
2. 当前页面不允许再出现大面积纯白 panel。
3. 状态 badge 统一使用深色主题下的状态色系统。
4. 一级标题、正文、辅助文案对比度要满足可读性，不要因为深色切换导致层级模糊。
