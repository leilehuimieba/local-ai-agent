use crate::contracts::{ConfirmationRequest, RunRequest};
use crate::paths::resolve_workspace_path;
use crate::planner::{PlannedAction, normalize_mode};

#[derive(Clone, Debug)]
pub(crate) enum RiskOutcome {
    Proceed,
    RequireConfirmation(ConfirmationRequest),
    Blocked(String),
}

pub(crate) fn assess_risk(request: &RunRequest, action: &PlannedAction) -> RiskOutcome {
    if let Some(outcome) = workspace_access_outcome(request) {
        return outcome;
    }
    if let Some(outcome) = mode_guard_outcome(request, action) {
        return outcome;
    }
    if let Some(outcome) = high_risk_outcome(request, action) {
        return outcome;
    }
    RiskOutcome::Proceed
}

fn workspace_access_outcome(request: &RunRequest) -> Option<RiskOutcome> {
    let confirmation_id = workspace_confirmation_id(request);
    if !workspace_first_seen(request) || confirmation_approved(request, &confirmation_id) {
        return None;
    }
    Some(RiskOutcome::RequireConfirmation(ConfirmationRequest {
        confirmation_id,
        run_id: request.run_id.clone(),
        risk_level: "medium".to_string(),
        action_summary: format!("首次接触目录：{}", request.workspace_ref.root_path),
        reason: "这是当前会话第一次进入该工作区，系统需要先确认是否继续。".to_string(),
        impact_scope: "当前工作区及其子目录".to_string(),
        target_paths: vec![request.workspace_ref.root_path.clone()],
        reversible: true,
        hazards: vec!["可能在陌生目录中执行读写或命令动作".to_string()],
        alternatives: vec!["取消本次任务".to_string(), "切回已信任的工作区".to_string()],
        kind: "workspace_access".to_string(),
    }))
}

fn workspace_confirmation_id(request: &RunRequest) -> String {
    format!("confirm-workspace-{}", request.run_id)
}

fn workspace_first_seen(request: &RunRequest) -> bool {
    request
        .context_hints
        .get("workspace_first_seen")
        .is_some_and(|value| value == "true")
}

fn mode_guard_outcome(request: &RunRequest, action: &PlannedAction) -> Option<RiskOutcome> {
    if matches!(normalize_mode(&request.mode).as_str(), "observe") && is_mutating_action(action) {
        return Some(RiskOutcome::Blocked(
            "当前处于观察模式，系统不会执行修改性动作。请切换到标准模式或全权限模式后重试。"
                .to_string(),
        ));
    }
    None
}

fn high_risk_outcome(request: &RunRequest, action: &PlannedAction) -> Option<RiskOutcome> {
    let confirmation = high_risk_confirmation(request, action)?;
    if confirmation_approved(request, &confirmation.confirmation_id) {
        return None;
    }
    Some(RiskOutcome::RequireConfirmation(confirmation))
}

fn confirmation_approved(request: &RunRequest, expected_id: &str) -> bool {
    request.confirmation_decision.as_ref().is_some_and(|decision| {
        decision.decision == "approve" && decision.confirmation_id == expected_id
    })
}

