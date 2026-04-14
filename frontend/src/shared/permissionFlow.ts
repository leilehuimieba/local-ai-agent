type MetadataMap = Record<string, string> | undefined;

type PermissionCarrier = {
  completion_status?: string;
  event_type?: string;
  metadata?: MetadataMap;
};

export type PermissionSnapshot = {
  chainStep: string;
  decision: string;
  decisionSource: string;
  flowStep: string;
  resumeStrategy: string;
  ruleLayer: string;
};

export function readPermissionSnapshot(carrier: PermissionCarrier): PermissionSnapshot {
  return {
    chainStep: readMetadata(carrier.metadata, "confirmation_chain_step"),
    decision: readMetadata(carrier.metadata, "permission_decision"),
    decisionSource: readMetadata(carrier.metadata, "confirmation_decision_source"),
    flowStep: readMetadata(carrier.metadata, "permission_flow_step"),
    resumeStrategy: readMetadata(carrier.metadata, "confirmation_resume_strategy"),
    ruleLayer: readMetadata(carrier.metadata, "permission_rule_layer"),
  };
}

export function hasPermissionSignal(carrier: PermissionCarrier) {
  const snapshot = readPermissionSnapshot(carrier);
  return Boolean(
    snapshot.decision
      || snapshot.flowStep
      || snapshot.ruleLayer
      || snapshot.chainStep
      || snapshot.decisionSource
      || snapshot.resumeStrategy,
  );
}

export function isPermissionAwaiting(carrier: PermissionCarrier) {
  const snapshot = readPermissionSnapshot(carrier);
  if (carrier.event_type === "confirmation_required") return true;
  if (carrier.completion_status === "confirmation_required") return true;
  return snapshot.decision === "require_confirmation"
    || snapshot.flowStep === "ask_required"
    || snapshot.chainStep === "required";
}

export function isPermissionBlocked(carrier: PermissionCarrier) {
  const snapshot = readPermissionSnapshot(carrier);
  return snapshot.decision === "blocked"
    || snapshot.flowStep === "rule_blocked"
    || snapshot.chainStep === "rule_blocked";
}

export function isPermissionResolved(carrier: PermissionCarrier) {
  const snapshot = readPermissionSnapshot(carrier);
  return isResolvedFlowStep(snapshot.flowStep)
    || isResolvedChainStep(snapshot.chainStep);
}

export function readPermissionTag(carrier: PermissionCarrier) {
  if (!hasPermissionSignal(carrier)) return "";
  const snapshot = readPermissionSnapshot(carrier);
  const flowLabel = readPermissionFlowLabel(snapshot.flowStep);
  const decisionLabel = readPermissionDecisionLabel(snapshot.decision);
  return `权限 ${flowLabel || decisionLabel || "已记录"}`;
}

export function readPermissionSummary(carrier: PermissionCarrier) {
  if (!hasPermissionSignal(carrier)) return "";
  const snapshot = readPermissionSnapshot(carrier);
  const parts = [
    snapshot.decision ? `决策=${readPermissionDecisionLabel(snapshot.decision)}` : "",
    snapshot.flowStep ? `流程=${readPermissionFlowLabel(snapshot.flowStep)}` : "",
    snapshot.ruleLayer ? `守卫=${readPermissionRuleLabel(snapshot.ruleLayer)}` : "",
    snapshot.chainStep ? `链路=${readChainStepLabel(snapshot.chainStep)}` : "",
  ].filter(Boolean);
  if (parts.length === 0) return "权限链：已记录";
  return `权限链：${parts.join("；")}`;
}

export function readPermissionDecisionLabel(value: string) {
  if (value === "proceed") return "放行";
  if (value === "blocked") return "阻断";
  if (value === "require_confirmation") return "待确认";
  return value || "未附带";
}

export function readPermissionFlowLabel(value: string) {
  if (value === "rule_passed") return "规则放行";
  if (value === "rule_blocked") return "规则阻断";
  if (value === "ask_required") return "发起确认";
  if (value === "ask_approved") return "确认通过";
  if (value === "ask_reject") return "确认拒绝";
  if (value === "ask_cancel") return "确认取消";
  return value || "未附带";
}

export function readPermissionRuleLabel(value: string) {
  if (value === "workspace_guard") return "工作区守卫";
  if (value === "mode_guard") return "模式守卫";
  if (value === "high_risk_guard") return "高风险守卫";
  if (value === "risk_guard") return "风险守卫";
  if (value === "none") return "无";
  return value || "未附带";
}

export function readChainStepLabel(value: string) {
  if (value === "required") return "等待确认";
  if (value === "approved") return "确认通过";
  if (value === "closed") return "确认结束";
  if (value === "resumed") return "恢复执行";
  if (value === "resume_skipped") return "恢复跳过";
  if (value === "rule_blocked") return "规则阻断";
  return value || "未附带";
}

function isResolvedFlowStep(value: string) {
  return [
    "rule_passed",
    "rule_blocked",
    "ask_approved",
    "ask_reject",
    "ask_cancel",
  ].includes(value);
}

function isResolvedChainStep(value: string) {
  return ["approved", "closed", "resumed", "resume_skipped", "rule_blocked"].includes(value);
}

function readMetadata(metadata: MetadataMap, key: string) {
  return metadata?.[key]?.trim() || "";
}
