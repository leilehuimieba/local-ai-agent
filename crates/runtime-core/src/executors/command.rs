use crate::contracts::RunRequest;
use crate::execution::ActionExecution;
use crate::text::summarize_text;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

const CACHE_REASON: &str = "命令执行结果依赖实时环境，不使用回答缓存。";
const COMMAND_SINGLE_RESULT_BUDGET_CHARS: usize = 30_000;
const DEFAULT_TIMEOUT_SECS: u64 = 30;
const MAX_TIMEOUT_SECS: u64 = 300;

const DANGEROUS_COMMANDS: &[&str] = &[
    "rm -rf /",
    "rm -rf --no-preserve-root",
    "del /f /s C:\\",
    "rd /s /q C:\\",
    "format ",
    "mkfs.",
    "dd if=",
    "> /dev/sda",
    "shutdown",
    "reboot",
    ":(){ :|:& };:",
    "chmod 777 /",
    "chmod -R 777 /",
    "chown -R ",
    "wget ",
    "curl ",
    "nc ",
    "ncat ",
    "netcat ",
    "eval ",
    "exec ",
    "Invoke-Expression",
    "iex ",
    "Start-Process",
];

pub(crate) fn execute_command(
    request: &RunRequest,
    command: &str,
    timeout_secs: Option<u32>,
) -> ActionExecution {
    if let Some(reason) = check_dangerous_command(command) {
        return ActionExecution::bypass_fail(
            format!("尝试执行命令：{}", command),
            format!("命令被拒绝：{}", reason),
            format!("命令执行被安全策略拒绝：{}", reason),
            "检测到潜在危险命令，已阻断执行。".to_string(),
            CACHE_REASON,
        );
    }
    let timeout = timeout_secs
        .map(|t| Duration::from_secs(u64::min(t as u64, MAX_TIMEOUT_SECS)))
        .unwrap_or(Duration::from_secs(DEFAULT_TIMEOUT_SECS));
    let output = match run_command_with_timeout(request, command, timeout) {
        Ok(output) => output,
        Err(error) => {
            return ActionExecution::bypass_fail(
                format!("尝试执行命令：{}", command),
                format!("命令执行失败：{}", error),
                format!("命令执行失败：{}", error),
                "命令进程启动或运行失败，按错误收口。".to_string(),
                CACHE_REASON,
            );
        }
    };
    let (summary, final_answer, detail_preview, raw_output) = output_answer(request, command, &output);
    let execution = ActionExecution::bypass(
        format!("在工作区 `{}` 中执行命令：{}", request.workspace_ref.name, command),
        summary,
        final_answer,
        output.status.success(),
        None,
        "执行用户命令，含超时控制与危险命令阻断。".to_string(),
        CACHE_REASON,
    );
    with_command_output_contract(execution, detail_preview, raw_output)
}

fn check_dangerous_command(command: &str) -> Option<&'static str> {
    let lower = command.to_lowercase();
    for dangerous in DANGEROUS_COMMANDS {
        if lower.contains(&dangerous.to_lowercase()) {
            return Some("命令匹配危险模式，已拒绝执行。");
        }
    }
    None
}

fn run_command_with_timeout(
    request: &RunRequest,
    command: &str,
    timeout: Duration,
) -> Result<std::process::Output, String> {
    let cmd_str = command.to_string();
    let root_path = request.workspace_ref.root_path.clone();
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let result = execute_command_blocking(&root_path, &cmd_str);
        let _ = tx.send(result);
    });
    match rx.recv_timeout(timeout) {
        Ok(result) => result.map_err(|e| e.to_string()),
        Err(mpsc::RecvTimeoutError::Timeout) => {
            Err(format!("命令执行超时（{} 秒），已终止。", timeout.as_secs()))
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            Err("命令执行线程意外退出。".to_string())
        }
    }
}

fn execute_command_blocking(root_path: &str, command: &str) -> Result<std::process::Output, std::io::Error> {
    if cfg!(target_os = "windows") {
        let wrapped_command = format!(
            "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; $OutputEncoding = [System.Text.Encoding]::UTF8; chcp 65001 > $null; Set-Location -LiteralPath {}; {}",
            ps_single_quote(root_path),
            command
        );
        let mut cmd = Command::new("powershell");
        #[cfg(target_os = "windows")]
        {
            cmd.creation_flags(CREATE_NO_WINDOW);
        }
        cmd.arg("-NoProfile")
            .arg("-Command")
            .arg(wrapped_command)
            .current_dir(system32_workdir())
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .current_dir(PathBuf::from(root_path))
            .output()
    }
}

