export function MarkdownContent({ text }: { text: string }) {
  if (!text.trim()) return null;

  const blocks = parseBlocks(text.trim());
  return (
    <div className="kb-markdown">
      {blocks.map((block, i) => renderBlock(block, i))}
    </div>
  );
}

type Block =
  | { type: "heading"; level: number; text: string }
  | { type: "code"; lang: string; text: string }
  | { type: "list"; ordered: boolean; items: string[] }
  | { type: "quote"; text: string }
  | { type: "paragraph"; text: string };

function parseBlocks(text: string): Block[] {
  const lines = text.split("\n");
  const blocks: Block[] = [];
  let i = 0;

  while (i < lines.length) {
    const line = lines[i];

    if (line.trim() === "") {
      i++;
      continue;
    }

    const heading = line.match(/^(#{1,6})\s+(.*)$/);
    if (heading) {
      blocks.push({ type: "heading", level: heading[1].length, text: heading[2].trim() });
      i++;
      continue;
    }

    if (line.trim().startsWith("```")) {
      const lang = line.trim().slice(3).trim();
      const codeLines: string[] = [];
      i++;
      while (i < lines.length && !lines[i].trim().startsWith("```")) {
        codeLines.push(lines[i]);
        i++;
      }
      blocks.push({ type: "code", lang, text: codeLines.join("\n") });
      i++;
      continue;
    }

    if (line.trim().startsWith("> ")) {
      const quoteLines: string[] = [line.trim().slice(2)];
      i++;
      while (i < lines.length && lines[i].trim().startsWith("> ")) {
        quoteLines.push(lines[i].trim().slice(2));
        i++;
      }
      blocks.push({ type: "quote", text: quoteLines.join("\n") });
      continue;
    }

    const listMatch = line.match(/^(\s*)([-*]|\d+\.)\s+(.*)$/);
    if (listMatch) {
      const ordered = /^\d+\./.test(listMatch[2]);
      const items: string[] = [listMatch[3]];
      i++;
      while (i < lines.length) {
        const next = lines[i];
        if (next.trim() === "") {
          i++;
          continue;
        }
        if (/^(\s*)([-*]|\d+\.)\s+/.test(next)) {
          items.push(next.replace(/^(\s*)([-*]|\d+\.)\s+/, ""));
          i++;
        } else if (next.startsWith("  ") || next.startsWith("\t")) {
          items[items.length - 1] += "\n" + next.trim();
          i++;
        } else {
          break;
        }
      }
      blocks.push({ type: "list", ordered, items });
      continue;
    }

    const paraLines: string[] = [line];
    i++;
    while (i < lines.length && lines[i].trim() !== "" && !/^(#{1,6}\s|>|\s*[-*]\s+|\s*\d+\.\s+|```)/.test(lines[i])) {
      paraLines.push(lines[i]);
      i++;
    }
    blocks.push({ type: "paragraph", text: paraLines.join("\n") });
  }

  return blocks;
}

function renderBlock(block: Block, key: number) {
  switch (block.type) {
    case "heading": {
      const level = block.level;
      const children = renderInline(block.text);
      if (level === 1) return <h1 key={key} className="kb-md-h1">{children}</h1>;
      if (level === 2) return <h2 key={key} className="kb-md-h2">{children}</h2>;
      if (level === 3) return <h3 key={key} className="kb-md-h3">{children}</h3>;
      if (level === 4) return <h4 key={key} className="kb-md-h4">{children}</h4>;
      if (level === 5) return <h5 key={key} className="kb-md-h5">{children}</h5>;
      return <h6 key={key} className="kb-md-h6">{children}</h6>;
    }
    case "code":
      return (
        <pre key={key} className="kb-md-pre">
          <code className={block.lang ? `language-${block.lang}` : undefined}>{block.text}</code>
        </pre>
      );
    case "list":
      const ListTag = block.ordered ? "ol" : "ul";
      return (
        <ListTag key={key} className={block.ordered ? "kb-md-ol" : "kb-md-ul"}>
          {block.items.map((item, idx) => (
            <li key={idx}>{renderInline(item)}</li>
          ))}
        </ListTag>
      );
    case "quote":
      return <blockquote key={key} className="kb-md-blockquote">{renderInline(block.text)}</blockquote>;
    case "paragraph":
      return <p key={key} className="kb-md-p">{renderInline(block.text)}</p>;
  }
}

function renderInline(text: string) {
  const parts: React.ReactNode[] = [];
  const regex = /(\*\*.*?\*\*|\[.*?\]\(.*?\)|`.*?`)/g;
  let lastIndex = 0;
  let match: RegExpExecArray | null;
  let key = 0;

  while ((match = regex.exec(text)) !== null) {
    if (match.index > lastIndex) {
      parts.push(<span key={key++}>{text.slice(lastIndex, match.index)}</span>);
    }

    const m = match[0];
    if (m.startsWith("**") && m.endsWith("**")) {
      parts.push(<strong key={key++}>{m.slice(2, -2)}</strong>);
    } else if (m.startsWith("`") && m.endsWith("`")) {
      parts.push(<code key={key++} className="kb-md-code">{m.slice(1, -1)}</code>);
    } else if (m.startsWith("[") && m.includes("](")) {
      const linkMatch = m.match(/^\[(.*?)\]\((.*?)\)$/);
      if (linkMatch) {
        parts.push(
          <a key={key++} href={linkMatch[2]} target="_blank" rel="noopener noreferrer" className="kb-md-link">
            {linkMatch[1]}
          </a>
        );
      } else {
        parts.push(<span key={key++}>{m}</span>);
      }
    } else {
      parts.push(<span key={key++}>{m}</span>);
    }

    lastIndex = regex.lastIndex;
  }

  if (lastIndex < text.length) {
    parts.push(<span key={key++}>{text.slice(lastIndex)}</span>);
  }

  return parts;
}
