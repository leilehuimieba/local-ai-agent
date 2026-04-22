# 技术方案

## 影响范围

- 涉及模块：
  1. `frontend/src/shell/AppShell.tsx`
  2. `frontend/src/workspace/ContextSidebar.tsx`
  3. `frontend/src/workspace/TopBar.tsx`
  4. `frontend/src/workspace/BottomPanel.tsx`
  5. `frontend/src/workspace/BottomLogsDrawer.tsx`
  6. `frontend/src/chat/ChatPanel.tsx`
  7. `frontend/src/events/EventTimeline.tsx`
  8. `frontend/src/logs/LogsPanel.tsx`
  9. `frontend/src/settings/*`
  10. `frontend/src/history/*`
  11. `frontend/src/resources/*`
  12. `frontend/src/ui/primitives/*`
  13. `frontend/src/styles/tokens.css`
  14. `frontend/src/styles/base.css`
  15. `frontend/src/styles/index.css`

- 涉及文档或 contract：
  1. 本次以前端信息架构和设计系统收口为主，不改后端 contract。
  2. 仅引用现有 `shared/contracts/*`，不新增协议版本。

## 方案

- 核心做法：
  1. 采用“两层收口”策略推进前端重构：
     - 第一层：Shell 收口，先统一 App Shell、Sidebar、TopBar、BottomPanel、全局 token 与 panel/card 语义。
     - 第二层：内容块收口，再逐步统一 Chat、EventTimeline、Logs、History、Settings、Resources 的模块视觉与交互。
  2. 保持当前 React + Vite + Zustand 轻栈，不引入大型组件库；成熟框架只借鉴范式，不直接迁入重依赖 UI 框架。
  3. 以 Codex 风格工作台为目标：深色、克制、高密度、任务流而非 IM 气泡、工具调用块与普通消息分层清楚、底部输入器具备“命令工作台”感。
  4. 先冻结最小 design tokens：颜色层级、文本层级、边框、圆角、状态色、间距、等宽字体与 panel 阴影规则。

## 现状模块 -> 目标模块映射

| 现状模块 | 当前职责 | 当前问题 | 目标模块形态 | 本轮优先级 |
| --- | --- | --- | --- | --- |
| `shell/AppShell.tsx` | 页面外壳 | 当前只承担简单布局容器，缺少稳定工作台骨架语义 | 升级为统一 Workbench Shell，固定导航、主任务区、底部输入/调查层槽位 | P0 |
| `workspace/ContextSidebar.tsx` | 侧向检查器 | 当前更像检查器面板，不足以承担统一导航与上下文入口 | 收口为 Workbench Sidebar，统一承载会话入口、上下文状态、风险/仓库信息 | P0 |
| `workspace/TopBar.tsx` | 顶部品牌+状态+导航 | 信息较碎，导航与上下文提示并置但语义未完全统一 | 收口为轻量 Workbench Header，强化产品标题、全局状态、模式标签和主导航 | P0 |
| `workspace/BottomPanel.tsx` | 调查层 | 已具备事件流/焦点详情雏形，但视觉语义未与主线程统一 | 作为 Investigation Drawer 保留，统一为辅助轨道而非独立风格页面 | P0 |
| `chat/ChatPanel.tsx` | 任务主线程 + 输入器 | 更接近消息流，工具执行感不足，输入区仍偏普通表单 | 改为任务流 Task Stream，用户输入、assistant 输出、状态更新、待确认卡片统一为任务模块 | P1 |
| `events/EventTimeline.tsx` | 事件时间线 | 信息清晰但与聊天/日志/状态块风格未统一 | 作为 Tool/Event Block 系统核心，统一状态标签、摘要、证据字段和选中态 | P1 |
| `logs/LogsPanel.tsx` | 历史复盘页 | 已有结构，但与主工作台视觉语言未完全收敛 | 改为 Workbench 的 Logs / Review 视图，和主任务区共享同一 panel/card 体系 | P1 |
| `settings/*` | 配置控制 | 当前更像独立设置页 | 改为统一的 Settings Workspace / 控制侧板语义 | P2 |
| `history/*` | 复盘视图 | 结构较完整，但与主线程/事件流关系未完全统一 | 收口为 Review Workspace，强调可追溯、时间线、证据卡片 | P2 |
| `resources/*` | 记忆/资源区 | 更像独立功能区 | 改为 Resources / Memory 面板，纳入统一工作台信息架构 | P2 |
| `ui/primitives/*` | 基础 UI 组件 | 已有原语，但缺少一套明确的工作台语义边界 | 收口为 workbench primitives：Panel、Section、Status、Metric、EmptyState、ToolBlock | P0 |
| `styles/tokens.css` | 设计 token | 当前为偏亮色系统，与目标深色工作台不一致 | 冻结深色 Codex 风格 token 体系，支持统一背景/边框/状态/文本语义 | P0 |

## 状态流转或调用链变化

1. 本次优先改变的是前端视图组织方式，不改变后端事件流与 contract。
2. 页面级路由/切换仍沿用当前 `home / task / logs / settings` 基本结构，但其视觉表达和模块组合方式将向统一工作台收口。
3. `BottomPanel` 继续作为“调查层”，但在工作台语义中变为辅助轨道，不再与主线程形成割裂风格。

## 风险与回退

- 主要风险：
  1. 视觉重构过大，导致当前已存在的工作区认知断裂。
  2. 一次性修改过多组件，造成局部页面回归成本升高。
  3. 为了追求 Codex 风格而引入与本项目能力不匹配的 UI 结构，反而增加维护成本。

- 回退方式：
  1. 优先按 Shell -> 内容块 的顺序渐进替换，保证每一步都可独立回滚。
  2. Design tokens 若验证不通过，可先保留旧亮色 token 分支并以 feature flag 或分文件方式并存。
  3. 内容块重构前先冻结最小截图与 walkthrough 基线，若认知成本上升则回退到上一步 shell 方案。
