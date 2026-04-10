import { ChatRetryRequest, ChatRunAccepted, ModelRef, WorkspaceRef } from "../shared/contracts";

type SubmitChatRunPayload = {
  sessionId: string;
  userInput: string;
  mode: string;
  model: ModelRef;
  workspace: WorkspaceRef;
};

export async function submitChatRun(payload: SubmitChatRunPayload): Promise<ChatRunAccepted> {
  const response = await fetch("/api/v1/chat/run", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      session_id: payload.sessionId,
      user_input: payload.userInput,
      mode: payload.mode,
      model: payload.model,
      workspace: payload.workspace,
    }),
  });

  if (!response.ok) {
    throw new Error(`提交任务失败: ${await readErrorText(response)}`);
  }

  return (await response.json()) as ChatRunAccepted;
}

export async function submitChatRetry(payload: ChatRetryRequest): Promise<ChatRunAccepted> {
  const response = await fetch("/api/v1/chat/retry", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(payload),
  });

  if (!response.ok) {
    throw new Error(`提交重试失败: ${await readErrorText(response)}`);
  }

  return (await response.json()) as ChatRunAccepted;
}

async function readErrorText(response: Response): Promise<string> {
  const text = (await response.text()).trim();
  return text || String(response.status);
}
