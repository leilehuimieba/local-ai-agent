import { describe, expect, it } from "vitest"
import { render, screen } from "@testing-library/react"
import { EntryIdentity } from "../entry-identity"

describe("EntryIdentity", () => {
  it("renders main title and description", () => {
    render(<EntryIdentity />)
    expect(screen.getByText(/主线总控 Agent：围绕当前主目标运行，先看证据，再改判断。/)).toBeInTheDocument()
    expect(screen.getByText(/当前主入口优先服务主目标接管/)).toBeInTheDocument()
  })

  it("renders core capability badges", () => {
    render(<EntryIdentity />)
    expect(screen.getByText("核心链路")).toBeInTheDocument()
    expect(screen.getByText("主目标接管")).toBeInTheDocument()
    expect(screen.getByText("关键证据更新")).toBeInTheDocument()
    expect(screen.getByText("临时切主线")).toBeInTheDocument()
    expect(screen.getByText("自动恢复")).toBeInTheDocument()
  })

  it("renders auxiliary capability badges", () => {
    render(<EntryIdentity />)
    expect(screen.getByText("辅助能力")).toBeInTheDocument()
    expect(screen.getByText("历史")).toBeInTheDocument()
    expect(screen.getByText("知识库")).toBeInTheDocument()
    expect(screen.getByText("设置")).toBeInTheDocument()
  })
})
