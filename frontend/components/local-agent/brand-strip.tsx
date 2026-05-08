"use client"

const coreLane = ["主目标接管", "关键证据更新", "临时切主线", "自动恢复"]
const auxiliaryLane = ["历史", "知识库", "设置"]

function PillRow(props: { title: string; items: string[]; muted?: boolean }) {
  const className = props.muted
    ? "border-border bg-muted/40 text-muted-foreground"
    : "border-primary/20 bg-primary/10 text-primary"
  return (
    <div className="flex flex-wrap items-center gap-2">
      <span className="text-xs font-medium text-muted-foreground">{props.title}</span>
      {props.items.map((item) => <span key={item} className={`rounded-full border px-3 py-1 text-xs ${className}`}>{item}</span>)}
    </div>
  )
}

export function BrandStrip() {
  return (
    <section className="border-b border-border bg-muted/30 px-4 py-3">
      <div className="mx-auto flex max-w-6xl flex-col gap-2">
        <p className="text-sm font-semibold text-foreground">主线总控 Agent：围绕当前主目标运行，先看证据，再改判断。</p>
        <p className="text-xs text-muted-foreground">主入口优先服务主目标接管、晚间证据、关键证据和临时切主线；其他能力继续保留，但降级为辅助入口。</p>
        <PillRow title="核心链路" items={coreLane} />
        <PillRow title="辅助入口" items={auxiliaryLane} muted />
      </div>
    </section>
  )
}
