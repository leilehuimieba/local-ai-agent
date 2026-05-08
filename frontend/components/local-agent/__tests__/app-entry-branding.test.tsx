import { describe, expect, it } from "vitest"
import { render, screen } from "@testing-library/react"
import { BrandStrip } from "../brand-strip"
import { TaskEntryCard } from "../task-entry-card"

describe("app entry branding", () => {
  it("主入口说明条强调主线总控语义", () => {
    render(<BrandStrip />)
    expect(screen.getByText(/主线总控 Agent：围绕当前主目标运行/)).toBeInTheDocument()
    expect(screen.getByText("核心链路")).toBeInTheDocument()
    expect(screen.getByText("辅助入口")).toBeInTheDocument()
  })

  it("任务首屏说明卡强调主目标优先", () => {
    render(<TaskEntryCard />)
    expect(screen.getByText("当前产品口径")).toBeInTheDocument()
    expect(screen.getByText(/这是一个证据驱动的主线总控入口/)).toBeInTheDocument()
    expect(screen.getByText("晚间证据包")).toBeInTheDocument()
    expect(screen.getByText("知识库")).toBeInTheDocument()
  })
})
