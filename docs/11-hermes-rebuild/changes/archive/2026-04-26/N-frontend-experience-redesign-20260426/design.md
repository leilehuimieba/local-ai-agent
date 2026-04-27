# Design: 前端体验重设计

## 1. 聊天对话流改造（P0）

### 1.1 当前问题

`ChatPanel` 把运行时事件（恢复结论、当前动作、恢复说明、建议下一步）渲染为顶部固定卡片，用户消息气泡在右下角，AI 回复也在底部。结果是：用户打开页面先看到一堆结论卡片，往下翻才看到对话上下文，阅读顺序与发生顺序相反。

### 1.2 目标形态

参考 Codex 的呈现方式：

- **消息流按时间排序**：用户消息 → AI 思考/执行 → AI 回复，依次向下排列。
- **最新消息在底部**：页面默认滚动到底部，展示最新状态。
- **执行过程可展开**：AI 的工具调用、中间思考以"详细过程"折叠卡片形式插在 AI 回复之前，用户可选择展开或保持收起。
- **状态卡片融入流**：恢复结论、建议下一步等不再是顶部孤立卡片，而是以"系统消息"或"AI 消息附件"的形式出现在对应位置。

### 1.3 组件调整

```
ChatPanel
├── MessageStream（新增/改造）
│   ├── UserMessageBubble
│   ├── AIMessageBubble
│   │   └── ProcessCard（可折叠：思考/工具/结果）
│   └── SystemNotice（恢复结论、建议下一步等）
├── Composer（保持在底部）
└── StatusBar（精简为底部一行，不占用主消息区）
```

### 1.4 数据映射

现有 `runtime.messages`（user/assistant 角色）直接映射为对话消息。

现有 `runtime.events` 按时间戳插入到对应 assistant 消息附近：
- `assistant_answer` 之前的事件 → 归入该回复的 ProcessCard
- `verification`、`tool_call` → 展开后展示
- `confirmation_request` → 以交互卡片形式插在流中

### 1.5 滚动行为

- 新消息到达时自动平滑滚动到底部（若用户未手动向上滚动）。
- 用户手动向上滚动时，暂停自动滚动，显示"回到底部"悬浮按钮。

---

## 2. 工作历史页左侧导航面板（P1）

### 2.1 当前问题

记录页（`LogsPanel` / `HistoryTimelineSection`）左侧仅有 `task-nav-rail` 图标栏，无展开面板，导致左侧 62px 之外全部闲置，右侧时间线卡片拥挤。

### 2.2 目标形态

与任务页保持一致：点击"记录"图标后，左侧展开 `HistoryNavPanel`（与 `TaskNavPanel` 并列结构）。

### 2.3 面板内容

```
HistoryNavPanel
├── 标题区：工作历史 / 全局筛选
├── 搜索框：按任务标题/关键词搜索
├── 状态筛选：全部 / 进行中 / 完成 / 失败
├── 时间筛选：今日 / 近7天 / 近30天 / 自定义
├── 标签分组：按工具类型或任务来源分组（预留）
└── 快捷入口：最近查看的 3 条记录
```

### 2.4 组件调整

在 `workspaceViewModel.tsx` 的 `TaskLeftNav` 中，根据当前 view 动态渲染：
- `view === "task"` → 现有 `TaskNavPanel`
- `view === "logs"` → 新增 `HistoryNavPanel`
- `view === "release"` → 可保持收起或放发布快捷入口
- `view === "knowledge"` / `"settings"` → 保持收起（这些视图用 drawer 或全页）

### 2.5 数据流

`HistoryNavPanel` 的筛选状态存于本地 `useState`，不提升为全局状态。筛选结果直接过滤 `app.logs.logs` 传入 `LogsPanel`。

---

## 3. 知识库 NotebookLM + Obsidian 化（P2）

### 3.1 当前问题

`KnowledgeBasePanel` 是简单的卡片列表，只能添加/删除条目，缺乏：
- 资料源聚合管理
- 基于资料的对话能力
- 知识关联可视化

### 3.2 目标形态

重设计为三个可切换视图：

#### 视图 A：Sources（资料源）

类似 NotebookLM 的 Sources 列表：
- 左侧/顶部为资料源列表（文档标题、类型、添加时间）
- 点击资料源后右侧展示内容摘要和原文
- 支持"基于这些资料提问"的入口

#### 视图 B：Chat（基于资料对话）

- 选中一个或多个 Sources 后进入对话模式
- 对话上下文绑定到选中的资料集
- AI 回答时标注引用来源（如"根据《XX文档》第3节"）

#### 视图 C：Graph（图谱）

类似 Obsidian Graph View：
- 每个知识条目是一个节点
- 节点之间按关键词共现、手动标签、链接关系建立边
- 支持缩放、拖拽、聚焦
- 点击节点打开该知识条目的详情页/笔记页

### 3.3 技术实现

**Sources 视图**：
- 复用现有 `knowledge-base/api.ts` 获取知识列表
- 增加 `KnowledgeSourceDetail` 组件展示单条知识全文

**Chat 视图**：
- 复用 `ChatPanel` 的对话流组件
- 增加 `sourceIds` 参数传入，前端在提问时附加到请求体
- 后端现有 `/api/v1/chat` 或类似接口需支持 `context_sources`（如需新接口另提变更）

**Graph 视图**：
- 使用 `d3-force` 或 `vis-network` 做力导向图渲染（轻量优先，可先手写 canvas/svg）
- 节点数据来自知识列表，边数据来自：
  - 知识条目的 `tags` 重叠（共享标签则连边）
  - 知识内容中的 `[[笔记名]]` 双向链接语法（Obsidian 风格）
- 交互：点击节点 → 打开详情面板；双击节点 → 进入 Source 视图该条目

### 3.4 数据模型扩展（前端）

```ts
type KnowledgeItem = {
  id: string;
  title: string;
  content: string;
  tags: string[];
  links: string[]; // 从 [[笔记名]] 解析出的双向链接
  sourceType: "upload" | "manual" | "memory";
  createdAt: string;
};

type GraphNode = {
  id: string;
  label: string;
  radius: number; // 基于内容长度或链接数
  color: string;  // 基于 tag 分类色
};

type GraphEdge = {
  source: string;
  target: string;
  strength: number; // 共享标签数或链接权重
};
```

### 3.5 界面布局

```
KnowledgeBasePanel
├── ViewToggle（Sources | Chat | Graph）
├── SourceView（默认）
│   ├── SourceListSidebar
│   └── SourceDetailPanel
├── ChatView
│   ├── ActiveSourcesBar（显示当前绑定的 Sources）
│   └── ChatStream（复用 MessageStream）
└── GraphView
    ├── GraphCanvas（全页或主区）
    └── NodeDetailDrawer（点击节点后从右侧滑出）
```

---

## 4. 通用设计原则

1. **左侧面板一致性**：任务页、记录页、知识库（SourceView）的左侧列表面板保持相似的 padding、宽度（270px）、背景色、滚动行为。
2. **对话流组件复用**：任务页、知识库 Chat 视图共用同一套 `MessageStream` + `ProcessCard` 组件。
3. **渐进加载**：Graph 视图节点数过多时（>200）启用采样或层级聚类，避免性能崩溃。
4. **移动端降级**：Graph 视图在窄屏下改为列表+缩略图，不渲染完整力导向图。
