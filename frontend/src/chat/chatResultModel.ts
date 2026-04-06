import { RunState } from "../runtime/state";
import { ChatMessage, ConfirmationRequest, RunEvent } from "../shared/contracts";

export type ResultSection = {
  kind: "action" | "detail" | "next" | "risk";
  text: string;
  title: string;
};

type ResultField = {
  kind: ResultSection["kind"];
  text?: string;
  title?: string;
};

export const TASK_RESULT_SLOT_ORDER = ["summary", "action", "detail", "risk", "next"] as const;

export function buildAssistantResult(content: string, event?: RunEvent) {
  if (event) return buildAssistantEventResult(content, event);
  const paragraphs = readResultParagraphs(content);
  return {
    sections: orderResultSections(paragraphs.slice(1).map(toResultSection)),
    summary: paragraphs[0] || "没有附带额外结果。",
  };
}

export function buildResultFromFields(summary: string, fields: ResultField[]) {
  return {
    sections: orderResultSections(fields.filter(hasResultField).map(toFieldSection)),
    summary: summary || "没有附带额外结果。",
  };
}

export function readFailureTitle(event?: RunEvent) {
  return event?.metadata?.error_code || "任务执行失败";
}

export function readFailureBody(event?: RunEvent, submitError?: string | null) {
  return event?.detail || event?.result_summary || submitError || "任务没有返回新的结果消息。";
}

export function readFailureAdvice(event?: RunEvent) {
  return event?.metadata?.next_step || "检查 Runtime、模型配置或补充更具体的任务输入。";
}

export function readRunStateHeadline(runState: RunState | undefined, event?: RunEvent) {
  if (runState === "submitting") return "提交已发送";
  if (runState === "streaming") return "任务运行中";
  if (runState === "awaiting_confirmation") return "等待确认";
  if (runState === "resuming") return "任务恢复中";
  if (runState === "completed") return "任务已完成";
  if (runState === "failed") return readFailureTitle(event);
  return "等待任务";
}

export function readRunStateBody(args: {
  runState: RunState | undefined;
  currentTaskTitle?: string;
  latestFailureEvent?: RunEvent;
  submitError?: string | null;
}) {
  if (args.runState === "submitting") return `任务“${args.currentTaskTitle || "当前任务"}”已提交，正在等待首个结果和事件返回。`;
  if (args.runState === "streaming") return "当前任务正在执行，主线程、检查器和调查层会持续刷新。";
  if (args.runState === "awaiting_confirmation") return "当前任务已暂停，等待你处理确认后继续推进。";
  if (args.runState === "resuming") return "确认已提交，任务正在恢复执行。";
  if (args.runState === "completed") return "当前任务已完成，本轮不会继续产生新的运行步骤。";
  if (args.runState === "failed") return readFailureBody(args.latestFailureEvent, args.submitError);
  return "输入明确目标后，这里会持续显示当前任务推进。";
}

export function readRunStateNextStep(args: {
  runState: RunState | undefined;
  latestFailureEvent?: RunEvent;
  latestEvent?: RunEvent;
}) {
  if (args.runState === "submitting") return "继续等待系统建立运行流。";
  if (args.runState === "streaming") return "优先查看最近动作，等待新的阶段推进。";
  if (args.runState === "awaiting_confirmation") return "先处理当前确认，再继续观察后续动作。";
  if (args.runState === "resuming") return "等待恢复后的下一条结果或事件。";
  if (args.runState === "completed") return args.latestEvent?.metadata?.next_step || "查看最终结果，决定继续追问、验收或开始下一轮任务。";
  if (args.runState === "failed") return readFailureAdvice(args.latestFailureEvent);
  return "进入任务页并提交明确目标。";
}

export function formatEntryIndex(index: number) {
  return index < 10 ? `0${index}` : String(index);
}

export function shouldShowPendingMessages(runState: RunState | undefined, messages: ChatMessage[]) {
  return messages.length <= 1 && (runState === "submitting" || runState === "streaming" || runState === "resuming");
}

export function shouldShowMessageFailure(
  runState: RunState | undefined,
  messages: ChatMessage[],
  submitError: string | null | undefined,
  latestFailureEvent?: RunEvent,
) {
  return messages.length <= 1 && runState === "failed" && Boolean(submitError || latestFailureEvent);
}

