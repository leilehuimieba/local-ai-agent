import { useEffect, useState } from "react";
import { knowledgeStore } from "./store";
import { useKnowledgeFilterStore } from "./filterStore";

export function KnowledgeNavPanel() {
  const [categories, setCategories] = useState<string[]>(["全部"]);
  const [tags, setTags] = useState<string[]>([]);
  const filter = useKnowledgeFilterStore();

  useEffect(() => {
    knowledgeStore.getCategories().then(setCategories).catch(() => {});
    knowledgeStore.getTags().then(setTags).catch(() => {});
  }, []);

  return (
    <section className="task-nav-panel">
      <header className="task-nav-panel-head">
        <strong>知识库</strong>
        <span>分类导航</span>
      </header>
      <div className="task-nav-group">
        <header>分类</header>
        {categories.map((c) => (
          <button
            key={c}
            type="button"
            className={filter.category === c ? "task-history-item active" : "task-history-item"}
            onClick={() => filter.setCategory(c)}
          >
            {c}
          </button>
        ))}
      </div>
      <div className="task-nav-group">
        <header>标签</header>
        {tags.slice(0, 24).map((t) => (
          <button
            key={t}
            type="button"
            className={filter.tag === t ? "task-history-item active" : "task-history-item"}
            onClick={() => filter.setTag(t)}
          >
            {t}
          </button>
        ))}
      </div>
    </section>
  );
}
