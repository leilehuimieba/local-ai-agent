use super::{knowledge_summary, knowledge_type};
    use crate::capabilities::{ToolDefinition, ToolExecutionTrace};
    use crate::verify::{VerificationOutcome, VerificationReport};

    #[test]
    fn knowledge_type_accepts_agent_resolve_when_verified() {
        let trace = sample_trace("agent_resolve", "任务已完成", "可复用流程说明");
        let report = sample_report(true);
        assert_eq!(
            knowledge_type(&trace, &report),
            Some("workflow_pattern".to_string())
        );
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
                policy: "check_result_summary".to_string(),
                evidence: vec![],
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
