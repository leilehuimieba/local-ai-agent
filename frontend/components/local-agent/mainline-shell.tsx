"use client"

import { useEffect, useRef, type ReactNode } from "react"
import { AlertTriangle, CalendarClock, CalendarDays, ChevronDown, ChevronUp, ClipboardCheck, RotateCcw, Sparkles, Target } from "lucide-react"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import { Checkbox } from "@/components/ui/checkbox"
import { Input } from "@/components/ui/input"
import { buildMainlineEntryRecommendation, type MainlineEntryAction } from "@/lib/local-agent/mainline-entry-rules"
import { Textarea } from "@/components/ui/textarea"
import { getTaskTypeLabel } from "@/lib/local-agent/personalized-followup-rules"
import { cn } from "@/lib/utils"
import { useUIStore, type UIStore } from "@/lib/local-agent/store"
import { TodayPlanCard } from "./today-plan-card"
import { NextDayPlanCard } from "./next-day-plan-card"
import { TodayPlanHistoryCard } from "./today-plan-history-card"

const riskClasses = {
  green: "border-success/40 bg-success/10 text-success",
  yellow: "border-warning/40 bg-warning/10 text-warning",
  orange: "border-orange-500/40 bg-orange-500/10 text-orange-600 dark:text-orange-400",
  red: "border-destructive/40 bg-destructive/10 text-destructive",
} as const

const bannerToneClasses = {
  warning: "border-warning/40 bg-warning/10 text-warning",
  success: "border-success/40 bg-success/10 text-success",
  muted: "border-border/70 bg-muted/50 text-muted-foreground",
} as const

function formatProbability(value: number | null) {
  return typeof value === "number" ? `${value}%` : "未知"
}

function buildRiskLabel(level: keyof typeof riskClasses) {
  return { green: "稳定", yellow: "提醒", orange: "预警", red: "高风险" }[level]
}

function MainlineHeader() {
  const { mainlineShell, mainlineExpanded, setMainlineExpanded } = useUIStore()
  const riskClass = riskClasses[mainlineShell.riskLevel]
  return (
    <button type="button" onClick={() => setMainlineExpanded(!mainlineExpanded)} className="flex w-full items-start justify-between gap-3 text-left">
      <div className="min-w-0 space-y-2">
        <div className="flex items-center gap-2 text-xs text-muted-foreground">
          <Target className="h-3.5 w-3.5" />
          <span>当前主目标</span>
        </div>
        <p className="line-clamp-2 text-sm font-semibold text-foreground">{mainlineShell.currentGoalLabel}</p>
        <div className="flex items-center gap-2">
          <span className="text-2xl font-bold tracking-tight">{formatProbability(mainlineShell.probabilityValue)}</span>
          <Badge variant="outline" className={cn("border", riskClass)}>{buildRiskLabel(mainlineShell.riskLevel)}</Badge>
        </div>
      </div>
      {mainlineExpanded ? <ChevronDown className="mt-1 h-4 w-4 shrink-0" /> : <ChevronUp className="mt-1 h-4 w-4 shrink-0" />}
    </button>
  )
}

function StatusBanner() {
  const { mainlineShell } = useUIStore()
  return (
    <div className="space-y-2">
      {mainlineShell.execution.needsReopen && <Banner tone="warning" text="已进入新的一天，请先重开今天主线。" />}
      {mainlineShell.evidenceSubmitted && <Banner tone="success" text="今晚证据已提交" />}
      {mainlineShell.eveningReview.status === "late_allowed" && <Banner tone="warning" text="昨晚证据可补交，次日中午前先补昨天。" />}
      {mainlineShell.eveningReview.status === "reroute" && <Banner tone="warning" text={mainlineShell.eveningReview.branchText} />}
      {mainlineShell.evidenceExpired && <Banner tone="warning" text="请尽快补充关键证据" />}
      {mainlineShell.nextDayPlan.needsRefresh && mainlineShell.nextDayPlan.refreshReason && <Banner tone="warning" text={mainlineShell.nextDayPlan.refreshReason} />}
      {mainlineShell.keyEvidenceFeedback && <Banner tone="muted" text={mainlineShell.keyEvidenceFeedback} />}
      {mainlineShell.switchFeedback && <Banner tone="muted" text={mainlineShell.switchFeedback} />}
      {mainlineShell.personalizedFeedback && <Banner tone="muted" text={mainlineShell.personalizedFeedback} />}
      {mainlineShell.timeBudgetFeedback && <Banner tone="muted" text={mainlineShell.timeBudgetFeedback} />}
    </div>
  )
}

