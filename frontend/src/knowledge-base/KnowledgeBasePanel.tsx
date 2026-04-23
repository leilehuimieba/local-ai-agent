import { useEffect, useMemo, useState } from "react";
import { KnowledgeItem, KnowledgeFilter } from "./types";
import { knowledgeStore } from "./store";
import { KnowledgeItemCard } from "./KnowledgeItemCard";
import { KnowledgeItemDetail } from "./KnowledgeItemDetail";
import { AddItemModal } from "./AddItemModal";

type AskResult = {
  answer: string;
  sources: KnowledgeItem[];
};

export function KnowledgeBasePanel() {
  const [items, setItems] = useState<KnowledgeItem[]>([]);
  const [categories, setCategories] = useState<string[]>(["全部"]);
  const [allTags, setAllTags] = useState<string[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [filter, setFilter] = useState<KnowledgeFilter>({
    category: "全部",
    tag: "",
    search: "",
    sortBy: "updated",
  });
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [showAdd, setShowAdd] = useState(false);
  const [askQuestion, setAskQuestion] = useState("");
  const [askLoading, setAskLoading] = useState(false);
  const [askResult, setAskResult] = useState<AskResult | null>(null);
  const [askError, setAskError] = useState<string | null>(null);

  const loadData = async () => {
    try {
      setLoading(true);
      setError(null);
      const [allItems, cats, tags] = await Promise.all([
        knowledgeStore.getAll(),
        knowledgeStore.getCategories(),
        knowledgeStore.getTags(),
      ]);
      setItems(allItems);
      setCategories(cats);
      setAllTags(tags);
    } catch (err) {
      setError(err instanceof Error ? err.message : "加载失败");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadData();
  }, []);

  const filteredItems = useMemo(() => {
    let result = [...items];
    if (filter.category && filter.category !== "全部") {
      result = result.filter((i) => i.category === filter.category);
    }
    if (filter.tag) {
      result = result.filter((i) => i.tags.includes(filter.tag));
    }
    if (filter.search.trim()) {
      const q = filter.search.trim().toLowerCase();
      result = result.filter(
        (i) =>
          i.title.toLowerCase().includes(q) ||
          i.summary.toLowerCase().includes(q) ||
          i.tags.some((t) => t.toLowerCase().includes(q)),
      );
    }
    result.sort((a, b) => {
      if (filter.sortBy === "updated") return b.updatedAt.localeCompare(a.updatedAt);
      if (filter.sortBy === "created") return b.createdAt.localeCompare(a.createdAt);
      return b.citationCount - a.citationCount;
    });
    return result;
  }, [items, filter]);

  const selectedItem = useMemo(
    () => (selectedId ? items.find((i) => i.id === selectedId) ?? null : null),
    [selectedId, items],
  );

  const handleAdd = async (data: Omit<KnowledgeItem, "id" | "createdAt" | "updatedAt" | "citationCount">) => {
    await knowledgeStore.add(data);
    await loadData();
    setShowAdd(false);
  };

  const handleSave = async (id: string, patch: Partial<KnowledgeItem>) => {
    await knowledgeStore.update(id, patch);
    await loadData();
  };

  const handleDelete = async (id: string) => {
    await knowledgeStore.remove(id);
    setSelectedId(null);
    await loadData();
  };

  const handleAsk = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!askQuestion.trim()) return;
    setAskLoading(true);
    setAskError(null);
    setAskResult(null);
    try {
      const response = await fetch("/api/v1/knowledge/ask", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ question: askQuestion.trim() }),
      });
      if (!response.ok) {
        const text = await response.text();
        throw new Error(text || `HTTP ${response.status}`);
      }
      const result = (await response.json()) as AskResult;
      setAskResult(result);
    } catch (err) {
      setAskError(err instanceof Error ? err.message : "提问失败");
    } finally {
      setAskLoading(false);
    }
  };

  if (loading && items.length === 0) {
    return (
      <section className="kb-panel">
        <div className="kb-empty">
          <p>加载中...</p>
        </div>
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
        <div className="kb-toolbar-actions">
          <input
            type="search"
            className="kb-search"
            placeholder="搜索知识..."
            value={filter.search}
            onChange={(e) => setFilter((f) => ({ ...f, search: e.target.value }))}
          />
          <button type="button" className="primary-action" onClick={() => setShowAdd(true)}>
            + 添加条目
          </button>
        </div>
      </header>

      <div className="kb-filters">
        <div className="kb-filter-group">
          {categories.map((c) => (
            <button
              key={c}
              type="button"
              className={filter.category === c ? "kb-filter active" : "kb-filter"}
              onClick={() => setFilter((f) => ({ ...f, category: c }))}
            >
              {c}
            </button>
          ))}
        </div>
        <div className="kb-filter-group">
          <select
            value={filter.tag}
            onChange={(e) => setFilter((f) => ({ ...f, tag: e.target.value }))}
          >
            <option value="">全部标签</option>
            {allTags.map((t) => (
              <option key={t} value={t}>{t}</option>
            ))}
          </select>
          <select
            value={filter.sortBy}
            onChange={(e) => setFilter((f) => ({ ...f, sortBy: e.target.value as KnowledgeFilter["sortBy"] }))}
          >
            <option value="updated">最近更新</option>
            <option value="created">最近添加</option>
            <option value="cited">最常引用</option>
          </select>
        </div>
      </div>

      <div className="kb-layout">
        <div className="kb-grid">
          {filteredItems.length === 0 ? (
            <div className="kb-empty">
              <span className="kb-empty-icon">📚</span>
              <p>还没有资料</p>
              <button type="button" className="primary-action" onClick={() => setShowAdd(true)}>
                添加第一篇知识
              </button>
            </div>
          ) : (
            filteredItems.map((item) => (
              <KnowledgeItemCard
                key={item.id}
                item={item}
                onClick={() => setSelectedId(item.id)}
              />
            ))
          )}
        </div>

        {selectedItem && (
          <KnowledgeItemDetail
            item={selectedItem}
            onSave={handleSave}
            onDelete={handleDelete}
            onClose={() => setSelectedId(null)}
            categories={categories}
          />
        )}
      </div>

      <div className="kb-ask-section">
        <h3>基于这些资料提问</h3>
        <form className="kb-ask-form" onSubmit={handleAsk}>
          <input
            type="text"
            className="kb-ask-input"
            placeholder="输入问题，例如：项目的架构是什么？"
            value={askQuestion}
            onChange={(e) => setAskQuestion(e.target.value)}
          />
          <button type="submit" className="primary-action" disabled={askLoading}>
            {askLoading ? "思考中..." : "提问"}
          </button>
        </form>
        {askError && <p className="kb-error">{askError}</p>}
        {askResult && (
          <div className="kb-ask-result">
            <div className="kb-answer">
              <strong>回答</strong>
              <pre>{askResult.answer}</pre>
            </div>
            {askResult.sources.length > 0 && (
              <div className="kb-sources">
                <strong>引用来源</strong>
                <ul>
                  {askResult.sources.map((s) => (
                    <li key={s.id}>
                      <button type="button" className="kb-source-link" onClick={() => setSelectedId(s.id)}>
                        {s.title}
                      </button>
                    </li>
                  ))}
                </ul>
              </div>
            )}
          </div>
        )}
      </div>

      {showAdd && (
        <AddItemModal
          onSave={handleAdd}
          onClose={() => setShowAdd(false)}
          categories={categories}
        />
      )}
    </section>
  );
}
