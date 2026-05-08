import { CalendarDays } from "lucide-react"
import type { NextDayPlanState } from "@/lib/local-agent/types"

export function NextDayPlanCard(props: { plan: NextDayPlanState }) {
  const { plan } = props
  return (
    <div className="rounded-lg border border-border/70 bg-muted/40 p-3 text-xs">
      <div className="mb-2 flex items-center gap-2 font-medium text-foreground">
        <CalendarDays className="h-3.5 w-3.5" />
        <span>明日计划</span>
      </div>
      <div className="space-y-1 text-muted-foreground">
        <p>状态：{plan.statusText}</p>
        <p>目标日期：{plan.targetDate || "尚未生成"}</p>
        <p>依据证据日期：{plan.basedOnEvidenceDate || "尚无关键证据"}</p>
        <p>今日接管：{formatTakeoverStatus(plan.takeoverStatus)}</p>
        <p>明日唯一核心任务：{plan.coreTaskLabel}</p>
        <p>{plan.supportTaskLabel}</p>
        <p>依据：{plan.rationale}</p>
        <p>{plan.restoreNote || "恢复状态：暂无恢复说明"}</p>
        {plan.refreshReason && <p>{plan.refreshReason}</p>}
      </div>
    </div>
  )
}

function formatTakeoverStatus(status: "idle" | "pending_today" | "taken_over" | "completed" | "closed") {
  return {
    idle: "尚未接管",
    pending_today: "今日待接管",
    taken_over: "已接管到今天",
    completed: "计划核心任务已完成",
    closed: "今日已按计划收尾",
  }[status]
}
