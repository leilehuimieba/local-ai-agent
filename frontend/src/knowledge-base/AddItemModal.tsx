import { useRef, useState } from "react";
import { KnowledgeItem } from "./types";

export function AddItemModal({
  onSave,
  onClose,
  categories,
}: {
  onSave: (item: Omit<KnowledgeItem, "id" | "createdAt" | "updatedAt" | "citationCount">) => void;
  onClose: () => void;
  categories: string[];
}) {
  const [mode, setMode] = useState<"manual" | "upload">("manual");
  const [title, setTitle] = useState("");
  const [summary, setSummary] = useState("");
  const [content, setContent] = useState("");
  const [category, setCategory] = useState(categories[1] || "文档");
  const [tags, setTags] = useState("");
  const [source, setSource] = useState("");
  const [uploading, setUploading] = useState(false);
  const [uploadError, setUploadError] = useState<string | null>(null);
  const fileRef = useRef<HTMLInputElement>(null);

  const handleManualSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!title.trim()) return;
    onSave({
      title: title.trim(),
      summary: summary.trim() || title.trim(),
      content: content.trim(),
      category,
      tags: tags.split(",").map((t) => t.trim()).filter(Boolean),
      source: source.trim() || undefined,
    });
    onClose();
  };

  const handleFileUpload = async (e: React.FormEvent) => {
    e.preventDefault();
    const file = fileRef.current?.files?.[0];
    if (!file) return;

    setUploading(true);
    setUploadError(null);
    try {
      const formData = new FormData();
      formData.append("file", file);
      const response = await fetch("/api/v1/knowledge/upload", {
        method: "POST",
        body: formData,
      });
      if (!response.ok) {
        const text = await response.text();
        throw new Error(text || `HTTP ${response.status}`);
      }
      onClose();
    } catch (err) {
      setUploadError(err instanceof Error ? err.message : "上传失败");
    } finally {
      setUploading(false);
    }
  };

  return (
    <div className="kb-modal-backdrop" onClick={onClose}>
      <div className="kb-modal" onClick={(e) => e.stopPropagation()}>
        <h2>添加知识条目</h2>
        <div className="kb-modal-tabs">
          <button
            type="button"
            className={mode === "manual" ? "active" : ""}
            onClick={() => setMode("manual")}
          >
            手动输入
          </button>
          <button
            type="button"
            className={mode === "upload" ? "active" : ""}
            onClick={() => setMode("upload")}
          >
            上传文件
          </button>
        </div>

        {mode === "manual" ? (
          <form onSubmit={handleManualSubmit}>
            <label>
              标题 *
              <input value={title} onChange={(e) => setTitle(e.target.value)} placeholder="输入标题" required />
            </label>
            <label>
              分类
              <select value={category} onChange={(e) => setCategory(e.target.value)}>
                {categories.filter((c) => c !== "全部").map((c) => (
                  <option key={c} value={c}>{c}</option>
                ))}
              </select>
            </label>
            <label>
              标签（逗号分隔）
              <input value={tags} onChange={(e) => setTags(e.target.value)} placeholder="例如：架构, 项目" />
            </label>
            <label>
              摘要
              <input value={summary} onChange={(e) => setSummary(e.target.value)} placeholder="简短描述" />
            </label>
            <label>
              正文（支持 Markdown）
              <textarea value={content} onChange={(e) => setContent(e.target.value)} rows={8} placeholder="输入详细内容..." />
            </label>
            <label>
              来源（可选）
              <input value={source} onChange={(e) => setSource(e.target.value)} placeholder="URL 或文件路径" />
            </label>
            <div className="kb-modal-actions">
              <button type="button" className="secondary-button" onClick={onClose}>取消</button>
              <button type="submit" className="primary-action">确认添加</button>
            </div>
          </form>
        ) : (
          <form onSubmit={handleFileUpload}>
            <label>
              选择文件（支持 .txt, .md, .pdf, .docx）
              <input
                ref={fileRef}
                type="file"
                accept=".txt,.md,.markdown,.pdf,.docx"
                required
              />
            </label>
            {uploadError && <p className="kb-error">{uploadError}</p>}
            <div className="kb-modal-actions">
              <button type="button" className="secondary-button" onClick={onClose}>取消</button>
              <button type="submit" className="primary-action" disabled={uploading}>
                {uploading ? "上传中..." : "上传并提取"}
              </button>
            </div>
          </form>
        )}
      </div>
    </div>
  );
}
