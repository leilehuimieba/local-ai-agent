# Wave 3 最小入口

## 目标

Wave 3 不再处理主线程和调查层，而是把附属工作区纳入统一工作台语言。

本波次目标是：

1. 让 `Logs / History / Settings / Resources` 看起来属于同一产品，而不是四套页面系统。
2. 复用 Wave 1 与 Wave 2 已冻结的 shell / panel / card / status / primitives 语言。
3. 保持“收口而非推翻重写”的策略，不改后端 contract，不引入大型 UI 框架。

## 范围

### 优先进入

1. `D:/newwork/本地智能体/frontend/src/logs/LogsPanel.tsx`
2. `D:/newwork/本地智能体/frontend/src/history/*`
3. `D:/newwork/本地智能体/frontend/src/settings/*`
4. `D:/newwork/本地智能体/frontend/src/resources/*`

### 本波次不进入

1. `D:/newwork/本地智能体/frontend/src/chat/ChatPanel.tsx`
2. `D:/newwork/本地智能体/frontend/src/events/EventTimeline.tsx`
3. `D:/newwork/本地智能体/frontend/src/shell/*`

以上文件只允许做必要的复用接线，不再继续重排。

## 建议顺序

### 1. LogsPanel.tsx

优先级：**P1**

原因：

1. Logs 与当前任务流、事件流关系最近
2. 最容易复用 Wave 2 的卡片与状态语言
3. 适合作为附属工作区收口的第一块试点

目标：

1. 改成 `Workbench Logs / Review` 视图
2. 共享 `panel / card / status / tags` 语言
3. 让日志页与当前任务页看起来属于同一产品

### 2. history/*

优先级：**P1**

原因：

1. History 与调查层天然相邻
2. 可直接复用 timeline / review / spotlight 的既有语言

目标：

1. 收口为 `Review Workspace`
2. 与 Chat / Events / Logs 构成同一条工作流闭环

### 3. settings/*

优先级：**P2**

原因：

1. 当前功能完整，但更像传统设置页
2. 风险低，适合放在 Logs / History 之后统一语义

目标：

1. 保留配置能力
2. 收口为 `Settings Workspace`
3. 与工作台其他区块共享相同面板/状态表达

### 4. resources/*

优先级：**P2**

原因：

1. 独立功能感较强
2. 适合放在最后纳入统一语言

目标：

1. 收口为 `Resources / Memory Workspace`
2. 保持独立功能，但进入同一设计系统

## 最小实施边界

Wave 3 只做下面三类动作：

1. 统一页面头部、区块标题、面板层级
2. 统一状态标签、空状态、卡片样式
3. 复用 `ui/primitives/*` 与已冻结的深色 token

不做：

1. 不重写数据逻辑
2. 不重排复杂业务状态
3. 不引入新的大型抽象层
4. 不新增大型依赖

## 最小验收

Wave 3 第一阶段完成后，至少应满足：

1. Logs 与 History 的页面壳层和卡片语言与任务页一致
2. Settings 与 Resources 不再显得像外部页面拼接
3. 新用户能明显感知这些页面都属于同一工作台产品
4. `verify.md` 中至少新增一次多视图一致性检查记录

## 建议入口动作

如果下一步开始 Wave 3，建议先做：

1. 输出 `Wave 3 文件级实施计划`
2. 从 `LogsPanel.tsx` 开第一轮改造
3. 再进入 `history/*`
4. 最后处理 `settings/*` 与 `resources/*`
