import { useState } from "react";
import { KnowledgeItem } from "./types";
import { MarkdownContent } from "./MarkdownContent";
import { cleanTitle, fallbackSummary } from "./utils";

export function KnowledgeItemDetail({
  item,
  onSave,
  onDelete,
  onClose,
  categories,
}: {
  item: KnowledgeItem;
  onSave: (id: string, patch: Partial<KnowledgeItem>) => void;
  onDelete: (id: string) => void;
  onClose: () => void;
  categories: string[];
}) {
  const [isEditing, setIsEditing] = useState(false);
  const [title, setTitle] = useState(item.title);
  const [summary, setSummary] = useState(item.summary);
  const [content, setContent] = useState(item.content);
  const [category, setCategory] = useState(item.category);
  const [tags, setTags] = useState(item.tags.join(", "));
  const [source, setSource] = useState(item.source || "");

  const handleSave = () => {
    onSave(item.id, {
      title: title.trim(),
      summary: summary.trim(),
      content: content.trim(),
      category,
      tags: tags.split(",").map((t) => t.trim()).filter(Boolean),
      source: source.trim() || undefined,
    });
    setIsEditing(false);
  };

  return (
    <aside className="kb-detail">
      <header className="kb-detail-head">
        <button type="button" className="secondary-button" onClick={onClose}>← 返回</button>
        <div className="kb-detail-actions">
          {!isEditing ? (
            <>
              <button type="button" className="secondary-button" onClick={() => setIsEditing(true)}>编辑</button>
              <button type="button" className="secondary-button danger" onClick={() => onDelete(item.id)}>删除</button>
            </>
          ) : (
            <>
              <button type="button" className="secondary-button" onClick={() => setIsEditing(false)}>取消</button>
              <button type="button" className="primary-action" onClick={handleSave}>保存</button>
            </>
          )}
        </div>
      </header>

      {isEditing ? (
        <div className="kb-detail-form">
          <label>标题 <input value={title} onChange={(e) => setTitle(e.target.value)} /></label>
          <label>分类
            <select value={category} onChange={(e) => setCategory(e.target.value)}>
              {categories.filter((c) => c !== "全部").map((c) => (
                <option key={c} value={c}>{c}</option>
              ))}
            </select>
          </label>
          <label>标签 <input value={tags} onChange={(e) => setTags(e.target.value)} /></label>
          <label>摘要 <input value={summary} onChange={(e) => setSummary(e.target.value)} /></label>
          <label>正文 <textarea value={content} onChange={(e) => setContent(e.target.value)} rows={12} /></label>
          <label>来源 <input value={source} onChange={(e) => setSource(e.target.value)} /></label>
        </div>
      ) : (
        <div className="kb-detail-view">
          <span className="kb-detail-category">{item.category}</span>
          <h2>{cleanTitle(item)}</h2>
          <div className="kb-detail-meta">
            <span>更新于 {new Date(item.updatedAt).toLocaleString("zh-CN")}</span>
            <span>引用 {item.citationCount} 次</span>
          </div>
          <div className="kb-detail-tags">
            {item.tags.map((t) => <span key={t} className="kb-tag">#{t}</span>)}
          </div>
          {item.source && <p className="kb-detail-source">来源：{item.source}</p>}
          <hr />
          {item.content.trim() === "" ? (
            <div className="kb-detail-empty">
              <span>⏳</span>
              <p>内容待识别</p>
              <small>该资料正文暂未提取成功，稍后自动重试</small>
            </div>
          ) : (
            <div className="kb-detail-content">
              <MarkdownContent text={item.content} />
            </div>
          )}
        </div>
      )}
    </aside>
  );
}
