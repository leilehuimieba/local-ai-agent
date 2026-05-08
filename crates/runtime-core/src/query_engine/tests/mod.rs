use super::{RuntimeEnvelope, RuntimeRunState, replan_state, should_replan};
use crate::capabilities::{ToolCallResult, ToolDefinition, ToolExecutionTrace};
use crate::context_builder::{DynamicPromptBlock, ProjectPromptBlock, RuntimeContextEnvelope, StaticPromptBlock};
use crate::planner::{PlanEnvelope, PlannedAction};
use crate::query_engine::browser_followup::browser_interaction_verify_failed;
use crate::query_engine_testkit::testkit::{
    sample_checkpoint, sample_checkpoint_with_tool, sample_repo_context, sample_request, sample_session,
};
use crate::run_recover_action::resumed_prepared_state;
use crate::run_resume::apply_resume_checkpoint;
use crate::skill_catalog::SkillCatalog;
use crate::tool_registry::{ToolCall, runtime_tool_registry};
use crate::verify::{VerificationOutcome, VerificationReport};

mod browser_followup;
mod knowledge_followup;
mod resume;

pub(super) fn browser_failed_state() -> RuntimeRunState {
    sample_browser_state(browser_click_action(), "浏览器点击")
}

pub(super) fn browser_read_page_failed_state() -> RuntimeRunState {
    sample_browser_state(browser_read_page_action_for_test(), "浏览器回读")
}

pub(super) fn sample_runtime_envelope(request: crate::contracts::RunRequest) -> RuntimeEnvelope {
    RuntimeEnvelope {
        request,
        session_context: sample_session(),
        repo_context: sample_repo_context(),
        skill_catalog: SkillCatalog::default(),
        context_envelope: sample_context_envelope(),
        visible_tools: Vec::new(),
    }
}

pub(super) fn sample_context_envelope() -> RuntimeContextEnvelope {
    RuntimeContextEnvelope {
        user_input: "retry task".to_string(),
        mode: "standard".to_string(),
        workspace_root: "D:/repo".to_string(),
        static_block: StaticPromptBlock {
            role_prompt: String::new(),
            mode_prompt: String::new(),
        },
        project_block: ProjectPromptBlock {
            workspace_root: "D:/repo".to_string(),
            repo_summary: String::new(),
            doc_summary: String::new(),
        },
        dynamic_block: DynamicPromptBlock::default(),
    }
}

pub(super) fn sample_plan_envelope() -> PlanEnvelope {
    PlanEnvelope {
        goal: "浏览器交互".to_string(),
        current_step: "调用 MCP：browser/click".to_string(),
        remaining_steps: vec!["完成验证并决定是否收口".to_string()],
        stop_condition: "当前动作已验证通过".to_string(),
        max_iterations: 3,
        iteration_index: 1,
        needs_verification: true,
    }
}

pub(super) fn sample_browser_tool_call(action: PlannedAction) -> ToolCall {
    let display_name = browser_display_name(&action);
    ToolCall {
        action,
        spec: ToolDefinition {
            tool_name: display_name.replace('/', "__").replacen("MCP: ", "mcp__", 1),
            display_name,
            category: "mcp".to_string(),
            risk_level: "medium".to_string(),
            input_schema: "json".to_string(),
            output_kind: "json_preview".to_string(),
            requires_confirmation: true,
            model_schema: None,
        },
    }
}

pub(super) fn sample_browser_state(action: PlannedAction, task_title: &str) -> RuntimeRunState {
    let request = sample_request("browser_verify_failed");
    RuntimeRunState {
        envelope: sample_runtime_envelope(request),
        plan_envelope: sample_plan_envelope(),
        action: action.clone(),
        tool_call: sample_browser_tool_call(action),
        task_title: task_title.to_string(),
        analysis_detail: "test".to_string(),
        risk_outcome: crate::risk::RiskOutcome::Proceed,
        tool_trace: Some(sample_browser_trace()),
        verification_report: Some(sample_browser_report()),
    }
}

