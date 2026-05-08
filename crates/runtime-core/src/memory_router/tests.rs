use super::knowledge_write::{knowledge_summary, knowledge_type};
use super::write_policy::{auto_memory_layer, duplicate_entry_outcome, govern_entry, reject_entry_outcome};
use super::{failure_lesson_entry, preference_entry};
use crate::capabilities::{ToolDefinition, ToolExecutionTrace};
use crate::contracts::{ModelRef, ProviderRef, RunRequest, WorkspaceRef};
use crate::verify::{VerificationOutcome, VerificationReport};
use std::collections::BTreeMap;

#[test]
fn knowledge_type_accepts_agent_resolve_when_verified() {
    let trace = sample_trace("agent_resolve", "任务已完成", "可复用流程说明");
    let report = sample_report(true);
    assert_eq!(knowledge_type(&trace, &report), Some("workflow_pattern".to_string()));
}

#[test]
fn knowledge_summary_falls_back_to_final_answer_when_short() {
    let trace = sample_trace(
        "agent_resolve",
        "完成",
        "这是一段可复用的较长知识摘要文本，用于验证回退策略有效。",
    );
    assert_eq!(
        knowledge_summary(&trace),
        "这是一段可复用的较长知识摘要文本，用于验证回退策略有效。"
    );
}

#[test]
fn auto_memory_workspace_summary_stays_working_only() {
    let trace = sample_trace("project_answer", "项目当前状态", "一次性状态回显");
    let entry = super::auto_memory_entry(&sample_request(), &trace, &sample_report(true));
    assert_eq!(auto_memory_layer(&entry), "working_only");
}

#[test]
fn preference_entry_goes_semantic_or_procedural() {
    let request = sample_request_with_input("请用中文回答，最小改动实现");
    let entry = preference_entry(&request).unwrap();
    let outcome = super::memory_written_outcome(&request, &entry, "用户偏好");
    assert_eq!(outcome.audit.memory_write_layer, "semantic_or_procedural_memory");
}

#[test]
fn failure_lesson_entry_goes_episodic() {
    let trace = sample_failed_trace("command", "连接被拒绝", "connection refused");
    let entry = failure_lesson_entry(&sample_request(), &trace, &sample_report(false)).unwrap();
    let outcome = super::memory_written_outcome(&sample_request(), &entry, "失败教训");
    assert_eq!(outcome.audit.memory_write_layer, "episodic_memory");
}

#[test]
fn duplicate_outcome_marks_duplicate_skipped() {
    let mut entry = preference_entry(&sample_request_with_input("请用中文回答")).unwrap();
    govern_entry(
        &mut entry,
        "semantic_or_procedural_memory",
        "accepted",
        "用户偏好具备长期复用价值。",
        "none",
    );
    let outcome = duplicate_entry_outcome(&mut entry, "命中重复用户偏好，跳过写入。");
    assert_eq!(outcome.audit.memory_write_decision, "duplicate_skipped");
    assert_eq!(outcome.audit.memory_duplicate_strategy, "same_kind_title_summary");
}

#[test]
fn rejected_workspace_summary_marks_working_only() {
    let trace = sample_trace("project_answer", "项目当前状态", "一次性状态回显");
    let mut entry = super::auto_memory_entry(&sample_request(), &trace, &sample_report(true));
    let outcome = reject_entry_outcome(
        &mut entry,
        "working_only",
        "当前结果更像一次性项目状态回显，不进入长期层。",
    );
    assert_eq!(outcome.layer, "working_only");
    assert_eq!(outcome.audit.memory_write_decision, "rejected");
    assert_eq!(outcome.audit.memory_write_layer, "working_only");
    assert!(outcome.audit.memory_write_reason.contains("不进入长期层"));
}

fn sample_trace(tool_name: &str, summary: &str, final_answer: &str) -> ToolExecutionTrace {
    ToolExecutionTrace {
        tool: sample_tool(tool_name),
        action_summary: "测试动作".to_string(),
        result: sample_result(summary, final_answer),
    }
}

fn sample_tool(tool_name: &str) -> ToolDefinition {
    ToolDefinition {
        tool_name: tool_name.to_string(),
        display_name: "测试工具".to_string(),
        category: "agent".to_string(),
        risk_level: "low".to_string(),
        input_schema: "none".to_string(),
        output_kind: "text_preview".to_string(),
        requires_confirmation: false,
        model_schema: None,
    }
}

fn sample_result(summary: &str, final_answer: &str) -> crate::capabilities::ToolCallResult {
    crate::capabilities::ToolCallResult {
        summary: summary.to_string(),
        final_answer: final_answer.to_string(),
        artifact_path: None,
        detail_preview: summary.to_string(),
        raw_output_ref: None,
        result_chars: summary.chars().count(),
        single_result_budget_chars: 30_000,
        single_result_budget_hit: false,
        error_code: None,
        elapsed_ms: 10,
        retryable: false,
        success: true,
        memory_write_summary: None,
        reasoning_summary: "测试推理".to_string(),
        cache_status: "bypass".to_string(),
        cache_reason: String::new(),
    }
}

fn sample_report(passed: bool) -> VerificationReport {
    VerificationReport {
        outcome: VerificationOutcome {
            passed,
            code: "verified".to_string(),
            browser_failure_type: String::new(),
            browser_page_id: String::new(),
            browser_selector: String::new(),
            browser_recovery_attempted: false,
            browser_recovery_exhausted: false,
            policy: "check_result_summary".to_string(),
            task_type: "generic".to_string(),
            evidence: vec![],
            evidence_count: 0,
            has_citation: false,
            fact_inference_split: false,
            capability_risk_checked: true,
            permission_boundary_respected: true,
            skill_hit_effective: passed,
            skill_hit_reason: "验证".to_string(),
            guard_downgraded: false,
            guard_decision_ref: "tool=agent_resolve;decision=allow".to_string(),
            summary: "验证".to_string(),
            next_step: "继续".to_string(),
        },
        tool_elapsed_ms: 10,
        result_chars: 0,
        single_result_budget_chars: 30_000,
        single_result_budget_hit: false,
    }
}

fn sample_request() -> RunRequest {
    sample_request_with_input("测试输入")
}

fn sample_request_with_input(user_input: &str) -> RunRequest {
    RunRequest {
        request_id: "request-1".to_string(),
        run_id: "run-1".to_string(),
        session_id: "session-1".to_string(),
        trace_id: "trace-1".to_string(),
        user_input: user_input.to_string(),
        mode: "standard".to_string(),
        model_ref: ModelRef {
            provider_id: "provider".to_string(),
            model_id: "model".to_string(),
            display_name: "Model".to_string(),
        },
        provider_ref: ProviderRef::default(),
        workspace_ref: WorkspaceRef {
            workspace_id: "workspace-1".to_string(),
            name: "Workspace".to_string(),
            root_path: "D:/repo".to_string(),
            is_active: true,
        },
        context_hints: BTreeMap::new(),
        resume_from_checkpoint_id: String::new(),
        resume_strategy: String::new(),
        confirmation_decision: None,
    }
}

fn sample_failed_trace(tool_name: &str, summary: &str, final_answer: &str) -> ToolExecutionTrace {
    let mut trace = sample_trace(tool_name, summary, final_answer);
    trace.result.success = false;
    trace.result.error_code = Some("connection_refused".to_string());
    trace
}