function Banner({ tone, text }: { tone: "warning" | "success" | "muted"; text: string }) {
  return <div className={cn("rounded-lg border px-3 py-2 text-xs", bannerToneClasses[tone])}>{text}</div>
}

function MainlineActions() {
  const store = useUIStore()
  const temp = store.mainlineShell.temporaryMainline
  return (
    <div className="space-y-3 border-t border-border/70 pt-3">
      <StatusBanner />
      <EveningReviewCard />
      <NextStepCard />
      <ExecutionStateCard />
      <TodayPlanCard mainlineShell={store.mainlineShell} />
      <TodayPlanHistoryCard items={store.mainlineShell.recentPlanHistory} />
      <FollowthroughCard />
      <NextDayPlanCard plan={store.mainlineShell.nextDayPlan} />
      <TimeBudgetInsightCard />
      <PersonalizedInsightCard />
      <PrimaryActionRows />
      <RestoreRow active={temp.isActive} onRestore={store.restoreOriginalMainline} />
      <div className="space-y-1 text-xs text-muted-foreground">
        <p>本周目标：可展开查看</p>
        <p>最终目标：安全通过区</p>
        <p>模块级调整：{store.mainlineShell.latestAdjustment}</p>
        <p>切主线状态：{temp.isActive ? `进行中（${temp.currentTemporaryGoal}）` : "未切换"}</p>
      </div>
    </div>
  )
}

function PrimaryActionRows() {
  const store = useUIStore()
  const nextStep = buildMainlineEntryRecommendation(store.mainlineShell)
  const evidenceLabel = getEvidenceActionLabel(store.mainlineShell.eveningReview.status)
  return (
    <>
      <InlineExecutionAction action={nextStep.action} label={nextStep.label} store={store} />
      <div className="grid grid-cols-2 gap-2">
        <ActionButton icon={<ClipboardCheck className="h-4 w-4" />} label={evidenceLabel} onClick={() => store.setEvidencePanelOpen(true)} {...buildActionTone(nextStep.action, "evidence_packet")} />
        <ActionButton icon={<CalendarDays className="h-4 w-4" />} label="补关键证据" onClick={() => store.setKeyEvidencePanelOpen(true)} {...buildActionTone(nextStep.action, "key_evidence")} />
      </div>
      <div className="grid grid-cols-2 gap-2">
        <ActionButton icon={<AlertTriangle className="h-4 w-4" />} label="临时切主线" onClick={() => store.setSwitchPanelOpen(true)} muted={nextStep.action !== "noop"} />
        <ActionButton icon={<Sparkles className="h-4 w-4" />} label="个性化记录" onClick={() => store.setPersonalizedPanelOpen(true)} muted={nextStep.action !== "noop"} />
      </div>
      <div className="grid grid-cols-1 gap-2">
        <ActionButton icon={<CalendarClock className="h-4 w-4" />} label="时间预算接管" onClick={() => store.setTimeBudgetPanelOpen(true)} {...buildActionTone(nextStep.action, "time_budget")} />
        <ActionButton icon={<CalendarDays className="h-4 w-4" />} label="生成明日计划" onClick={() => store.generateNextDayPlan()} {...buildActionTone(nextStep.action, "next_day_plan")} />
      </div>
    </>
  )
}

function InlineExecutionAction(props: { action: MainlineEntryAction; label: string; store: UIStore }) {
  if (props.action !== "complete_core_task" && props.action !== "close_day" && props.action !== "noop") return null
  return (
    <div className="grid grid-cols-1 gap-2">
      <ActionButton icon={<Target className="h-4 w-4" />} label={props.label} onClick={buildEntryActionHandler(props.action, props.store)} recommended muted={props.action === "noop"} disabled={props.action === "noop"} />
    </div>
  )
}

