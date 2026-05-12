import { describe, it, expect } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import {
  MCPObservabilityPanel,
  MCPToolBadges,
  mcpPolicyLabel,
  mcpRiskLabel,
} from "../mcp-observability-panel";
import type { MCPServerInfo, MCPTool, MCPAuditRecord } from "@/lib/local-agent/types";

function buildServer(overrides: Partial<MCPServerInfo> = {}): MCPServerInfo {
  return {
    id: "s1",
    name: "Server1",
    type: "stdio",
    url: "",
    enabled: true,
    ready: true,
    tool_count: 2,
    allowed_tool_count: 1,
    blocked_tool_count: 1,
    requires_policy: false,
    ...overrides,
  };
}

function buildTool(overrides: Partial<MCPTool> = {}): MCPTool {
  return {
    name: "tool1",
    description: "desc1",
    allowed: true,
    risk_level: "low",
    requires_confirmation: false,
    audit_enabled: true,
    policy_source: "config",
    server_id: "s1",
    ...overrides,
  };
}

function buildAudit(overrides: Partial<MCPAuditRecord> = {}): MCPAuditRecord {
  return {
    audit_id: "a1",
    timestamp: "2024-01-01T00:00:00Z",
    server_id: "s1",
    tool_name: "tool1",
    allowed: true,
    risk_level: "low",
    requires_confirmation: false,
    audit_enabled: true,
    policy_source: "config",
    arguments_hash: "",
    outcome: "success",
    elapsed_ms: 100,
    ...overrides,
  };
}

describe("MCPObservabilityPanel", () => {
  it("renders overview stats", () => {
    render(<MCPObservabilityPanel servers={[buildServer()]} tools={[buildTool()]} />);
    expect(screen.getByText("服务器")).toBeInTheDocument();
    expect(screen.getByText("工具")).toBeInTheDocument();
    expect(screen.getByText("已配置入口")).toBeInTheDocument();
    expect(screen.getAllByText("1").length).toBeGreaterThanOrEqual(2);
  });

  it("filters tools by query", () => {
    render(
      <MCPObservabilityPanel
        servers={[buildServer()]}
        tools={[buildTool({ name: "alpha", description: "first" }), buildTool({ name: "beta", description: "second" })]}
      />,
    );
    const input = screen.getByPlaceholderText("搜索工具名、描述或 server id");
    fireEvent.change(input, { target: { value: "alpha" } });
    expect(screen.getByText("alpha")).toBeInTheDocument();
    expect(screen.queryByText("beta")).not.toBeInTheDocument();
  });

  it("shows empty message when no tools match", () => {
    render(<MCPObservabilityPanel servers={[]} tools={[]} emptyMessage="无数据" />);
    expect(screen.getByText("无数据")).toBeInTheDocument();
  });

  it("filters by allowed view", () => {
    render(
      <MCPObservabilityPanel
        servers={[buildServer()]}
        tools={[buildTool({ allowed: true }), buildTool({ allowed: false })]}
      />,
    );
    const selects = screen.getAllByRole("combobox");
    fireEvent.change(selects[1], { target: { value: "allowed" } });
    expect(screen.getAllByText("已允许").length).toBeGreaterThan(0);
  });

  it("renders blocked and allowed tools", () => {
    render(
      <MCPObservabilityPanel
        servers={[buildServer()]}
        tools={[buildTool({ allowed: true, name: "ok" }), buildTool({ allowed: false, name: "no" })]}
      />,
    );
    expect(screen.getByText("ok")).toBeInTheDocument();
    expect(screen.getByText("no")).toBeInTheDocument();
    expect(screen.getByText("已拦截")).toBeInTheDocument();
  });

  it("renders high-risk and low-risk tools", () => {
    render(
      <MCPObservabilityPanel
        servers={[buildServer()]}
        tools={[buildTool({ risk_level: "low", name: "safe" }), buildTool({ risk_level: "high", name: "danger" })]}
      />,
    );
    expect(screen.getByText("safe")).toBeInTheDocument();
    expect(screen.getByText("danger")).toBeInTheDocument();
    expect(screen.getAllByText("高风险").length).toBeGreaterThanOrEqual(1);
  });

  it("renders audit section with data", () => {
    render(
      <MCPObservabilityPanel
        servers={[buildServer()]}
        tools={[]}
        audits={[buildAudit({ tool_name: "auditTool" })]}
      />,
    );
    expect(screen.getByText("auditTool")).toBeInTheDocument();
    expect(screen.getByText("成功")).toBeInTheDocument();
  });

  it("renders audit error message", () => {
    render(
      <MCPObservabilityPanel
        servers={[buildServer()]}
        tools={[]}
        auditError="加载审计失败"
      />,
    );
    expect(screen.getByText("加载审计失败")).toBeInTheDocument();
  });
});

describe("MCPToolBadges", () => {
  it("renders allowed badge", () => {
    render(<MCPToolBadges tool={buildTool({ allowed: true, risk_level: "low" })} />);
    expect(screen.getByText("已允许")).toBeInTheDocument();
    expect(screen.getByText("低风险")).toBeInTheDocument();
  });

  it("renders blocked badge", () => {
    render(<MCPToolBadges tool={buildTool({ allowed: false, risk_level: "critical" })} />);
    expect(screen.getByText("已拦截")).toBeInTheDocument();
    expect(screen.getByText("关键风险")).toBeInTheDocument();
  });

  it("renders audit badge when enabled", () => {
    render(<MCPToolBadges tool={buildTool({ audit_enabled: true })} />);
    expect(screen.getByText("审计")).toBeInTheDocument();
  });

  it("does not render audit badge when disabled", () => {
    render(<MCPToolBadges tool={buildTool({ audit_enabled: false })} />);
    expect(screen.queryByText("审计")).not.toBeInTheDocument();
  });
});

describe("mcpPolicyLabel", () => {
  it("maps known policies", () => {
    expect(mcpPolicyLabel("config")).toBe("配置策略");
    expect(mcpPolicyLabel("default_deny")).toBe("默认拒绝");
  });

  it("falls back to raw value", () => {
    expect(mcpPolicyLabel("custom")).toBe("custom");
  });

  it("handles empty value", () => {
    expect(mcpPolicyLabel("")).toBe("未知策略");
  });
});

describe("mcpRiskLabel", () => {
  it("maps known risk levels", () => {
    expect(mcpRiskLabel("low")).toBe("低风险");
    expect(mcpRiskLabel("medium")).toBe("中风险");
    expect(mcpRiskLabel("high")).toBe("高风险");
    expect(mcpRiskLabel("critical")).toBe("关键风险");
  });

  it("falls back for unknown values", () => {
    expect(mcpRiskLabel("unknown")).toBe("unknown");
    expect(mcpRiskLabel("")).toBe("未知风险");
  });
});
