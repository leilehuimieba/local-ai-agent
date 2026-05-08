import { describe, expect, it } from "vitest"
import { render, screen } from "@testing-library/react"
import { NextDayPlanCard } from "../next-day-plan-card"
import type { NextDayPlanState } from "@/lib/local-agent/types"

function buildPlan(overrides: Partial<NextDayPlanState> = {}): NextDayPlanState {
  return {
    generatedAt: "2026-05-07T21:00:00.000+08:00",
    ready: true,
    coreTaskLabel: "四级听力 90 分钟",
    supportTaskLabel: "补充项：无，先保唯一核心任务",
    rationale: "概率状态：70%；时间预算：正常",
    restoreNote: "已恢复原主线：四级冲刺",
    targetDate: "2026-05-08",
    basedOnEvidenceDate: "2026-05-07",
    needsRefresh: false,
    statusText: "计划目标日：2026-05-08",
    refreshReason: null,
    todayTaskLabel: null,
    takeoverStatus: "idle",
    takeoverHint: null,
    ...overrides,
  }
}

describe("NextDayPlanCard", () => {
  it("renders all plan fields", () => {
    render(<NextDayPlanCard plan={buildPlan()} />)
    expect(screen.getByText("明日计划")).toBeInTheDocument()
    expect(screen.getByText(/状态：计划目标日：2026-05-08/)).toBeInTheDocument()
    expect(screen.getByText(/目标日期：2026-05-08/)).toBeInTheDocument()
    expect(screen.getByText(/依据证据日期：2026-05-07/)).toBeInTheDocument()
    expect(screen.getByText(/今日接管：尚未接管/)).toBeInTheDocument()
    expect(screen.getByText(/明日唯一核心任务：四级听力 90 分钟/)).toBeInTheDocument()
    expect(screen.getByText(/补充项：无，先保唯一核心任务/)).toBeInTheDocument()
    expect(screen.getByText(/依据：概率状态：70%；时间预算：正常/)).toBeInTheDocument()
    expect(screen.getByText(/已恢复原主线：四级冲刺/)).toBeInTheDocument()
  })

  it("shows placeholder when targetDate is missing", () => {
    render(<NextDayPlanCard plan={buildPlan({ targetDate: null })} />)
    expect(screen.getByText(/目标日期：尚未生成/)).toBeInTheDocument()
  })

  it("shows placeholder when evidence date is missing", () => {
    render(<NextDayPlanCard plan={buildPlan({ basedOnEvidenceDate: null })} />)
    expect(screen.getByText(/依据证据日期：尚无关键证据/)).toBeInTheDocument()
  })

  it("shows default restore note when restoreNote is null", () => {
    render(<NextDayPlanCard plan={buildPlan({ restoreNote: null })} />)
    expect(screen.getByText(/恢复状态：暂无恢复说明/)).toBeInTheDocument()
  })

  it("shows refresh reason when present", () => {
    render(<NextDayPlanCard plan={buildPlan({ refreshReason: "关键证据已更新" })} />)
    expect(screen.getByText("关键证据已更新")).toBeInTheDocument()
  })

  it("formats takeover statuses correctly", () => {
    const { rerender } = render(<NextDayPlanCard plan={buildPlan({ takeoverStatus: "pending_today" })} />)
    expect(screen.getByText(/今日接管：今日待接管/)).toBeInTheDocument()

    rerender(<NextDayPlanCard plan={buildPlan({ takeoverStatus: "taken_over" })} />)
    expect(screen.getByText(/今日接管：已接管到今天/)).toBeInTheDocument()

    rerender(<NextDayPlanCard plan={buildPlan({ takeoverStatus: "completed" })} />)
    expect(screen.getByText(/今日接管：计划核心任务已完成/)).toBeInTheDocument()

    rerender(<NextDayPlanCard plan={buildPlan({ takeoverStatus: "closed" })} />)
    expect(screen.getByText(/今日接管：今日已按计划收尾/)).toBeInTheDocument()
  })
})