function buildActionTone(currentAction: MainlineEntryAction, action: MainlineEntryAction) {
  return { recommended: currentAction === action, muted: currentAction !== action }
}

function EveningReviewCard() {
  const { eveningReview: review, todayPlanReconcileText } = useUIStore((state) => state.mainlineShell)
  return (
    <div className="rounded-lg border border-border/70 bg-muted/40 p-3 text-xs">
      <div className="mb-2 flex items-center gap-2 font-medium text-foreground">
        <ClipboardCheck className="h-3.5 w-3.5" />
        <span>{review.scheduledLabel}</span>
      </div>
      <div className="space-y-1 text-muted-foreground">
        <p>状态：{review.statusText}</p>
        <p>{review.guidance}</p>
        <p>{review.branchText}</p>
        {review.deadlineLabel && <p>{review.deadlineLabel}</p>}
        {todayPlanReconcileText && <p>{todayPlanReconcileText}</p>}
      </div>
    </div>
  )
}

function NextStepCard() {
  const store = useUIStore()
  const nextStep = buildMainlineEntryRecommendation(store.mainlineShell)
  const disabled = nextStep.action === "noop"
  return (
    <div className="rounded-lg border border-border/70 bg-muted/40 p-3 text-xs">
      <div className="mb-2 flex items-center gap-2 font-medium text-foreground">
        <Target className="h-3.5 w-3.5" />
        <span>下一步动作</span>
      </div>
      <p className="font-medium text-foreground">{nextStep.label}</p>
      <p className="mt-1 text-muted-foreground">{nextStep.helperText}</p>
      <Button
        size="sm"
        className="mt-3 w-full justify-start gap-2"
        onClick={buildEntryActionHandler(nextStep.action, store)}
        aria-label={`下一步动作：${nextStep.label}`}
        disabled={disabled}
      >
        {nextStep.label}
      </Button>
    </div>
  )
}

function ExecutionStateCard() {
  const execution = useUIStore((state) => state.mainlineShell.execution)
  return (
    <div className="rounded-lg border border-border/70 bg-muted/40 p-3 text-xs">
      <div className="mb-2 flex items-center gap-2 font-medium text-foreground">
        <CalendarClock className="h-3.5 w-3.5" />
        <span>执行态</span>
      </div>
      <div className="space-y-1 text-muted-foreground">
        <p>执行状态：{execution.statusText}</p>
        <p>当前核心任务：{execution.currentTaskLabel || "尚未锁定"}</p>
        <p>执行日期：{execution.activeDate || "尚未开始"}</p>
        <p>{execution.helperText}</p>
      </div>
    </div>
  )
}

function FollowthroughCard() {
  const followthrough = useUIStore((state) => state.mainlineShell.followthrough)
  if (!followthrough.lastActionLabel) return null
  return (
    <div className="rounded-lg border border-border/70 bg-muted/40 p-3 text-xs">
      <div className="mb-2 flex items-center gap-2 font-medium text-foreground">
        <Sparkles className="h-3.5 w-3.5" />
        <span>连续引导</span>
      </div>
      <div className="space-y-1 text-muted-foreground">
        <p>刚完成：{followthrough.lastActionLabel}</p>
        <p>下一步：{followthrough.nextActionLabel}</p>
        <p>{followthrough.nextActionHelper}</p>
        {followthrough.planUsesLatestEvidence !== null && (
          <p>{followthrough.planUsesLatestEvidence ? "当前计划已基于最新关键证据" : "当前计划尚未基于最新关键证据"}</p>
        )}
      </div>
    </div>
  )
}

function getEvidenceActionLabel(status: "pending" | "submitted" | "late_allowed" | "reroute") {
  if (status === "late_allowed") return "补交昨晚证据"
  if (status === "submitted") return "重看晚间证据"
  return "晚间证据"
}

function buildEntryActionHandler(action: MainlineEntryAction, store: UIStore) {
  if (action === "key_evidence") return () => store.setKeyEvidencePanelOpen(true)
  if (action === "next_day_plan") return () => store.generateNextDayPlan()
  if (action === "time_budget") return () => store.setTimeBudgetPanelOpen(true)
  if (action === "complete_core_task") return () => store.markTodayCoreTaskCompleted()
  if (action === "close_day") return () => store.closeToday()
  if (action === "noop") return () => undefined
  return () => store.setEvidencePanelOpen(true)
}

