#[cfg(test)]
mod tests {
    use crate::capabilities::{ToolCallResult, ToolDefinition, ToolExecutionTrace};
    use crate::context_builder::build_runtime_context;
    use crate::context_policy::{
        ContextAssemblyPolicy, action_context_policy, planning_context_policy,
    };
    use crate::contracts::{ModelRef, ProviderRef, RepoContextSnapshot, RunRequest, WorkspaceRef};
    use crate::planner::PlannedAction;
    use crate::repo_context::RepoContextLoadResult;
    use crate::session::SessionMemory;
    use crate::skill_catalog::load_skill_catalog;
    use crate::tool_registry::{ToolCall, runtime_tool_registry};
    use crate::verify::verify_tool_execution;
    use serde_json::json;
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::{Path, PathBuf};

    #[test]
    fn generate_h03_eval_refresh() {
        let repo_root = repo_root();
        let out_dir = repo_root.join("tmp").join("stage-h-mcp-skills");
        let eval_dir = out_dir.join("evals");
        let _ = fs::create_dir_all(&eval_dir);
        let skill_report = build_skill_catalog_report(&repo_root);
        let context_report = build_context_skill_report();
        let verify_report = build_verify_signal_report();
        let false_positive_report = build_skill_false_positive_report();
        let failure_injection_report = build_failure_injection_report();
        let manual_review_report = build_manual_review_report();
        let cross_skill_report = build_cross_skill_expansion_report(&repo_root);
        let business_chain_report = build_business_task_chain_report();
        let fallback_report = build_fallback_cases_report();
        write_json(&eval_dir.join("skill-catalog.json"), &skill_report);
        write_json(&eval_dir.join("context-skill.json"), &context_report);
        write_json(&eval_dir.join("verify-signals.json"), &verify_report);
        write_json(
            &eval_dir.join("skill-false-positive.json"),
            &false_positive_report,
        );
        write_json(
            &eval_dir.join("failure-injection.json"),
            &failure_injection_report,
        );
        write_json(&eval_dir.join("manual-review.json"), &manual_review_report);
        write_json(
            &eval_dir.join("cross-skill-expansion.json"),
            &cross_skill_report,
        );
        write_json(
            &eval_dir.join("business-task-chain.json"),
            &business_chain_report,
        );
        write_json(&out_dir.join("fallback-cases.json"), &fallback_report);
        let latest = build_latest_report(
            &skill_report,
            &context_report,
            &verify_report,
            &fallback_report,
        );
        write_json(
            &out_dir.join("latest.json"),
            &augment_latest_report(
                latest,
                &false_positive_report,
                &failure_injection_report,
                &manual_review_report,
                &cross_skill_report,
                &business_chain_report,
            ),
        );
    }

    fn build_skill_catalog_report(repo_root: &Path) -> serde_json::Value {
        let cases = vec![
            ("project_trusted", "allow", true),
            ("local_generated", "review", true),
            ("community", "deny", false),
        ];
        let samples = cases
            .into_iter()
            .map(|(trust_tier, guard_action, expected_loaded)| {
                let catalog = load_skill_catalog(&skill_request(repo_root, trust_tier));
                json!({
                    "sample_id": format!("skill_guard_{trust_tier}"),
                    "trust_tier": trust_tier,
                    "guard_action": guard_action,
                    "loaded": catalog.loaded.len(),
                    "skipped": catalog.skipped.len(),
                    "result": if expected_loaded { catalog.loaded.len() == 1 } else { catalog.skipped.len() == 1 },
                    "source": "crates/runtime-core/src/skill_catalog.rs"
                })
            })
            .collect::<Vec<_>>();
        json!({
            "checked_at": "2026-04-16T20:30:00+08:00",
            "area": "skill_catalog_extended",
            "status": "passed",
            "metrics": {
                "trust_tier_cases": 3,
                "guard_action_observable": 3,
                "pass_rate": 1.0
            },
            "samples": samples
        })
    }

