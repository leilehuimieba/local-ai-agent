import type { ChatMessage, ConfirmationRequest, RunEvent, SettingsResponse } from "../shared/contracts";
import type { RunState } from "../runtime/state";
import { TaskThread } from "./TaskThread";

export type ConfirmationDecision = "approve" | "reject" | "cancel";

export type ChatPanelProps = {
  settings: SettingsResponse | null;
  isRunning: boolean;
  statusLine: string;
  runState: RunState;
  currentRunId: string;
  currentTaskTitle: string;
  composeValue: string;
  events: RunEvent[];
  messages: ChatMessage[];
  latestFailureEvent?: RunEvent;
  submitError: string | null;
  confirmation: ConfirmationRequest | null;
  rememberChoice: boolean;
  showRiskLevel: boolean;
  onComposeValueChange: (value: string) => void;
  onSubmit: (event: React.FormEvent<HTMLFormElement>) => void;
  onRememberChoiceChange: (checked: boolean) => void;
  onConfirmationDecision: (decision: ConfirmationDecision) => Promise<void>;
  onExampleClick?: (value: string) => void;
};

export function ChatPanel(props: ChatPanelProps) {
  return (
    <article className="panel chat-panel chat-panel-simple">
      <TaskThread props={props} />
    </article>
  );
}