function ActionButton(props: { icon: ReactNode; label: string; onClick: () => void; recommended?: boolean; muted?: boolean; disabled?: boolean }) {
  return (
    <Button
      size="sm"
      variant={props.recommended ? "default" : "outline"}
      className={cn("justify-start gap-2", props.muted && "opacity-55")}
      onClick={props.onClick}
      disabled={props.disabled}
      data-recommended={props.recommended ? "true" : "false"}
      data-muted={props.muted ? "true" : "false"}
    >
      {props.icon}
      {props.label}
    </Button>
  )
}

function RestoreRow(props: { active: boolean; onRestore: () => void }) {
  return (
    <div className="grid grid-cols-1 gap-2">
      <ActionButton icon={<RotateCcw className="h-4 w-4" />} label="自动恢复" onClick={props.onRestore} disabled={!props.active} />
    </div>
  )
}

function PersonalizedInsightCard() {
  const insight = useUIStore((state) => state.mainlineShell.personalizedInsight)
  return (
    <div className="rounded-lg border border-border/70 bg-muted/40 p-3 text-xs">
      <div className="mb-2 flex items-center gap-2 font-medium text-foreground">
        <Sparkles className="h-3.5 w-3.5" />
        <span>更适合你的推进方式</span>
      </div>
      <p className="text-muted-foreground">{insight.recommendation}</p>
      <div className="mt-2 space-y-1 text-muted-foreground">
        <p>基于记录：{insight.basedOnCount} 次</p>
        <p>更易完成：{formatTaskType(insight.preferredTaskType)}</p>
        <p>短期收益最高：{formatTaskType(insight.boostTaskType)}</p>
        <p>当前阻力偏高：{formatTaskType(insight.riskTaskType)}</p>
      </div>
    </div>
  )
}

function TimeBudgetInsightCard() {
  const { timeBudgetEntry, timeBudgetInsight, timeBlocks } = useUIStore((state) => state.mainlineShell)
  return (
    <div className="rounded-lg border border-border/70 bg-muted/40 p-3 text-xs">
      <div className="mb-2 flex items-center gap-2 font-medium text-foreground">
        <CalendarClock className="h-3.5 w-3.5" />
        <span>今日主线接管</span>
      </div>
      <div className="space-y-1 text-muted-foreground">
        <p>今日唯一核心任务：{timeBudgetEntry.coreTaskLabel}</p>
        <p>可用时间：{formatMinutes(timeBudgetInsight.totalAvailableMinutes)}</p>
        <p>预算状态：{formatBudgetLevel(timeBudgetInsight.level)}</p>
        <p>已录入时间块：{timeBlocks.length} 个</p>
      </div>
      <p className="mt-2 text-muted-foreground">{timeBudgetInsight.recommendation}</p>
      {timeBlocks.length > 0 && (
        <div className="mt-2 space-y-1 text-muted-foreground">
          {timeBlocks.map((item, index) => (
            <p key={`${item.startTime}-${item.endTime}-${index}`}>
              {item.startTime}-{item.endTime} · {item.label}
            </p>
          ))}
        </div>
      )}
    </div>
  )
}

function EvidencePanel() {
  const store = useUIStore()
  if (!store.evidencePanelOpen) return null
  const packet = store.mainlineShell.evidencePacket
  const canSubmit = packet.nextAdjustment.trim().length > 0
  return (
    <div className="space-y-3 border-t border-border/70 pt-3">
      <PanelHeader title="晚间证据包" desc="先对账，再决定下一步调整。" />
      <EvidenceTextarea label="今天做了什么" value={packet.didWhat} onChange={(value) => store.updateEvidencePacket({ didWhat: value })} />
      <EvidenceTextarea label="可验证结果" value={packet.resultSummary} onChange={(value) => store.updateEvidencePacket({ resultSummary: value })} />
      <EvidenceTextarea label="失败或卡点" value={packet.blockers} onChange={(value) => store.updateEvidencePacket({ blockers: value })} />
      <EvidenceTextarea label="下一步调整" value={packet.nextAdjustment} onChange={(value) => store.updateEvidencePacket({ nextAdjustment: value })} />
      <PanelActions onClose={() => store.setEvidencePanelOpen(false)} onSubmit={store.submitEvidencePacket} submitLabel="提交证据" disabled={!canSubmit} />
    </div>
  )
}

