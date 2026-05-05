use crate::contracts::{ConfirmationRequest, RunRequest};
use crate::executors::patch::preview_apply_patch_report;
use crate::paths::resolve_workspace_path;
use crate::planner::{PlannedAction, normalize_mode};
use crate::tool_registry::action_arguments_json;

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
        tool_name: String::new(),
        tool_arguments_json: String::new(),
        patch_preview_report_json: String::new(),
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
            "当前处于观察模式，系统不会执行修改性动作。请切换到标准模式或全权限模式后重试。".to_string(),
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
    request
        .confirmation_decision
        .as_ref()
        .is_some_and(|decision| decision.decision == "approve" && decision.confirmation_id == expected_id)
}

fn high_risk_confirmation(request: &RunRequest, action: &PlannedAction) -> Option<ConfirmationRequest> {
    match action {
        PlannedAction::DeletePath { path } => delete_confirmation(request, action, path),
        PlannedAction::ApplyPatch { diff, .. } => Some(patch_confirmation(request, action, diff)),
        PlannedAction::RunCommand { command } if is_dangerous_command(command) => {
            Some(command_confirmation(request, action, command))
        }
        _ => None,
    }
}

fn delete_confirmation(request: &RunRequest, action: &PlannedAction, path: &str) -> Option<ConfirmationRequest> {
    let target = resolve_workspace_path(&request.workspace_ref.root_path, path)
        .ok()
        .map(|item| item.display().to_string())
        .unwrap_or_else(|| path.to_string());
    Some(base_high_risk_confirmation(
        request,
        "irreversible",
        RiskConfirmationArgs {
            action_summary: format!("删除路径：{}", target),
            reason: "删除动作具有高风险，且可能无法回退；建议先读取或列出目标路径确认影响范围。".to_string(),
            impact_scope: "目标文件或目录，以及其下所有内容".to_string(),
            target_paths: vec![target],
            reversible: false,
            hazards: vec!["数据可能永久丢失".to_string()],
            alternatives: vec![
                "先读取或列出目标路径再确认".to_string(),
                "改成更小范围或更可回退的动作".to_string(),
            ],
            tool_name: "workspace_delete".to_string(),
            tool_arguments_json: action_arguments_json(action),
            patch_preview_report_json: String::new(),
        },
    ))
}

fn patch_confirmation(request: &RunRequest, action: &PlannedAction, diff: &str) -> ConfirmationRequest {
    let report = preview_apply_patch_report(request, diff);
    base_high_risk_confirmation(
        request,
        "medium",
        RiskConfirmationArgs {
            action_summary: "应用代码补丁".to_string(),
            reason: "patch 将修改工作区文件；建议先核对 dry-run 预览，再确认是否执行写入。".to_string(),
            impact_scope: "patch 涉及的目标文件".to_string(),
            target_paths: patch_target_paths(request, &report),
            reversible: true,
            hazards: vec!["可能改写多个文件内容".to_string()],
            alternatives: vec![
                "先查看 dry-run 预览".to_string(),
                "取消本次写入并调整 patch 参数".to_string(),
            ],
            tool_name: "workspace_apply_patch".to_string(),
            tool_arguments_json: action_arguments_json(action),
            patch_preview_report_json: report,
        },
    )
}

fn command_confirmation(request: &RunRequest, action: &PlannedAction, command: &str) -> ConfirmationRequest {
    base_high_risk_confirmation(
        request,
        "high",
        RiskConfirmationArgs {
            action_summary: format!("执行高风险命令：{}", command),
            reason: "命令中包含删除或不可逆变更特征；建议先确认命令作用范围和替代方案。".to_string(),
            impact_scope: "当前工作区及命令影响到的路径".to_string(),
            target_paths: vec![request.workspace_ref.root_path.clone()],
            reversible: false,
            hazards: vec!["可能删除文件或造成环境状态变化".to_string()],
            alternatives: vec![
                "先使用 list/read 检查目标".to_string(),
                "改成更安全的命令版本".to_string(),
            ],
            tool_name: "run_command".to_string(),
            tool_arguments_json: action_arguments_json(action),
            patch_preview_report_json: String::new(),
        },
    )
}

fn base_high_risk_confirmation(
    request: &RunRequest,
    risk_level: &str,
    args: RiskConfirmationArgs,
) -> ConfirmationRequest {
    ConfirmationRequest {
        confirmation_id: format!("confirm-risk-{}", request.run_id),
        run_id: request.run_id.clone(),
        risk_level: risk_level.to_string(),
        action_summary: args.action_summary,
        reason: args.reason,
        impact_scope: args.impact_scope,
        target_paths: args.target_paths,
        reversible: args.reversible,
        hazards: args.hazards,
        alternatives: args.alternatives,
        kind: "high_risk_action".to_string(),
        tool_name: args.tool_name,
        tool_arguments_json: args.tool_arguments_json,
        patch_preview_report_json: args.patch_preview_report_json,
    }
}

struct RiskConfirmationArgs {
    action_summary: String,
    reason: String,
    impact_scope: String,
    target_paths: Vec<String>,
    reversible: bool,
    hazards: Vec<String>,
    alternatives: Vec<String>,
    tool_name: String,
    tool_arguments_json: String,
    patch_preview_report_json: String,
}

fn patch_target_paths(request: &RunRequest, report: &str) -> Vec<String> {
    patch_paths_from_report(report).unwrap_or_else(|| vec![request.workspace_ref.root_path.clone()])
}

fn patch_paths_from_report(report: &str) -> Option<Vec<String>> {
    let value: serde_json::Value = serde_json::from_str(report).ok()?;
    let changes = value.get("changes")?.as_array()?;
    let paths = changes
        .iter()
        .filter_map(|item| item.get("path")?.as_str().map(str::to_string))
        .collect::<Vec<_>>();
    (!paths.is_empty()).then_some(paths)
}

fn is_mutating_action(action: &PlannedAction) -> bool {
    matches!(
        action,
        PlannedAction::WriteFile { .. }
            | PlannedAction::ApplyPatch { .. }
            | PlannedAction::DeletePath { .. }
            | PlannedAction::RunCommand { .. }
            | PlannedAction::MCPCall { .. }
    )
}

fn is_dangerous_command(command: &str) -> bool {
    let lower = command.to_lowercase();
    ["remove-item", "del ", " rd ", "rm ", "rm-", "rmdir", "erase "]
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
        request
            .context_hints
            .insert("workspace_first_seen".to_string(), "true".to_string());
        let outcome = assess_risk(&request, &PlannedAction::ListFiles { path: None });
        assert!(matches!(outcome, RiskOutcome::RequireConfirmation(_)));
    }

    #[test]
    fn blocks_mutating_action_in_observe_mode_before_confirmation_flow() {
        let request = sample_request("observe");
        let action = PlannedAction::RunCommand {
            command: "rm test.txt".to_string(),
        };
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
        let action = PlannedAction::RunCommand {
            command: "rm test.txt".to_string(),
        };
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
            model_ref: ModelRef {
                provider_id: "p".to_string(),
                model_id: "m".to_string(),
                display_name: "model".to_string(),
            },
            provider_ref: ProviderRef::default(),
            workspace_ref: WorkspaceRef {
                workspace_id: "w1".to_string(),
                name: "ws".to_string(),
                root_path: "D:/repo".to_string(),
                is_active: true,
            },
            context_hints: BTreeMap::new(),
            resume_from_checkpoint_id: String::new(),
            resume_strategy: String::new(),
            confirmation_decision: None,
        }
    }
}
