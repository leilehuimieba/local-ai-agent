use runtime_core::{
    ModelRef, ProviderRef, RunEvent, RunRequest, WorkspaceRef, persist_lifecycle_observations,
    rank_observations,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct EvalCase {
    query: String,
    expected_event_type: String,
    hit: bool,
    top_event_types: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct EvalReport {
    case_count: usize,
    hit_count: usize,
    top5_hit_rate: f64,
    threshold: f64,
    passed: bool,
    cases: Vec<EvalCase>,
}

fn sample_event(event_type: &str, stage: &str, sequence: u32, summary: &str) -> RunEvent {
    serde_json::from_value(json!({
        "event_id": format!("event-{sequence}"),
        "event_type": event_type,
        "trace_id": "trace-m3",
        "session_id": "session-m3",
        "run_id": "run-m3",
        "sequence": sequence,
        "timestamp": format!("{}", 1713000000000u128 + sequence as u128),
        "stage": stage,
        "summary": summary
    }))
    .expect("sample event must be valid")
}

fn sample_events() -> Vec<RunEvent> {
    vec![
        sample_event("run_started", "Analyze", 1, "start flow"),
        sample_event("analysis_ready", "Analyze", 2, "analysis memory quality"),
        sample_event("plan_ready", "Plan", 3, "plan retrieval pipeline"),
        sample_event("action_completed", "Observe", 4, "action search timeline"),
        sample_event(
            "verification_completed",
            "Verify",
            5,
            "verification ranking",
        ),
        sample_event("run_finished", "Finish", 6, "finish memory flow"),
    ]
}

fn eval_pairs() -> Vec<(&'static str, &'static str)> {
    vec![
        ("analysis", "analysis_ready"),
        ("plan", "plan_ready"),
        ("action", "action_completed"),
        ("verification", "verification_completed"),
        ("finish", "run_finished"),
    ]
}

fn sample_request() -> RunRequest {
    let workspace_id = format!("workspace-m3-eval-{}", unix_millis());
    RunRequest {
        request_id: "mem-17-request".to_string(),
        run_id: "run-m3".to_string(),
        session_id: "session-m3".to_string(),
        trace_id: "trace-m3".to_string(),
        user_input: "eval observations".to_string(),
        mode: "standard".to_string(),
        model_ref: ModelRef {
            provider_id: "local".to_string(),
            model_id: "test-model".to_string(),
            display_name: "test-model".to_string(),
        },
        provider_ref: ProviderRef::default(),
        workspace_ref: workspace_ref_for(&workspace_id),
        context_hints: BTreeMap::new(),
        resume_from_checkpoint_id: String::new(),
        resume_strategy: String::new(),
        confirmation_decision: None,
    }
}

fn workspace_ref_for(workspace_id: &str) -> WorkspaceRef {
    WorkspaceRef {
        workspace_id: workspace_id.to_string(),
        name: workspace_id.to_string(),
        root_path: ".".to_string(),
        is_active: true,
    }
}

fn unix_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}

fn eval_case(request: &RunRequest, query: &str, expected: &str) -> EvalCase {
    let report = rank_observations(request, query, 5);
    let top = report
        .items
        .iter()
        .map(|item| item.event_type.clone())
        .collect::<Vec<_>>();
    EvalCase {
        query: query.to_string(),
        expected_event_type: expected.to_string(),
        hit: top.iter().any(|item| item == expected),
        top_event_types: top,
    }
}

fn eval_report(request: &RunRequest) -> EvalReport {
    let threshold = 70.0;
    let cases = eval_pairs()
        .into_iter()
        .map(|(query, expected)| eval_case(request, query, expected))
        .collect::<Vec<_>>();
    let hit_count = cases.iter().filter(|item| item.hit).count();
    let top5_hit_rate = (hit_count as f64 * 100.0) / cases.len() as f64;
    EvalReport {
        case_count: cases.len(),
        hit_count,
        top5_hit_rate,
        threshold,
        passed: top5_hit_rate >= threshold,
        cases,
    }
}

fn main() {
    let request = sample_request();
    let _ = persist_lifecycle_observations(&request, &sample_events());
    let report = eval_report(&request);
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("eval report should serialize")
    );
}