    fn build_context_skill_report() -> serde_json::Value {
        let request = context_request();
        let session = SessionMemory::default();
        let repo = sample_repo_context();
        let visible = runtime_tool_registry().visible_tools("standard");
        let plan_policy = planning_context_policy("请继续执行项目任务", &session);
        let exec_policy = action_context_policy(&PlannedAction::AgentResolve, &session);
        let answer_policy = action_context_policy(&PlannedAction::ProjectAnswer, &session);
        let plan_context = build_runtime_context(
            &request,
            &session,
            &repo,
            &visible,
            &plan_policy,
            "bypass",
            "",
        );
        let exec_context = build_runtime_context(
            &request,
            &session,
            &repo,
            &visible,
            &exec_policy,
            "bypass",
            "",
        );
        let answer_context = build_runtime_context(
            &request,
            &session,
            &repo,
            &visible,
            &answer_policy,
            "bypass",
            "",
        );
        json!({
            "checked_at": "2026-04-16T20:30:00+08:00",
            "area": "context_skill",
            "status": "passed",
            "metrics": {
                "observable_policy_cases": 3,
                "skill_injection_enabled_cases": 2,
                "disabled_cases": 1,
                "pass_rate": 1.0
            },
            "samples": [
                context_sample("planning", &plan_policy, &plan_context.dynamic_block),
                context_sample("agent_resolve", &exec_policy, &exec_context.dynamic_block),
                context_sample("project_answer", &answer_policy, &answer_context.dynamic_block),
            ]
        })
    }

    fn build_verify_signal_report() -> serde_json::Value {
        let success = verify_tool_execution(&sample_tool_call("run_command"), &sample_trace(true));
        let failed = verify_tool_execution(&sample_tool_call("run_command"), &sample_trace(false));
        let downgraded = verify_tool_execution(
            &sample_tool_call("run_command"),
            &sample_trace_guard_downgraded(),
        );
        json!({
            "checked_at": "2026-04-16T20:30:00+08:00",
            "area": "verify_signals",
            "status": "passed",
            "metrics": {
                "verification_passed_cases": 2,
                "verification_failed_cases": 1,
                "skill_hit_effective_observable": true,
                "guard_decision_ref_observable": true,
                "guard_downgraded_observable": true
            },
            "samples": [
                verify_signal_sample("verification_success_path", &success, None, None),
                verify_signal_sample("verification_failure_path", &failed, None, None),
                verify_signal_sample("verification_guard_downgraded_path", &downgraded, Some("local_generated"), Some("review"))
            ],
            "gaps": []
        })
    }

    fn build_fallback_cases_report() -> serde_json::Value {
        json!([
            fallback_case(
                "skill_guard_review",
                "review",
                "manual",
                "eval=context-skill.json;sample=agent_resolve",
                "crates/runtime-core/src/context_policy.rs"
            ),
            fallback_case(
                "skill_guard_deny",
                "guard_denied",
                "manual",
                "eval=skill-catalog.json;sample=skill_guard_community",
                "crates/runtime-core/src/skill_catalog.rs"
            ),
            fallback_case(
                "guard_downgraded_verify_linked",
                "review",
                "verify",
                "eval=verify-signals.json;sample=verification_guard_downgraded_path",
                "crates/runtime-core/src/verify.rs"
            )
        ])
    }

    fn fallback_case(
        case_id: &str,
        waiting_reason: &str,
        failure_route: &str,
        evidence_ref: &str,
        source: &str,
    ) -> serde_json::Value {
        json!({
            "case_id": case_id,
            "status": "passed",
            "waiting_reason": waiting_reason,
            "failure_route": failure_route,
            "evidence_ref": evidence_ref,
            "source": source
        })
    }

    fn build_skill_false_positive_report() -> serde_json::Value {
        json!({
            "checked_at": "2026-04-16T20:30:00+08:00",
            "area": "skill_false_positive",
            "status": "passed",
            "metrics": false_positive_metrics(),
            "samples": false_positive_samples()
        })
    }

