import { FormEvent, useState } from "react";
import { KnowledgeItem } from "../types";

type ChatMessage = {
  id: string;
  role: "user" | "assistant";
  content: string;
  sources?: KnowledgeItem[];
};

export type ChatViewProps = {
  items: KnowledgeItem[];
};

export function ChatView(props: ChatViewProps) {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState("");
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    const question = input.trim();
    if (!question || loading) return;

    const userMsg: ChatMessage = { id: `u-${Date.now()}`, role: "user", content: question };
    setMessages((prev) => [...prev, userMsg]);
    setInput("");
    setLoading(true);

    try {
      const response = await fetch("/api/v1/knowledge/ask", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ question }),
      });
      if (!response.ok) throw new Error(`HTTP ${response.status}`);
      const result = (await response.json()) as { answer: string; sources: KnowledgeItem[] };
      const assistantMsg: ChatMessage = {
        id: `a-${Date.now()}`,
        role: "assistant",
        content: result.answer,
        sources: result.sources || [],
      };
      setMessages((prev) => [...prev, assistantMsg]);
    } catch {
      const errorMsg: ChatMessage = {
        id: `a-${Date.now()}`,
        role: "assistant",
        content: "抱歉，基于知识库回答时出错，请稍后重试。",
      };
      setMessages((prev) => [...prev, errorMsg]);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="kb-chat-layout">
      <div className="kb-chat-messages">
        {messages.length === 0 ? (
          <div className="kb-empty">
            <span className="kb-empty-icon">💬</span>
            <p>基于知识库提问</p>
            <p className="kb-empty-hint">AI 会引用知识库中的资料来回答</p>
          </div>
        ) : (
          messages.map((msg) => (
            <article key={msg.id} className={msg.role === "user" ? "kb-chat-message user" : "kb-chat-message assistant"}>
              <div className="kb-chat-bubble">
                <div className="kb-chat-text">{msg.content}</div>
                {msg.sources && msg.sources.length > 0 && (
                  <div className="kb-chat-sources">
                    <span>引用：</span>
                    {msg.sources.map((s) => (
                      <span key={s.id} className="kb-chat-source-tag">{s.title}</span>
                    ))}
                  </div>
                )}
              </div>
            </article>
          ))
        )}
        {loading && (
          <article className="kb-chat-message assistant">
            <div className="kb-chat-bubble">
              <div className="thinking-dots" aria-label="思考中" role="status">
                <span /><span /><span />
              </div>
            </div>
          </article>
        )}
      </div>
      <form className="kb-chat-composer" onSubmit={handleSubmit}>
        <input
          type="text"
          className="kb-chat-input"
          placeholder="输入问题，例如：项目的架构是什么？"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          disabled={loading}
        />
        <button type="submit" className="primary-action" disabled={loading || !input.trim()}>
          {loading ? "思考中..." : "提问"}
        </button>
      </form>
    </div>
  );
}
