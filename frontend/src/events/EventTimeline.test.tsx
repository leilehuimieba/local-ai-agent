import { render, screen } from "@testing-library/react";
import { ComponentProps } from "react";
import { describe, expect, it, vi } from "vitest";

import { testRunEvent } from "../test/fixtures";
import { EventTimeline } from "./EventTimeline";

const baseProps: ComponentProps<typeof EventTimeline> = {
  autoFollow: false,
  events: [],
  onLeaveLatest: vi.fn(),
  selectedEventId: undefined,
  onSelectEvent: vi.fn(),
};

describe("EventTimeline", () => {
  it("在无事件时显示空状态", () => {
    render(<EventTimeline {...baseProps} />);
    expect(screen.getByText("没有事件记录")).toBeInTheDocument();
  });

  it("在有事件时显示阶段、摘要与状态", () => {
    render(<EventTimeline {...baseProps} events={[testRunEvent]} selectedEventId="event-1" />);
    expect(screen.getAllByText("verify").length).toBeGreaterThan(0);
    expect(screen.getByText("完成最小验证")).toBeInTheDocument();
    expect(screen.getByText("最新")).toBeInTheDocument();
  });
});