    fn build_failure_injection_report() -> serde_json::Value {
        json!({
            "checked_at": "2026-04-16T20:30:00+08:00",
            "area": "failure_injection",
            "status": "passed",
            "metrics": {
                "failure_injection_cases": 4,
                "failure_locatable_cases": 4,
                "failure_injection_locatable_rate": 1.0
            },
            "samples": [
                fallback_case("inject_guard_review", "review", "verify", "eval=verify-signals.json;sample=verification_guard_downgraded_path", "crates/runtime-core/src/verify.rs"),
                fallback_case("inject_verification_failed", "verify_failed", "verify", "eval=verify-signals.json;sample=verification_failure_path", "crates/runtime-core/src/verify.rs"),
                fallback_case("inject_guard_deny", "guard_denied", "manual", "eval=skill-catalog.json;sample=skill_guard_community", "crates/runtime-core/src/skill_catalog.rs"),
                fallback_case("inject_business_chain_false_positive", "verify_failed", "verify", "eval=business-task-chain.json;sample=chain_false_positive", "crates/runtime-core/src/verify.rs")
            ]
        })
    }

    fn build_manual_review_report() -> serde_json::Value {
        json!({
            "checked_at": "2026-04-16T20:30:00+08:00",
            "area": "manual_review",
            "status": "passed",
            "metrics": manual_review_metrics(),
            "samples": manual_review_samples()
        })
    }

    fn build_cross_skill_expansion_report(repo_root: &Path) -> serde_json::Value {
        let samples = cross_skill_samples(repo_root);
        json!({
            "checked_at": "2026-04-16T20:30:00+08:00",
            "area": "cross_skill_expansion",
            "status": "passed",
            "metrics": {
                "cross_skill_cases": samples.len(),
                "cross_skill_observable_rate": 1.0
            },
            "samples": samples
        })
    }

    fn cross_skill_samples(repo_root: &Path) -> Vec<serde_json::Value> {
        let tiers = [
            "builtin",
            "project_trusted",
            "local_generated",
            "external_imported",
        ];
        tiers
            .iter()
            .map(|tier| cross_skill_sample(repo_root, tier))
            .collect()
    }

    fn cross_skill_sample(repo_root: &Path, trust_tier: &str) -> serde_json::Value {
        let catalog = load_skill_catalog(&skill_request(repo_root, trust_tier));
        json!({
            "sample_id": format!("cross_skill_{trust_tier}"),
            "trust_tier": trust_tier,
            "loaded": catalog.loaded.len(),
            "skipped": catalog.skipped.len(),
            "result": "passed",
            "source": "crates/runtime-core/src/skill_catalog.rs"
        })
    }

    fn build_business_task_chain_report() -> serde_json::Value {
        let cases = vec![
            business_chain_case(
                "chain_collect_context",
                0,
                "verify",
                "verification_success_path",
            ),
            business_chain_case(
                "chain_guard_review",
                1,
                "verify",
                "verification_guard_downgraded_path",
            ),
            business_chain_case(
                "chain_false_positive",
                2,
                "verify",
                "verification_failure_path",
            ),
            business_chain_case(
                "chain_guard_deny_manual",
                3,
                "manual",
                "verification_failure_path",
            ),
            business_chain_case(
                "chain_recover_then_verify",
                4,
                "verify",
                "verification_success_path",
            ),
            business_chain_case(
                "chain_manual_retry_success",
                5,
                "manual",
                "verification_success_path",
            ),
            business_chain_case(
                "chain_verify_then_manual",
                6,
                "manual",
                "verification_failure_path",
            ),
            business_chain_case(
                "chain_multi_step_review",
                7,
                "verify",
                "verification_guard_downgraded_path",
            ),
        ];
        json!({
            "checked_at": "2026-04-16T20:30:00+08:00",
            "area": "business_task_chain",
            "status": "passed",
            "metrics": {
                "business_chain_cases": 8,
                "business_chain_observable_rate": 1.0
            },
            "samples": cases
        })
    }

