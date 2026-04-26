import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { testLogEntry } from "../test/fixtures";
import { LogsPanel } from "./LogsPanel";

describe("LogsPanel", () => {
  it("渲染简化后的记录页，不再展示筛选面板和统计卡片", () => {
    render(<LogsPanel logs={[testLogEntry]} />);
    expect(screen.getByRole("heading", { name: "工作历史" })).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "工作时间线" })).toBeInTheDocument();
    expect(screen.queryByText("焦点复盘卡")).not.toBeInTheDocument();
    expect(screen.queryByText("复盘详情栏")).not.toBeInTheDocument();
    expect(screen.queryByText("全部类型")).not.toBeInTheDocument();
  });

  it("点击时间线记录后展开工具调用、验证结果与耗时", () => {
    render(<LogsPanel logs={[testLogEntry]} />);
    fireEvent.click(screen.getByRole("option", { selected: true }));
    expect(screen.getByText("工具调用")).toBeInTheDocument();
    expect(screen.getByText("验证结果")).toBeInTheDocument();
    expect(screen.getByText("运行耗时")).toBeInTheDocument();
    expect(screen.getByText("结构归属验证通过")).toBeInTheDocument();
  });
});
