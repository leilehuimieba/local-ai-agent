import { History } from "lucide-react"
import type { PlanHistoryItem } from "@/lib/local-agent/types"

export function TodayPlanHistoryCard(props: { items: PlanHistoryItem[] }) {
  const items = props.items ?? []
  if (items.length === 0) return null
  return (
    <div className="rounded-lg border border-border/70 bg-muted/40 p-3 text-xs">
      <div className="mb-2 flex items-center gap-2 font-medium text-foreground">
        <History className="h-3.5 w-3.5" />
        <span>最近计划记录</span>
      </div>
      <div className="space-y-2 text-muted-foreground">
        {items.map((item) => (
          <div key={`${item.targetDate}-${item.taskLabel}`} className="rounded-md border border-border/60 px-2 py-2">
            <p>目标日期：{item.targetDate}</p>
            <p>任务：{item.taskLabel}</p>
            <p>状态：{item.statusText}</p>
            <p>更新：{item.updatedAt}</p>
            <p>对账：{item.reconcileText}</p>
          </div>
        ))}
      </div>
    </div>
  )
}
