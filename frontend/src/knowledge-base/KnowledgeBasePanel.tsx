import { useEffect, useState } from "react";
import { KnowledgeItem } from "./types";
import { knowledgeStore } from "./store";
import { SourcesView } from "./views/SourcesView";
import { ChatView } from "./views/ChatView";
import { GraphView } from "./views/GraphView";

const MOCK_ITEMS: KnowledgeItem[] = [
  {
    id: "mock-1",
    title: "项目架构概述",
    summary: "本地智能体整体架构：前端 React + Vite，后端 Go Gateway，运行时 Rust。",
    content: "系统由三部分组成：\n1. 前端（React + Vite + TypeScript）\n2. 后端 Gateway（Go 1.22+）\n3. 运行时（Rust）\n\n详见 [[Rust 运行时设计]] 和 [[Gateway API 设计]]。",
    category: "架构",
    tags: ["架构", "设计"],
    source: "docs/02-architecture",
    citationCount: 12,
    createdAt: "2026-04-01T08:00:00Z",
    updatedAt: "2026-04-20T10:00:00Z",
  },
  {
    id: "mock-2",
    title: "Rust 运行时设计",
    summary: "运行时核心负责进程隔离、事件流和状态管理。",
    content: "运行时核心采用 async Rust 实现，关键模块：\n- runtime-core：事件总线与状态机\n- runtime-host：进程隔离与沙箱\n\n与 [[Gateway API 设计]] 通过 gRPC 通信。",
    category: "架构",
    tags: ["架构", "Rust"],
    source: "docs/03-runtime",
    citationCount: 8,
    createdAt: "2026-04-02T09:00:00Z",
    updatedAt: "2026-04-18T14:00:00Z",
  },
  {
    id: "mock-3",
    title: "Gateway API 设计",
    summary: "Go Gateway 提供 REST API，负责路由、认证和代理。",
    content: "Gateway 使用标准库 net/http，关键端点：\n- /api/v1/chat\n- /api/v1/settings\n- /api/v1/knowledge/items\n\n前端状态管理详见 [[前端状态管理]]。",
    category: "API",
    tags: ["API", "Go", "架构"],
    source: "docs/04-api",
    citationCount: 5,
    createdAt: "2026-04-03T10:00:00Z",
    updatedAt: "2026-04-19T11:00:00Z",
  },
  {
    id: "mock-4",
    title: "前端状态管理",
    summary: "使用 Zustand 管理全局状态，React Context 用于主题和布局。",
    content: "状态分层：\n1. RuntimeStore（Zustand）：连接、事件、消息\n2. ViewState（useState）：当前视图、右侧面板\n3. 知识库 Store：独立管理资料、标签、分类\n\n参考 [[项目架构概述]] 的整体设计。",
    category: "前端",
    tags: ["前端", "React"],
    source: "docs/06-development",
    citationCount: 3,
    createdAt: "2026-04-04T11:00:00Z",
    updatedAt: "2026-04-21T09:00:00Z",
  },
  {
    id: "mock-5",
    title: "知识库图谱设计",
    summary: "Canvas 力导向图实现，支持双向链接和标签聚类。",
    content: "图谱视图使用手写 Canvas 2D 渲染，物理引擎模拟：\n- 节点斥力（Coulomb）\n- 连线引力（Hooke）\n- 中心引力\n\n双向链接解析 [[Obsidian 双向链接]] 的 [[笔记名]] 语法。",
    category: "设计",
    tags: ["设计", "前端"],
    source: "docs/11-hermes-rebuild",
    citationCount: 2,
    createdAt: "2026-04-22T08:00:00Z",
    updatedAt: "2026-04-25T16:00:00Z",
  },
  {
    id: "mock-6",
    title: "Obsidian 双向链接",
    summary: "借鉴 Obsidian 的 [[笔记名]] 语法，实现知识节点关联。",
    content: "双向链接规则：\n1. 正则匹配 \\[[(.+?)\\]]\n2. 匹配到的标题作为目标节点\n3. 共享标签自动生成弱连线\n\n在 [[知识库图谱设计]] 中可视化呈现。",
    category: "知识管理",
    tags: ["设计", "知识管理"],
    source: "docs/11-hermes-rebuild",
    citationCount: 1,
    createdAt: "2026-04-23T10:00:00Z",
    updatedAt: "2026-04-24T12:00:00Z",
  },
];

