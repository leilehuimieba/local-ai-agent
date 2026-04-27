import { readErrorText } from "../shared/apiUtils";

export type ReleaseStepId = "prelaunch" | "package" | "doctor" | "rc";

export type ReleaseRunResponse = {
  step: ReleaseStepId;
  command: string;
  artifact: string;
  exit_code: number;
  status: "passed" | "failed";
  duration_ms: number;
  stdout: string;
  stderr: string;
};

export async function runReleaseStep(step: ReleaseStepId): Promise<ReleaseRunResponse> {
  const response = await fetch("/api/v1/release/run", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ step }),
  });
  if (!response.ok) {
    throw new Error(`执行上线步骤失败: ${await readErrorText(response)}`);
  }
  return (await response.json()) as ReleaseRunResponse;
}

