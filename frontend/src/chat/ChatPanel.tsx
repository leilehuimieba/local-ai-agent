import { FormEvent } from "react";

import { ConfirmationCard } from "../confirmations/ConfirmationCard";
import {
  buildAssistantResult,
  formatEntryIndex,
  getStreamLiveLabel,
  readFailureAdvice,
  readFailureBody,
  readRunStateBody,
  readRunStateHeadline,
  readRunStateNextStep,
  readThreadStatusClass,
  ResultSection,
  shouldShowMessageFailure,
  shouldShowConfirmationRecord,
  shouldShowInlineFailureNotice,
  shouldShowPendingMessages,
} from "./chatResultModel";
import { RunState } from "../runtime/state";
import { ChatMessage, ConfirmationRequest, RunEvent, SettingsResponse } from "../shared/contracts";
import { EmptyStateBlock, MetricChip, SectionHeader, StatusPill } from "../ui/primitives";

type ConfirmationDecision = "approve" | "reject" | "cancel";

type ChatPanelProps = {
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
  onSubmit: (event: FormEvent<HTMLFormElement>) => void;
  onRememberChoiceChange: (checked: boolean) => void;
  onConfirmationDecision: (decision: ConfirmationDecision) => Promise<void>;
};

export function ChatPanel(props: ChatPanelProps) {
  return (
    <article className="panel chat-panel">
      <TaskHeader props={props} />
      <TaskThread props={props} />
      <TaskComposer props={props} />
    </article>
  );
}

function TaskHeader(props: { props: ChatPanelProps }) {
  return (
    <section className="task-header">
      <div className="task-header-main">
        <span className="section-kicker">任务</span>
        <h1>{props.props.currentTaskTitle || "等待任务"}</h1>
      </div>
      <div className="task-meta-row">
        <SummaryChip label="状态" value={props.props.statusLine} />
        <SummaryChip label="运行" value={props.props.currentRunId || "等待生成"} />
        <SummaryChip label="工作区" value={props.props.settings?.workspace.name || "未加载"} />
      </div>
    </section>
  );
}

function TaskThread(props: { props: ChatPanelProps }) {
  return (
    <section className="stream-shell">
      <ThreadHeader props={props.props} />
      <div className="sr-only" aria-live="polite">{getStreamLiveLabel(props.props.runState, props.props.messages.length)}</div>
      <div className="messages" aria-live="polite" aria-relevant="additions text">
        <ThreadContent props={props.props} />
      </div>
    </section>
  );
}

function ThreadHeader(props: { props: ChatPanelProps }) {
  return (
    <SectionHeader kicker="记录" className="stream-header" title="任务记录流" action={<StatusPill className={readThreadStatusClass(props.props.runState)} label={props.props.statusLine} />} />
  );
}

function ThreadContent(props: { props: ChatPanelProps }) {
  if (shouldShowPendingMessages(props.props.runState, props.props.messages)) {
    return <PendingMessageState taskTitle={props.props.currentTaskTitle || "当前任务"} />;
  }
  if (shouldShowMessageFailure(props.props.runState, props.props.messages, props.props.submitError, props.props.latestFailureEvent)) {
    return <PrimaryErrorState latestFailureEvent={props.props.latestFailureEvent} submitError={props.props.submitError} />;
  }
  if (props.props.messages.length === 0) {
    return <EmptyWorkbench settings={props.props.settings} />;
  }
  return <ThreadRecords props={props.props} />;
}

function ThreadRecords(props: { props: ChatPanelProps }) {
  const tailRecord = readThreadTailRecord(props.props);
  const toolEvents = getToolEvents(props.props.events);
  return (
    <>
      {[...props.props.messages].reverse().map((message, index) => (
        <MessageRecord
          key={message.id}
          index={index}
          message={message}
          runEvent={findTerminalRunEvent(props.props, message.runId)}
        />
      ))}
      {toolEvents.length > 0 ? <ToolEventFeed events={toolEvents} /> : null}
      {tailRecord}
    </>
  );
}

function MessageRecord(props: { message: ChatMessage; index: number; runEvent?: RunEvent }) {
  if (props.message.role === "user") return <UserRecord index={props.index} message={props.message} />;
  return <AssistantRecord index={props.index} message={props.message} runEvent={props.runEvent} />;
}

function UserRecord(props: { message: ChatMessage; index: number }) {
  return (
    <article className="thread-record user">
      <RecordHead index={props.index} role="任务输入" />
      <div className="thread-record-copy">
        {splitMessage(props.message.content).map((item, index) => <p key={`${props.message.id}-${index}`}>{item}</p>)}
      </div>
    </article>
  );
}