const MOCK_CATEGORIES = ["全部", "架构", "API", "前端", "设计", "知识管理"];
const MOCK_TAGS = ["架构", "设计", "Rust", "API", "Go", "前端", "React", "知识管理"];

type ViewTab = "sources" | "chat" | "graph";

export function KnowledgeBasePanel() {
  const [view, setView] = useState<ViewTab>("sources");
  const [items, setItems] = useState<KnowledgeItem[]>([]);
  const [categories, setCategories] = useState<string[]>(["全部"]);
  const [allTags, setAllTags] = useState<string[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [usingMock, setUsingMock] = useState(false);

  const loadData = async () => {
    try {
      setLoading(true);
      setError(null);
      setUsingMock(false);
      const [allItems, cats, tags] = await Promise.all([
        knowledgeStore.getAll(),
        knowledgeStore.getCategories(),
        knowledgeStore.getTags(),
      ]);
      setItems(allItems);
      setCategories(cats);
      setAllTags(tags);
    } catch (err) {
      if (import.meta.env.DEV) {
        setItems(MOCK_ITEMS);
        setCategories(MOCK_CATEGORIES);
        setAllTags(MOCK_TAGS);
        setUsingMock(true);
      } else {
        setError(err instanceof Error ? err.message : "加载失败");
      }
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadData();
  }, []);

  const handleAdd = async (data: Omit<KnowledgeItem, "id" | "createdAt" | "updatedAt" | "citationCount">) => {
    await knowledgeStore.add(data);
    await loadData();
  };

  const handleSave = async (id: string, patch: Partial<KnowledgeItem>) => {
    await knowledgeStore.update(id, patch);
    await loadData();
  };

  const handleDelete = async (id: string) => {
    await knowledgeStore.remove(id);
    await loadData();
  };

  if (loading && items.length === 0) {
    return (
      <section className="kb-panel">
        <div className="kb-empty"><p>加载中...</p></div>
      </section>
    );
  }

  if (error && items.length === 0) {
    return (
      <section className="kb-panel">
        <div className="kb-empty">
          <p>加载失败：{error}</p>
          <button type="button" className="secondary-button" onClick={loadData}>重试</button>
        </div>
      </section>
    );
  }

  return (
    <section className="kb-panel">
      <header className="kb-toolbar">
        <h2>知识库</h2>
        <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
          <button type="button" className="secondary-button" disabled={loading} onClick={loadData} title="刷新知识库数据">
            {loading ? "刷新中..." : "刷新"}
          </button>
          <div className="kb-view-tabs">
            <button type="button" className={view === "sources" ? "kb-tab active" : "kb-tab"} onClick={() => setView("sources")}>资料源</button>
            <button type="button" className={view === "chat" ? "kb-tab active" : "kb-tab"} onClick={() => setView("chat")}>对话</button>
            <button type="button" className={view === "graph" ? "kb-tab active" : "kb-tab"} onClick={() => setView("graph")}>图谱</button>
          </div>
        </div>
      </header>

      {usingMock ? (
        <div className="kb-mock-banner">
          <span>当前使用示例数据，请确认知识库服务可用后</span>
          <button type="button" className="kb-banner-refresh" onClick={loadData}>刷新</button>
        </div>
      ) : null}

      {!loading && items.length === 0 && !usingMock ? (
        <div className="kb-empty">
          <span className="kb-empty-icon">📚</span>
          <p>知识库还没有内容</p>
          <p className="kb-empty-hint">通过任务让智能体阅读和整理本地文档，或点击「+ 添加」手动导入资料</p>
        </div>
      ) : null}

      {view === "sources" && (
        <SourcesView
          items={items}
          categories={categories}
          allTags={allTags}
          onAdd={handleAdd}
          onSave={handleSave}
          onDelete={handleDelete}
        />
      )}
      {view === "chat" && <ChatView items={items} />}
      {view === "graph" && <GraphView items={items} onSelectItem={(id) => setView("sources")} />}
    </section>
  );
}
