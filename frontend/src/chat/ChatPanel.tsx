import { FormEvent } from "react";

import { ConfirmationCard } from "../confirmations/ConfirmationCard";
import {
  buildAssistantResult,
  formatEntryIndex,
  getStreamLiveLabel,
  readPendingAdvice,
  readPendingBody,
  readPendingHeadline,
  readFailureAdvice,
  readFailureBody,
  readRunStateBody,
  readRunStateHeadline,
  readRunStateNextStep,
  ResultSection,
  shouldShowMessageFailure,
  shouldShowConfirmationRecord,
  shouldShowInlinePendingNotice,
  shouldShowInlineFailureNotice,
  shouldShowPendingMessages,
} from "./chatResultModel";
import { RunState } from "../runtime/state";
import { ChatMessage, ConfirmationRequest, RunEvent, SettingsResponse } from "../shared/contracts";

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
    <article className="panel chat-panel chat-panel-simple">
      <TaskThread props={props} />
      <TaskComposer props={props} />
    </article>
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
    <div className="stream-header stream-header-simple">
      <strong>聊天</strong>
      <span className="chat-status-text">{props.props.statusLine}</span>
    </div>
  );
}

function ThreadContent(props: { props: ChatPanelProps }) {
  if (shouldShowPendingMessages(props.props.runState, props.props.messages, props.props.events, props.props.currentRunId)) {
    return (
      <PendingMessageState
        currentRunId={props.props.currentRunId}
        runState={props.props.runState}
        taskTitle={props.props.currentTaskTitle || "当前任务"}
      />
    );
  }
  if (shouldShowMessageFailure(props.props.runState, props.props.messages, props.props.submitError, props.props.latestFailureEvent)) {
    return <PrimaryErrorState latestFailureEvent={props.props.latestFailureEvent} submitError={props.props.submitError} />;
  }
  if (shouldShowConfirmationRecord(props.props.runState, props.props.confirmation)) {
    return <ConfirmationOnlyState props={props.props} />;
  }
  if (props.props.messages.length === 0) {
    return <EmptyWorkbench />;
  }
  return <ThreadRecords props={props.props} />;
}

function ConfirmationOnlyState(props: { props: ChatPanelProps }) {
  return (
    <>
      <EmptyWorkbench />
      <ConfirmationRecord props={props.props} />
    </>
  );
}

function ThreadRecords(props: { props: ChatPanelProps }) {
  const tailRecord = readThreadTailRecord(props.props);
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
      <RecordHead index={props.index} role={result.roleLabel} tag={result.statusTag} />
      <div className="thread-record-copy">
        <ResultSummary label={result.summaryLabel} summary={result.summary} />
        {result.sections.map((section, index) => <ResultBlock key={`${props.message.id}-${index}`} section={section} />)}
      </div>
      {isThinking ? <ThinkingDots /> : null}
    </article>
  );
}

function RecordHead(props: { role: string; index: number; tag?: string }) {
  return (
    <div className="thread-record-head">
      <div className="thread-record-meta">
        <span className="thread-record-role">{props.role}</span>
        {props.tag ? <span className="thread-tag">{props.tag}</span> : null}
      </div>
      <span className="bubble-index">{formatEntryIndex(props.index + 1)}</span>
    </div>
  );
}

function ResultSummary(props: { label: string; summary: string }) {
  return (
    <section className="result-block result-block-summary">
      <strong>{props.label}</strong>
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
    <section id="task-confirmation-anchor" className="thread-record confirmation" tabIndex={-1}>
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

function EmptyWorkbench() {
  return (
    <div className="chat-empty-simple">
      <strong>开始一个任务</strong>
      <p>输入目标后，这里会按时间顺序显示回复和执行状态。</p>
    </div>
  );
}

function PendingMessageState(props: { taskTitle: string; runState: RunState; currentRunId: string }) {
  return (
    <div className="pending-thread">
      <article className="thread-record user">
        <RecordHead index={0} role="任务输入" />
        <div className="thread-record-copy"><p>{props.taskTitle}</p></div>
      </article>
      <WaitingForFirstEventRecord taskTitle={props.taskTitle} runState={props.runState} currentRunId={props.currentRunId} />
    </div>
  );
}

function PrimaryErrorState(props: { latestFailureEvent?: RunEvent; submitError?: string | null }) {
  return (
    <StateRecord
      state="failed"
      title={readRunStateHeadline("failed", props.latestFailureEvent)}
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
      state="completed"
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
  if (shouldShowInlinePendingNotice(props.runState, props.messages, props.events, props.currentRunId)) {
    return (
      <WaitingForFirstEventRecord
        currentRunId={props.currentRunId}
        runState={props.runState}
        taskTitle={props.currentTaskTitle || "当前任务"}
      />
    );
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
  state: "failed" | "running" | "completed";
  title: string;
  body: string;
  advice: string;
}) {
  const badge = readStateBadge(props.state);
  return (
    <article className={`thread-record state-record state-record-${props.state}`}>
      <div className="thread-record-head">
        <span className="thread-record-role">{badge.label}</span>
        <span className={`status-badge ${badge.className}`}>{badge.label}</span>
      </div>
      <div className="thread-record-copy">
        <strong>{props.title}</strong>
        <p>{props.body}</p>
        <p>{props.advice}</p>
      </div>
    </article>
  );
}

function WaitingForFirstEventRecord(props: {
  taskTitle: string;
  runState: RunState;
  currentRunId: string;
}) {
  return (
    <StateRecord
      state="running"
      title={readPendingHeadline(props.runState)}
      body={readPendingBody({ currentRunId: props.currentRunId, taskTitle: props.taskTitle })}
      advice={readPendingAdvice(props.runState)}
    />
  );
}

function readStateBadge(state: "failed" | "running" | "completed") {
  if (state === "failed") return { className: "status-failed", label: "失败" };
  if (state === "completed") return { className: "status-completed", label: "完成" };
  return { className: "status-running", label: "运行中" };
}

function TaskComposer(props: { props: ChatPanelProps }) {
  const isDisabled = !props.props.settings || props.props.isRunning || !props.props.composeValue.trim();
  return (
    <form className="composer composer-simple" onSubmit={props.props.onSubmit}>
      <div className="simple-composer-shell">
        <input
          id="task-composer-input"
          className="simple-composer-input"
          aria-label="任务输入"
          type="text"
          value={props.props.composeValue}
          disabled={!props.props.settings || props.props.isRunning}
          placeholder="输入任务，按回车发送"
          onChange={(event) => props.props.onComposeValueChange(event.target.value)}
        />
        <button className="primary-action" type="submit" disabled={isDisabled}>
          {readSubmitLabel(props.props.isRunning)}
        </button>
      </div>
    </form>
  );
}

function readSubmitLabel(isRunning: boolean) {
  return isRunning ? "发送中" : "发送";
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