function KeyEvidencePanel() {
  const store = useUIStore()
  if (!store.keyEvidencePanelOpen) return null
  const entry = store.mainlineShell.keyEvidenceEntry
  const submit = () => {
    store.refreshMainlineEvidenceState()
    store.submitKeyEvidenceEntry()
  }
  return (
    <div className="space-y-3 border-t border-border/70 pt-3">
      <PanelHeader title="录入最新模拟题 / 真题结果" desc="先录总分，必须当场补全分项得分。" />
      <TypeRow value={entry.sourceType} onChange={(value) => store.updateKeyEvidenceEntry({ sourceType: value })} />
      <ScoreGrid />
      <ExamConditionRow />
      <EvidenceTextarea label="备注 / 说明" value={entry.notes} onChange={(value) => store.updateKeyEvidenceEntry({ notes: value })} />
      <PanelActions onClose={() => store.setKeyEvidencePanelOpen(false)} onSubmit={submit} submitLabel="更新安全通过概率" disabled={false} />
    </div>
  )
}

function SwitchPanel() {
  const store = useUIStore()
  const form = store.mainlineShell.switchForm
  if (!store.switchPanelOpen) return null
  const canSubmit = form.temporaryGoalLabel.trim().length > 0 && form.reason.trim().length > 0 && form.dueDate.trim().length > 0
  return (
    <div className="space-y-3 border-t border-border/70 pt-3">
      <PanelHeader title="临时切主线" desc="用户先提，Agent 复核；若仍坚持，可记账后执行。" />
      <FieldInput label="临时目标" value={form.temporaryGoalLabel} onChange={(value) => store.updateSwitchForm({ temporaryGoalLabel: value })} />
      <EvidenceTextarea label="切换原因" value={form.reason} onChange={(value) => store.updateSwitchForm({ reason: value })} />
      <FieldInput label="截止时间" value={form.dueDate} onChange={(value) => store.updateSwitchForm({ dueDate: value })} />
      <SwitchCheckbox
        label="属于紧急事项"
        checked={form.urgent}
        onCheckedChange={(checked) => store.updateSwitchForm({ urgent: checked === true })}
      />
      <SwitchCheckbox
        label="属于重要事项"
        checked={form.important}
        onCheckedChange={(checked) => store.updateSwitchForm({ important: checked === true })}
      />
      <SwitchCheckbox
        label="若未通过复核，仍坚持并记账"
        checked={form.insistAfterReject}
        onCheckedChange={(checked) => store.updateSwitchForm({ insistAfterReject: checked === true })}
      />
      <PanelActions onClose={() => store.setSwitchPanelOpen(false)} onSubmit={store.submitTemporaryMainlineSwitch} submitLabel="提交切主线申请" disabled={!canSubmit} />
    </div>
  )
}

function PersonalizedPanel() {
  const store = useUIStore()
  const entry = store.mainlineShell.personalizedEntry
  if (!store.personalizedPanelOpen) return null
  return (
    <div className="space-y-3 border-t border-border/70 pt-3">
      <PanelHeader title="个性化推进记录" desc="先通用、后个性化：只记录会改变后续安排的真实反馈。" />
      <TaskTypeRow value={entry.taskType} onChange={(value) => store.updatePersonalizedEntry({ taskType: value })} />
      <SwitchCheckbox label="这是今天的核心任务" checked={entry.isCoreTask} onCheckedChange={(checked) => store.updatePersonalizedEntry({ isCoreTask: checked === true })} />
      <BooleanRow
        label="这项任务是否完成"
        value={entry.completed}
        onChange={(value) => store.updatePersonalizedEntry({ completed: value })}
      />
      <BooleanRow
        label="这项任务是否带来短期收益"
        value={entry.hasShortTermGain}
        onChange={(value) => store.updatePersonalizedEntry({ hasShortTermGain: value })}
      />
      <EvidenceTextarea label="结果反馈 / 备注" value={entry.resultNote} onChange={(value) => store.updatePersonalizedEntry({ resultNote: value })} />
      <PanelActions onClose={() => store.setPersonalizedPanelOpen(false)} onSubmit={store.submitPersonalizedEntry} submitLabel="记录这次反馈" disabled={false} />
    </div>
  )
}

