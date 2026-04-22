import { render, screen } from "@testing-library/react";
import { ComponentProps } from "react";
import { describe, expect, it, vi } from "vitest";

import { testMemory, testSettings } from "../test/fixtures";
import { SettingsPanel } from "./SettingsPanel";

const baseProps: ComponentProps<typeof SettingsPanel> = {
  settings: testSettings,
  providerSettings: { active_provider_id: "openai", providers: [] },
  bootstrapError: null,
  providerBootstrapError: null,
  providerActions: {},
  isRunning: false,
  pendingAction: null,
  actionError: null,
  lastSuccess: null,
  memories: [testMemory],
  memoryError: null,
  memoryPendingAction: null,
  memoryActionError: null,
  memoryActionSuccess: null,
  deletingMemoryId: "",
  onModeChange: vi.fn(),
  onModelChange: vi.fn(),
  onWorkspaceChange: vi.fn(),
  onDirectoryPromptEnabledChange: vi.fn(),
  onShowRiskLevelChange: vi.fn(),
  onRevokeDirectoryApproval: vi.fn(),
  onRunExternalConnectionAction: vi.fn(),
  onCheckDiagnostics: vi.fn(),
  onRefreshProviderSettings: vi.fn(async () => {}),
  onTestProvider: vi.fn(async () => {}),
  onSaveProvider: vi.fn(async () => {}),
  onApplyProvider: vi.fn(async () => {}),
  onRemoveProvider: vi.fn(async () => {}),
  onDeleteMemory: vi.fn(),
  onRefreshMemories: vi.fn(),
  isActionPending: vi.fn(() => false),
};

describe("SettingsPanel", () => {
  it("渲染设置工作台主标题", () => {
    render(<SettingsPanel {...baseProps} />);
    expect(screen.getByText("Settings Workspace")).toBeInTheDocument();
    expect(screen.getByText("记忆与资源")).toBeInTheDocument();
  });

  it("在设置结构内渲染资源工作区模块", () => {
    render(<SettingsPanel {...baseProps} />);
    expect(screen.getAllByText("Memory / Resources Workspace").length).toBeGreaterThan(0);
    expect(screen.getByText("保持中文输出")).toBeInTheDocument();
    expect(screen.getByText("资源工作区")).toBeInTheDocument();
  });
});
