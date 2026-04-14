import { LogEntry } from "../shared/contracts";
import { hasPermissionSignal, readPermissionTag } from "../shared/permissionFlow";

export type AuditFilter = "all" | "confirmation_chain" | "tool_elapsed" | "governance";

export function hasAuditSignal(log: LogEntry, filter: AuditFilter) {
  if (filter === "all") return true;
  if (filter === "confirmation_chain") return hasConfirmationChainSignal(log);
  if (filter === "tool_elapsed") return hasToolElapsedSignal(log);
  return hasGovernanceSignal(log);
}

export function hasConfirmationChainSignal(log: LogEntry) {
  return Boolean(
    readConfirmationStep(log)
      || readMetadata(log, "confirmation_decision")
      || readMetadata(log, "confirmation_resume_strategy")
      || readMetadata(log, "checkpoint_id"),
  ) || hasPermissionSignal(log);
}

export function hasToolElapsedSignal(log: LogEntry) {
  return Boolean(readMetadata(log, "tool_elapsed_ms"));
}

export function hasGovernanceSignal(log: LogEntry) {
  return Boolean(
    readMetadata(log, "governance_status")
      || readMetadata(log, "governance_version")
      || readMetadata(log, "governance_reason")
      || readMetadata(log, "archive_reason"),
  );
}

export function readAuditTags(log: LogEntry) {
  return compactTags([
    readPermissionTag(log),
    readToolElapsedTag(log),
    readConfirmationTag(log),
    readGovernanceTag(log),
    readArchiveTag(log),
  ]);
}

function readToolElapsedTag(log: LogEntry) {
  const elapsed = readMetadata(log, "tool_elapsed_ms");
  return elapsed ? `耗时 ${elapsed}ms` : "";
}

function readConfirmationTag(log: LogEntry) {
  const step = readConfirmationStep(log);
  return step ? `确认链 ${step}` : "";
}

function readConfirmationStep(log: LogEntry) {
  return readMetadata(log, "confirmation_chain_step")
    || readMetadata(log, "permission_flow_step")
    || readMetadata(log, "confirmation_resume_strategy")
    || "";
}

function readGovernanceTag(log: LogEntry) {
  const status = readMetadata(log, "governance_status");
  return status ? `治理 ${status}` : "";
}

function readArchiveTag(log: LogEntry) {
  return readMetadata(log, "archive_reason") ? "含归档理由" : "";
}

function readMetadata(log: LogEntry, key: string) {
  return log.metadata?.[key]?.trim() || "";
}

function compactTags(tags: string[]) {
  const unique = new Set(tags.filter(Boolean));
  return [...unique].slice(0, 3);
}
