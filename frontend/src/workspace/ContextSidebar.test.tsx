import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { testSettings, testSidebarMemoryEvent, testSidebarToolEvent } from "../test/fixtures";
import { ContextSidebar } from "./ContextSidebar";

describe("ContextSidebar", () => {
  it("在首页模式下渲染上下文与风险概览", () => {
    render(<ContextSidebar settings={testSettings} statusLine="完成" variant="home" runState="completed" connectionState="connected" connectionLabel="连接正常" sessionId="session-1" currentRunId="run-1" events={[testSidebarToolEvent, testSidebarMemoryEvent]} confirmation={null} bootstrapError={null} />);
    expect(screen.getByText("首页上下文与风险概览")).toBeInTheDocument();
    expect(screen.getByText("状态沉淀与续接依据")).toBeInTheDocument();
    expect(screen.getByText("风险与记忆")).toBeInTheDocument();
    expect(screen.getByText("会话续接")).toBeInTheDocument();
  });

  it("在任务模式下渲染关键动作与最近记忆", () => {
    render(<ContextSidebar settings={testSettings} statusLine="完成" variant="task" runState="completed" connectionState="connected" connectionLabel="连接正常" sessionId="session-1" currentRunId="run-1" events={[testSidebarToolEvent, testSidebarMemoryEvent]} confirmation={null} bootstrapError={null} />);
    expect(screen.getByText("当前任务工作区")).toBeInTheDocument();
    expect(screen.getByText("关键动作与下一步")).toBeInTheDocument();
    expect(screen.getByText("状态沉淀")).toBeInTheDocument();
    expect(screen.getAllByText("最近记忆").length).toBeGreaterThan(0);
  });
});