fn system32_workdir() -> PathBuf {
    let root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());
    PathBuf::from(root).join("System32")
}

fn ps_single_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

fn output_answer(
    request: &RunRequest,
    command: &str,
    output: &std::process::Output,
) -> (String, String, String, String) {
    if output.status.success() {
        return success_answer(request, command, output);
    }
    error_answer(request, command, output)
}

fn success_answer(
    request: &RunRequest,
    command: &str,
    output: &std::process::Output,
) -> (String, String, String, String) {
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let detail_preview = summarize_text(&stdout);
    let summary = if detail_preview.is_empty() {
        "命令执行成功，未返回可见输出。".to_string()
    } else {
        detail_preview.clone()
    };
    (
        summary.clone(),
        format!(
            "命令已执行完成。\n工作区：{}\n命令：{}\n输出摘要：{}",
            request.workspace_ref.root_path, command, summary
        ),
        detail_preview,
        stdout,
    )
}

fn error_answer(
    request: &RunRequest,
    command: &str,
    output: &std::process::Output,
) -> (String, String, String, String) {
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let merged = merge_error_output(&stdout, &stderr);
    let detail_preview = summarize_text(&merged);
    let summary = if detail_preview.is_empty() {
        "命令执行失败，未返回可见错误输出。".to_string()
    } else {
        detail_preview.clone()
    };
    (
        summary.clone(),
        format!(
            "命令执行失败。\n工作区：{}\n命令：{}\n错误摘要：{}",
            request.workspace_ref.root_path, command, summary
        ),
        detail_preview,
        merged,
    )
}

fn merge_error_output(stdout: &str, stderr: &str) -> String {
    if stdout.trim().is_empty() {
        return stderr.to_string();
    }
    if stderr.trim().is_empty() {
        return stdout.to_string();
    }
    format!("[stdout]\n{}\n\n[stderr]\n{}", stdout, stderr)
}

fn with_command_output_contract(
    mut execution: ActionExecution,
    detail_preview: String,
    raw_output: String,
) -> ActionExecution {
    let budget = command_budget(&raw_output);
    execution.detail_preview = budget_preview(detail_preview, &budget);
    execution.raw_output = raw_output;
    execution.result_chars = budget.result_chars;
    execution.single_result_budget_chars = budget.budget_chars;
    execution.single_result_budget_hit = budget.hit;
    execution
}

fn command_budget(raw_output: &str) -> CommandBudget {
    let result_chars = raw_output.chars().count();
    CommandBudget {
        result_chars,
        budget_chars: COMMAND_SINGLE_RESULT_BUDGET_CHARS,
        hit: result_chars > COMMAND_SINGLE_RESULT_BUDGET_CHARS,
    }
}

fn budget_preview(detail_preview: String, budget: &CommandBudget) -> String {
    if !budget.hit {
        return detail_preview;
    }
    let base = if detail_preview.is_empty() {
        "无可显示内容。".to_string()
    } else {
        detail_preview
    };
    format!(
        "输出较长（{} 字符，超出单结果预算 {}），完整原文已外置。摘要：{}",
        budget.result_chars, budget.budget_chars, base
    )
}

struct CommandBudget {
    result_chars: usize,
    budget_chars: usize,
    hit: bool,
}

#[cfg(test)]
mod tests {
    use super::{COMMAND_SINGLE_RESULT_BUDGET_CHARS, budget_preview, command_budget};

    #[test]
    fn marks_budget_hit_when_output_exceeds_single_result_budget() {
        let text = "a".repeat(COMMAND_SINGLE_RESULT_BUDGET_CHARS + 1);
        let budget = command_budget(&text);
        assert!(budget.hit);
        assert_eq!(budget.result_chars, COMMAND_SINGLE_RESULT_BUDGET_CHARS + 1);
    }

    #[test]
    fn appends_budget_hint_into_preview_when_budget_hit() {
        let text = "a".repeat(COMMAND_SINGLE_RESULT_BUDGET_CHARS + 20);
        let budget = command_budget(&text);
        let preview = budget_preview("摘要".to_string(), &budget);
        assert!(preview.contains("输出较长"));
        assert!(preview.contains("完整原文已外置"));
    }
}
