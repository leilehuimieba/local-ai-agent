# Knowledge Base 实现设计（design）

更新时间：2026-04-23

## 数据模型

```typescript
type KnowledgeItem = {
  id: string;
  title: string;
  summary: string;
  content: string;       // Markdown
  category: string;      // 文档 / 笔记 / 代码 / 链接
  tags: string[];
  source?: string;
  citationCount: number;
  createdAt: string;
  updatedAt: string;
};
```

## 架构决策

1. **前端先行**：先实现 UI 和 localStorage 数据层，后端 API 完成后替换即可
2. **独立页面**：不与 Memory 混用，独立工作区视图
3. **最小实现**：只支持手动输入 Markdown，不支持文件上传

## 组件结构

```
frontend/src/knowledge-base/
  types.ts           # 类型定义
  store.ts           # localStorage 数据层
  KnowledgeBasePanel.tsx    # 主面板
  KnowledgeItemCard.tsx     # 卡片组件
  KnowledgeItemDetail.tsx   # 详情/编辑面板
  AddItemModal.tsx          # 添加条目模态框
```

## 导航集成

- `AppView` 扩展 `"knowledge"`
- `LeftNav` 增加知识库图标（📚）
- `TaskNavRail` 同步增加
- `workspaceViewModel` 增加 `renderKnowledgeBaseView`

## 状态管理

- 使用 React state + localStorage，不引入新依赖
- localStorage key: `knowledge-base-items`
- 默认初始化 3 条示例数据

## 样式

- 复用现有 CSS 变量和设计令牌
- 不新增外部样式库
