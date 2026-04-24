# Wave 1 文件级实施计划

## 目标

Wave 1 只把当前前端收口成稳定的深色工作台外壳，不进入 Chat / EventTimeline / Logs 的深层业务重写。
本轮重点是把已有页面骨架、视觉 token 和辅助轨道统一成同一套 workbench 语言。

## 前置约束

1. 当前主推进仍是 `H-gate-h-signoff-20260416`，本计划只服务并行 change `H-frontend-workbench-reframe-20260421`。
2. 本轮不改后端 contract，不引入大型 UI 框架，不重写事件流逻辑。
3. 尽量复用现有类名和结构，在 `tokens.css / base.css / index.css` 上完成第一层收口。
4. 实现前先补一组最小基线截图：首页、任务页、调查层展开态。

## 推荐实施顺序

1. `frontend/src/styles/tokens.css`
2. `frontend/src/styles/base.css`
3. `frontend/src/styles/index.css`
4. `frontend/src/shell/AppShell.tsx`
5. `frontend/src/workspace/TopBar.tsx`
6. `frontend/src/workspace/ContextSidebar.tsx`
7. `frontend/src/workspace/BottomPanel.tsx`

这个顺序的原因是：先冻结视觉语义，再改壳层布局，最后把顶部、侧栏、底部调查层挂到同一套语义上，能把回归面控制在最小范围。

## 文件级计划

### 1. `D:/newwork/本地智能体/frontend/src/styles/tokens.css`

#### 先改什么

1. 把当前 `light` 主导 token 收口为深色 workbench token。
2. 优先保留并重映射现有高频变量，先覆盖：
   - `--bg-canvas`
   - `--bg-panel`
   - `--bg-panel-soft`
   - `--bg-emphasis`
   - `--text-primary / secondary / tertiary`
   - `--border-subtle / default / strong`
   - `--accent-primary / hover / soft`
   - `--state-*`
3. 补一层更明确的工作台语义别名，供后续内容块波次继续复用：
   - `--bg-app`
   - `--bg-app-elevated`
   - `--bg-card`
   - `--border-accent`
   - `--shadow-panel`
   - `--radius-sm / md / lg`

#### 为什么先改它

1. 现有 `body`、`topbar`、`sidebar-card`、`bottom-panel` 都已经依赖 token，先改 token 可以以最小代码改动看到整体风格变化。
2. 若不先冻结 token，后续 Shell 和 Panel 会一边改布局一边改颜色，容易失控。

#### 完成后应看到什么

1. 全局底色从偏亮色切到深色工作台基调。
2. 状态 badge、按钮、面板边框的语义不变，但整体观感变成统一深色系统。

#### 风险 / 回退点

1. 风险：旧样式大量直接依赖亮色变量，可能出现局部对比度过低。
2. 回退：先保留现有变量名，只替换取值；不要第一轮就大面积改变量命名。

#### 最小验证

1. 打开首页和任务页，确认文字可读、分隔层级清楚。
2. 检查 `status-badge` 在 running / awaiting / failed / completed 四种状态下是否仍可分辨。

### 2. `D:/newwork/本地智能体/frontend/src/styles/base.css`

#### 先改什么

1. 清掉当前 `body` 上偏亮的渐变背景，把页面总背景改成稳定深色底。
2. 统一焦点态、过渡态与全局控件阴影，让深色下的可访问性不退化。
3. 调整 `.skip-link`、`button:focus-visible` 等全局基础交互，避免深色主题切换后出现视觉断层。

#### 为什么先改它

1. `base.css` 决定整个应用的根背景和基础交互，不先收口这里，局部 panel 再深也会被亮色底拖回去。
2. 这里改完后，后续截图才能真实反映 shell 层效果。

#### 完成后应看到什么

1. 页面不再有明显的 SaaS 亮色渐变感，而是接近终端式工作台底盘。
2. 焦点高亮、按钮交互在深色下依然清楚。

#### 风险 / 回退点

1. 风险：背景改深后，某些浅边框和弱文本会突然消失。
2. 回退：保留现有过渡规则，只替换背景与焦点强度，不在这一轮重做动效系统。

#### 最小验证

1. 键盘 Tab 导航一遍页面，确认焦点框仍清晰。
2. 检查 `body`、`#root`、主内容区之间是否还存在亮色漏底。

