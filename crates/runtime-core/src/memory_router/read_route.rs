use crate::context_policy::ContextAssemblyPolicy;
use crate::contracts::RunRequest;

const SYSTEM_LAYER: &str = "system views";
const OBJECT_LAYER: &str = "current memory object";
const HISTORY_LAYER: &str = "history entries";

#[derive(Clone, Debug)]
pub(crate) struct MemoryRouteSelection {
    pub route: String,
    pub selected_layers: Vec<String>,
    pub skipped_layers: Vec<String>,
    pub match_reason: String,
    pub reuse_confidence: String,
}

pub(crate) fn select_memory_route(
    request: &RunRequest,
    query: &str,
    policy: Option<&ContextAssemblyPolicy>,
) -> MemoryRouteSelection {
    if is_repair_route(request, query, policy) {
        return repair_route();
    }
    if is_learn_route(query, policy) {
        return learn_route();
    }
    if is_ask_route(query, policy) {
        return ask_route();
    }
    act_route()
}

fn is_repair_route(request: &RunRequest, query: &str, policy: Option<&ContextAssemblyPolicy>) -> bool {
    profile_is(policy, "repair_profile")
        || !request.resume_from_checkpoint_id.trim().is_empty()
        || contains_any(query, &["失败", "恢复", "重试", "报错", "异常", "阻塞", "checkpoint"])
}

fn is_learn_route(query: &str, policy: Option<&ContextAssemblyPolicy>) -> bool {
    profile_is(policy, "learn_profile")
        || contains_any(query, &["学习", "沉淀", "复用", "经验", "原则", "流程", "模式", "总结"])
}

fn is_ask_route(query: &str, policy: Option<&ContextAssemblyPolicy>) -> bool {
    profile_is(policy, "ask_profile")
        || contains_any(
            query,
            &[
                "是什么",
                "为什么",
                "规则",
                "背景",
                "架构",
                "关系",
                "阶段",
                "进度",
                "项目",
            ],
        )
}

fn profile_is(policy: Option<&ContextAssemblyPolicy>, expected: &str) -> bool {
    policy.is_some_and(|current| current.profile == expected)
}

fn ask_route() -> MemoryRouteSelection {
    MemoryRouteSelection {
        route: "ask_route".to_string(),
        selected_layers: vec![SYSTEM_LAYER.to_string(), HISTORY_LAYER.to_string()],
        skipped_layers: vec![OBJECT_LAYER.to_string()],
        match_reason: "当前更像定义、规则或背景问答，优先读取稳定规则与历史经验。".to_string(),
        reuse_confidence: "high".to_string(),
    }
}

fn act_route() -> MemoryRouteSelection {
    MemoryRouteSelection {
        route: "act_route".to_string(),
        selected_layers: vec![OBJECT_LAYER.to_string(), HISTORY_LAYER.to_string()],
        skipped_layers: vec![SYSTEM_LAYER.to_string()],
        match_reason: "当前更像执行推进，优先读取当前对象和少量直接可用经验。".to_string(),
        reuse_confidence: "medium".to_string(),
    }
}

fn repair_route() -> MemoryRouteSelection {
    MemoryRouteSelection {
        route: "repair_route".to_string(),
        selected_layers: vec![OBJECT_LAYER.to_string(), HISTORY_LAYER.to_string()],
        skipped_layers: vec![SYSTEM_LAYER.to_string()],
        match_reason: "当前处于恢复或失败处理上下文，优先读取对象状态与失败经验。".to_string(),
        reuse_confidence: "high".to_string(),
    }
}

fn learn_route() -> MemoryRouteSelection {
    MemoryRouteSelection {
        route: "learn_route".to_string(),
        selected_layers: vec![SYSTEM_LAYER.to_string(), HISTORY_LAYER.to_string()],
        skipped_layers: vec![OBJECT_LAYER.to_string()],
        match_reason: "当前更像学习沉淀或复用总结，优先读取稳定规则与可复用经验。".to_string(),
        reuse_confidence: "high".to_string(),
    }
}

fn contains_any(input: &str, tokens: &[&str]) -> bool {
    tokens.iter().any(|token| input.contains(token))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{ModelRef, ProviderRef, RunRequest, WorkspaceRef};

    #[test]
    fn selects_ask_route_for_rule_question() {
        let route = select_memory_route(&sample_request("当前项目规则是什么"), "当前项目规则是什么", None);
        assert_eq!(route.route, "ask_route");
        assert!(route.selected_layers.contains(&SYSTEM_LAYER.to_string()));
    }

    #[test]
    fn selects_repair_route_for_resume_request() {
        let route = select_memory_route(&resume_request(), "为什么上次失败", None);
        assert_eq!(route.route, "repair_route");
        assert!(route.selected_layers.contains(&OBJECT_LAYER.to_string()));
    }

    #[test]
    fn selects_learn_route_for_reuse_query() {
        let route = select_memory_route(&sample_request("复用原则"), "复用原则", None);
        assert_eq!(route.route, "learn_route");
        assert!(route.selected_layers.contains(&HISTORY_LAYER.to_string()));
    }

    fn sample_request(user_input: &str) -> RunRequest {
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
            context_hints: Default::default(),
            resume_from_checkpoint_id: String::new(),
            resume_strategy: String::new(),
            confirmation_decision: None,
        }
    }

    fn resume_request() -> RunRequest {
        RunRequest {
            resume_from_checkpoint_id: "cp-1".to_string(),
            resume_strategy: "retry_failure".to_string(),
            ..sample_request("为什么上次失败")
        }
    }
}
