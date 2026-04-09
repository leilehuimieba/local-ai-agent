use crate::contracts::{ConfirmationRequest, RunRequest};
use crate::paths::resolve_workspace_path;
use crate::planner::{normalize_mode, PlannedAction};

#[derive(Clone, Debug)]
pub(crate) enum RiskOutcome {
    Proceed,
    RequireConfirmation(ConfirmationRequest),
    Blocked(String),
}

pub(crate) fn assess_risk(request: &RunRequest, action: &PlannedAction) -> RiskOutcome {
    let mode = normalize_mode(&request.mode);
    let workspace_first_seen = request
        .context_hints
        .get("workspace_first_seen")
        .map(|value| value == "true")
        .unwrap_or(false);

    if workspace_first_seen {
        let approved = request
            .confirmation_decision
            .as_ref()
            .map(|decision| decision.decision == "approve")
            .unwrap_or(false);
        if !approved {
            return RiskOutcome::RequireConfirmation(ConfirmationRequest {
                confirmation_id: format!("confirm-workspace-{}", request.run_id),
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
            });
        }
    }

    let high_risk = high_risk_confirmation(request, action);
    if matches!(mode.as_str(), "observe") && is_mutating_action(action) {
        return RiskOutcome::Blocked(
            "当前处于观察模式，系统不会执行修改性动作。请切换到标准模式或全权限模式后重试。"
                .to_string(),
        );
    }

    if let Some(confirmation) = high_risk {
        let approved = request
            .confirmation_decision
            .as_ref()
            .map(|decision| {
                decision.decision == "approve"
                    && decision.confirmation_id == confirmation.confirmation_id
            })
            .unwrap_or(false);
        if !approved {
            return RiskOutcome::RequireConfirmation(confirmation);
        }
    }

    RiskOutcome::Proceed
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