pub(super) fn sample_knowledge_state(user_input: &str) -> RuntimeRunState {
    let mut request = sample_request("knowledge_search");
    request.user_input = user_input.to_string();
    RuntimeRunState {
        envelope: knowledge_runtime_envelope(request),
        plan_envelope: sample_plan_envelope(),
        action: PlannedAction::SearchKnowledge {
            query: user_input.to_string(),
        },
        tool_call: sample_knowledge_tool_call(user_input),
        task_title: "知识检索".to_string(),
        analysis_detail: "test".to_string(),
        risk_outcome: crate::risk::RiskOutcome::Proceed,
        tool_trace: Some(sample_knowledge_trace()),
        verification_report: Some(sample_knowledge_report()),
    }
}

fn knowledge_runtime_envelope(request: crate::contracts::RunRequest) -> RuntimeEnvelope {
    RuntimeEnvelope {
        request,
        session_context: crate::session::SessionMemory::default(),
        repo_context: sample_repo_context(),
        skill_catalog: SkillCatalog::default(),
        context_envelope: sample_context_envelope(),
        visible_tools: Vec::new(),
    }
}

pub(super) fn browser_click_action() -> PlannedAction {
    PlannedAction::MCPCall {
        server_id: "browser".to_string(),
        tool_name: "click".to_string(),
        arguments_json: r##"{"page_id":"page_01","selector":"#advance"}"##.to_string(),
    }
}

pub(super) fn browser_select_action() -> PlannedAction {
    PlannedAction::MCPCall {
        server_id: "browser".to_string(),
        tool_name: "select".to_string(),
        arguments_json: r##"{"page_id":"page_01","selector":"#tier","value":"advanced"}"##.to_string(),
    }
}

pub(super) fn browser_submit_action() -> PlannedAction {
    PlannedAction::MCPCall {
        server_id: "browser".to_string(),
        tool_name: "submit".to_string(),
        arguments_json: r##"{"page_id":"page_01","selector":"#submit"}"##.to_string(),
    }
}

pub(super) fn browser_upload_action() -> PlannedAction {
    PlannedAction::MCPCall {
        server_id: "browser".to_string(),
        tool_name: "upload".to_string(),
        arguments_json: r##"{"page_id":"page_01","selector":"#upload","file_paths":["D:/repo/tmp/upload.txt"]}"##
            .to_string(),
    }
}

pub(super) fn browser_read_page_action_for_test() -> PlannedAction {
    PlannedAction::MCPCall {
        server_id: "browser".to_string(),
        tool_name: "read_page".to_string(),
        arguments_json: r#"{"page_id":"page_01"}"#.to_string(),
    }
}

fn browser_display_name(action: &PlannedAction) -> String {
    let PlannedAction::MCPCall {
        server_id, tool_name, ..
    } = action
    else {
        return "MCP: browser/click".to_string();
    };
    format!("MCP: {server_id}/{tool_name}")
}

fn sample_browser_trace() -> ToolExecutionTrace {
    ToolExecutionTrace {
        tool: ToolDefinition {
            tool_name: "mcp__browser__click".to_string(),
            display_name: "MCP: browser/click".to_string(),
            category: "mcp".to_string(),
            risk_level: "medium".to_string(),
            input_schema: "json".to_string(),
            output_kind: "json_preview".to_string(),
            requires_confirmation: true,
            model_schema: None,
        },
        action_summary: "调用 MCP 工具：browser/click".to_string(),
        result: tool_result(
            "MCP 工具 `browser/click` 已返回结果。",
            r##"{"ok":true,"page_id":"page_01","elapsed_ms":35}"##,
            r##"{"ok":true,"page_id":"page_01","elapsed_ms":35}"##,
            48,
            1200,
            "Runtime 调用外部工具并返回结果。",
        ),
    }
}

fn sample_knowledge_tool_call(user_input: &str) -> ToolCall {
    ToolCall {
        action: PlannedAction::SearchKnowledge {
            query: user_input.to_string(),
        },
        spec: ToolDefinition {
            tool_name: "knowledge_search".to_string(),
            display_name: "知识检索".to_string(),
            category: "knowledge_read".to_string(),
            risk_level: "low".to_string(),
            input_schema: "query".to_string(),
            output_kind: "text_preview".to_string(),
            requires_confirmation: false,
            model_schema: None,
        },
    }
}

