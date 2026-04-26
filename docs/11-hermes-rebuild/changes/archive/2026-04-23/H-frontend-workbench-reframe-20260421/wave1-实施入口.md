# Wave 1 实施入口

## 目标

Wave 1 只做前端工作台外壳收口，不进入深业务逻辑重写。

本波次完成后，应该看到：

1. 全局主题从当前偏亮色切到深色工作台基调。
2. App Shell 不再只是“页面容器”，而是明确的工作台骨架。
3. Sidebar、TopBar、BottomPanel 的视觉与信息层级统一。
4. 主任务区、辅助调查层、全局导航的边界更清晰。
5. 后续 Chat / EventTimeline / Logs 改造可以挂在统一外壳上推进。

## 波次范围

### 必改文件

1. `D:/newwork/本地智能体/frontend/src/styles/tokens.css`
2. `D:/newwork/本地智能体/frontend/src/styles/base.css`
3. `D:/newwork/本地智能体/frontend/src/styles/index.css`
4. `D:/newwork/本地智能体/frontend/src/shell/AppShell.tsx`
5. `D:/newwork/本地智能体/frontend/src/workspace/ContextSidebar.tsx`
6. `D:/newwork/本地智能体/frontend/src/workspace/TopBar.tsx`
7. `D:/newwork/本地智能体/frontend/src/workspace/BottomPanel.tsx`

### 暂不进入

1. `D:/newwork/本地智能体/frontend/src/chat/ChatPanel.tsx`
2. `D:/newwork/本地智能体/frontend/src/events/EventTimeline.tsx`
3. `D:/newwork/本地智能体/frontend/src/logs/LogsPanel.tsx`
4. `D:/newwork/本地智能体/frontend/src/history/*`
5. `D:/newwork/本地智能体/frontend/src/settings/*`
6. `D:/newwork/本地智能体/frontend/src/resources/*`

## 文件级改造目标

## 1. AppShell.tsx

### 当前问题

1. 仅表达 `topbar / overlays / content / bottomPanel` 的线性堆叠。
2. 没有显式“工作台壳层”语义，后续模块很难自然挂载。

### 目标

1. 形成固定的 workbench shell 结构。
2. 明确主内容区与辅助区层级。
3. 为 sidebar / main content / bottom drawer 提供稳定布局槽位。

### 完成后应看到

1. 页面骨架更像桌面工作台，而不是普通单页内容区。
2. 主任务区拥有明确的最大宽度、内边距和视觉层级。

## 2. ContextSidebar.tsx

### 当前问题

1. 当前更偏“检查器卡片堆叠”，承担的信息多，但导航角色不明确。
2. 卡片语义与工作台导航区语义混用。

### 目标

1. 明确 Sidebar 角色：导航 + 上下文 + 风险摘要。
2. 保留检查器信息，但收敛成统一侧栏模块。
3. 降低信息碎片感，让用户更容易理解“当前在哪、任务是什么、风险在哪”。

### 完成后应看到

1. Sidebar 成为稳定左侧工作区，而不是附属信息板。
2. 状态卡、仓库信息、风险信息在同一视觉系统里。

## 3. TopBar.tsx

### 当前问题

1. 顶部信息过碎，品牌、模型、模式、导航、会话指标并列但优先级不清。
2. 容易让用户第一眼找不到主要控制区。

### 目标

1. 顶部只保留最关键的全局信息和主导航。
2. 强化产品标题、当前工作区、运行状态和视图切换。
3. 将低优先级指标收敛为次级信息。

### 完成后应看到

1. 顶栏一眼可识别为“全局控制条”。
2. 导航更清晰，状态更稳定，碎片指标减少。

## 4. BottomPanel.tsx

### 当前问题

1. 调查层功能完整，但视觉上更像另一套页面系统。
2. 与主线程之间缺少统一的工作台关系。

### 目标

1. 让 BottomPanel 成为 workbench 的辅助调查轨道。
2. 在视觉上与主任务区共享同一套 panel/card/status 体系。
3. 保持现有事件流与焦点详情能力，不重写内部逻辑。

### 完成后应看到

1. 展开调查层时像“工作台辅助抽屉”，而不是独立页面。
2. Header / summary / body 结构与全局设计系统一致。

## 5. styles/tokens.css / base.css / index.css

### 当前问题

1. 当前 token 明显偏亮色，和目标深色工作台冲突。
2. 背景、边框、文字、状态色还没有形成 agent workbench 语言。

### 目标

1. 冻结深色主题 token。
2. 统一 panel、card、status badge、button、empty state 的基础语义。
3. 保持现有 CSS 架构，先改 token 和全局骨架，不大规模拆 CSS。

### 完成后应看到

1. 页面整体进入深色工作台风格。
2. 组件之间更像一个产品，不再像多种风格混搭。

## 风险

1. 一次性改太多 CSS 可能影响所有页面。
2. 如果 TopBar 和 Sidebar 同时重排，容易引发旧布局错位。
3. 亮色转深色会暴露很多依赖旧颜色对比的局部组件问题。

## 回退策略

1. 先只改 token 与 shell，不改深业务内容块。
2. 每完成一个文件，保留前后截图和最小对比证据。
3. 如发现深色 token 扩散影响太大，可先保留兼容 token 分层，逐步替换。

## 建议实施顺序

1. `tokens.css`
2. `base.css` / `index.css`
3. `AppShell.tsx`
4. `TopBar.tsx`
5. `ContextSidebar.tsx`
6. `BottomPanel.tsx`

## 最小验收

1. 页面进入统一深色主题。
2. 顶栏、侧栏、主内容、调查层边界清楚。
3. 不改后端 contract，不破坏当前 view 切换。
4. 至少补一组首页/任务页截图对比证据。
