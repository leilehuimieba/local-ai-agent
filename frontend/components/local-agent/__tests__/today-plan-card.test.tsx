import { describe, expect, it } from "vitest"
import { render, screen } from "@testing-library/react"
import { TodayPlanCard } from "../today-plan-card"
import type { MainlineShellState } from "@/lib/local-agent/types"

function buildShell(
  planOverrides: Partial<MainlineShellState["nextDayPlan"]> = {},
  shellOverrides: Partial<Pick<MainlineShellState, "todayPlanStatusText" | "todayPlanReconcileText">> = {},
): MainlineShellState {
  return {
    nextDayPlan: {
      takeoverStatus: "taken_over",
      todayTaskLabel: "四级听力 90 分钟",
      coreTaskLabel: "四级听力 90 分钟",
      takeoverHint: "今天主线已承接这份计划。",
      statusText: "计划目标日：2026-05-08",
      generatedAt: null,
      ready: true,
      supportTaskLabel: "补充项：无",
      rationale: "概率状态：70%",
      restoreNote: null,
      targetDate: "2026-05-08",
      basedOnEvidenceDate: "2026-05-07",
      needsRefresh: false,
      refreshReason: null,
      ...planOverrides,
    },
    todayPlanStatusText: "今日接管：已接管到今天",
    todayPlanReconcileText: "今晚对账：今天计划仍在执行中",
    ...shellOverrides,
  } as MainlineShellState
}

describe("TodayPlanCard", () => {
  it("renders plan info when takeoverStatus is not idle", () => {
    render(<TodayPlanCard mainlineShell={buildShell()} />)
    expect(screen.getByText("今日计划")).toBeInTheDocument()
    expect(screen.getByText(/今日接管：已接管到今天/)).toBeInTheDocument()
    expect(screen.getByText(/今日任务：四级听力 90 分钟/)).toBeInTheDocument()
    expect(screen.getByText(/计划来源：昨晚生成的目标日计划/)).toBeInTheDocument()
    expect(screen.getByText(/今天主线已承接这份计划/)).toBeInTheDocument()
    expect(screen.getByText(/今晚对账：今天计划仍在执行中/)).toBeInTheDocument()
  })

  it("renders todayTaskLabel when available", () => {
    render(<TodayPlanCard mainlineShell={buildShell({ todayTaskLabel: "今日任务A" })} />)
    expect(screen.getByText(/今日任务：今日任务A/)).toBeInTheDocument()
  })

  it("falls back to coreTaskLabel when todayTaskLabel is empty", () => {
    render(<TodayPlanCard mainlineShell={buildShell({ todayTaskLabel: null, coreTaskLabel: "核心任务B" })} />)
    expect(screen.getByText(/今日任务：核心任务B/)).toBeInTheDocument()
  })

  it("returns null when takeoverStatus is idle", () => {
    const { container } = render(<TodayPlanCard mainlineShell={buildShell({ takeoverStatus: "idle" })} />)
    expect(container.firstChild).toBeNull()
  })

  it("does not render takeoverHint when it is null", () => {
    const { container } = render(<TodayPlanCard mainlineShell={buildShell({ takeoverHint: null })} />)
    expect(container.textContent).not.toContain("今天主线已承接这份计划")
  })

  it("does not render reconcile text when it is null", () => {
    const { container } = render(<TodayPlanCard mainlineShell={buildShell({ takeoverHint: null }, { todayPlanReconcileText: null })} />)
    expect(container.textContent).not.toContain("今晚对账")
  })
})
