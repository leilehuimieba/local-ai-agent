import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { WorkbenchOverview } from "./WorkbenchOverview";

function createProps() {
  return {
    kind: "first_use" as const,
    navHint: "可从首页开始",
    composeValue: "整理当前仓库状态",
    canSubmit: true,
    isSubmitting: false,
    eventCount: 3,
    hasConfirmation: false,
    envItems: [{ label: "Runtime", value: "可达" }, { label: "工作区", value: "默认工作区" }],
    examples: [{ id: "ex-1", label: "快速检查", prompt: "检查当前项目问题" }],
    resumeCard: { recentTask: "补齐测试", recentStage: "验证中", latestSummary: "已完成首轮", nextStep: "继续检查截图", runId: "run-1", sessionId: "session-1", contextItems: [{ label: "模式", value: "standard" }], evidenceItems: [{ label: "证据", value: "logs" }] },
    systemCard: { judgement: "稳定", connection: "已连接", mode: "standard", workspace: "默认工作区" },
    blockCard: null,
    recentActivities: [{ id: "a-1", label: "最近任务", text: "补齐前端测试" }],
    onComposeValueChange: vi.fn(), onOpenLogsPage: vi.fn(), onReconnect: vi.fn(), onOpenSettingsPage: vi.fn(),
    onOpenTaskPage: vi.fn(), onOpenTaskPageForConfirmation: vi.fn(), onPrefillExample: vi.fn(), onSubmit: vi.fn(),
  };
}

describe("WorkbenchOverview", () => {
  it("首屏模式渲染快速开始与环境信息", () => {
    const props = createProps();
    render(<WorkbenchOverview {...props} />);
    expect(screen.getByRole("heading", { level: 1, name: "今天想让本地智能体帮你完成什么？" })).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: /快速检查/ }));
    expect(props.onPrefillExample).toHaveBeenCalledWith("检查当前项目问题");
  });

  it("恢复模式在待确认时进入确认处理流", () => {
    const props = { ...createProps(), kind: "resume" as const, hasConfirmation: true };
    render(<WorkbenchOverview {...props} />);
    expect(screen.getAllByText("补齐测试").length).toBeGreaterThan(0);
    fireEvent.click(screen.getByRole("button", { name: /处理待确认动作/ }));
    expect(props.onOpenTaskPageForConfirmation).toHaveBeenCalledTimes(1);
  });

  it("阻塞模式显示建议动作并跳转设置", () => {
    const props = { ...createProps(), kind: "blocked" as const, blockCard: { action: "model" as const, title: "模型未配置", body: "需要先切换模型", detail: "当前 provider 不可用" } };
    render(<WorkbenchOverview {...props} />);
    expect(screen.getByRole("heading", { level: 1, name: "模型未配置" })).toBeInTheDocument();
    expect(screen.getByText("当前 provider 不可用")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: /前往设置切换模型/ }));
    expect(props.onOpenSettingsPage).toHaveBeenCalledTimes(1);
  });
});
