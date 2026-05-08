# AY 最小实现落点建议

## 目标

基于当前仓库已有前端壳、页面结构与状态层，确定 `AY-desktop-calendar-shell-20260507` 的最小实现落点，避免在实现前继续抽象猜测“桌面日历插件常驻壳”应该接到哪里。

## 当前结构观察

### 页面总壳

- 入口文件：
  - `D:/newwork/本地智能体/frontend/app/page.tsx`
- 当前结构：
  - 左侧导航：`LeftSidebar`
  - 顶部栏：`TopBar`
  - 主内容区：按 `activeView` 切换 `TaskView / LogsView / KnowledgeView / SettingsView`
  - 右侧抽屉：`RightDrawer`

结论：

1. `page.tsx` 是最适合挂“全局常驻壳”的第一落点。
2. 如果要让主线总控插件跨视图存在，不能只挂在 `TaskView` 里。

### 顶部与侧边壳

- 顶栏：
  - `D:/newwork/本地智能体/frontend/components/local-agent/top-bar.tsx`
- 左侧栏：
  - `D:/newwork/本地智能体/frontend/components/local-agent/left-sidebar.tsx`
- 右侧抽屉：
  - `D:/newwork/本地智能体/frontend/components/local-agent/right-drawer.tsx`

结论：

1. `TopBar` 适合放“收缩态状态入口”或“展开 / 收起动作入口”。
2. `RightDrawer` 当前只在 `task` 视图启用，且承担的是任务侧栏，不适合直接复用成全局主线总控插件。
3. `LeftSidebar` 是导航壳，不适合塞“当前主目标 + 概率”这种持续状态卡。

### 当前主工作视图

- 任务视图：
  - `D:/newwork/本地智能体/frontend/components/local-agent/views/task-view.tsx`

结论：

1. `TaskView` 目前承担聊天、消息、确认、上传、搜索等大量职责。
2. 不建议把 AY 第一刀直接塞进 `TaskView` 内部，否则会把“全局常驻壳”误做成“任务页子组件”。

### 当前状态层

- 运行时与 UI store：
  - `D:/newwork/本地智能体/frontend/lib/local-agent/store.ts`

结论：

1. `store.ts` 已承载 UI / Runtime 多类状态，是 AY 新增轻量状态的自然入口。
2. AY 第一刀应尽量新增一个独立的小状态切片，避免把“主线总控壳”的 UI 状态继续揉进已有聊天运行态逻辑。

## 推荐实现落点

### 推荐新增组件

建议新增：

- `D:/newwork/本地智能体/frontend/components/local-agent/mainline-shell.tsx`

职责：

1. 渲染桌面日历插件常驻壳
2. 负责收缩态 / 展开态切换
3. 展示“当前主目标 + 安全通过概率 / 未知状态”
4. 暴露晚间证据入口与补关键证据入口

### 推荐挂载位置

建议在：

- `D:/newwork/本地智能体/frontend/app/page.tsx`

中把 `MainlineShell` 挂在：

1. `LeftSidebar` 与主内容壳之外
2. `TopBar` 与内容区结构之内或之上都可，但必须跨 `activeView`

更具体地说：

- 第一优先：作为全局浮动组件挂在页面根层
- 第二优先：作为主内容区内固定定位组件挂在 `page.tsx`

不建议：

1. 先挂到 `TaskView`
2. 先复用 `RightDrawer`
3. 先塞到 `LeftSidebar`

## 收缩态建议落点

### 推荐形式

1. 固定浮动小卡片
2. 跨所有视图可见
3. 默认低打扰，不抢主内容焦点

### 原因

这最符合 AX 中已经定义好的：

1. “桌面日历插件常驻闭环”
2. “收缩态 C 位长期显示当前主目标 + 安全通过概率”
3. “平时只显示状态，不主动打扰”

## 展开态建议落点

### 推荐形式

1. 由 `MainlineShell` 自己展开
2. 不依赖 `RightDrawer`
3. 允许未来再决定是抽屉、popover、sheet 还是固定面板

### 当前阶段建议

AY 第一刀先只把展开态做成“最小入口容器”，不追求最终交互样式定版。

## 最小状态需求

AY 第一刀建议只新增下面这些状态：

1. 当前主目标文案
2. 当前概率显示值或 `unknown`
3. 风险等级颜色
4. 插件是否展开
5. 最近关键证据是否过期

暂不要求：

1. 真正概率计算逻辑
2. 真正证据包提交逻辑
3. 真正临时切主线逻辑

这些在 AY 中只需要入口占位和状态承载。

## 对 AY 的实现结论

### 最小实现边界

1. 在 `page.tsx` 全局挂入 `MainlineShell`
2. 在 `store.ts` 增加轻量主线总控壳状态
3. 在 `mainline-shell.tsx` 内完成收缩态 / 展开态最小壳
4. 入口按钮先连到占位动作或后续 change 的入口

### 不应在 AY 内做的事

1. 不把聊天逻辑和主线总控壳强耦合
2. 不把概率计算硬写在插件组件里
3. 不把 `TaskView` 继续做成更重的总控承载页
