import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { testLogEntry } from "../../test/fixtures";
import { HistoryTimelineSection } from "./HistoryTimelineSection";

describe("HistoryTimelineSection", () => {
  it("在无记录时显示空状态", () => {
    render(<HistoryTimelineSection logs={[]} selectedLogId="" onSelectLog={vi.fn()} />);
    expect(screen.getByText("当前筛选下没有记录")).toBeInTheDocument();
  });

  it("在有记录时显示时间线并支持选中", () => {
    const onSelectLog = vi.fn();
    render(<HistoryTimelineSection logs={[testLogEntry]} selectedLogId="log-1" onSelectLog={onSelectLog} />);
    expect(screen.getByText("稳定记录流")).toBeInTheDocument();
    expect(screen.getByText("review")).toBeInTheDocument();
    expect(screen.getByText("工作台结构稳定")).toBeInTheDocument();
    fireEvent.click(screen.getByText("工作台结构稳定"));
    expect(onSelectLog).toHaveBeenCalledWith("log-1");
  });
});
