import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { testLogEntry } from "../test/fixtures";
import { LogsPanel } from "./LogsPanel";

describe("LogsPanel", () => {
  it("在日志工作区内渲染 Review 结构", () => {
    render(<LogsPanel logs={[testLogEntry]} />);
    expect(screen.getByRole("heading", { name: "记录" })).toBeInTheDocument();
    expect(screen.getByText("焦点复盘卡")).toBeInTheDocument();
    expect(screen.getByText("复盘详情栏")).toBeInTheDocument();
  });

  it("在 Logs 工作区内展示时间线与焦点详情", () => {
    render(<LogsPanel logs={[testLogEntry]} />);
    expect(screen.getByText("稳定记录流")).toBeInTheDocument();
    expect(screen.getByText("工作台结构稳定")).toBeInTheDocument();
    expect(screen.getByText("结果摘要")).toBeInTheDocument();
    expect(screen.getAllByText("History / Review 挂在 Logs 工作区内。").length).toBeGreaterThan(0);
  });
});
