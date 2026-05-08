import { CalendarClock } from "lucide-react"
import type { MainlineShellState } from "@/lib/local-agent/types"

export function TodayPlanCard(props: { mainlineShell: MainlineShellState }) {
  if (!shouldShowTodayPlan(props.mainlineShell)) return null
  const { nextDayPlan, todayPlanStatusText, todayPlanReconcileText } = props.mainlineShell
  return (
    <div className="rounded-lg border border-border/70 bg-muted/40 p-3 text-xs">
      <div className="mb-2 flex items-center gap-2 font-medium text-foreground">
        <CalendarClock className="h-3.5 w-3.5" />
        <span>今日计划</span>
      </div>
      <div className="space-y-1 text-muted-foreground">
        <p>状态：{todayPlanStatusText}</p>
        <p>今日任务：{nextDayPlan.todayTaskLabel || nextDayPlan.coreTaskLabel}</p>
        <p>计划来源：昨晚生成的目标日计划</p>
        {nextDayPlan.takeoverHint && <p>{nextDayPlan.takeoverHint}</p>}
        {todayPlanReconcileText && <p>{todayPlanReconcileText}</p>}
      </div>
    </div>
  )
}

function shouldShowTodayPlan(mainlineShell: MainlineShellState) {
  return mainlineShell.nextDayPlan.takeoverStatus !== "idle"
}
