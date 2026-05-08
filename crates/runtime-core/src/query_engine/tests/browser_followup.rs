use super::*;

#[test]
fn browser_verify_failure_requests_replan() {
    let state = browser_failed_state();
    assert!(browser_interaction_verify_failed(&state));
    assert!(should_replan(&state));
}

#[test]
fn browser_verify_failure_replans_to_read_page() {
    let state = browser_failed_state();
    let next = replan_state(&state, 2).expect("next state");
    assert!(matches!(
        next.action,
        PlannedAction::MCPCall { ref server_id, ref tool_name, ref arguments_json }
        if server_id == "browser"
            && tool_name == "read_page"
            && arguments_json.contains(r#""page_id":"page_01""#)
    ));
}

#[test]
fn browser_select_failure_replans_to_read_page() {
    let state = sample_browser_state(browser_select_action(), "浏览器选择");
    let next = replan_state(&state, 2).expect("next state");
    assert!(matches!(
        next.action,
        PlannedAction::MCPCall { ref server_id, ref tool_name, ref arguments_json }
        if server_id == "browser"
            && tool_name == "read_page"
            && arguments_json.contains(r#""page_id":"page_01""#)
    ));
}

#[test]
fn browser_submit_failure_replans_to_read_page() {
    let state = sample_browser_state(browser_submit_action(), "浏览器提交");
    let next = replan_state(&state, 2).expect("next state");
    assert!(matches!(
        next.action,
        PlannedAction::MCPCall { ref server_id, ref tool_name, ref arguments_json }
        if server_id == "browser"
            && tool_name == "read_page"
            && arguments_json.contains(r#""page_id":"page_01""#)
    ));
}

#[test]
fn browser_upload_failure_replans_to_read_page() {
    let state = sample_browser_state(browser_upload_action(), "浏览器上传");
    let next = replan_state(&state, 2).expect("next state");
    assert!(matches!(
        next.action,
        PlannedAction::MCPCall { ref server_id, ref tool_name, ref arguments_json }
        if server_id == "browser"
            && tool_name == "read_page"
            && arguments_json.contains(r#""page_id":"page_01""#)
    ));
}

#[test]
fn browser_read_page_failure_stops_replan_and_handoffs() {
    let state = browser_read_page_failed_state();
    assert!(!should_replan(&state));
    assert!(browser_interaction_verify_failed(&state));
}
