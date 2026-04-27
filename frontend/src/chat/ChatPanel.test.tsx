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

  it("消息按时间正序排列（用户消息在 assistant 消息之前）", () => {
    const props = {
      ...baseProps,
      messages: [
        { id: "m1", role: "user" as const, content: "第一条用户消息" },
        { id: "m2", role: "assistant" as const, content: "第一条 AI 回复" },
        { id: "m3", role: "user" as const, content: "第二条用户消息" },
      ],
    };
    render(<ChatPanel {...props} />);
    const texts = screen.getAllByText(/第.*条/);
    expect(texts[0]).toHaveTextContent("第一条用户消息");
    expect(texts[1]).toHaveTextContent("第一条 AI 回复");
    expect(texts[2]).toHaveTextContent("第二条用户消息");
  });

  it("assistant 消息含详细过程时显示可折叠按钮", () => {
    const props = {
      ...baseProps,
      messages: [
        { id: "m1", role: "user" as const, content: "执行任务" },
        {
          id: "m2",
          role: "assistant" as const,
          content: "执行结论",
          runId: "run-1",
        },
      ],
      events: [
        {
          event_id: "e1",
          event_type: "run_finished",
          session_id: "s1",
          run_id: "run-1",
          sequence: 1,
          timestamp: "2026-04-26T10:00:00Z",
          stage: "verify",
          summary: "运行完成",
          result_summary: "结果说明",
          metadata: { next_step: "建议下一步" },
        },
      ],
    };
    render(<ChatPanel {...props} />);
    expect(screen.getByText("执行结论")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "查看详细过程" })).toBeInTheDocument();
  });
});
