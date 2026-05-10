import Link from "next/link"

const entries = [
  {
    href: "/acceptance/mcp-observability",
    title: "MCP 观测验收",
    detail: "验收 MCP 运营视图的概览卡、筛选、只读列表、空态和缺字段态。",
  },
  {
    href: "/acceptance/confirmation-preview",
    title: "Patch Confirmation Preview",
    detail: "验收 structured preview、fallback 摘要，以及“确认应用 / 取消应用”交互文案。",
  },
]

export default function AcceptanceIndexPage() {
  return (
    <main className="min-h-dvh bg-background p-6 text-foreground">
      <div className="mx-auto max-w-4xl space-y-6">
        <section className="space-y-2">
          <h1 className="text-xl font-semibold">Frontend Acceptance</h1>
          <p className="text-sm text-muted-foreground">这里收口前端人工验收入口，优先放可复现、可截图、可回归的页面。</p>
        </section>
        <div className="space-y-3">
          {entries.map((entry) => <AcceptanceEntry key={entry.href} href={entry.href} title={entry.title} detail={entry.detail} />)}
        </div>
      </div>
    </main>
  )
}

function AcceptanceEntry({ href, title, detail }: { href: string; title: string; detail: string }) {
  return (
    <Link href={href} className="block rounded-lg border border-border bg-card px-4 py-3 transition-colors hover:bg-muted/40">
      <p className="text-sm font-medium text-foreground">{title}</p>
      <p className="mt-1 text-sm text-muted-foreground">{detail}</p>
    </Link>
  )
}