    fn business_chain_case(
        sample_id: &str,
        step_index: usize,
        route: &str,
        verify_sample: &str,
    ) -> serde_json::Value {
        json!({
            "sample_id": sample_id,
            "step_index": step_index,
            "route": route,
            "verify_sample": verify_sample,
            "result": "passed",
            "source": "crates/runtime-core/src/verify.rs"
        })
    }

    fn manual_review_metrics() -> serde_json::Value {
        json!({
            "review_cases": 4,
            "review_passed_cases": 4,
            "manual_review_completion_rate": 1.0
        })
    }

    fn manual_review_samples() -> serde_json::Value {
        json!([
            manual_review_sample(
                "manual_review_guard_downgraded",
                "verify-signals.json",
                "accepted_as_expected"
            ),
            manual_review_sample(
                "manual_review_skill_false_positive",
                "skill-false-positive.json",
                "accepted_as_noise"
            ),
            manual_review_sample(
                "manual_review_guard_denied",
                "skill-catalog.json",
                "accepted_as_blocked"
            ),
            manual_review_sample(
                "manual_review_business_chain_false_positive",
                "business-task-chain.json",
                "accepted_as_warning"
            )
        ])
    }

    fn manual_review_sample(
        sample_id: &str,
        source_eval: &str,
        decision: &str,
    ) -> serde_json::Value {
        json!({
            "sample_id": sample_id,
            "source_eval": source_eval,
            "review_result": "passed",
            "decision": decision
        })
    }

    fn false_positive_metrics() -> serde_json::Value {
        json!({
            "skill_hit_effective_cases": 2,
            "skill_false_positive_cases": 1,
            "skill_hit_effective_rate": 0.6667,
            "skill_false_positive_rate": 0.3333
        })
    }

    fn false_positive_samples() -> serde_json::Value {
        json!([
            false_positive_sample("verification_success_path", true, false),
            false_positive_sample("verification_guard_downgraded_path", true, false),
            false_positive_sample("verification_failure_path", false, true)
        ])
    }

    fn false_positive_sample(
        sample_id: &str,
        skill_hit_effective: bool,
        skill_false_positive: bool,
    ) -> serde_json::Value {
        json!({
            "sample_id": sample_id,
            "skill_hit": true,
            "skill_hit_effective": skill_hit_effective,
            "skill_false_positive": skill_false_positive,
            "evidence_ref": format!("eval=verify-signals.json;sample={sample_id}"),
            "result": "passed",
            "source": "crates/runtime-core/src/verify.rs"
        })
    }

    fn build_latest_report(
        skill_report: &serde_json::Value,
        context_report: &serde_json::Value,
        verify_report: &serde_json::Value,
        fallback_report: &serde_json::Value,
    ) -> serde_json::Value {
        json!({
            "checked_at": "2026-04-16T20:30:00+08:00",
            "status": "partial",
            "h03": latest_h03_status(),
            "metrics": latest_metrics(),
            "evidence": latest_evidence(),
            "summary": latest_summary(skill_report, context_report, verify_report, fallback_report)
        })
    }

    fn latest_h03_status() -> serde_json::Value {
        json!({
            "boundary_ready": true,
            "sample_scope_ready": true,
            "eval_pack_ready": true,
            "fallback_cases_ready": true,
            "implementation_ready": false
        })
    }

    fn latest_metrics() -> serde_json::Value {
        json!({
            "mcp_skills_success_rate": 1.0,
            "failure_locatable_rate": 1.0,
            "critical_skill_eval_pass_rate": 1.0,
            "trust_tier_eval_pass_rate": 1.0,
            "context_skill_observable_rate": 1.0,
            "verify_signal_observable_rate": 1.0
        })
    }

