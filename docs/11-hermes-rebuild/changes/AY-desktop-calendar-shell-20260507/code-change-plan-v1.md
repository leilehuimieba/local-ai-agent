# AY 第一轮代码级改动点清单

## 目标

在不直接开写全部实现的前提下，把 `AY-desktop-calendar-shell-20260507` 第一轮需要改动的具体文件、各自职责、边界与顺序写清楚，作为后续真正进入代码改动时的准执行说明。

## 总体策略

1. 第一轮只做“壳”和“入口”。
2. 第一轮不把复杂逻辑塞进组件内部。
3. 第一轮优先新增组件与轻量状态，尽量少动已有重组件。
4. 第一轮只接占位数据或最小静态状态，不提前耦合后续 BA / BB 等 change。

## 建议改动文件

### 1. `frontend/lib/local-agent/types.ts`

#### 改动目标

新增 AY 第一轮所需的轻量类型定义。

#### 建议新增内容

1. 主线总控壳状态类型
2. 概率展示状态类型
3. 风险等级类型

#### 第一轮建议最小类型

- `MainlineRiskLevel = "green" | "yellow" | "orange" | "red"`
- `MainlineProbabilityState = "known" | "unknown"`
- `MainlineShellState`
  - `currentGoalLabel`
  - `probabilityValue`
  - `probabilityState`
  - `riskLevel`
  - `expanded`
  - `evidenceExpired`

#### 不做

1. 不在这里引入复杂概率算法类型
2. 不提前引入完整证据包 schema

### 2. `frontend/lib/local-agent/store.ts`

#### 改动目标

为 AY 增加轻量主线总控壳状态与基础 action。

#### 建议新增内容

1. `mainlineShell` 初始状态
2. `setMainlineExpanded`
3. `setMainlineGoal`
4. `setMainlineProbability`
5. `markMainlineProbabilityUnknown`
6. `setMainlineRiskLevel`

#### 设计要求

1. 状态必须轻量
2. 不和聊天运行态硬耦合
3. 不在第一轮接真实引擎数据时，也能先跑界面壳

#### 不做

1. 不在 AY 内把 48 小时规则完整自动化
2. 不在 AY 内把模考录入逻辑写进 store

### 3. `frontend/components/local-agent/mainline-shell.tsx`

#### 改动目标

新增 AY 第一轮核心组件，承担桌面日历插件常驻壳。

#### 职责

1. 渲染收缩态
2. 渲染展开态
3. 展示当前主目标与概率 / 未知状态
4. 展示风险颜色
5. 提供展开 / 收起动作
6. 提供两个入口：
   - 晚间证据包入口
   - 补关键证据入口

#### 第一轮 UI 要求

1. 先用现有 `Card` 体系快速搭建
2. 默认浮动小卡片
3. 样式尽量不侵入主内容布局

#### 不做

1. 不直接承载聊天内容
2. 不在第一轮接入复杂图表
3. 不在第一轮塞入大量历史状态

### 4. `frontend/app/page.tsx`

#### 改动目标

把 `MainlineShell` 挂到全局主壳中。

#### 挂载建议

1. 保持 `LeftSidebar / TopBar / 主内容 / RightDrawer` 结构不破坏
2. 新增 `MainlineShell` 作为跨视图浮动组件
3. 位置应覆盖 `task / logs / knowledge / settings`

#### 原则

1. 不改动现有 activeView 切换逻辑
2. 不把主线总控壳限定在 `task` 视图

### 5. `frontend/app/globals.css`

#### 改动目标

补充 AY 所需的最小全局样式支持。

#### 建议内容

1. 浮动卡片定位类
2. 收缩态 / 展开态基础尺寸
3. 风险颜色轻量映射类
4. 低打扰阴影与层级

#### 原则

1. 新增样式应尽量局部命名
2. 不污染现有大面积布局

### 6. `frontend/components/local-agent/top-bar.tsx`

#### 改动目标

仅在需要时提供主线总控壳的辅助触发位。

#### 第一轮建议

1. 默认可以不改
2. 若浮动卡片需要额外显式入口，再考虑补一个轻量按钮

#### 原则

不要因为 AY 第一刀就把 `TopBar` 变成新的状态面板。

## 第一轮建议不改的文件

### `frontend/components/local-agent/views/task-view.tsx`

原因：

1. 当前过重
2. 职责偏聊天与运行态
3. 不适合作为全局常驻壳主挂载点

### `frontend/components/local-agent/right-drawer.tsx`

原因：

1. 当前只在 task 视图启用
2. 语义是任务右侧抽屉，不是跨视图总控插件

### `frontend/components/local-agent/left-sidebar.tsx`

原因：

1. 当前是导航组件
2. 不适合承载持续状态卡

## 第一轮实现顺序建议

1. 在 `types.ts` 定义最小主线壳类型
2. 在 `store.ts` 新增轻量主线壳状态
3. 新建 `mainline-shell.tsx`
4. 在 `page.tsx` 挂入 `MainlineShell`
5. 在 `globals.css` 补最小定位与状态样式
6. 跑最小页面验证

## 第一轮验收目标

完成后，至少应能证明：

1. 页面上存在一个跨视图可见的主线总控壳
2. 收缩态能显示“当前主目标 + 概率 / 未知状态”
3. 展开态能进入晚间证据入口与补关键证据入口
4. 整体没有强耦合到聊天主视图
5. 常驻壳不会破坏原有首页结构