export function shouldShowInlineFailureNotice(
  runState: RunState | undefined,
  messages: ChatMessage[],
  submitError: string | null | undefined,
  latestFailureEvent?: RunEvent,
) {
  return messages.length > 1 && runState === "failed" && Boolean(submitError || latestFailureEvent);
}

export function shouldShowConfirmationRecord(
  runState: RunState | undefined,
  confirmation: ConfirmationRequest | null,
) {
  return runState === "awaiting_confirmation" && Boolean(confirmation);
}

export function getStreamLiveLabel(runState: RunState | undefined, messageCount: number) {
  if (runState === "submitting") return "任务已提交，等待系统返回首个结果。";
  if (runState === "streaming" || runState === "resuming") return "任务执行流已更新。";
  if (runState === "awaiting_confirmation") return "当前任务需要确认后才能继续。";
  if (runState === "failed") return "当前任务执行失败。";
  if (runState === "completed" && messageCount > 0) return "当前任务已完成。";
  return "";
}

export function readThreadStatusClass(runState: RunState | undefined) {
  if (runState === "failed") return "status-failed";
  if (runState === "awaiting_confirmation") return "status-awaiting";
  if (runState === "completed") return "status-completed";
  if (runState === "streaming" || runState === "resuming" || runState === "submitting") return "status-running";
  return "status-idle";
}

function toResultSection(text: string): ResultSection {
  if (matchesResultKind(text, ["下一步", "建议", "后续"])) return createResultSection("next", "下一步建议", text);
  if (matchesResultKind(text, ["风险", "注意", "警告", "失败"])) return createResultSection("risk", "风险提醒", text);
  if (matchesResultKind(text, ["执行", "动作", "修改", "命令", "工具"])) return createResultSection("action", "关键动作", text);
  return createResultSection("detail", "补充说明", text);
}

function orderResultSections(sections: ResultSection[]) {
  const kinds = TASK_RESULT_SLOT_ORDER.filter((item) => item !== "summary") as ResultSection["kind"][];
  return kinds.flatMap((kind) => sections.filter((section) => section.kind === kind));
}

function hasResultField(field: ResultField) {
  return Boolean(field.text);
}

function toFieldSection(field: ResultField) {
  if (field.kind === "next") return createResultSection("next", field.title || "下一步建议", field.text as string);
  if (field.kind === "risk") return createResultSection("risk", field.title || "风险提醒", field.text as string);
  if (field.kind === "action") return createResultSection("action", field.title || "关键动作", field.text as string);
  return createResultSection("detail", field.title || "补充说明", field.text as string);
}

function createResultSection(kind: ResultSection["kind"], title: string, text: string) {
  return { kind, text, title };
}

function matchesResultKind(text: string, keywords: string[]) {
  return keywords.some((keyword) => text.includes(keyword));
}

function splitMessage(content: string) {
  const parts = content.split(/\n{2,}/).map((item) => item.trim()).filter(Boolean);
  return parts.length > 0 ? parts : [content];
}

function readResultParagraphs(content: string) {
  const parts = splitMessage(content).flatMap(splitResultBlock);
  return parts.length > 0 ? parts : [content];
}

function buildAssistantEventResult(content: string, event: RunEvent) {
  const summary = event.final_answer || content || event.summary || "没有附带额外结果。";
  return buildResultFromFields(summary, [
    { kind: "action", title: "当前动作", text: readAssistantAction(event) },
    { kind: "detail", title: "结果摘要", text: readAssistantSummary(event, summary) },
    { kind: "detail", title: "验证结果", text: event.verification_snapshot?.summary },
    { kind: "detail", title: "停在这里", text: event.completion_reason },
    { kind: "detail", title: "处理依据", text: event.context_snapshot?.reasoning_summary },
    { kind: "detail", title: "产物路径", text: event.artifact_path },
    { kind: "next", title: "下一步", text: event.metadata?.next_step },
  ]);
}

function readAssistantAction(event: RunEvent) {
  if (event.tool_display_name && event.tool_category) return `${event.tool_display_name} / ${event.tool_category}`;
  return event.tool_display_name || event.tool_name || event.tool_category || "";
}

function readAssistantSummary(event: RunEvent, summary: string) {
  if (event.result_summary && event.result_summary !== summary) return event.result_summary;
  return event.summary !== summary ? event.summary : "";
}

function splitResultBlock(block: string) {
  const lines = block.split("\n").map((item) => item.trim()).filter(Boolean);
  if (lines.length <= 2) return [block];
  return lines;
}