function AssistantRecord(props: { message: ChatMessage; index: number; runEvent?: RunEvent }) {
  const result = buildAssistantResult(props.message.content, props.runEvent);
  const isThinking = !props.runEvent;
  return (
    <article className="thread-record assistant">
      <RecordHead index={props.index} role="执行结果" />
      <div className="thread-record-copy">
        <ResultSummary summary={result.summary} />
        {result.sections.map((section, index) => <ResultBlock key={`${props.message.id}-${index}`} section={section} />)}
      </div>
      {isThinking ? <ThinkingDots /> : null}
    </article>
  );
}

function RecordHead(props: { role: string; index: number }) {
  return (
    <div className="thread-record-head">
      <span className="thread-record-role">{props.role}</span>
      <span className="bubble-index">{formatEntryIndex(props.index + 1)}</span>
    </div>
  );
}

function ResultSummary(props: { summary: string }) {
  return (
    <section className="result-block result-block-summary">
      <strong>最终结论</strong>
      <p>{props.summary}</p>
    </section>
  );
}

function ResultBlock(props: { section: ResultSection }) {
  return (
    <section className={`result-block result-block-${props.section.kind}`}>
      <strong>{props.section.title}</strong>
      <p>{props.section.text}</p>
    </section>
  );
}

function ToolEventFeed(props: { events: RunEvent[] }) {
  return (
    <div className="tool-event-feed">
      {props.events.map((event) => (
        <ToolEventBadge key={event.event_id} event={event} />
      ))}
    </div>
  );
}

function ToolEventBadge(props: { event: RunEvent }) {
  const label = getToolEventLabel(props.event);
  const isCompleted = isToolCompletedEvent(props.event);
  if (!label) return null;
  if (isCompleted) {
    return <div className="status-badge status-completed">✅ {label}</div>;
  }
  return (
    <div className="status-badge status-running">
      <span className="tool-event-spinner" aria-hidden="true" />
      ⚙️ 正在{label}
    </div>
  );
}

function ThinkingDots() {
  return (
    <div className="thinking-dots" aria-label="正在思考" role="status">
      <span />
      <span />
      <span />
    </div>
  );
}

function ConfirmationRecord(props: { props: ChatPanelProps }) {
  return (
    <section className="thread-record confirmation">
      <div className="thread-record-head">
        <span className="thread-record-role">待确认项</span>
        <span className="thread-tag">待确认</span>
      </div>
      <ConfirmationCard
        confirmation={props.props.confirmation as ConfirmationRequest}
        rememberChoice={props.props.rememberChoice}
        showRiskLevel={props.props.showRiskLevel}
        onDecision={props.props.onConfirmationDecision}
        onRememberChoiceChange={props.props.onRememberChoiceChange}
      />
    </section>
  );
}

function EmptyWorkbench(props: { settings: SettingsResponse | null }) {
  return (
    <div>
      <EmptyStateBlock title="没有任务记录" text="输入明确目标后，这里会持续显示执行结果。" />
      <div className="empty-status-strip">
        <span>{props.settings?.model.display_name || "未加载模型"}</span>
        <span>{props.settings?.workspace.name || "未加载工作区"}</span>
      </div>
    </div>
  );
}

function PendingMessageState(props: { taskTitle: string }) {
  return (
    <div className="pending-thread">
      <article className="thread-record user">
        <RecordHead index={0} role="任务输入" />
        <div className="thread-record-copy"><p>{props.taskTitle}</p></div>
      </article>
      <StateRecord
        state="running"
        title={readRunStateHeadline("submitting")}
        body={readRunStateBody({ currentTaskTitle: props.taskTitle, runState: "submitting" })}
        advice={readRunStateNextStep({ runState: "submitting" })}
      />
    </div>
  );
}

function PrimaryErrorState(props: { latestFailureEvent?: RunEvent; submitError?: string | null }) {
  return (
    <StateRecord
      state="failed"
      title="任务没有可读结果"
      body={readFailureBody(props.latestFailureEvent, props.submitError)}
      advice={readFailureAdvice(props.latestFailureEvent)}
    />
  );
}