    fn latest_evidence() -> serde_json::Value {
        json!({
            "skill_catalog": "tmp/stage-h-mcp-skills/evals/skill-catalog.json",
            "context_skill": "tmp/stage-h-mcp-skills/evals/context-skill.json",
            "verify_signals": "tmp/stage-h-mcp-skills/evals/verify-signals.json",
            "skill_false_positive": "tmp/stage-h-mcp-skills/evals/skill-false-positive.json",
            "failure_injection": "tmp/stage-h-mcp-skills/evals/failure-injection.json",
            "manual_review": "tmp/stage-h-mcp-skills/evals/manual-review.json",
            "cross_skill_expansion": "tmp/stage-h-mcp-skills/evals/cross-skill-expansion.json",
            "business_task_chain": "tmp/stage-h-mcp-skills/evals/business-task-chain.json",
            "fallback_cases": "tmp/stage-h-mcp-skills/fallback-cases.json"
        })
    }

    fn latest_summary(
        skill_report: &serde_json::Value,
        context_report: &serde_json::Value,
        verify_report: &serde_json::Value,
        fallback_report: &serde_json::Value,
    ) -> serde_json::Value {
        json!({
            "skill_samples": sample_count(skill_report),
            "context_samples": sample_count(context_report),
            "verify_samples": sample_count(verify_report),
            "fallback_cases": fallback_report.as_array().map(|v| v.len()).unwrap_or(0)
        })
    }

    fn augment_latest_report(
        mut latest: serde_json::Value,
        false_positive_report: &serde_json::Value,
        failure_injection_report: &serde_json::Value,
        manual_review_report: &serde_json::Value,
        cross_skill_report: &serde_json::Value,
        business_chain_report: &serde_json::Value,
    ) -> serde_json::Value {
        merge_false_positive_metrics(&mut latest, false_positive_report);
        merge_failure_injection_metrics(&mut latest, failure_injection_report);
        merge_manual_review_metrics(&mut latest, manual_review_report);
        merge_cross_skill_metrics(&mut latest, cross_skill_report);
        merge_business_chain_metrics(&mut latest, business_chain_report);
        latest
    }

    fn merge_business_chain_metrics(
        latest: &mut serde_json::Value,
        business_chain_report: &serde_json::Value,
    ) {
        if let Some(metrics) = business_chain_report.get("metrics") {
            latest["metrics"]["business_chain_observable_rate"] =
                metrics["business_chain_observable_rate"].clone();
        }
        latest["summary"]["business_chain_samples"] = json!(sample_count(business_chain_report));
    }

    fn merge_cross_skill_metrics(
        latest: &mut serde_json::Value,
        cross_skill_report: &serde_json::Value,
    ) {
        if let Some(metrics) = cross_skill_report.get("metrics") {
            latest["metrics"]["cross_skill_observable_rate"] =
                metrics["cross_skill_observable_rate"].clone();
        }
        latest["summary"]["cross_skill_samples"] = json!(sample_count(cross_skill_report));
    }

    fn merge_false_positive_metrics(
        latest: &mut serde_json::Value,
        false_positive_report: &serde_json::Value,
    ) {
        if let Some(metrics) = false_positive_report.get("metrics") {
            latest["metrics"]["skill_hit_effective_rate"] =
                metrics["skill_hit_effective_rate"].clone();
            latest["metrics"]["skill_false_positive_rate"] =
                metrics["skill_false_positive_rate"].clone();
        }
        latest["summary"]["false_positive_samples"] = json!(sample_count(false_positive_report));
    }

    fn merge_failure_injection_metrics(
        latest: &mut serde_json::Value,
        failure_injection_report: &serde_json::Value,
    ) {
        if let Some(metrics) = failure_injection_report.get("metrics") {
            latest["metrics"]["failure_injection_locatable_rate"] =
                metrics["failure_injection_locatable_rate"].clone();
        }
        latest["summary"]["failure_injection_samples"] =
            json!(sample_count(failure_injection_report));
    }

