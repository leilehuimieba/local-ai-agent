"use client"

import { useState } from "react"
import { cn } from "@/lib/utils"

interface LightweightMarkdownProps {
  content: string
  className?: string
  highlightText?: string
}

function CodeBlock({ lang, code }: { lang: string; code: string }) {
  const [copied, setCopied] = useState(false)
  const handleCopy = () => {
    navigator.clipboard.writeText(code)
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  return (
    <div className="mt-2 rounded-lg bg-muted overflow-hidden">
      <div className="flex items-center justify-between px-3 py-1.5 border-b border-border/50">
        <span className="text-[10px] text-muted-foreground uppercase">{lang || "code"}</span>
        <button
          onClick={handleCopy}
          className="text-[10px] text-muted-foreground hover:text-foreground transition-colors"
        >
          {copied ? "已复制" : "复制"}
        </button>
      </div>
      <pre className="p-3 text-xs font-mono overflow-x-auto">
        <code>{code}</code>
      </pre>
    </div>
  )
}

export function LightweightMarkdown({ content, className, highlightText }: LightweightMarkdownProps) {
  const lines = content.split("\n")
  const elements: React.ReactNode[] = []
  let i = 0
  let key = 0

  while (i < lines.length) {
    const line = lines[i]

    // Code block
    if (line.trim().startsWith("```")) {
      const lang = line.trim().slice(3).trim()
      const codeLines: string[] = []
      i++
      while (i < lines.length && !lines[i].trim().startsWith("```")) {
        codeLines.push(lines[i])
        i++
      }
      i++ // skip closing ```
      elements.push(
        <CodeBlock key={key++} lang={lang} code={codeLines.join("\n")} />
      )
      continue
    }

    // Heading
    const headingMatch = line.match(/^(#{1,4})\s+(.*)$/)
    if (headingMatch) {
      const level = headingMatch[1].length
      const text = headingMatch[2]
      const sizes = ["text-lg", "text-base", "text-sm", "text-sm"]
      const weights = ["font-bold", "font-semibold", "font-medium", "font-medium"]
      const margins = ["mt-4 mb-2", "mt-3 mb-1.5", "mt-2 mb-1", "mt-2 mb-1"]
      elements.push(
        <div key={key++} className={cn(sizes[level - 1], weights[level - 1], margins[level - 1])}>
          {renderInline(text, highlightText)}
        </div>
      )
      i++
      continue
    }

    // List item
    if (line.trim().startsWith("- ") || line.trim().startsWith("* ")) {
      const items: string[] = []
      while (i < lines.length && (lines[i].trim().startsWith("- ") || lines[i].trim().startsWith("* "))) {
        items.push(lines[i].trim().slice(2))
        i++
      }
      elements.push(
        <ul key={key++} className="mt-2 space-y-1">
          {items.map((item, idx) => (
            <li key={idx} className="flex items-start gap-2 text-sm">
              <span className="mt-1.5 h-1.5 w-1.5 rounded-full bg-primary shrink-0" />
              {renderInline(item, highlightText)}
            </li>
          ))}
        </ul>
      )
      continue
    }

    // Empty line
    if (line.trim() === "") {
      elements.push(<div key={key++} className="h-2" />)
      i++
      continue
    }

    // Regular paragraph
    elements.push(
      <p key={key++} className="text-sm text-foreground leading-relaxed">
        {renderInline(line, highlightText)}
      </p>
    )
    i++
  }

  return <div className={className}>{elements}</div>
}

function renderInline(text: string, highlight?: string): React.ReactNode {
  const parts: React.ReactNode[] = []
  let remaining = text
  let key = 0

  while (remaining.length > 0) {
    // Bold + italic ***text***
    const boldItalicMatch = remaining.match(/^(.*?)\*\*\*(.+?)\*\*\*(.*)$/)
    if (boldItalicMatch) {
      if (boldItalicMatch[1]) parts.push(...highlightSpans(boldItalicMatch[1], highlight, key++))
      parts.push(<strong key={key++} className="italic">{boldItalicMatch[2]}</strong>)
      remaining = boldItalicMatch[3]
      continue
    }

    // Bold **text**
    const boldMatch = remaining.match(/^(.*?)\*\*(.+?)\*\*(.*)$/)
    if (boldMatch) {
      if (boldMatch[1]) parts.push(...highlightSpans(boldMatch[1], highlight, key++))
      parts.push(<strong key={key++}>{boldMatch[2]}</strong>)
      remaining = boldMatch[3]
      continue
    }

    // Italic *text*
    const italicMatch = remaining.match(/^(.*?)\*(.+?)\*(.*)$/)
    if (italicMatch) {
      if (italicMatch[1]) parts.push(...highlightSpans(italicMatch[1], highlight, key++))
      parts.push(<em key={key++}>{italicMatch[2]}</em>)
      remaining = italicMatch[3]
      continue
    }

    // Inline code `text`
    const codeMatch = remaining.match(/^(.*?)`([^`]+)`(.*)$/)
    if (codeMatch) {
      if (codeMatch[1]) parts.push(...highlightSpans(codeMatch[1], highlight, key++))
      parts.push(
        <code key={key++} className="px-1 py-0.5 bg-muted rounded text-xs font-mono">
          {codeMatch[2]}
        </code>
      )
      remaining = codeMatch[3]
      continue
    }

    // Link [text](url)
    const linkMatch = remaining.match(/^(.*?)\[([^\]]+)\]\(([^)]+)\)(.*)$/)
    if (linkMatch) {
      if (linkMatch[1]) parts.push(...highlightSpans(linkMatch[1], highlight, key++))
      parts.push(
        <a key={key++} href={linkMatch[3]} target="_blank" rel="noopener noreferrer" className="text-primary underline">
          {linkMatch[2]}
        </a>
      )
      remaining = linkMatch[4]
      continue
    }

    parts.push(...highlightSpans(remaining, highlight, key++))
    break
  }

  return <>{parts}</>
}

function highlightSpans(text: string, highlight: string | undefined, baseKey: number): React.ReactNode[] {
  if (!highlight?.trim()) return [<span key={baseKey}>{text}</span>]
  const q = highlight.toLowerCase()
  const result: React.ReactNode[] = []
  let remaining = text
  let subKey = 0
  while (remaining.length > 0) {
    const idx = remaining.toLowerCase().indexOf(q)
    if (idx < 0) {
      result.push(<span key={`${baseKey}-${subKey++}`}>{remaining}</span>)
      break
    }
    if (idx > 0) {
      result.push(<span key={`${baseKey}-${subKey++}`}>{remaining.slice(0, idx)}</span>)
    }
    result.push(
      <mark key={`${baseKey}-${subKey++}`} className="bg-yellow-200 text-foreground rounded px-0.5">
        {remaining.slice(idx, idx + highlight.length)}
      </mark>
    )
    remaining = remaining.slice(idx + highlight.length)
  }
  return result
}