fn high_risk_confirmation(
    request: &RunRequest,
    action: &PlannedAction,
) -> Option<ConfirmationRequest> {
    match action {
        PlannedAction::DeletePath { path } => {
            let target = resolve_workspace_path(&request.workspace_ref.root_path, path)
                .ok()
                .map(|item| item.display().to_string())
                .unwrap_or_else(|| path.clone());
            Some(ConfirmationRequest {
                confirmation_id: format!("confirm-risk-{}", request.run_id),
                run_id: request.run_id.clone(),
                risk_level: "irreversible".to_string(),
                action_summary: format!("删除路径：{}", target),
                reason:
                    "删除动作具有高风险，且可能无法回退；建议先读取或列出目标路径确认影响范围。"
                        .to_string(),
                impact_scope: "目标文件或目录，以及其下所有内容".to_string(),
                target_paths: vec![target],
                reversible: false,
                hazards: vec!["数据可能永久丢失".to_string()],
                alternatives: vec![
                    "先读取或列出目标路径再确认".to_string(),
                    "改成更小范围或更可回退的动作".to_string(),
                ],
                kind: "high_risk_action".to_string(),
            })
        }
        PlannedAction::RunCommand { command } if is_dangerous_command(command) => {
            Some(ConfirmationRequest {
                confirmation_id: format!("confirm-risk-{}", request.run_id),
                run_id: request.run_id.clone(),
                risk_level: "high".to_string(),
                action_summary: format!("执行高风险命令：{}", command),
                reason: "命令中包含删除或不可逆变更特征；建议先确认命令作用范围和替代方案。"
                    .to_string(),
                impact_scope: "当前工作区及命令影响到的路径".to_string(),
                target_paths: vec![request.workspace_ref.root_path.clone()],
                reversible: false,
                hazards: vec!["可能删除文件或造成环境状态变化".to_string()],
                alternatives: vec![
                    "先使用 list/read 检查目标".to_string(),
                    "改成更安全的命令版本".to_string(),
                ],
                kind: "high_risk_action".to_string(),
            })
        }
        _ => None,
    }
}

fn is_mutating_action(action: &PlannedAction) -> bool {
    matches!(
        action,
        PlannedAction::WriteFile { .. }
            | PlannedAction::DeletePath { .. }
            | PlannedAction::RunCommand { .. }
    )
}

fn is_dangerous_command(command: &str) -> bool {
    let lower = command.to_lowercase();
    [
        "remove-item",
        "del ",
        " rd ",
        "rm ",
        "rm-",
        "rmdir",
        "erase ",
    ]
    .iter()
    .any(|token| lower.contains(token))
}

#[cfg(test)]
mod tests {
    use super::{RiskOutcome, assess_risk};
    use crate::contracts::{ConfirmationDecision, ModelRef, ProviderRef, RunRequest, WorkspaceRef};
    use crate::planner::PlannedAction;
    use std::collections::BTreeMap;

    #[test]
    fn requires_workspace_confirmation_on_first_seen_workspace() {
        let mut request = sample_request("standard");
        request.context_hints.insert("workspace_first_seen".to_string(), "true".to_string());
        let outcome = assess_risk(&request, &PlannedAction::ListFiles { path: None });
        assert!(matches!(outcome, RiskOutcome::RequireConfirmation(_)));
    }

    #[test]
    fn blocks_mutating_action_in_observe_mode_before_confirmation_flow() {
        let request = sample_request("observe");
        let action = PlannedAction::RunCommand { command: "rm test.txt".to_string() };
        let outcome = assess_risk(&request, &action);
        assert!(matches!(outcome, RiskOutcome::Blocked(_)));
    }

    #[test]
    fn skips_high_risk_confirmation_after_matching_approval() {
        let mut request = sample_request("standard");
        request.confirmation_decision = Some(ConfirmationDecision {
            confirmation_id: "confirm-risk-run-1".to_string(),
            run_id: "run-1".to_string(),
            decision: "approve".to_string(),
            note: String::new(),
            remember: false,
        });
        let action = PlannedAction::RunCommand { command: "rm test.txt".to_string() };
        let outcome = assess_risk(&request, &action);
        assert!(matches!(outcome, RiskOutcome::Proceed));
    }

    fn sample_request(mode: &str) -> RunRequest {
        RunRequest {
            request_id: "request-1".to_string(),
            run_id: "run-1".to_string(),
            session_id: "session-1".to_string(),
            trace_id: "trace-1".to_string(),
            user_input: "test".to_string(),
            mode: mode.to_string(),
            model_ref: ModelRef { provider_id: "p".to_string(), model_id: "m".to_string(), display_name: "model".to_string() },
            provider_ref: ProviderRef::default(),
            workspace_ref: WorkspaceRef { workspace_id: "w1".to_string(), name: "ws".to_string(), root_path: "D:/repo".to_string(), is_active: true },
            context_hints: BTreeMap::new(),
            resume_from_checkpoint_id: String::new(),
            resume_strategy: String::new(),
            confirmation_decision: None,
        }
    }
}
