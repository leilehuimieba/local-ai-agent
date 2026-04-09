use crate::contracts::RunRequest;
use crate::execution::ActionExecution;
use crate::text::summarize_text;
use std::path::PathBuf;
use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

const CACHE_REASON: &str = "命令执行结果依赖实时环境，不使用回答缓存。";

pub(crate) fn execute_command(request: &RunRequest, command: &str) -> ActionExecution {
    let output = match run_command(request, command) {
        Ok(output) => output,
        Err(error) => {
            return ActionExecution::bypass_fail(
                format!("尝试执行命令：{}", command),
                format!("命令启动失败：{}", error),
                format!("命令没有成功启动：{}", error),
                "命令进程未能启动，直接按运行错误收口。".to_string(),
                CACHE_REASON,
            );
        }
    };
    let (summary, final_answer) = output_answer(request, command, &output);
    ActionExecution::bypass(
        format!(
            "在工作区 `{}` 中执行命令：{}",
            request.workspace_ref.name, command
        ),
        summary,
        final_answer,
        output.status.success(),
        None,
        "直接执行用户给定命令，并基于 stdout 或 stderr 生成摘要。".to_string(),
        CACHE_REASON,
    )
}

fn run_command(
    request: &RunRequest,
    command: &str,
) -> Result<std::process::Output, std::io::Error> {
    if cfg!(target_os = "windows") {
        let wrapped_command = format!(
            "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; $OutputEncoding = [System.Text.Encoding]::UTF8; chcp 65001 > $null; Set-Location -LiteralPath {}; {}",
            ps_single_quote(&request.workspace_ref.root_path),
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
            .current_dir(command_workdir(request))
            .output()
    }
}

fn command_workdir(request: &RunRequest) -> PathBuf {
    if cfg!(target_os = "windows") {
        PathBuf::from(request.workspace_ref.root_path.replace('/', "\\"))
    } else {
        PathBuf::from(&request.workspace_ref.root_path)
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
) -> (String, String) {
    if output.status.success() {
        return success_answer(request, command, output);
    }
    error_answer(request, command, output)
}

fn success_answer(
    request: &RunRequest,
    command: &str,
    output: &std::process::Output,
) -> (String, String) {
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let summary = summarize_text(&stdout);
    (
        format!("命令执行成功，输出摘要：{}", summary),
        format!(
            "命令已执行完成。\n工作区：{}\n命令：{}\n输出摘要：{}",
            request.workspace_ref.root_path, command, summary
        ),
    )
}

fn error_answer(
    request: &RunRequest,
    command: &str,
    output: &std::process::Output,
) -> (String, String) {
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let summary = summarize_text(&stderr);
    (
        format!("命令执行失败，错误摘要：{}", summary),
        format!(
            "命令执行失败。\n工作区：{}\n命令：{}\n错误摘要：{}",
            request.workspace_ref.root_path, command, summary
        ),
    )
}