function TimeBudgetPanel() {
  const store = useUIStore()
  const entry = store.mainlineShell.timeBudgetEntry
  if (!store.timeBudgetPanelOpen) return null
  return (
    <div className="space-y-3 border-t border-border/70 pt-3">
      <PanelHeader title="时间预算 / 时间块接管" desc="先录入今天真实可用时间，再决定是否缩量保主线。" />
      <FieldInput label="今日唯一核心任务" value={entry.coreTaskLabel} onChange={(value) => store.updateTimeBudgetEntry({ coreTaskLabel: value })} />
      <EvidenceTextarea label="现实时间预算变化" value={entry.budgetChangeNote} onChange={(value) => store.updateTimeBudgetEntry({ budgetChangeNote: value })} />
      <TimeBlockDraftFields />
      <PanelActions onClose={() => store.setTimeBudgetPanelOpen(false)} onSubmit={store.submitTimeBudgetEntry} submitLabel="更新今日主线接管" disabled={false} />
    </div>
  )
}

function PanelHeader({ title, desc }: { title: string; desc: string }) {
  return (
    <div className="space-y-1">
      <h3 className="text-sm font-semibold">{title}</h3>
      <p className="text-xs text-muted-foreground">{desc}</p>
    </div>
  )
}

function EvidenceTextarea(props: { label: string; value: string; onChange: (value: string) => void }) {
  return (
    <label className="block space-y-1 text-xs">
      <span className="text-muted-foreground">{props.label}</span>
      <Textarea aria-label={props.label} value={props.value} onChange={(e) => props.onChange(e.target.value)} />
    </label>
  )
}

function PanelActions(props: { onClose: () => void; onSubmit: () => void; submitLabel: string; disabled: boolean }) {
  return (
    <div className="flex gap-2">
      <Button size="sm" variant="outline" className="flex-1" onClick={props.onClose}>暂不提交</Button>
      <Button size="sm" className="flex-1" disabled={props.disabled} onClick={props.onSubmit}>{props.submitLabel}</Button>
    </div>
  )
}

function TypeRow(props: { value: "mock_exam" | "real_exam"; onChange: (value: "mock_exam" | "real_exam") => void }) {
  return (
    <div className="grid grid-cols-2 gap-2">
      <Button size="sm" variant={props.value === "mock_exam" ? "default" : "outline"} onClick={() => props.onChange("mock_exam")}>模拟题</Button>
      <Button size="sm" variant={props.value === "real_exam" ? "default" : "outline"} onClick={() => props.onChange("real_exam")}>真题</Button>
    </div>
  )
}

function TaskTypeRow(props: { value: Parameters<typeof getTaskTypeLabel>[0]; onChange: (value: Parameters<typeof getTaskTypeLabel>[0]) => void }) {
  const taskTypes = ["vocabulary", "mock_exam", "reading", "listening", "writing_translation", "professional_skill", "algorithm", "other"] as const
  return (
    <div className="grid grid-cols-2 gap-2">
      {taskTypes.map((taskType) => (
        <Button key={taskType} size="sm" variant={props.value === taskType ? "default" : "outline"} onClick={() => props.onChange(taskType)}>
          {getTaskTypeLabel(taskType)}
        </Button>
      ))}
    </div>
  )
}

function TimeBlockDraftFields() {
  const draft = useUIStore((state) => state.mainlineShell.timeBudgetEntry.timeBlockDraft)
  const update = useUIStore((state) => state.updateTimeBlockDraft)
  return (
    <div className="grid grid-cols-2 gap-2">
      <FieldInput label="开始时间" value={draft.startTime} onChange={(value) => update({ startTime: value })} />
      <FieldInput label="结束时间" value={draft.endTime} onChange={(value) => update({ endTime: value })} />
      <div className="col-span-2">
        <FieldInput label="时间块说明" value={draft.label} onChange={(value) => update({ label: value })} />
      </div>
    </div>
  )
}