### 3. `D:/newwork/本地智能体/frontend/src/styles/index.css`

#### 先改什么

1. 先围绕现有高频类名做结构收口，不大规模改 class API：
   - `.app-shell`
   - `.app-shell-main`
   - `.topbar`
   - `.topbar-group`
   - `.topbar-brand`
   - `.topbar-summary`
   - `.topbar-utility`
   - `.context-sidebar`
   - `.sidebar-card`
   - `.bottom-panel`
   - `.bottom-panel-summary`
   - `.bottom-panel-body`
2. 把这些类统一为 panel / card / drawer 三层语义：
   - shell 外层：应用壳层
   - sidebar / topbar / bottom-panel：面板层
   - card / status / inline note：内容块层
3. 收敛间距、边框、圆角和阴影，让顶栏、侧栏、底部调查层共享同一套 panel 语言。

#### 为什么先改它

1. Wave 1 主要靠样式收口，不先明确这些高频类的布局和视觉规则，组件层改动不会稳定。
2. 这里是连接 TSX 结构和 token 的关键层。

#### 完成后应看到什么

1. Shell 骨架的背景、边框、留白一致。
2. Sidebar、TopBar、BottomPanel 不再像三套独立页面系统。

#### 风险 / 回退点

1. 风险：`index.css` 体量较大，连锁回归面广。
2. 回退：Wave 1 只改上述高频工作台类，不触碰 Chat / Logs / Settings 专属块样式。

#### 最小验证

1. 首页、任务页、设置页切换后，壳层风格保持一致。
2. 不展开调查层和展开调查层两种状态下，布局都没有错位。

### 4. `D:/newwork/本地智能体/frontend/src/shell/AppShell.tsx`

#### 先改什么

1. 把当前“`topbar -> overlays -> content -> bottomPanel` 线性堆叠”改成显式 workbench shell 槽位。
2. 维持现有 props contract，但在结构上补出更清楚的壳层容器，例如：
   - shell 根容器
   - header 槽位
   - body 槽位
   - main workspace 槽位
   - bottom drawer 槽位
3. 保留 `skip-link` 和 `#main-content`，避免可访问性倒退。

#### 为什么先改它

1. 它是整个前端的骨架入口，后续 Chat、EventTimeline、Logs 都要挂在这个壳层里。
2. 不先把壳层结构定下来，TopBar / Sidebar / BottomPanel 的改动只会停留在“局部美化”。

#### 完成后应看到什么

1. 任务页更像工作台，而不是普通页面内容容器。
2. 主内容区和辅助区域有明确边界，后续扩展新模块时不必再改页面主骨架。

#### 风险 / 回退点

1. 风险：结构改动会影响响应式和现有页面滚动。
2. 回退：第一轮不改 props，不改路由注入方式，只增加稳定布局包裹层。

#### 最小验证

1. 首页和任务页都能正常渲染。
2. `skip-link` 仍能正确跳到主内容区。
3. BottomPanel 展开时不会把主内容完全挤坏。

### 5. `D:/newwork/本地智能体/frontend/src/workspace/TopBar.tsx`

#### 先改什么

1. 维持现有三段式结构，但重新划分信息优先级：
   - 第一优先级：品牌 / 当前工作区 / 运行状态
   - 第二优先级：主导航
   - 第三优先级：模型、模式、连接、会话、运行编号
2. 收敛当前 `ConfigSummary` 和 `UtilityBlock` 的碎片指标，避免首屏出现过多平权信息。
3. 保留“新建任务”“前往设置调整”等动作，但改成次级控制，不与主导航争抢注意力。

#### 为什么先改它

1. 用户第一眼先看顶栏；如果顶栏优先级不稳，再好的内容块也会显得复杂。
2. 当前组件已经具备必要数据，不需要先动状态流就能收口认知层级。

#### 完成后应看到什么

1. 顶栏一眼能识别出：我在哪个工作区、系统在什么状态、可切哪些主视图。
2. 模型、模式、连接等信息保留，但退到次级层。

#### 风险 / 回退点

1. 风险：过度压缩指标后，调试信息查找成本上升。
2. 回退：不删数据，只调整展示层级；低优先级信息可以收敛成 badge / meta 组。

#### 最小验证

