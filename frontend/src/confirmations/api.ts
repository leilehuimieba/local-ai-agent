import { ConfirmationRequest, RunEvent } from "../shared/contracts";

type SubmitConfirmationPayload = {
  confirmationId: string;
  runId: string;
  decision: "approve" | "reject" | "cancel";
  remember: boolean;
};

export async function submitConfirmationDecision(
  payload: SubmitConfirmationPayload,
): Promise<void> {
  const response = await fetch("/api/v1/chat/confirm", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      confirmation_id: payload.confirmationId,
      run_id: payload.runId,
      decision: payload.decision,
      remember: payload.remember,
    }),
  });

  if (!response.ok) {
    throw new Error(`提交确认失败: ${await readErrorText(response)}`);
  }
}

export function confirmationFromEvent(event: RunEvent): ConfirmationRequest {
  return {
    confirmation_id: event.metadata?.confirmation_id || "",
    run_id: event.run_id,
    risk_level: event.metadata?.risk_level || "medium",
    action_summary: event.metadata?.action_summary || event.summary,
    reason: event.metadata?.reason || event.detail || event.summary,
    impact_scope: event.metadata?.impact_scope || "",
    target_paths: event.metadata?.target_paths
      ? event.metadata.target_paths.split("\n").filter(Boolean)
      : [],
    reversible: event.metadata?.reversible === "true",
    hazards: event.metadata?.hazards ? event.metadata.hazards.split("\n").filter(Boolean) : [],
    alternatives: event.metadata?.alternatives
      ? event.metadata.alternatives.split("\n").filter(Boolean)
      : [],
    kind: event.metadata?.kind || "high_risk_action",
  };
}

async function readErrorText(response: Response): Promise<string> {
  const text = (await response.text()).trim();
  return text || String(response.status);
}
