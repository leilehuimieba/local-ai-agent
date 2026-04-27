import { useEffect, useRef, useState } from "react";

import { ConfirmationCard } from "../confirmations/ConfirmationCard";
import { EmptyStateBlock, InfoCard, StatusPill } from "../ui/primitives";
import {
  buildAssistantResult,
  getStreamLiveLabel,
  readPendingAdvice,
  readPendingBody,
  readFailureAdvice,
  readFailureBody,
  readRunStateBody,
  readRunStateNextStep,
  ResultSection,
  shouldShowMessageFailure,
  shouldShowConfirmationRecord,
  shouldShowInlinePendingNotice,
  shouldShowInlineFailureNotice,
  shouldShowPendingMessages,
} from "./chatResultModel";
import { readUnifiedStatusMeta, RunState } from "../runtime/state";
import { ChatMessage, RunEvent } from "../shared/contracts";
import type { ChatPanelProps } from "./ChatPanel";
import { TaskComposer } from "./TaskComposer";

export function TaskThread(props: { props: ChatPanelProps }) {
  const messagesRef = useRef<HTMLDivElement>(null);
  const [autoScroll, setAutoScroll] = useState(true);

  useEffect(() => {
    if (autoScroll && messagesRef.current) {
      messagesRef.current.scrollTop = messagesRef.current.scrollHeight;
    }
  }, [props.props.messages.length, props.props.events.length, autoScroll]);

  const handleScroll = () => {
    const el = messagesRef.current;
    if (!el) return;
    const nearBottom = el.scrollHeight - el.scrollTop - el.clientHeight < 60;
    setAutoScroll(nearBottom);
  };

  return (
    <section className="stream-shell">
      <div className="sr-only" aria-live="polite">{getStreamLiveLabel(props.props.runState, props.props.messages.length)}</div>
      <div ref={messagesRef} className="messages" onScroll={handleScroll} aria-live="polite" aria-relevant="additions text">
        <ThreadContent props={props.props} />
      </div>
      <TaskComposer props={props.props} />
    </section>
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
  if (props.props.messages.length === 0 && props.props.events.length > 0) {
    return <EventOnlyState props={props.props} />;
  }
  if (props.props.messages.length === 0) {
    return <IdleWorkspace onExampleClick={props.props.onExampleClick} />;
  }
  return <ThreadRecords props={props.props} />;
}

function ConfirmationOnlyState(props: { props: ChatPanelProps }) {
  return (
    <>
      <IdleWorkspace onExampleClick={props.props.onExampleClick} />
      <ConfirmationRecord props={props.props} />
    </>
  );
}

function ThreadRecords(props: { props: ChatPanelProps }) {
  const tailRecord = readThreadTailRecord(props.props);
  return (
    <>
      {props.props.messages.map((message, index) => (
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
      <div className="thread-record-copy">
        {splitMessage(props.message.content).map((item, index) => <p key={`${props.message.id}-${index}`}>{item}</p>)}
      </div>
    </article>
  );
}

function AssistantRecord(props: { message: ChatMessage; index: number; runEvent?: RunEvent }) {
  const result = buildAssistantResult(props.message.content, props.runEvent);
  const isThinking = !props.runEvent;
  const isPlainText = result.sections.length === 0 && !result.statusTag;
  return (
    <article className={readAssistantRecordClass(result.mode)}>
      {isPlainText ? (
        <div className="thread-record-copy">
          {splitMessage(props.message.content).map((item, index) => <p key={`${props.message.id}-${index}`}>{item}</p>)}
        </div>
      ) : (
        <ResultBlockStack
          messageId={props.message.id}
          mode={result.mode}
          summaryLabel={result.summaryLabel}
          summary={result.summary}
          sections={result.sections}
        />
      )}
      {isThinking ? <ThinkingDots /> : null}
    </article>
  );
}

function ResultSummary(props: { label: string; mode: string; summary: string }) {
  return (
    <InfoCard className={`result-block result-block-summary result-block-summary-${props.mode}`}>
      <strong>{props.label}</strong>
      <p>{props.summary}</p>
    </InfoCard>
  );
}

function ResultBlock(props: { section: ResultSection }) {
  return (
    <InfoCard className={readResultBlockClass(props.section.kind)}>
      <strong>{props.section.title}</strong>
      <p>{props.section.text}</p>
    </InfoCard>
  );
}

function ResultBlockStack(props: {
  messageId: string;
  mode: string;
  summaryLabel: string;
  summary: string;
  sections: ResultSection[];
}) {
  const [expanded, setExpanded] = useState(false);
  const hasSections = props.sections.length > 0;
  return (
    <div className={`thread-record-copy result-block-stack result-block-stack-${props.mode}`}>
      <ResultSummary label={props.summaryLabel} mode={props.mode} summary={props.summary} />
      {hasSections ? (
        <div className="process-card">
          <button
            type="button"
            className="process-toggle"
            onClick={() => setExpanded((v) => !v)}
            aria-expanded={expanded}
          >
            {expanded ? "收起详细过程" : "查看详细过程"}
          </button>
          {expanded ? (
            <div className="process-details">
              {props.sections.map((section, index) => (
                <ResultBlock key={`${props.messageId}-${index}`} section={section} />
              ))}
            </div>
          ) : null}
        </div>
      ) : null}
    </div>
  );
}

function readAssistantRecordClass(mode: string) {
  return `thread-record assistant assistant-record assistant-record-${mode}`;
}

function readResultBlockClass(kind: ResultSection["kind"]) {
  const density = kind === "detail" ? "subtle" : "strong";
  return `result-block result-block-${kind} result-block-${density}`;
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
        <StatusPill className="status-awaiting" label="待确认" />
      </div>
      <ConfirmationCard
        confirmation={props.props.confirmation}
        rememberChoice={props.props.rememberChoice}
        showRiskLevel={props.props.showRiskLevel}
        onDecision={props.props.onConfirmationDecision}
        onRememberChoiceChange={props.props.onRememberChoiceChange}
      />
    </section>
  );
}

function EventOnlyState(props: { props: ChatPanelProps }) {
  return (
    <StateRecord
      state={readEventOnlyState(props.props.runState)}
      body={readRunStateBody({
        currentTaskTitle: props.props.currentTaskTitle,
        latestFailureEvent: props.props.latestFailureEvent,
        runState: props.props.runState,
        submitError: props.props.submitError,
      })}
      advice={readRunStateNextStep({
        latestEvent: props.props.events[props.props.events.length - 1],
        latestFailureEvent: props.props.latestFailureEvent,
        runState: props.props.runState,
      })}
    />
  );
}

function readEventOnlyState(runState: RunState) {
  if (runState === "failed") return "failed";
  if (runState === "completed") return "completed";
  return "running";
}

function PendingMessageState(props: { taskTitle: string; runState: RunState; currentRunId: string }) {
  return (
    <div className="pending-thread">
      <article className="thread-record user">
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
  body: string;
  advice: string;
}) {
  const badge = readStateBadge(props.state);
  const title = readCompactStateTitle(props.state);
  const copy = readCompactStateCopy(props.body, props.advice);
  return (
    <article className={readStateRecordClass(props.state)}>
      <div className="thread-record-head">
        <span className="thread-record-role">状态更新</span>
        <StatusPill className={badge.className} label={badge.label} />
      </div>
      <div className="thread-record-copy">
        <strong>{title}</strong>
        <p>{copy}</p>
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
      body={readPendingBody({ currentRunId: props.currentRunId, taskTitle: props.taskTitle })}
      advice={readPendingAdvice(props.runState)}
    />
  );
}

function readStateBadge(state: "failed" | "running" | "completed") {
  return readStateMeta(state);
}

function readStateMeta(state: "failed" | "running" | "completed") {
  if (state === "failed") return readUnifiedStatusMeta("failed");
  if (state === "completed") return readUnifiedStatusMeta("completed");
  return readUnifiedStatusMeta("running");
}

function readStateRecordClass(state: "failed" | "running" | "completed") {
  return `thread-record state-record state-record-${state}`;
}

function readCompactStateTitle(state: "failed" | "running" | "completed") {
  return readStateMeta(state).label;
}

function readCompactStateCopy(body: string, advice: string) {
  const head = body.trim();
  const tail = advice.trim();
  if (!head) return tail;
  if (!tail || head === tail) return head;
  if (head.includes(tail)) return head;
  if (tail.includes(head)) return tail;
  return `${head} ${tail}`;
}

function IdleWorkspace(props: { onExampleClick?: (value: string) => void }) {
  const examples = [
    { id: "fix-file", label: "修改项目文件", prompt: "帮我检查当前项目里最需要修的一个问题，并做最小改动修复" },
    { id: "build-debug", label: "执行命令并排错", prompt: "帮我运行构建命令，定位失败原因并给出修复建议" },
    { id: "docs-summary", label: "整理项目说明", prompt: "根据 docs 和当前代码，说明这个项目现在做到什么程度" },
    { id: "knowledge-search", label: "检索本地知识", prompt: "从本地文档中检索当前项目的正式需求和验收口径" },
  ] as const;

  return (
    <div className="idle-workspace">
      <div className="idle-hero">
        <h2 className="idle-hero-title">今天想让本地智能体帮你完成什么？</h2>
        <p className="idle-hero-subtitle">输入任务目标，AI 将自动执行并汇报进度</p>
      </div>
      <div className="idle-examples">
        {examples.map((ex) => (
          <button
            key={ex.id}
            className="idle-example-chip"
            type="button"
            onClick={() => props.onExampleClick?.(ex.prompt)}
          >
            <span className="idle-example-label">{ex.label}</span>
            <span className="idle-example-arrow">→</span>
          </button>
        ))}
      </div>
    </div>
  );
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
