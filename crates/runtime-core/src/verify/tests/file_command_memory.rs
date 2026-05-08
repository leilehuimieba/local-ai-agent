use super::{
    command_trace, file_change_trace, file_write_tool_call, memory_write_tool_call, memory_write_trace,
    sample_tool_call, verify_tool_execution,
};

#[test]
fn file_change_requires_path_and_effect_signal() {
    let report = verify_tool_execution(&file_write_tool_call(), &file_change_trace(false));
    assert!(!report.outcome.passed);
    assert_eq!(report.outcome.task_type, "file_change");
    assert_eq!(report.outcome.code, "file_change_insufficient");
}

#[test]
fn file_change_passes_with_path_and_effect_signal() {
    let report = verify_tool_execution(&file_write_tool_call(), &file_change_trace(true));
    assert!(report.outcome.passed);
    assert_eq!(report.outcome.task_type, "file_change");
    assert_eq!(report.outcome.policy, "confirm_write_effect");
}

#[test]
fn command_execution_requires_output_and_artifact_signal() {
    let report = verify_tool_execution(&sample_tool_call(), &command_trace(false));
    assert!(!report.outcome.passed);
    assert_eq!(report.outcome.task_type, "command_execution");
    assert_eq!(report.outcome.code, "command_execution_insufficient");
}

#[test]
fn command_execution_passes_with_output_and_artifact_signal() {
    let report = verify_tool_execution(&sample_tool_call(), &command_trace(true));
    assert!(report.outcome.passed);
    assert_eq!(report.outcome.task_type, "command_execution");
    assert_eq!(report.outcome.policy, "inspect_command_result");
}

#[test]
fn memory_write_requires_summary_and_type_signal() {
    let report = verify_tool_execution(&memory_write_tool_call(), &memory_write_trace(false));
    assert!(!report.outcome.passed);
    assert_eq!(report.outcome.task_type, "memory_write");
    assert_eq!(report.outcome.code, "memory_write_insufficient");
}

#[test]
fn memory_write_passes_with_summary_and_type_signal() {
    let report = verify_tool_execution(&memory_write_tool_call(), &memory_write_trace(true));
    assert!(report.outcome.passed);
    assert_eq!(report.outcome.task_type, "memory_write");
    assert_eq!(report.outcome.policy, "confirm_memory_persisted");
}