function RecoveryRecord(props: { latestFailureEvent?: RunEvent; submitError?: string | null }) {
  if (!props.submitError && !props.latestFailureEvent) return null;
  return (
    <StateRecord
      state="failed"
      title={readRunStateHeadline("failed", props.latestFailureEvent)}
      body={readRunStateBody({ latestFailureEvent: props.latestFailureEvent, runState: "failed", submitError: props.submitError })}
      advice={readRunStateNextStep({ latestFailureEvent: props.latestFailureEvent, runState: "failed" })}
    />
  );
}

function CompletionRecord(props: { props: ChatPanelProps }) {
  if (props.props.runState !== "completed" || props.props.messages.length === 0) return null;
  return (
    <StateRecord
      state="running"
      title={readRunStateHeadline("completed")}
      body={readRunStateBody({ runState: "completed" })}
      advice={readRunStateNextStep({ runState: "completed" })}
    />
  );
}

function readThreadTailRecord(props: ChatPanelProps) {
  if (shouldShowConfirmationRecord(props.runState, props.confirmation)) {
    return <ConfirmationRecord props={props} />;
  }
  if (shouldShowInlineFailureNotice(props.runState, props.messages, props.submitError, props.latestFailureEvent)) {
    return <RecoveryRecord latestFailureEvent={props.latestFailureEvent} submitError={props.submitError} />;
  }
  if (props.runState === "completed") {
    return <CompletionRecord props={props} />;
  }
  return null;
}

function StateRecord(props: {
  state: "failed" | "running";
  title: string;
  body: string;
  advice: string;
}) {
  return (
    <article className={`thread-record state-record state-record-${props.state}`}>
      <div className="thread-record-copy">
        <strong>{props.title}</strong>
        <p>{props.body}</p>
        <p>{props.advice}</p>
      </div>
    </article>
  );
}

function TaskComposer(props: { props: ChatPanelProps }) {
  return (
    <form className="composer" onSubmit={props.props.onSubmit}>
      <ComposerHeader />
      <ComposerInput
        composeValue={props.props.composeValue}
        isDisabled={!props.props.settings || props.props.isRunning}
        onComposeValueChange={props.props.onComposeValueChange}
      />
      <ComposerFooter props={props.props} />
    </form>
  );
}

function ComposerHeader() {
  return (
    <SectionHeader kicker="输入" className="composer-header" title="输入任务" />
  );
}

function ComposerInput(props: {
  composeValue: string;
  isDisabled: boolean;
  onComposeValueChange: (value: string) => void;
}) {
  return (
    <textarea
      className="composer-input"
      aria-label="任务输入"
      rows={4}
      value={props.composeValue}
      disabled={props.isDisabled}
      placeholder="描述要完成的任务"
      onChange={(event) => props.onComposeValueChange(event.target.value)}
    />
  );
}

function ComposerFooter(props: { props: ChatPanelProps }) {
  const isDisabled = !props.props.settings || props.props.isRunning || !props.props.composeValue.trim();
  return (
    <div className="composer-footer">
      <div className="composer-meta">
        <span>{props.props.settings?.model.display_name || "未加载模型"}</span>
        <span>{props.props.settings?.workspace.name || "未加载工作区"}</span>
      </div>
      <button className="primary-action" type="submit" disabled={isDisabled}>
        {props.props.isRunning ? "发送任务中" : "发送任务"}
      </button>
    </div>
  );
}

function SummaryChip(props: { label: string; value: string }) {
  return <MetricChip className="metric-chip" label={props.label} value={props.value} />;
}

function splitMessage(content: string) {
  const parts = content.split(/\n{2,}/).map((item) => item.trim()).filter(Boolean);
  return parts.length > 0 ? parts : [content];
}

function findTerminalRunEvent(props: ChatPanelProps, runId?: string) {
  if (!runId) return undefined;
  return [...props.events].reverse().find((event) => {
    if (event.run_id !== runId) return false;
    return event.event_type === "run_finished" || event.event_type === "run_failed";
  });
}

function getToolEvents(events: RunEvent[]) {
  return events.filter((event) => isToolStartedEvent(event) || hasToolName(event));
}

function isToolStartedEvent(event: RunEvent) {
  return event.event_type === "tool_started";
}

function isToolCompletedEvent(event: RunEvent) {
  return event.event_type === "tool_completed" || event.event_type === "tool_finished" || event.event_type === "action_completed";
}

function hasToolName(event: RunEvent) {
  return Boolean(getToolEventLabel(event));
}

function getToolEventLabel(event: RunEvent) {
  return (
    event.tool_display_name ||
    event.tool_call_snapshot?.display_name ||
    event.tool_name ||
    event.tool_category ||
    event.metadata?.tool_name ||
    ""
  );
}
