import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { SectionHeader } from "./SectionHeader";

describe("SectionHeader", () => {
  it("默认渲染 section-header 与 h3 标题", () => {
    const { container } = render(<SectionHeader title="运行摘要" />);
    expect(container.firstElementChild).toHaveClass("section-header");
    expect(screen.getByRole("heading", { level: 3, name: "运行摘要" })).toBeInTheDocument();
  });

  it("按 page 模式渲染 kicker、描述与动作", () => {
    const { container } = render(
      <SectionHeader
        title="设置中心"
        kind="page"
        level="h1"
        kicker="Settings"
        description="查看当前运行态"
        action={<button type="button">刷新</button>}
      />,
    );
    expect(container.firstElementChild).toHaveClass("page-shell-header");
    expect(screen.getByText("Settings")).toBeInTheDocument();
    expect(screen.getByRole("heading", { level: 1, name: "设置中心" })).toBeInTheDocument();
    expect(screen.getByText("查看当前运行态")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "刷新" })).toBeInTheDocument();
  });

  it("按 head 模式渲染 section-head 类名", () => {
    const { container } = render(<SectionHeader title="日志视图" kind="head" level="h2" />);
    expect(container.firstElementChild).toHaveClass("section-head");
    expect(screen.getByRole("heading", { level: 2, name: "日志视图" })).toBeInTheDocument();
  });
});
