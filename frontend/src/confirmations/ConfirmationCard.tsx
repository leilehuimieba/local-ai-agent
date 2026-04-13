import { useEffect, useState } from "react";

import { ConfirmationRequest } from "../shared/contracts";

type ConfirmationDecision = "approve" | "reject" | "cancel";
type ConfirmationFeedback = {
  tone: "running" | "failed" | "completed";
  message: string;
};

type ConfirmationCardProps = {
  confirmation: ConfirmationRequest;
  rememberChoice: boolean;
  showRiskLevel: boolean;
  onRememberChoiceChange: (checked: boolean) => void;
  onDecision: (decision: ConfirmationDecision) => Promise<void>;
};

export function ConfirmationCard(props: ConfirmationCardProps) {
  const [pendingDecision, setPendingDecision] = useState<ConfirmationDecision | null>(null);
  const [feedback, setFeedback] = useState<ConfirmationFeedback | null>(null);
  useEffect(() => {
    setPendingDecision(null);
    setFeedback(null);
  }, [props.confirmation.confirmation_id]);
  const riskClass = props.confirmation.risk_level === 'high' ? 'risk-high' : '';
  return (
    <div className={`confirmation-card confirmation-card-warning ${riskClass}`}>
      <ConfirmationHeader confirmation={props.confirmation} showRiskLevel={props.showRiskLevel} />
      <div className="panel confirmation-layout">
        <div className="confirmation-left">
          <ConfirmationTerminalView confirmation={props.confirmation} />
        </div>
        <div className="confirmation-right">
          <RememberChoice
            confirmation={props.confirmation}
            disabled={Boolean(pendingDecision)}
            rememberChoice={props.rememberChoice}
            onRememberChoiceChange={props.onRememberChoiceChange}
          />
          {feedback ? <ConfirmationFeedbackBlock feedback={feedback} /> : null}
          <ConfirmationActions
            pendingDecision={pendingDecision}
            onDecision={createDecisionHandler(props.onDecision, setFeedback, setPendingDecision)}
          />
        </div>
      </div>
    </div>
  );
}

function ConfirmationHeader(props: {
  confirmation: ConfirmationRequest;
  showRiskLevel: boolean;
}) {
  return (
    <div className="confirmation-header" style={{ padding: "16px", borderBottom: "1px solid var(--border-subtle)" }}>
      <div>
        <span className="section-kicker">确认</span>
        <h3>执行确认</h3>
      </div>
      {props.showRiskLevel ? <span className={`risk-pill ${readRiskClass(props.confirmation.risk_level)}`}>{props.confirmation.risk_level}</span> : <span className="status-badge status-awaiting">待确认</span>}
    </div>
  );
}

function buildTerminalDetails(c: ConfirmationRequest) {
  const paths = c.target_paths.join('\n') || "(None)";
  const hazards = c.hazards.join('\n') || "(None)";
  const alts = c.alternatives.join('\n') || "(None)";
  return `# Action: ${c.kind}\n# Scope: ${c.impact_scope}\n# Risk: ${c.risk_level}\n\n> Summary:\n${c.action_summary}\n\n> Targets:\n${paths}\n\n> Reason:\n${c.reason}\n\n> Hazards:\n${hazards}\n\n> Alternatives:\n${alts}`;
}

function ConfirmationTerminalView({ confirmation }: { confirmation: ConfirmationRequest }) {
  return (
    <div className="confirmation-terminal">
      <strong>Execution Details</strong>
      {buildTerminalDetails(confirmation)}
    </div>
  );
}

function RememberChoice(props: {
  confirmation: ConfirmationRequest;
  disabled: boolean;
  rememberChoice: boolean;
  onRememberChoiceChange: (checked: boolean) => void;
}) {
  if (props.confirmation.kind !== "workspace_access") return null;
  const fieldId = `remember-choice-${props.confirmation.confirmation_id}`;
  return (
    <label className="remember-row">
      <input
        id={fieldId}
        name={fieldId}
        type="checkbox"
        checked={props.rememberChoice}
        disabled={props.disabled}
        onChange={(event) => props.onRememberChoiceChange(event.target.checked)}
      />
      记住选择
    </label>
  );
}

function ConfirmationFeedbackBlock(props: { feedback: ConfirmationFeedback }) {
  return <p className={`settings-inline-feedback settings-inline-feedback-${props.feedback.tone}`}>{props.feedback.message}</p>;
}

function ConfirmationActions(props: {
  pendingDecision: ConfirmationDecision | null;
  onDecision: (decision: ConfirmationDecision) => void;
}) {
  return (
    <>
      <button type="button" className="btn-high-contrast btn-approve" onClick={() => props.onDecision("approve")} disabled={Boolean(props.pendingDecision)}>{readDecisionLabel("approve", props.pendingDecision)}</button>
      <button type="button" className="btn-high-contrast btn-reject" onClick={() => props.onDecision("reject")} disabled={Boolean(props.pendingDecision)}>{readDecisionLabel("reject", props.pendingDecision)}</button>
      <button type="button" className="btn-high-contrast btn-cancel" onClick={() => props.onDecision("cancel")} disabled={Boolean(props.pendingDecision)}>{readDecisionLabel("cancel", props.pendingDecision)}</button>
    </>
  );
}

function createDecisionHandler(
  onDecision: (decision: ConfirmationDecision) => Promise<void>,
  setFeedback: (value: ConfirmationFeedback | null) => void,
  setPendingDecision: (value: ConfirmationDecision | null) => void,
) {
  return (decision: ConfirmationDecision) => {
    void runDecision(decision, onDecision, setFeedback, setPendingDecision);
  };
}

async function runDecision(
  decision: ConfirmationDecision,
  onDecision: (decision: ConfirmationDecision) => Promise<void>,
  setFeedback: (value: ConfirmationFeedback | null) => void,
  setPendingDecision: (value: ConfirmationDecision | null) => void,
) {
  setPendingDecision(decision);
  setFeedback({ tone: "running", message: readPendingMessage(decision) });
  try {
    await onDecision(decision);
    setFeedback({ tone: "completed", message: readSuccessMessage(decision) });
  } catch (error) {
    setFeedback({ tone: "failed", message: readDecisionError(error, decision) });
  } finally {
    setPendingDecision(null);
  }
}

function readDecisionLabel(
  decision: ConfirmationDecision,
  pendingDecision: ConfirmationDecision | null,
) {
  if (pendingDecision === decision) return "提交中";
  if (decision === "approve") return "批准操作";
  if (decision === "reject") return "拒绝操作";
  return "取消处理";
}

function readPendingMessage(decision: ConfirmationDecision) {
  if (decision === "approve") return "正在提交批准请求。";
  if (decision === "reject") return "正在提交拒绝请求。";
  return "正在提交取消请求。";
}

function readSuccessMessage(decision: ConfirmationDecision) {
  if (decision === "approve") return "已提交批准，任务将继续执行。";
  if (decision === "reject") return "已提交拒绝，当前确认不会继续执行。";
  return "已提交取消，本次确认已结束。";
}

function readDecisionError(error: unknown, decision: ConfirmationDecision) {
  const prefix = decision === "approve" ? "批准提交失败" : decision === "reject" ? "拒绝提交失败" : "取消提交失败";
  return `${prefix}：${error instanceof Error ? error.message : "请重试"}`;
}

function readRiskClass(level: string) {
  if (level === "high") return "risk-high";
  if (level === "medium") return "risk-medium";
  return "risk-low";
}