function ScoreGrid() {
  const entry = useUIStore((state) => state.mainlineShell.keyEvidenceEntry)
  const update = useUIStore((state) => state.updateKeyEvidenceEntry)
  return (
    <div className="grid grid-cols-2 gap-2">
      <FieldInput label="做题日期" value={entry.completedDate} onChange={(value) => update({ completedDate: value })} />
      <FieldInput label="总用时(分钟)" value={entry.durationMinutes} onChange={(value) => update({ durationMinutes: value })} />
      <FieldInput label="总分" value={entry.totalScore} onChange={(value) => update({ totalScore: value })} />
      <FieldInput label="听力分" value={entry.listeningScore} onChange={(value) => update({ listeningScore: value })} />
      <FieldInput label="阅读分" value={entry.readingScore} onChange={(value) => update({ readingScore: value })} />
      <FieldInput label="写作与翻译分" value={entry.writingTranslationScore} onChange={(value) => update({ writingTranslationScore: value })} />
    </div>
  )
}

function FieldInput(props: { label: string; value: string; onChange: (value: string) => void }) {
  return (
    <label className="space-y-1 text-xs">
      <span className="text-muted-foreground">{props.label}</span>
      <Input aria-label={props.label} value={props.value} onChange={(e) => props.onChange(e.target.value)} />
    </label>
  )
}

function ExamConditionRow() {
  const value = useUIStore((state) => state.mainlineShell.keyEvidenceEntry.underExamCondition)
  const update = useUIStore((state) => state.updateKeyEvidenceEntry)
  return <SwitchCheckbox label="是否完整按考试条件完成" checked={value === true} onCheckedChange={(checked) => update({ underExamCondition: checked === true })} />
}

function SwitchCheckbox(props: { label: string; checked: boolean; onCheckedChange: (checked: boolean | "indeterminate") => void }) {
  return (
    <label className="flex items-center gap-2 rounded-lg border border-border/70 px-3 py-2 text-xs">
      <Checkbox checked={props.checked} onCheckedChange={props.onCheckedChange} />
      <span>{props.label}</span>
    </label>
  )
}

function BooleanRow(props: { label: string; value: boolean | null; onChange: (value: boolean) => void }) {
  return (
    <div className="space-y-1 text-xs">
      <span className="text-muted-foreground">{props.label}</span>
      <div className="grid grid-cols-2 gap-2">
        <Button size="sm" variant={props.value === true ? "default" : "outline"} onClick={() => props.onChange(true)}>是</Button>
        <Button size="sm" variant={props.value === false ? "default" : "outline"} onClick={() => props.onChange(false)}>否</Button>
      </div>
    </div>
  )
}

function formatTaskType(taskType: Parameters<typeof getTaskTypeLabel>[0] | null) {
  return taskType ? getTaskTypeLabel(taskType) : "尚无判断"
}

function formatBudgetLevel(level: "ample" | "steady" | "tight" | "critical") {
  return {
    ample: "宽松",
    steady: "正常",
    tight: "紧张",
    critical: "极紧",
  }[level]
}

function formatMinutes(minutes: number) {
  if (minutes <= 0) return "尚未录入"
  const hours = Math.floor(minutes / 60)
  const mins = minutes % 60
  return hours > 0 ? `${hours} 小时 ${mins} 分钟` : `${mins} 分钟`
}

export function MainlineShell() {
  const { mainlineExpanded, refreshMainlineEvidenceState } = useUIStore()
  const refreshedRef = useRef(false)
  useEffect(() => {
    if (refreshedRef.current) return
    refreshedRef.current = true
    refreshMainlineEvidenceState()
  }, [refreshMainlineEvidenceState])
  return (
    <div className="pointer-events-none fixed right-4 top-16 z-40 hidden w-[320px] lg:block">
      <Card className="pointer-events-auto gap-0 border-border/80 bg-card/95 py-0 shadow-lg backdrop-blur">
        <CardContent className="space-y-3 p-4">
          <MainlineHeader />
          {mainlineExpanded && (
            <>
              <MainlineActions />
              <EvidencePanel />
              <KeyEvidencePanel />
              <SwitchPanel />
              <PersonalizedPanel />
              <TimeBudgetPanel />
            </>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