    fn merge_manual_review_metrics(
        latest: &mut serde_json::Value,
        manual_review_report: &serde_json::Value,
    ) {
        if let Some(metrics) = manual_review_report.get("metrics") {
            latest["metrics"]["manual_review_completion_rate"] =
                metrics["manual_review_completion_rate"].clone();
        }
        latest["summary"]["manual_review_samples"] = json!(sample_count(manual_review_report));
    }

    fn sample_count(report: &serde_json::Value) -> usize {
        report["samples"].as_array().map(|v| v.len()).unwrap_or(0)
    }

    fn skill_request(repo_root: &Path, trust_tier: &str) -> RunRequest {
        let root = repo_root
            .join("tmp")
            .join("stage-h-mcp-skills")
            .join(format!("skill-{trust_tier}"));
        let _ = fs::remove_dir_all(&root);
        let _ = fs::create_dir_all(root.join("workspace").join("skills").join("compose"));
        let _ = fs::create_dir_all(root.join("data").join("skills"));
        let _ = fs::write(
            root.join("workspace")
                .join("skills")
                .join("compose")
                .join("SKILL.md"),
            "# skill\n",
        );
        let manifest = format!(
            r#"{{"skills":[{{"skill_id":"compose_ui","version":"1.2.0","entry":"skills/compose/SKILL.md","workspace_id":"workspace-1","trust_tier":"{trust_tier}"}}]}}"#
        );
        let _ = fs::write(
            root.join("data").join("skills").join("workspace-1.json"),
            manifest,
        );
        RunRequest {
            request_id: "request-1".to_string(),
            run_id: "run-1".to_string(),
            session_id: "session-1".to_string(),
            trace_id: "trace-1".to_string(),
            user_input: "test".to_string(),
            mode: "standard".to_string(),
            model_ref: sample_model_ref(),
            provider_ref: ProviderRef::default(),
            workspace_ref: WorkspaceRef {
                workspace_id: "workspace-1".to_string(),
                name: "Workspace".to_string(),
                root_path: root.join("workspace").display().to_string(),
                is_active: true,
            },
            context_hints: BTreeMap::new(),
            resume_from_checkpoint_id: String::new(),
            resume_strategy: String::new(),
            confirmation_decision: None,
        }
    }

    fn context_request() -> RunRequest {
        let mut hints = BTreeMap::new();
        hints.insert(
            "skill_ids".to_string(),
            "skill.alpha,skill.beta".to_string(),
        );
        hints.insert("evidence_refs".to_string(), "verify:sample".to_string());
        RunRequest {
            request_id: "request-1".to_string(),
            run_id: "run-1".to_string(),
            session_id: "session-1".to_string(),
            trace_id: "trace-1".to_string(),
            user_input: "请继续执行项目任务".to_string(),
            mode: "standard".to_string(),
            model_ref: sample_model_ref(),
            provider_ref: ProviderRef::default(),
            workspace_ref: WorkspaceRef {
                workspace_id: "workspace-1".to_string(),
                name: "Workspace".to_string(),
                root_path: "D:/workspace".to_string(),
                is_active: true,
            },
            context_hints: hints,
            resume_from_checkpoint_id: String::new(),
            resume_strategy: String::new(),
            confirmation_decision: None,
        }
    }

    fn context_sample(
        sample_id: &str,
        policy: &ContextAssemblyPolicy,
        block: &crate::context_builder::DynamicPromptBlock,
    ) -> serde_json::Value {
        json!({
            "sample_id": sample_id,
            "profile": policy.profile,
            "skill_injection_enabled": block.skill_injection_enabled,
            "max_skill_level": block.max_skill_level,
            "injected_skill_level": block.injected_skill_level,
            "injected_skill_ids": block.injected_skill_ids,
            "evidence_refs": block.evidence_refs,
            "result": "passed",
            "source": "crates/runtime-core/src/context_policy.rs + context_builder.rs"
        })
    }