fn sample_knowledge_trace() -> ToolExecutionTrace {
    ToolExecutionTrace {
        tool: ToolDefinition {
            tool_name: "knowledge_search".to_string(),
            display_name: "知识检索".to_string(),
            category: "knowledge_read".to_string(),
            risk_level: "low".to_string(),
            input_schema: "query".to_string(),
            output_kind: "text_preview".to_string(),
            requires_confirmation: false,
            model_schema: None,
        },
        action_summary: "检索知识".to_string(),
        result: tool_result(
            "已从本地知识源返回 3 条摘要结果。",
            "本地知识检索结果",
            "",
            20,
            30000,
            "综合 README、开发文档和知识索引返回高相关知识片段。",
        ),
    }
}

fn sample_knowledge_report() -> VerificationReport {
    report(
        "verified",
        "generic",
        "",
        "tool=knowledge_search;decision=allow",
        "知识检索验证通过",
        "可继续形成回答",
        10,
        20,
        30000,
    )
}

fn sample_browser_report() -> VerificationReport {
    report(
        "browser_interaction_insufficient",
        "browser_interaction",
        "target_not_found",
        "tool=mcp__browser__click;decision=allow",
        "浏览器交互验证未通过",
        "建议先补充 read_page 或页面回读结果，再决定是否继续交互。",
        35,
        48,
        1200,
    )
}

fn tool_result(
    summary: &str,
    final_answer: &str,
    detail_preview: &str,
    result_chars: usize,
    budget_chars: usize,
    reasoning: &str,
) -> ToolCallResult {
    ToolCallResult {
        summary: summary.to_string(),
        final_answer: final_answer.to_string(),
        artifact_path: None,
        detail_preview: detail_preview.to_string(),
        raw_output_ref: None,
        result_chars,
        single_result_budget_chars: budget_chars,
        single_result_budget_hit: false,
        error_code: None,
        elapsed_ms: if budget_chars == 1200 { 35 } else { 10 },
        retryable: false,
        success: true,
        memory_write_summary: None,
        reasoning_summary: reasoning.to_string(),
        cache_status: "bypass".to_string(),
        cache_reason: String::new(),
    }
}

fn report(
    code: &str,
    task_type: &str,
    browser_failure_type: &str,
    decision_ref: &str,
    summary: &str,
    next_step: &str,
    elapsed_ms: u64,
    result_chars: usize,
    budget_chars: usize,
) -> VerificationReport {
    let outcome = report_outcome(code, task_type, browser_failure_type, decision_ref, summary, next_step);
    let single_result_budget_hit = false;
    VerificationReport {
        outcome,
        tool_elapsed_ms: elapsed_ms,
        result_chars,
        single_result_budget_chars: budget_chars,
        single_result_budget_hit,
    }
}

fn report_outcome(
    code: &str,
    task_type: &str,
    browser_failure_type: &str,
    decision_ref: &str,
    summary: &str,
    next_step: &str,
) -> VerificationOutcome {
    VerificationOutcome {
        passed: code == "verified",
        code: code.to_string(),
        browser_failure_type: browser_failure_type.to_string(),
        browser_page_id: page_id(browser_failure_type),
        browser_selector: String::new(),
        browser_recovery_attempted: false,
        browser_recovery_exhausted: false,
        policy: report_policy(task_type),
        task_type: task_type.to_string(),
        evidence: vec![report_evidence(task_type)],
        evidence_count: 1,
        has_citation: false,
        fact_inference_split: false,
        capability_risk_checked: true,
        permission_boundary_respected: true,
        skill_hit_effective: true,
        skill_hit_reason: report_skill_reason(task_type),
        guard_downgraded: false,
        guard_decision_ref: decision_ref.to_string(),
        summary: summary.to_string(),
        next_step: next_step.to_string(),
    }
}

fn report_policy(task_type: &str) -> String {
    if task_type == "browser_interaction" {
        "confirm_browser_state_change".to_string()
    } else {
        "check_result_relevance".to_string()
    }
}

fn report_evidence(task_type: &str) -> String {
    format!(
        "summary={}",
        if task_type == "generic" { "knowledge" } else { "browser" }
    )
}

fn report_skill_reason(task_type: &str) -> String {
    if task_type == "generic" {
        "knowledge search ok".to_string()
    } else {
        "insufficient".to_string()
    }
}

fn page_id(browser_failure_type: &str) -> String {
    if browser_failure_type.is_empty() {
        String::new()
    } else {
        "page_01".to_string()
    }
}
