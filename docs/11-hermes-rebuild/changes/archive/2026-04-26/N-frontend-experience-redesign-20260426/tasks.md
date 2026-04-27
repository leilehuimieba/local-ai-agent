# Tasks

## P0 — 聊天对话流改造

- [x] N-P0-01：改造 `ChatPanel` 消息结构，按时间顺序排列 `UserMessageBubble` + `AIMessageBubble`
- [x] N-P0-02：新增 `ProcessCard` 可折叠组件，展示 AI 思考/工具调用/验证过程
- [x] N-P0-03：把运行时事件（恢复结论、当前动作、建议下一步）从顶部卡片迁移到对话流中的 `SystemNotice`
- [x] N-P0-04：实现自动滚动到底部 + 手动上滚暂停
- [x] N-P0-05：确认请求（confirmation）以交互卡片形式嵌入对话流
- [x] N-P0-06：更新 `ChatPanel.test.tsx` 断言，补充对话流顺序测试
- [x] N-P0-07：运行 TS 编译 + 测试 + 全量回归

## P1 — 工作历史页左侧导航面板

- [x] N-P1-01：新增 `HistoryNavPanel` 组件（搜索、状态筛选、时间筛选、快捷入口）
- [x] N-P1-02：改造 `TaskLeftNav`，按当前 view 动态渲染 `TaskNavPanel` / `HistoryNavPanel`
- [x] N-P1-03：`LogsPanel` 接入筛选状态，支持按状态/时间/关键词过滤
- [x] N-P1-04：现有测试保持通过（未破坏既有断言）
- [x] N-P1-05：运行 TS 编译 + 测试 + 全量回归

## P2 — 知识库 NotebookLM + Obsidian 化

- [x] N-P2-01：扩展 `KnowledgeBasePanel` 为三视图结构（Sources / Chat / Graph）
- [x] N-P2-02：Sources 视图：左侧 SourceList + 右侧 SourceDetail
- [x] N-P2-03：Chat 视图：基于知识库问答，AI 回答带引用来源标签
- [x] N-P2-04：Graph 视图：Canvas 力导向图谱，支持拖拽节点、滚轮缩放、点击跳转
- [x] N-P2-05：解析知识内容中的 `[[笔记名]]` 双向链接，结合共享标签生成 GraphEdge
- [x] N-P2-06：点击节点切换回 Sources 视图查看详情
- [x] N-P2-07：Graph 视图基于 Canvas 渲染，性能由 requestAnimationFrame 保障
- [x] N-P2-08：窄屏下图谱仍可交互，未做降级（后续可按需补充）
- [x] N-P2-09：更新 `KnowledgeBasePanel.test.tsx` 适配新 UI
- [x] N-P2-10：运行 TS 编译 + 测试 + 全量回归
