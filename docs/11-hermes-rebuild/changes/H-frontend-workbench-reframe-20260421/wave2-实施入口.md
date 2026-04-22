# Wave 2 实施入口

## 目标

Wave 2 进入内容块收口，但仍不改后端 contract 与运行逻辑。

本波次完成后，应该看到：

1. 主线程从“消息流”进一步收口成“任务流”。
2. 事件时间线成为统一的工具/事件块视觉标准源头。
3. `ui/primitives/*` 不再只是零散原语，而是稳定的 workbench primitives 起点。
4. Wave 1 的深色壳层能承接主线程、事件流、状态块和确认块。

## 波次范围

### 必改文件

1. `D:/newwork/本地智能体/frontend/src/chat/ChatPanel.tsx`
2. `D:/newwork/本地智能体/frontend/src/events/EventTimeline.tsx`
3. `D:/newwork/本地智能体/frontend/src/ui/primitives/*`

### 暂不进入

1. `D:/newwork/本地智能体/frontend/src/logs/LogsPanel.tsx`
2. `D:/newwork/本地智能体/frontend/src/history/*`
3. `D:/newwork/本地智能体/frontend/src/settings/*`
4. `D:/newwork/本地智能体/frontend/src/resources/*`

## 文件级改造目标

## 1. ChatPanel.tsx

### 当前问题

1. 当前结构虽然已经不是普通 IM 气泡，但“任务流”语义还不够稳定。
2. 工具执行、状态更新、待确认块之间的视觉边界还不够统一。
3. 输入区仍偏普通表单，命令工作台感不够强。

### 目标

1. 把主线程收口成统一 Task Stream。
2. 统一用户输入块、assistant 输出块、状态块、待确认块、空状态块的样式语义。
3. 把 composer 提升为 agent composer，强化任务输入与动作区的工作台感。

## 2. EventTimeline.tsx

### 当前问题

1. 信息结构已经清楚，但还没有成为全局统一的工具块标准。
2. 状态标签、摘要、证据、失败分流、等待原因之间的层级仍可继续收口。

### 目标

1. 把 EventTimeline 收口成 Tool/Event Block System 的核心。
2. 统一时间线卡片的 header、summary、detail、tag row、selected/latest 状态。
3. 让调查层与主线程在视觉语义上形成同一产品语言。

## 3. ui/primitives/*

### 当前问题

1. 现有原语可复用，但还没有形成 workbench 级边界。
2. ChatPanel 和 EventTimeline 还缺少少量共享块可以稳定复用。

### 目标

1. 在不引入大型抽象层的前提下，补齐最小 primitives 边界。
2. 优先沉淀状态、节标题、空状态、指标块等可直接复用的基础块。
3. 保证 Wave 3 改 `Logs / History / Settings / Resources` 时能复用同一套表达。

## 风险

1. 如果同时重排 Chat 和 Timeline，容易把现有可读性打散。
2. 如果 primitives 抽象过多，会在 Wave 2 就引入不必要的新层。
3. 当前任务输入受 `加载设置失败: 502` 影响，真实 walkthrough 证据仍不完整。

## 回退策略

1. 先统一样式与块语义，不改消息解析、事件筛选、权限流逻辑。
2. primitives 只做最小抽取，避免为 Wave 3 提前搭过重结构。
3. 如主线程可读性下降，优先回退局部样式，不回退 Wave 1 壳层。

## 建议实施顺序

1. `ui/primitives/*`
2. `ChatPanel.tsx`
3. `EventTimeline.tsx`

## 最小验收

1. 主线程可明显读出“输入 -> 执行 -> 状态 -> 待确认/完成”的任务流。
2. 调查层中的事件卡与主线程块在 panel/card/status 语言上保持一致。
3. 空状态、状态 badge、节标题至少有一层 primitives 复用。
4. 至少补一组任务页主线程 + 调查层联动截图，且补一次最小 walkthrough 记录。
