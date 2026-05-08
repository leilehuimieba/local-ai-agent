use crate::capabilities::ToolExecutionTrace;
use crate::text::summarize_text;

pub(super) fn verification_evidence(trace: &ToolExecutionTrace) -> Vec<String> {
    let mut evidence = vec![format!("summary={}", summarize_text(&trace.result.summary))];
    evidence.push(format!("reasoning={}", summarize_text(&trace.result.reasoning_summary)));
    evidence.push(format!("result_chars={}", trace.result.result_chars));
    evidence.push(format!(
        "single_result_budget_chars={}",
        trace.result.single_result_budget_chars
    ));
    evidence.push(format!(
        "single_result_budget_hit={}",
        bool_text(trace.result.single_result_budget_hit)
    ));
    if let Some(path) = trace.result.artifact_path.as_ref() {
        evidence.push(format!("artifact={path}"));
    }
    evidence.push(format!("cache_status={}", trace.result.cache_status));
    evidence.push(format!("skill_hit_effective={}", bool_text(trace.result.success)));
    evidence.push(format!("guard_downgraded={}", bool_text(guard_downgraded(trace))));
    evidence.push(format!("guard_decision_ref={}", guard_decision_ref(trace)));
    evidence
}

pub(super) fn skill_hit_reason(trace: &ToolExecutionTrace, recovered: bool) -> String {
    if !trace.result.success {
        return "当前执行未成功，skill 命中未形成有效增益。".to_string();
    }
    if recovered {
        return "当前执行通过受控恢复完成，skill 命中产生部分有效增益。".to_string();
    }
    "当前执行成功，skill 命中对结果形成有效增益。".to_string()
}

pub(super) fn guard_downgraded(trace: &ToolExecutionTrace) -> bool {
    trace.result.reasoning_summary.contains("guard downgraded") || trace.result.summary.contains("guard downgraded")
}

pub(super) fn guard_decision_ref(trace: &ToolExecutionTrace) -> String {
    if guard_downgraded(trace) {
        return format!("tool={};decision=review", trace.tool.tool_name);
    }
    format!("tool={};decision=allow", trace.tool.tool_name)
}

fn bool_text(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}
