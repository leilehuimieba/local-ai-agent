"use client"

const coreLane = ["晚间证据包", "关键证据更新", "临时切主线", "自动恢复"]
const auxiliaryLane = ["历史", "知识库", "设置"]

function HintRow(props: { title: string; items: string[]; muted?: boolean }) {
  const className = props.muted
    ? "border-border bg-muted/40 text-muted-foreground"
    : "border-primary/20 bg-primary/10 text-primary"
  return (
    <div className="space-y-2">
      <p className="text-xs font-medium text-muted-foreground">{props.title}</p>
      <div className="flex flex-wrap gap-2">
        {props.items.map((item) => <span key={item} className={`rounded-full border px-3 py-1 text-xs ${className}`}>{item}</span>)}
      </div>
    </div>
  )
}

export function TaskEntryCard() {
  return (
    <div className="mb-8 w-full max-w-2xl rounded-2xl border border-border bg-card p-4">
      <div className="space-y-3">
        <div>
          <h3 className="text-sm font-semibold text-foreground">当前产品口径</h3>
          <p className="text-xs text-muted-foreground">这是一个证据驱动的主线总控入口，不再把“什么都能先聊聊”作为首页核心承诺。</p>
        </div>
        <HintRow title="核心链路" items={coreLane} />
        <HintRow title="辅助入口" items={auxiliaryLane} muted />
      </div>
    </div>
  )
}