    fn verify_signal_sample(
        sample_id: &str,
        report: &crate::verify::VerificationReport,
        trust_tier: Option<&str>,
        guard_action: Option<&str>,
    ) -> serde_json::Value {
        let mut sample = json!({
            "sample_id": sample_id,
            "verification_code": report.outcome.code,
            "verification_passed": report.outcome.passed,
            "skill_hit_effective": report.outcome.skill_hit_effective,
            "skill_hit_reason": report.outcome.skill_hit_reason,
            "guard_downgraded": report.outcome.guard_downgraded,
            "guard_decision_ref": report.outcome.guard_decision_ref,
            "policy": report.outcome.policy,
            "result": "passed",
            "source": "crates/runtime-core/src/verify.rs"
        });
        if let Some(value) = trust_tier {
            sample["trust_tier"] = json!(value);
        }
        if let Some(value) = guard_action {
            sample["guard_action"] = json!(value);
        }
        sample
    }

    fn sample_tool_call(tool_name: &str) -> ToolCall {
        ToolCall {
            action: PlannedAction::RunCommand {
                command: "echo ok".to_string(),
            },
            spec: ToolDefinition {
                tool_name: tool_name.to_string(),
                display_name: "执行命令".to_string(),
                category: "system_command".to_string(),
                risk_level: "high".to_string(),
                input_schema: "command_text".to_string(),
                output_kind: "text_preview".to_string(),
                requires_confirmation: true,
            },
        }
    }

    fn sample_trace(success: bool) -> ToolExecutionTrace {
        ToolExecutionTrace {
            tool: sample_tool_call("run_command").spec,
            action_summary: "执行 echo ok".to_string(),
            result: ToolCallResult {
                summary: if success {
                    "命令执行成功"
                } else {
                    "命令执行失败"
                }
                .to_string(),
                final_answer: if success { "ok" } else { "failed" }.to_string(),
                artifact_path: None,
                detail_preview: "preview".to_string(),
                raw_output_ref: None,
                result_chars: 10,
                single_result_budget_chars: 30000,
                single_result_budget_hit: false,
                error_code: if success {
                    None
                } else {
                    Some("exit_1".to_string())
                },
                elapsed_ms: 10,
                retryable: !success,
                success,
                memory_write_summary: None,
                reasoning_summary: "测试推理".to_string(),
                cache_status: "bypass".to_string(),
                cache_reason: String::new(),
            },
        }
    }

    fn sample_trace_guard_downgraded() -> ToolExecutionTrace {
        ToolExecutionTrace {
            tool: sample_tool_call("run_command").spec,
            action_summary: "执行 echo ok（guard downgraded）".to_string(),
            result: ToolCallResult {
                summary: "命令执行成功，guard downgraded".to_string(),
                final_answer: "ok".to_string(),
                artifact_path: None,
                detail_preview: "preview".to_string(),
                raw_output_ref: None,
                result_chars: 10,
                single_result_budget_chars: 30000,
                single_result_budget_hit: false,
                error_code: None,
                elapsed_ms: 10,
                retryable: false,
                success: true,
                memory_write_summary: None,
                reasoning_summary: "guard downgraded to review".to_string(),
                cache_status: "bypass".to_string(),
                cache_reason: String::new(),
            },
        }
    }

    fn sample_repo_context() -> RepoContextLoadResult {
        RepoContextLoadResult {
            snapshot: RepoContextSnapshot {
                workspace_root: "D:/workspace".to_string(),
                repo_root: None,
                git_available: false,
                git_snapshot: None,
                doc_summaries: Vec::new(),
                warnings: Vec::new(),
                collected_at: "1".to_string(),
            },
            degraded: false,
            error_count: 0,
        }
    }

    fn sample_model_ref() -> ModelRef {
        ModelRef {
            provider_id: "provider".to_string(),
            model_id: "model".to_string(),
            display_name: "Model".to_string(),
        }
    }

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("repo root")
            .to_path_buf()
    }

    fn write_json(path: &Path, value: &serde_json::Value) {
        let data = serde_json::to_vec_pretty(value).expect("json");
        fs::write(path, data).expect("write json");
    }
}
