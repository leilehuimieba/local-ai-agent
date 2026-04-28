import { KnowledgeItem } from "./types";
import { cleanTitle, fallbackSummary } from "./utils";

export function KnowledgeItemCard({
  item,
  onClick,
}: {
  item: KnowledgeItem;
  onClick: () => void;
}) {
  const timeStr = new Date(item.updatedAt).toLocaleDateString("zh-CN");
  return (
    <article
      className="kb-card"
      onClick={onClick}
      role="button"
      tabIndex={0}
      onKeyDown={(e) => {
        if (e.key === "Enter" || e.key === " ") onClick();
      }}
    >
      <header className="kb-card-head">
        <span className="kb-card-category">{item.category}</span>
        <span className="kb-card-time">{timeStr}</span>
      </header>
      <h3 className="kb-card-title">{cleanTitle(item)}</h3>
      <p className="kb-card-summary">{fallbackSummary(item)}</p>
      <footer className="kb-card-foot">
        <div className="kb-card-tags">
          {item.tags.map((t) => (
            <span key={t} className="kb-tag">#{t}</span>
          ))}
        </div>
        <span className="kb-card-cite">引用 {item.citationCount}</span>
      </footer>
    </article>
  );
}