1. 首页、任务页、记录页、设置页的主导航都可达。
2. 运行中、等待中、失败时的顶栏状态 badge 仍清楚。

### 6. `D:/newwork/本地智能体/frontend/src/workspace/ContextSidebar.tsx`

#### 先改什么

1. 不改 `buildHubModel` 的数据整理逻辑，先只改展示层次和分组语义。
2. 把当前“检查器”心智改成“工作台侧栏”，建议收口成三层：
   - 顶部：当前位置 / 当前任务 / 主状态
   - 中部：上下文沉淀 / 仓库线索
   - 底部：风险 / 记忆 / 初始化异常
3. `variant=home` 与 `variant=task` 保持差异，但视觉骨架尽量统一，不再表现为两套不同侧栏。
4. 保留 `sidebar-card`、`inspector-card` 等类名的兼容层，避免第一轮大面积改样式挂点。

#### 为什么先改它

1. 侧栏是用户理解“当前在哪、当前任务是什么、风险在哪”的关键位置。
2. 现有组件信息其实够用，问题主要在信息组织方式，而不是缺数据。

#### 完成后应看到什么

1. 左侧区域更像稳定工作区侧栏，而不是附属检查面板。
2. 用户可以更快读出任务、上下文和风险关系。

#### 风险 / 回退点

1. 风险：如果一次改太多标题和分组，老用户可能短期找不到原位置。
2. 回退：保留原有字段和 section，只先调整排序、标题和视觉密度。

#### 最小验证

1. `home` 与 `task` 两种 variant 都能正常显示。
2. `bootstrapError`、`repoWarnings`、`docPaths` 仍能在有值时正确出现。

### 7. `D:/newwork/本地智能体/frontend/src/workspace/BottomPanel.tsx`

#### 先改什么

1. 不动 `useInvestigationFocus`、`useRunCycleProgress`、`buildInvestigationModel` 等运行逻辑。
2. 先把 BottomPanel 的语义从“独立调查页面”收口成“工作台辅助抽屉”：
   - header 更强调调查轨道身份
   - summary 更强调与当前任务主线程的关系
   - body 维持双栏：事件流 / 焦点详情
3. 统一 `bottom-panel`、`bottom-panel-summary`、`investigation-lane`、`inspection-lane` 的 panel/card 语言。

#### 为什么先改它

1. 当前底部调查层是现成优势能力，不该重写；只需要把它纳入统一工作台关系。
2. 它是 Codex 风格工作台里很重要的“辅助轨道”表达。

#### 完成后应看到什么

1. 收起时像工作台状态抽屉，展开时像与主线程相连的辅助调查区。
2. 事件流和焦点详情依旧可用，但不再像切进另一套系统。

#### 风险 / 回退点

1. 风险：抽屉高度、滚动和双栏布局在不同页面上可能互相挤压。
2. 回退：先只改视觉和容器层，不改内部列表与焦点卡片的交互行为。

#### 最小验证

1. 调查层可正常展开/收起。
2. 自动跟随与手动查看两种模式不受影响。
3. 无事件、失败、有事件三种状态都能稳定显示。

## 实施检查点

### Checkpoint A：Token 和全局基底完成

完成标准：

1. `tokens.css`、`base.css` 已切到深色工作台基调。
2. 首页和任务页没有明显亮色漏底。
3. 焦点态、按钮态、状态 badge 基本可用。

### Checkpoint B：Shell 骨架完成

完成标准：

1. `AppShell.tsx` 已形成明确工作台槽位。
2. `index.css` 中 shell / topbar / sidebar / bottom-panel 的 panel 语言已统一。
3. 顶栏、侧栏、底部调查层在任务页中不再割裂。

### Checkpoint C：Wave 1 验证通过

完成标准：

1. 已补首页、任务页、调查层展开态截图。
2. 已完成一次最小 walkthrough：输入任务 -> 查看状态 -> 展开调查层 -> 返回主区。
3. 若认知负担明显下降，再进入 Wave 2 的内容块收口。

## Wave 1 完成后的最小下一步

1. 补一版视觉对比证据到 `verify.md`。
2. 进入 `ChatPanel.tsx + EventTimeline.tsx + ui/primitives/*` 的内容块收口。
3. 如 Wave 1 中发现类名耦合过高，再决定是否在 Wave 2 引入更清晰的 workbench primitive 抽象。
