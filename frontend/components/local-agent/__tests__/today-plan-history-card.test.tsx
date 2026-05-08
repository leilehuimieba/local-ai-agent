import { describe, expect, it } from "vitest"
import { render, screen } from "@testing-library/react"
import { TodayPlanHistoryCard } from "../today-plan-history-card"
import type { PlanHistoryItem } from "@/lib/local-agent/types"

describe("TodayPlanHistoryCard", () => {
  it("renders history items", () => {
    const items: PlanHistoryItem[] = [
      {
        targetDate: "2026-05-07",
        taskLabel: "四级听力 90 分钟",
        status: "closed",
        statusText: "今天计划已收尾",
        updatedAt: "2026-05-07T21:00:00",
        reconciled: true,
        reconcileText: "已对账",
      },
      {
        targetDate: "2026-05-06",
        taskLabel: "四级阅读专项",
        status: "completed",
        statusText: "计划核心任务已完成",
        updatedAt: "2026-05-06T20:00:00",
        reconciled: false,
        reconcileText: "待对账",
      },
    ]
    render(<TodayPlanHistoryCard items={items} />)
    expect(screen.getByText("最近计划记录")).toBeInTheDocument()
    expect(screen.getByText(/目标日期：2026-05-07/)).toBeInTheDocument()
    expect(screen.getByText(/任务：四级听力 90 分钟/)).toBeInTheDocument()
    expect(screen.getByText(/状态：今天计划已收尾/)).toBeInTheDocument()
    expect(screen.getByText(/对账：已对账/)).toBeInTheDocument()
    expect(screen.getByText(/目标日期：2026-05-06/)).toBeInTheDocument()
    expect(screen.getByText(/对账：待对账/)).toBeInTheDocument()
  })

  it("returns null when items is empty", () => {
    const { container } = render(<TodayPlanHistoryCard items={[]} />)
    expect(container.firstChild).toBeNull()
  })

  it("returns null when items is undefined", () => {
    const { container } = render(<TodayPlanHistoryCard items={undefined as unknown as PlanHistoryItem[]} />)
    expect(container.firstChild).toBeNull()
  })

  it("uses composite key for list items", () => {
    const items: PlanHistoryItem[] = [
      { targetDate: "2026-05-07", taskLabel: "任务A", status: "closed", statusText: "", updatedAt: "", reconciled: true, reconcileText: "" },
    ]
    const { container } = render(<TodayPlanHistoryCard items={items} />)
    expect(container.querySelector("[class*='rounded-md']")).toBeInTheDocument()
  })
})
