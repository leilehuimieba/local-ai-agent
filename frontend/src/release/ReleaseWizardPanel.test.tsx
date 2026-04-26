import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { ReleaseWizardPanel } from "./ReleaseWizardPanel";

describe("ReleaseWizardPanel", () => {
  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("展示完整上线向导步骤", () => {
    render(<ReleaseWizardPanel />);
    expect(screen.getByRole("heading", { name: "上线向导" })).toBeInTheDocument();
    expect(screen.getByText("上线前检查")).toBeInTheDocument();
    expect(screen.getByText("安装包构建")).toBeInTheDocument();
    expect(screen.getByText("Doctor 诊断")).toBeInTheDocument();
    expect(screen.getByText("发布候选验证")).toBeInTheDocument();
  });

  it("展示脚本、产物和失败处理口径", () => {
    render(<ReleaseWizardPanel />);
    expect(screen.getByText(/scripts\/run-full-regression\.ps1/)).toBeInTheDocument();
    expect(screen.getByText("tmp/stage-f-rc/latest.json")).toBeInTheDocument();
    expect(screen.getByText(/原因 \/ 影响 \/ 不影响 \/ 建议修复/)).toBeInTheDocument();
  });

  it("点击按钮会调用对应上线脚本接口", async () => {
    const fetchMock = mockReleaseFetch();
    render(<ReleaseWizardPanel />);
    fireEvent.click(screen.getByRole("button", { name: "运行Doctor 诊断" }));
    await waitFor(() => expect(screen.getByText(/通过 · 120ms · tmp\/release-wizard\/doctor\.json/)).toBeInTheDocument());
    expect(fetchMock).toHaveBeenCalledWith("/api/v1/release/run", expect.objectContaining({ body: JSON.stringify({ step: "doctor" }) }));
  });
});

function mockReleaseFetch() {
  return vi.spyOn(globalThis, "fetch").mockResolvedValue({
    ok: true,
    json: async () => ({
      step: "doctor",
      command: "doctor.ps1 -OutFile tmp/release-wizard/doctor.json",
      artifact: "tmp/release-wizard/doctor.json",
      exit_code: 0,
      status: "passed",
      duration_ms: 120,
      stdout: "ok",
      stderr: "",
    }),
  } as Response);
}