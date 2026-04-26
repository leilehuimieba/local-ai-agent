import { render, screen } from "@testing-library/react";
import { ComponentProps } from "react";
import { describe, expect, it, vi } from "vitest";

import { testSettings } from "../test/fixtures";
import { ChatPanel } from "./ChatPanel";

const baseProps: ComponentProps<typeof ChatPanel> = {
  settings: testSettings,
  isRunning: false,
  statusLine: "等待中",
  runState: "idle",
  currentRunId: "",
  currentTaskTitle: "",
  composeValue: "",
  events: [],
  messages: [],
  latestFailureEvent: undefined,
  submitError: null,
  confirmation: null,
  rememberChoice: true,
  showRiskLevel: true,
  onComposeValueChange: vi.fn(),
  onSubmit: vi.fn((event) => event.preventDefault()),
  onRememberChoiceChange: vi.fn(),
  onConfirmationDecision: vi.fn(async () => {}),
};

describe("ChatPanel", () => {
  it("在无消息时显示空闲工作台和输入区", () => {
    render(<ChatPanel {...baseProps} />);
    expect(screen.getByText("今天想让本地智能体帮你完成什么？")).toBeInTheDocument();
    expect(screen.getByText("修改项目文件")).toBeInTheDocument();
    expect(screen.getByPlaceholderText("输入任务，按回车发送")).toBeInTheDocument();
  });

  it("在有用户消息时渲染用户消息", () => {
    const props = { ...baseProps, currentTaskTitle: "核对前端状态", messages: [{ id: "m1", role: "user" as const, content: "请检查工作台布局" }] };
    render(<ChatPanel {...props} />);
    expect(screen.getByText("请检查工作台布局")).toBeInTheDocument();
    expect(screen.getByPlaceholderText("输入任务，按回车发送")).toBeInTheDocument();
  });
});
