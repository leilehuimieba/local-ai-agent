import { useMemo, useState } from "react";
import { KnowledgeItem } from "../types";
import { useKnowledgeFilterStore } from "../filterStore";
import { KnowledgeItemCard } from "../KnowledgeItemCard";
import { KnowledgeItemDetail } from "../KnowledgeItemDetail";
import { AddItemModal } from "../AddItemModal";

export type SourcesViewProps = {
  items: KnowledgeItem[];
  categories: string[];
  allTags: string[];
  onAdd: (data: Omit<KnowledgeItem, "id" | "createdAt" | "updatedAt" | "citationCount">) => Promise<void>;
  onSave: (id: string, patch: Partial<KnowledgeItem>) => Promise<void>;
  onDelete: (id: string) => Promise<void>;
};

export function SourcesView(props: SourcesViewProps) {
  const filter = useKnowledgeFilterStore();
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [showAdd, setShowAdd] = useState(false);

  const filteredItems = useMemo(() => {
    let result = [...props.items];
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
  }, [props.items, filter]);

  const selectedItem = useMemo(
    () => (selectedId ? props.items.find((i) => i.id === selectedId) ?? null : null),
    [selectedId, props.items],
  );

  return (
    <div className="kb-sources-layout">
      <div className="kb-sources-sidebar">
        <div className="kb-sources-toolbar">
          <input
            type="search"
            className="kb-search"
            placeholder="搜索资料..."
            value={filter.search}
            onChange={(e) => filter.setSearch(e.target.value)}
          />
          <button type="button" className="primary-action" onClick={() => setShowAdd(true)}>
            + 添加
          </button>
        </div>
        <div className="kb-sources-list">
          {filteredItems.length === 0 ? (
            <div className="kb-empty">
              <span className="kb-empty-icon">📚</span>
              <p>还没有资料</p>
            </div>
          ) : (
            filteredItems.map((item) => (
              <button
                key={item.id}
                type="button"
                className={selectedId === item.id ? "kb-source-row active" : "kb-source-row"}
                onClick={() => setSelectedId(item.id)}
              >
                <strong>{item.title}</strong>
                <span>{item.summary.slice(0, 60)}</span>
              </button>
            ))
          )}
        </div>
      </div>
      <div className="kb-sources-detail">
        {selectedItem ? (
          <KnowledgeItemDetail
            item={selectedItem}
            onSave={props.onSave}
            onDelete={props.onDelete}
            onClose={() => setSelectedId(null)}
            categories={props.categories}
          />
        ) : (
          <div className="kb-empty">
            <span className="kb-empty-icon">📖</span>
            <p>选择左侧资料查看详情</p>
          </div>
        )}
      </div>
      {showAdd && (
        <AddItemModal
          onSave={async (data) => {
            await props.onAdd(data);
            setShowAdd(false);
          }}
          onClose={() => setShowAdd(false)}
          categories={props.categories}
        />
      )}
    </div>
  );
}
