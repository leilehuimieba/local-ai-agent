import { LogEntry } from "../shared/contracts";
import { UnifiedStatusKey } from "../runtime/state";
import { readLogType } from "./logType";
import { isPermissionAwaiting, isPermissionBlocked, isPermissionResolved, readPermissionSummary } from "../shared/permissionFlow";

export function readHistoryStatusKey(log: LogEntry): UnifiedStatusKey {
  if (hasHistoryFailedSignal(log)) return "failed";
  if (hasHistoryAwaitingSignal(log)) return "awaiting_confirmation";
  if (hasHistoryCompletedSignal(log)) return "completed";
  const type = readLogType(log);
  if (type === "result" || type === "memory" || type === "verification") return "completed";
  return "running";
}

export function readTimelineTitle(log: LogEntry) {
  return log.metadata?.task_title || log.task_title || log.summary || "工作记录";
}

function hasHistoryFailedSignal(log: LogEntry) {
  return readLogType(log) === "error" || log.completion_status === "failed" || Boolean(log.error || log.metadata?.error_code) || isPermissionBlocked(log);
}

function hasHistoryAwaitingSignal(log: LogEntry) {
  if (isPermissionAwaiting(log)) return true;
  if (isPermissionResolved(log) || hasHistoryCompletedSignal(log)) return false;
  return readLogType(log) === "confirmation" || log.completion_status === "confirmation_required" || Boolean(log.confirmation_id);
}

function hasHistoryCompletedSignal(log: LogEntry) {
  return log.completion_status === "completed" || Boolean(log.final_answer) || isPermissionResolved(log);
}
