export type ChatRunAccepted = {
  accepted: boolean;
  session_id: string;
  run_id: string;
  initial_status: string;
};

export type ChatRetryRequest = {
  session_id: string;
  run_id: string;
  checkpoint_id?: string;
};

export type ChatMessage = {
  id: string;
  role: "user" | "assistant";
  content: string;
  runId?: string;
};
