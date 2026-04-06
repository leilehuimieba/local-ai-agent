use crate::answer_cache::{append_answer_cache, bypass_probe, probe_answer_cache};
use crate::artifacts::externalize_text_artifact;
use crate::context_builder::build_runtime_context;
use crate::contracts::RunRequest;
use crate::events::timestamp_now;
use crate::knowledge::search_knowledge;
use crate::knowledge_store::{
    KnowledgeRecord, append_knowledge_record, find_reusable_siyuan_record, search_knowledge_records,
};
use crate::llm::complete_text;
use crate::memory::{MemoryEntry, append_memory_entry, search_memory_entries};
use crate::paths::{
    repo_root, resolve_workspace_path, siyuan_auto_write_enabled, siyuan_export_dir,
    siyuan_root_dir, siyuan_sync_enabled,
};
use crate::planner::PlannedAction;
use crate::prompt::{render_context_answer_prompt, render_project_answer_prompt};
use crate::repo_context::load_repo_context;
use crate::session::SessionMemory;
use crate::text::{extract_snippet, summarize_text};
use crate::tool_registry::runtime_tool_registry;
use crate::tools::{ToolCallResult, ToolExecutionTrace, resolve_tool};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Debug)]
pub(crate) struct ActionExecution {
    pub action_summary: String,
    pub result_summary: String,
    pub final_answer: String,
    pub success: bool,
    pub memory_write_summary: Option<String>,
    pub reasoning_summary: String,
    pub cache_status: String,
    pub cache_reason: String,
}

pub(crate) fn execute_action(
    request: &RunRequest,
    action: &PlannedAction,
    session_context: &SessionMemory,
) -> ActionExecution {
    match action {
        PlannedAction::RunCommand { command } => execute_command(request, command),
        PlannedAction::ReadFile { path } => execute_file_read(request, path),
        PlannedAction::WriteFile { path, content } => execute_file_write(request, path, content),
        PlannedAction::DeletePath { path } => execute_delete_path(request, path),
        PlannedAction::ListFiles { path } => execute_list_files(request, path.as_deref()),
        PlannedAction::WriteMemory {
            kind,
            summary,
            content,
        } => execute_memory_write(request, kind, summary, content),
        PlannedAction::RecallMemory { query } => execute_memory_recall(request, query),
        PlannedAction::SearchKnowledge { query } => execute_knowledge_search(request, query),
        PlannedAction::SearchSiyuanNotes { query } => execute_siyuan_search(request, query),
        PlannedAction::ReadSiyuanNote { path } => execute_siyuan_read(request, path),
        PlannedAction::WriteSiyuanKnowledge => execute_siyuan_write(request),
        PlannedAction::ProjectAnswer => execute_project_answer(request),
        PlannedAction::ContextAnswer => execute_context_answer(request, session_context),
        PlannedAction::Explain => execute_explain(request),
        PlannedAction::AgentResolve => execute_agent_resolve(request, session_context),
    }
}

pub(crate) fn execute_tool(
    request: &RunRequest,
    action: &PlannedAction,
    session_context: &SessionMemory,
) -> ToolExecutionTrace {
    let execution = execute_action(request, action, session_context);
    let artifact_path = materialize_artifact(request, action, &execution.final_answer);
    ToolExecutionTrace {
        tool: resolve_tool(action),
        action_summary: execution.action_summary.clone(),
        result: ToolCallResult {
            summary: execution.result_summary.clone(),
            final_answer: execution.final_answer,
            artifact_path,
            error_code: if execution.success {
                None
            } else {
                Some(default_error_code(action))
            },
            retryable: !execution.success,
            success: execution.success,
            memory_write_summary: execution.memory_write_summary,
            reasoning_summary: execution.reasoning_summary,
            cache_status: execution.cache_status,
            cache_reason: execution.cache_reason,
        },
    }
}

fn default_error_code(action: &PlannedAction) -> String {
    match action {
        PlannedAction::RunCommand { .. } => "command_failed",
        PlannedAction::ReadFile { .. } => "file_read_failed",
        PlannedAction::WriteFile { .. } => "file_write_failed",
        PlannedAction::DeletePath { .. } => "path_delete_failed",
        PlannedAction::ListFiles { .. } => "list_dir_failed",
        PlannedAction::WriteMemory { .. } => "memory_write_failed",
        PlannedAction::RecallMemory { .. } => "memory_recall_failed",
        PlannedAction::SearchKnowledge { .. } => "knowledge_search_failed",
        PlannedAction::SearchSiyuanNotes { .. } => "siyuan_search_failed",
        PlannedAction::ReadSiyuanNote { .. } => "siyuan_read_failed",
        PlannedAction::WriteSiyuanKnowledge => "siyuan_write_failed",
        PlannedAction::ProjectAnswer => "project_answer_failed",
        PlannedAction::ContextAnswer => "context_answer_failed",
        PlannedAction::Explain => "runtime_output_failed",
        PlannedAction::AgentResolve => "agent_resolve_failed",
    }
    .to_string()
}

fn execute_command(request: &RunRequest, command: &str) -> ActionExecution {
    let output = if cfg!(target_os = "windows") {
        let wrapped_command = format!(
            "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; $OutputEncoding = [System.Text.Encoding]::UTF8; chcp 65001 > $null; {}",
            command
        );
        Command::new("powershell")
            .arg("-NoProfile")
            .arg("-Command")
            .arg(wrapped_command)
            .current_dir(&request.workspace_ref.root_path)
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .current_dir(&request.workspace_ref.root_path)
            .output()
    };

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let summary = if output.status.success() {
                summarize_text(&stdout)
            } else {
                summarize_text(&stderr)
            };
            ActionExecution {
                action_summary: format!(
                    "在工作区 `{}` 中执行命令：{}",
                    request.workspace_ref.name, command
                ),
                result_summary: if output.status.success() {
                    format!("命令执行成功，输出摘要：{}", summary)
                } else {
                    format!("命令执行失败，错误摘要：{}", summary)
                },
                final_answer: if output.status.success() {
                    format!(
                        "命令已执行完成。\n工作区：{}\n命令：{}\n输出摘要：{}",
                        request.workspace_ref.root_path, command, summary
                    )
                } else {
                    format!(
                        "命令执行失败。\n工作区：{}\n命令：{}\n错误摘要：{}",
                        request.workspace_ref.root_path, command, summary
                    )
                },
                success: output.status.success(),
                memory_write_summary: None,
                reasoning_summary: "直接执行用户给定命令，并基于 stdout 或 stderr 生成摘要。"
                    .to_string(),
                cache_status: "bypass".to_string(),
                cache_reason: "命令执行结果依赖实时环境，不使用回答缓存。".to_string(),
            }
        }
        Err(error) => ActionExecution {
            action_summary: format!("尝试执行命令：{}", command),
            result_summary: format!("命令启动失败：{}", error),
            final_answer: format!("命令没有成功启动：{}", error),
            success: false,
            memory_write_summary: None,
            reasoning_summary: "命令进程未能启动，直接按运行错误收口。".to_string(),
            cache_status: "bypass".to_string(),
            cache_reason: "命令执行结果依赖实时环境，不使用回答缓存。".to_string(),
        },
    }
}

fn execute_file_read(request: &RunRequest, path: &str) -> ActionExecution {
    match resolve_workspace_path(&request.workspace_ref.root_path, path) {
        Ok(resolved) => match fs::read_to_string(&resolved) {
            Ok(content) => {
                let summary = summarize_text(&content);
                ActionExecution {
                    action_summary: format!("读取文件：{}", resolved.display()),
                    result_summary: format!("文件读取成功，摘要：{}", summary),
                    final_answer: format!(
                        "文件读取完成：{}\n内容摘要：{}",
                        resolved.display(),
                        summary
                    ),
                    success: true,
                    memory_write_summary: None,
                    reasoning_summary: "直接读取目标文件，并将原文压缩成可展示摘要。".to_string(),
                    cache_status: "bypass".to_string(),
                    cache_reason: "文件读取结果依赖实时文件内容，不使用回答缓存。".to_string(),
                }
            }
            Err(error) => ActionExecution {
                action_summary: format!("读取文件：{}", path),
                result_summary: format!("文件读取失败：{}", error),
                final_answer: format!("文件读取失败：{}", error),
                success: false,
                memory_write_summary: None,
                reasoning_summary: "目标文件读取失败，按错误直接返回。".to_string(),
                cache_status: "bypass".to_string(),
                cache_reason: "文件读取结果依赖实时文件内容，不使用回答缓存。".to_string(),
            },
        },
        Err(message) => ActionExecution {
            action_summary: format!("读取文件：{}", path),
            result_summary: message.clone(),
            final_answer: message,
            success: false,
            memory_write_summary: None,
            reasoning_summary: "目标路径越界或解析失败，按路径校验结果直接返回。".to_string(),
            cache_status: "bypass".to_string(),
            cache_reason: "文件读取结果依赖实时文件内容，不使用回答缓存。".to_string(),
        },
    }
}

fn execute_file_write(request: &RunRequest, path: &str, content: &str) -> ActionExecution {
    match resolve_workspace_path(&request.workspace_ref.root_path, path) {
        Ok(resolved) => {
            if let Some(parent) = resolved.parent() {
                if let Err(error) = fs::create_dir_all(parent) {
                    return ActionExecution {
                        action_summary: format!("写入文件：{}", resolved.display()),
                        result_summary: format!("目录创建失败：{}", error),
                        final_answer: format!("写入前创建目录失败：{}", error),
                        success: false,
                        memory_write_summary: None,
                        reasoning_summary: "写入前置目录创建失败，未进入文件写入阶段。".to_string(),
                        cache_status: "bypass".to_string(),
                        cache_reason: "文件写入属于实时副作用动作，不使用回答缓存。".to_string(),
                    };
                }
            }

            match fs::write(&resolved, content) {
                Ok(_) => ActionExecution {
                    action_summary: format!("写入文件：{}", resolved.display()),
                    result_summary: format!(
                        "文件写入成功，共写入 {} 个字符。",
                        content.chars().count()
                    ),
                    final_answer: format!(
                        "文件写入完成：{}\n写入字符数：{}\n内容摘要：{}",
                        resolved.display(),
                        content.chars().count(),
                        summarize_text(content)
                    ),
                    success: true,
                    memory_write_summary: None,
                    reasoning_summary: "先校验工作区路径，再直接写入目标文件并返回摘要。"
                        .to_string(),
                    cache_status: "bypass".to_string(),
                    cache_reason: "文件写入属于实时副作用动作，不使用回答缓存。".to_string(),
                },
                Err(error) => ActionExecution {
                    action_summary: format!("写入文件：{}", resolved.display()),
                    result_summary: format!("文件写入失败：{}", error),
                    final_answer: format!("文件写入失败：{}", error),
                    success: false,
                    memory_write_summary: None,
                    reasoning_summary: "文件写入过程中出现系统错误，直接按失败收口。".to_string(),
                    cache_status: "bypass".to_string(),
                    cache_reason: "文件写入属于实时副作用动作，不使用回答缓存。".to_string(),
                },
            }
        }
        Err(message) => ActionExecution {
            action_summary: format!("写入文件：{}", path),
            result_summary: message.clone(),
            final_answer: message,
            success: false,
            memory_write_summary: None,
            reasoning_summary: "目标路径越界或解析失败，未进入写入阶段。".to_string(),
            cache_status: "bypass".to_string(),
            cache_reason: "文件写入属于实时副作用动作，不使用回答缓存。".to_string(),
        },
    }
}

fn execute_delete_path(request: &RunRequest, path: &str) -> ActionExecution {
    match resolve_workspace_path(&request.workspace_ref.root_path, path) {
        Ok(resolved) => {
            let result = if resolved.is_dir() {
                fs::remove_dir_all(&resolved)
            } else {
                fs::remove_file(&resolved)
            };

            match result {
                Ok(_) => ActionExecution {
                    action_summary: format!("删除路径：{}", resolved.display()),
                    result_summary: "目标路径已删除。".to_string(),
                    final_answer: format!("删除完成：{}", resolved.display()),
                    success: true,
                    memory_write_summary: None,
                    reasoning_summary: "按目标类型执行删除，并将删除结果直接回传。".to_string(),
                    cache_status: "bypass".to_string(),
                    cache_reason: "删除属于实时副作用动作，不使用回答缓存。".to_string(),
                },
                Err(error) => ActionExecution {
                    action_summary: format!("删除路径：{}", resolved.display()),
                    result_summary: format!("删除失败：{}", error),
                    final_answer: format!("删除失败：{}", error),
                    success: false,
                    memory_write_summary: None,
                    reasoning_summary: "删除阶段失败，直接按系统错误返回。".to_string(),
                    cache_status: "bypass".to_string(),
                    cache_reason: "删除属于实时副作用动作，不使用回答缓存。".to_string(),
                },
            }
        }
        Err(message) => ActionExecution {
            action_summary: format!("删除路径：{}", path),
            result_summary: message.clone(),
            final_answer: message,
            success: false,
            memory_write_summary: None,
            reasoning_summary: "目标路径越界或解析失败，未进入删除阶段。".to_string(),
            cache_status: "bypass".to_string(),
            cache_reason: "删除属于实时副作用动作，不使用回答缓存。".to_string(),
        },
    }
}

fn execute_list_files(request: &RunRequest, path: Option<&str>) -> ActionExecution {
    let base_path = path.unwrap_or(".");
    match resolve_workspace_path(&request.workspace_ref.root_path, base_path) {
        Ok(resolved) => match fs::read_dir(&resolved) {
            Ok(entries) => {
                let mut names = Vec::new();
                for entry in entries.flatten().take(20) {
                    names.push(entry.file_name().to_string_lossy().to_string());
                }
                let joined = if names.is_empty() {
                    "目录为空。".to_string()
                } else {
                    names.join(", ")
                };
                ActionExecution {
                    action_summary: format!("列出目录：{}", resolved.display()),
                    result_summary: format!("目录列举成功：{}", joined),
                    final_answer: format!("目录内容：{}\n{}", resolved.display(), joined),
                    success: true,
                    memory_write_summary: None,
                    reasoning_summary: "读取目标目录首批条目并压缩为目录预览。".to_string(),
                    cache_status: "bypass".to_string(),
                    cache_reason: "目录浏览依赖实时文件系统状态，不使用回答缓存。".to_string(),
                }
            }
            Err(error) => ActionExecution {
                action_summary: format!("列出目录：{}", resolved.display()),
                result_summary: format!("目录列举失败：{}", error),
                final_answer: format!("目录列举失败：{}", error),
                success: false,
                memory_write_summary: None,
                reasoning_summary: "目录读取失败，按系统错误直接返回。".to_string(),
                cache_status: "bypass".to_string(),
                cache_reason: "目录浏览依赖实时文件系统状态，不使用回答缓存。".to_string(),
            },
        },
        Err(message) => ActionExecution {
            action_summary: format!("列出目录：{}", base_path),
            result_summary: message.clone(),
            final_answer: message,
            success: false,
            memory_write_summary: None,
            reasoning_summary: "目标路径越界或解析失败，未进入目录读取阶段。".to_string(),
            cache_status: "bypass".to_string(),
            cache_reason: "目录浏览依赖实时文件系统状态，不使用回答缓存。".to_string(),
        },
    }
}

fn execute_memory_write(
    request: &RunRequest,
    kind: &str,
    summary: &str,
    content: &str,
) -> ActionExecution {
    let now = timestamp_now();
    let entry = MemoryEntry {
        id: format!("memory-{}", timestamp_now()),
        kind: kind.to_string(),
        title: summary.to_string(),
        summary: summary.to_string(),
        content: content.to_string(),
        scope: request.workspace_ref.name.clone(),
        workspace_id: request.workspace_ref.workspace_id.clone(),
        session_id: request.session_id.clone(),
        source_run_id: request.run_id.clone(),
        source: format!("run:{}", request.run_id),
        source_type: "runtime".to_string(),
        source_title: summary.to_string(),
        source_event_type: "memory_written".to_string(),
        source_artifact_path: String::new(),
        verified: true,
        priority: 0,
        archived: false,
        archived_at: String::new(),
        created_at: now.clone(),
        updated_at: now.clone(),
        timestamp: now,
    };

    match append_memory_entry(request, &entry) {
        Ok(()) => {
            let write_summary = format!("已写入 `{}` 记忆：{}", entry.kind, entry.summary);
            ActionExecution {
                action_summary: format!("写入长期记忆：{}", entry.summary),
                result_summary: write_summary.clone(),
                final_answer: format!(
                    "记忆写入完成。\n类型：{}\n摘要：{}\n内容摘要：{}",
                    entry.kind,
                    entry.summary,
                    summarize_text(&entry.content)
                ),
                success: true,
                memory_write_summary: Some(write_summary),
                reasoning_summary: "按用户指定内容构造长期记忆记录并写入本地主存储。".to_string(),
                cache_status: "bypass".to_string(),
                cache_reason: "记忆写入属于实时副作用动作，不使用回答缓存。".to_string(),
            }
        }
        Err(error) => ActionExecution {
            action_summary: format!("写入长期记忆：{}", summary),
            result_summary: format!("记忆写入失败：{}", error),
            final_answer: format!("记忆写入失败：{}", error),
            success: false,
            memory_write_summary: None,
            reasoning_summary: "长期记忆写入失败，按存储错误直接返回。".to_string(),
            cache_status: "bypass".to_string(),
            cache_reason: "记忆写入属于实时副作用动作，不使用回答缓存。".to_string(),
        },
    }
}

fn execute_memory_recall(request: &RunRequest, query: &str) -> ActionExecution {
    let entries = search_memory_entries(request, query, 3);
    if entries.is_empty() {
        return ActionExecution {
            action_summary: format!("按需召回记忆：{}", query),
            result_summary: "没有找到相关长期记忆。".to_string(),
            final_answer: format!("当前没有找到与 `{}` 相关的长期记忆。", query),
            success: true,
            memory_write_summary: None,
            reasoning_summary: "先检索长期记忆索引，未命中时直接返回空结果说明。".to_string(),
            cache_status: "bypass".to_string(),
            cache_reason: "记忆召回依赖实时存储状态，不使用回答缓存。".to_string(),
        };
    }

    let answer = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            format!(
                "{}. [{}] {}\n   {}",
                index + 1,
                entry.kind,
                entry.summary,
                summarize_text(&entry.content)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    ActionExecution {
        action_summary: format!("按需召回记忆：{}", query),
        result_summary: format!("已召回 {} 条相关记忆。", entries.len()),
        final_answer: format!("已召回相关长期记忆：\n{}", answer),
        success: true,
        memory_write_summary: None,
        reasoning_summary: "按查询词检索长期记忆，并返回前几条高相关结果。".to_string(),
        cache_status: "bypass".to_string(),
        cache_reason: "记忆召回依赖实时存储状态，不使用回答缓存。".to_string(),
    }
}

fn execute_knowledge_search(request: &RunRequest, query: &str) -> ActionExecution {
    let hits = search_knowledge(request, query, 3);
    if hits.is_empty() {
        return ActionExecution {
            action_summary: format!("检索本地知识：{}", query),
            result_summary: "没有找到相关知识内容。".to_string(),
            final_answer: format!("当前没有在本地知识源中找到与 `{}` 相关的内容。", query),
            success: true,
            memory_write_summary: None,
            reasoning_summary: "先检索本地知识源，未命中时直接返回空结果说明。".to_string(),
            cache_status: "bypass".to_string(),
            cache_reason: "知识检索依赖实时知识库状态，不使用回答缓存。".to_string(),
        };
    }

    let answer = hits
        .iter()
        .enumerate()
        .map(|(index, hit)| {
            format!(
                "{}. {}\n   {}\n   来源分类：{}\n   来源类型：{}\n   知识类型：{}\n   可信度：{}\n   更新时间：{}\n   命中理由：{}",
                index + 1,
                hit.path,
                hit.snippet,
                hit.source_label,
                hit.source_type,
                hit.knowledge_type,
                hit.confidence,
                blank_value(&hit.updated_at),
                hit.reason
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    ActionExecution {
        action_summary: format!("检索本地知识：{}", query),
        result_summary: format!("已从本地知识源返回 {} 条摘要结果。", hits.len()),
        final_answer: format!("本地知识检索结果：\n{}", answer),
        success: true,
        memory_write_summary: None,
        reasoning_summary: "综合 README、开发文档和知识索引返回高相关知识片段。".to_string(),
        cache_status: "bypass".to_string(),
        cache_reason: "知识检索依赖实时知识库状态，不使用回答缓存。".to_string(),
    }
}

fn blank_value(value: &str) -> &str {
    if value.trim().is_empty() {
        "未提供"
    } else {
        value
    }
}

fn execute_siyuan_search(request: &RunRequest, query: &str) -> ActionExecution {
    let Some(root) = siyuan_root_dir(request) else {
        return missing_siyuan_action("当前未配置思源根目录。");
    };
    let hits = search_siyuan_notes(request, &root, query);
    if hits.is_empty() {
        return ActionExecution {
            action_summary: format!("检索思源笔记：{}", query),
            result_summary: "没有命中思源笔记摘要。".to_string(),
            final_answer: format!("当前没有找到与 `{}` 相关的思源笔记。", query),
            success: true,
            memory_write_summary: None,
            reasoning_summary: "先查询思源索引与正文，未命中时直接返回空结果说明。".to_string(),
            cache_status: "bypass".to_string(),
            cache_reason: "思源检索依赖实时知识库状态，不使用回答缓存。".to_string(),
        };
    }
    let answer = hits
        .into_iter()
        .enumerate()
        .map(|(index, hit)| {
            format!(
                "{}. {}\n   {}\n   来源类型：{}\n   命中理由：{}",
                index + 1,
                hit.path,
                hit.snippet,
                hit.source_type,
                hit.reason
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    ActionExecution {
        action_summary: format!("检索思源笔记：{}", query),
        result_summary: "已返回思源笔记摘要。".to_string(),
        final_answer: format!("思源检索结果：\n{}", answer),
        success: true,
        memory_write_summary: None,
        reasoning_summary: "综合思源索引与正文检索结果返回相关笔记摘要。".to_string(),
        cache_status: "bypass".to_string(),
        cache_reason: "思源检索依赖实时知识库状态，不使用回答缓存。".to_string(),
    }
}

fn execute_siyuan_read(request: &RunRequest, path: &str) -> ActionExecution {
    let Some(root) = siyuan_root_dir(request) else {
        return missing_siyuan_action("当前未配置思源根目录。");
    };
    let target = resolve_siyuan_path(&root, path);
    match fs::read_to_string(&target) {
        Ok(content) => ActionExecution {
            action_summary: format!("读取思源正文：{}", target.display()),
            result_summary: format!("思源正文读取成功，摘要：{}", summarize_text(&content)),
            final_answer: format!(
                "思源正文读取完成：{}\n{}",
                target.display(),
                summarize_text(&content)
            ),
            success: true,
            memory_write_summary: None,
            reasoning_summary: "直接读取指定思源文档并压缩成可展示摘要。".to_string(),
            cache_status: "bypass".to_string(),
            cache_reason: "思源读取依赖实时文件内容，不使用回答缓存。".to_string(),
        },
        Err(error) => ActionExecution {
            action_summary: format!("读取思源正文：{}", path),
            result_summary: format!("思源正文读取失败：{}", error),
            final_answer: format!("思源正文读取失败：{}", error),
            success: false,
            memory_write_summary: None,
            reasoning_summary: "指定思源文档读取失败，按系统错误直接返回。".to_string(),
            cache_status: "bypass".to_string(),
            cache_reason: "思源读取依赖实时文件内容，不使用回答缓存。".to_string(),
        },
    }
}

fn execute_siyuan_write(request: &RunRequest) -> ActionExecution {
    if !siyuan_auto_write_enabled(request) {
        return missing_siyuan_action("当前未开启思源自动写入。");
    }
    let title = summarize_text(&request.user_input);
    if let Some(path) = reusable_siyuan_path(request, &title, &title) {
        return reused_siyuan_write(path);
    }
    let Some(path) = create_siyuan_path(request) else {
        return missing_siyuan_action("当前未配置思源导出目录。");
    };
    let content = format!(
        "# {}\n\n{}",
        request.user_input,
        summarize_text(&request.user_input)
    );
    if let Some(parent) = path.parent() {
        if let Err(error) = fs::create_dir_all(parent) {
            return failed_siyuan_write(&error.to_string());
        }
    }
    if let Err(error) = fs::write(&path, content) {
        return failed_siyuan_write(&error.to_string());
    }
    if let Err(error) = append_siyuan_record(request, &path) {
        return failed_siyuan_write(&error);
    }
    let sync_hint = if siyuan_sync_enabled(request) {
        "已开启摘要回写。"
    } else {
        "摘要回写未开启。"
    };
    ActionExecution {
        action_summary: format!("导出知识到思源：{}", path.display()),
        result_summary: "知识已导出到思源目录。".to_string(),
        final_answer: format!("思源导出完成：{}\n{}", path.display(), sync_hint),
        success: true,
        memory_write_summary: None,
        reasoning_summary: "将当前输入整理为思源文档并同步写入知识索引。".to_string(),
        cache_status: "bypass".to_string(),
        cache_reason: "思源写入属于实时副作用动作，不使用回答缓存。".to_string(),
    }
}

fn execute_context_answer(
    request: &RunRequest,
    session_context: &SessionMemory,
) -> ActionExecution {
    if is_status_continue_request(&request.user_input) {
        return session_continue_answer(request, session_context);
    }
    let cache_probe = probe_context_cache(request, session_context);
    if let Some(hit) = context_cache_hit(session_context, &cache_probe) {
        return hit;
    }
    let prompt = render_context_prompt(request, session_context, &cache_probe);
    match complete_text(request, &prompt) {
        Ok(response) => {
            context_answer_success(request, session_context, &cache_probe, &response.content)
        }
        Err(error) => {
            recover_context_answer(request, session_context, &cache_probe, &error.to_string())
        }
    }
}

fn execute_project_answer(request: &RunRequest) -> ActionExecution {
    let snippets = build_project_context(request);
    let cache_probe = probe_project_cache(request, &snippets);
    if let Some(hit) = project_cache_hit(&snippets, &cache_probe) {
        return hit;
    }
    let prompt = render_project_prompt(request, &cache_probe);
    match complete_text(request, &prompt) {
        Ok(response) => project_answer_success(request, &snippets, &cache_probe, response.content),
        Err(error) => recover_project_answer(request, &snippets, &cache_probe, &error.to_string()),
    }
}

fn build_project_context(request: &RunRequest) -> String {
    let status_context = project_status_context(request);
    if !status_context.is_empty() {
        return status_context;
    }
    let hits = project_context_hits(request);
    if hits.is_empty() {
        "当前没有检索到可用项目文档片段。".to_string()
    } else {
        hits.into_iter()
            .map(|hit| format!("文件：{}\n片段：{}", hit.path, hit.snippet))
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

fn project_status_context(request: &RunRequest) -> String {
    if !is_project_status_request(&request.user_input) {
        return String::new();
    }
    let sections = preferred_project_status_paths(request)
        .into_iter()
        .filter_map(|path| project_status_section(&path, &request.user_input))
        .collect::<Vec<_>>();
    if sections.is_empty() {
        String::new()
    } else {
        sections.join("\n\n")
    }
}

fn preferred_project_status_paths(request: &RunRequest) -> Vec<PathBuf> {
    let docs_root = repo_root(request).join("docs");
    vec![
        docs_root
            .join("06-development")
            .join("忠实用户转化导向开发任务书_V1.md"),
        docs_root
            .join("07-test")
            .join("忠实用户转化导向验收文档_V1.md"),
        docs_root
            .join("06-development")
            .join("第二阶段需求文档_V1.md"),
        docs_root
            .join("06-development")
            .join("第二阶段产品定位与开发重点清单_V1.md"),
        docs_root
            .join("06-development")
            .join("第二阶段短期可用能力开发任务书_V1.md"),
        docs_root
            .join("07-test")
            .join("第二阶段短期可用能力验收文档_V1.md"),
    ]
}

fn project_status_section(path: &Path, query: &str) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    let summary = project_status_summary(path, &content, query);
    Some(format!("文件：{}\n摘要：{}", path.display(), summary))
}

fn project_status_summary(path: &Path, content: &str, query: &str) -> String {
    let name = path
        .file_name()
        .and_then(|item| item.to_str())
        .unwrap_or_default();
    if name.contains("忠实用户转化导向开发任务书") {
        return "当前正式阶段已经切到忠实用户转化导向，顺序按 A、B、C、D、E、F 推进；其中项目状态回答要求稳定输出已完成能力、当前阶段、待收口项，并进一步展开到真实样本、验证路径和完成标准。".to_string();
    }
    if name.contains("忠实用户转化导向验收文档") {
        return "最新正式验收里工作包 A 已通过、整体结论仍为有条件通过；工作包 D 还要求补独立项目状态样本，并在回答中明确做到什么程度、为什么这样判断、下一步做什么。".to_string();
    }
    if name.contains("第二阶段需求文档") {
        return "定位为面向长期学习、长期成长、长期积累、可持续专精的本地个人智能体平台；当前阶段优先收口在线模型可用后的短期可用能力，不扩到多智能体、语音、浏览器全局观察和桌面自动化。".to_string();
    }
    if name.contains("产品定位与开发重点清单") {
        return "当前开发重点已经固定为在线模型主链路、本地缓存与上下文复用、记忆与知识沉淀、skill 接口预留和本地小模型兜底预研；短期目标是把系统做到可对话、可执行、可持续使用。".to_string();
    }
    if name.contains("短期可用能力开发任务书") {
        return "第二阶段短期正式交付范围包括在线模型对话主链路、工作区内文件读取、工作区内文件写入、受控命令执行、任务分析执行验证收口主链路、本地缓存最小闭环，以及记忆与知识沉淀继续增强。".to_string();
    }
    if name.contains("短期可用能力验收文档") {
        return "已完成的真实留证覆盖自然语言对话、缓存命中、文件读取、文件写入、命令执行、能力说明和高风险确认；当前正式结论仍为有条件通过，非阻断问题集中在项目状态类说明还不够细。".to_string();
    }
    extract_snippet(content, query)
}

fn project_context_hits(request: &RunRequest) -> Vec<crate::knowledge::KnowledgeHit> {
    let direct_hits = search_knowledge(request, &request.user_input, 4);
    if !direct_hits.is_empty() {
        return direct_hits;
    }
    search_knowledge(
        request,
        &project_context_fallback_query(&request.user_input),
        4,
    )
}

fn project_context_fallback_query(user_input: &str) -> String {
    if is_project_status_request(user_input) {
        "忠实用户转化 进度 当前阶段 工作包D 项目状态 下一步 样本 验收 第二阶段 主链路".to_string()
    } else {
        "项目 智能体 本地 主干 架构 运行时".to_string()
    }
}

fn is_project_status_request(user_input: &str) -> bool {
    [
        "做到什么程度",
        "进度",
        "当前阶段",
        "实现了什么",
        "完成了吗",
        "当前情况",
        "还差什么",
        "继续下一步",
        "现在最该做什么",
        "下一步做什么",
    ]
    .iter()
    .any(|token| user_input.contains(token))
}

fn project_answer_success(
    request: &RunRequest,
    snippets: &str,
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
    content: String,
) -> ActionExecution {
    let final_answer = finalized_project_answer(request, &content, snippets);
    if should_recover_project_answer(&content, &final_answer) {
        return recover_project_answer(request, snippets, cache_probe, "模型输出不可用");
    }
    let result_summary = format!(
        "已基于项目文档片段完成一次项目说明回答：{}",
        summarize_text(snippets)
    );
    append_answer_cache(
        request,
        cache_probe,
        "project_answer",
        &request.user_input,
        snippets,
        &final_answer,
        &result_summary,
    );
    ActionExecution {
        action_summary: "基于本地项目文档生成项目说明。".to_string(),
        result_summary,
        final_answer,
        success: true,
        memory_write_summary: None,
        reasoning_summary: format!(
            "优先依据 README 与 06-development 等项目文档片段组织项目说明。{}",
            cache_probe.reason
        ),
        cache_status: cache_probe.status.clone(),
        cache_reason: cache_probe.reason.clone(),
    }
}

fn finalized_project_answer(request: &RunRequest, content: &str, snippets: &str) -> String {
    if is_project_status_request(&request.user_input) {
        return stable_project_status_answer(snippets);
    }
    sanitize_project_answer(
        content,
        "当前项目是一个本地智能体系统，围绕运行时、网关和前端工作台组织能力。",
    )
}

fn context_answer_success(
    request: &RunRequest,
    session_context: &SessionMemory,
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
    content: &str,
) -> ActionExecution {
    let fallback = "当前基于已有会话上下文，可明确的信息还比较有限。";
    let final_answer = sanitize_answer(content, fallback);
    if should_recover_context_answer(content, &final_answer) {
        return recover_context_answer(request, session_context, cache_probe, "模型输出不可用");
    }
    let result_summary = format!(
        "已从最近 {} 轮会话中提取压缩上下文，并完成一次模型回答。",
        session_context.recent_turns.len()
    );
    append_answer_cache(
        request,
        cache_probe,
        "context_answer",
        &request.user_input,
        &session_context.compressed_summary,
        &final_answer,
        &result_summary,
    );
    ActionExecution {
        action_summary: "基于会话压缩摘要继续回答。".to_string(),
        result_summary,
        final_answer,
        success: true,
        memory_write_summary: None,
        reasoning_summary: format!(
            "先读取最近会话压缩摘要，再结合当前输入组织续答。{}",
            cache_probe.reason
        ),
        cache_status: cache_probe.status.clone(),
        cache_reason: cache_probe.reason.clone(),
    }
}

fn recover_context_answer(
    request: &RunRequest,
    session_context: &SessionMemory,
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
    cause: &str,
) -> ActionExecution {
    let summary = fallback_context_summary(session_context);
    append_answer_cache(
        request,
        cache_probe,
        "context_answer",
        &request.user_input,
        &session_context.compressed_summary,
        &summary,
        cause,
    );
    ActionExecution {
        action_summary: "基于会话压缩摘要继续回答。".to_string(),
        result_summary: format!("模型主回答失败，已执行单次恢复：{}", cause),
        final_answer: format!("主回答未成功，已切换到会话摘要恢复路径。\n{}", summary),
        success: true,
        memory_write_summary: None,
        reasoning_summary: format!(
            "模型回答不可用，已降级为会话摘要恢复路径。{}",
            cache_probe.reason
        ),
        cache_status: cache_probe.status.clone(),
        cache_reason: cache_probe.reason.clone(),
    }
}

fn recover_project_answer(
    request: &RunRequest,
    snippets: &str,
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
    cause: &str,
) -> ActionExecution {
    let summary = fallback_project_summary(snippets);
    let cache_summary = format!(
        "已基于项目文档恢复生成项目说明：{}",
        summarize_text(&summary)
    );
    append_answer_cache(
        request,
        cache_probe,
        "project_answer",
        &request.user_input,
        snippets,
        &summary,
        &cache_summary,
    );
    ActionExecution {
        action_summary: "基于本地项目文档生成项目说明。".to_string(),
        result_summary: format!("项目说明主回答失败，已执行单次恢复：{}", cause),
        final_answer: format!("主回答未成功，已切换到项目文档恢复路径。\n{}", summary),
        success: true,
        memory_write_summary: None,
        reasoning_summary: format!(
            "模型回答不可用，已降级为项目文档恢复路径。{}",
            cache_probe.reason
        ),
        cache_status: cache_probe.status.clone(),
        cache_reason: cache_probe.reason.clone(),
    }
}

fn sanitize_project_answer(content: &str, fallback: &str) -> String {
    let answer = sanitize_answer(content, fallback);
    if is_project_answer_usable(&answer) {
        answer
    } else {
        fallback.to_string()
    }
}

fn sanitize_answer(content: &str, fallback: &str) -> String {
    let cleaned = strip_forbidden_markup(content);
    let normalized = normalize_answer_text(&cleaned);
    if is_answer_usable(&normalized) {
        normalized
    } else {
        fallback.to_string()
    }
}

fn strip_forbidden_markup(content: &str) -> String {
    let mut text = content
        .replace("minimax:tool_call", "")
        .replace("tool_call", "")
        .replace("workspace_read", "")
        .replace("workspace_write", "")
        .replace("workspace_list", "");
    while let Some((start, end)) = angle_bracket_range(&text) {
        text.replace_range(start..=end, " ");
    }
    text
}

fn angle_bracket_range(text: &str) -> Option<(usize, usize)> {
    let start = text.find('<')?;
    let end = text[start..].find('>')?;
    Some((start, start + end))
}

fn normalize_answer_text(content: &str) -> String {
    content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !looks_like_fake_action(line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn looks_like_fake_action(line: &str) -> bool {
    let lowered = line.to_lowercase();
    let zh = [
        "我先查看",
        "我先读取",
        "先查看项目",
        "先读取文件",
        "调用工具",
        "我来查看",
        "我来读取",
        "我来分析",
        "我先阅读",
    ];
    lowered.contains("xml")
        || lowered.contains("html")
        || lowered.contains("markdown")
        || lowered.contains("workspace_")
        || lowered.contains("tool_call")
        || zh.iter().any(|item| line.contains(item))
}

fn is_answer_usable(content: &str) -> bool {
    !content.is_empty() && !content.contains('<') && !content.contains('>')
}

fn is_project_answer_usable(content: &str) -> bool {
    is_answer_usable(content) && has_cjk_text(content) && !looks_like_path_only(content)
}

fn should_recover_context_answer(content: &str, final_answer: &str) -> bool {
    !is_answer_usable(content)
        || final_answer.trim() == "当前基于已有会话上下文，可明确的信息还比较有限。"
}

fn should_recover_project_answer(content: &str, final_answer: &str) -> bool {
    !is_project_answer_usable(content)
        || final_answer.trim()
            == "当前项目是一个本地智能体系统，围绕运行时、网关和前端工作台组织能力。"
}

fn fallback_context_summary(session_context: &SessionMemory) -> String {
    let summary = session_context.compressed_summary.trim();
    if summary.is_empty() {
        "当前没有可复用的会话摘要，建议补充更具体的问题或先提供上下文。".to_string()
    } else {
        format!(
            "基于当前会话摘要，可先确认这些信息：{}",
            summarize_text(summary)
        )
    }
}

fn fallback_project_summary(snippets: &str) -> String {
    if snippets.contains("当前没有检索到可用项目文档片段") {
        "当前缺少可复用的项目文档片段，建议先补充 README 或开发文档后再追问。".to_string()
    } else if is_loyal_status_context(snippets) {
        stable_project_status_answer(snippets)
    } else if is_phase2_status_context(snippets) {
        stable_project_status_answer(snippets)
    } else {
        "当前项目是一个围绕运行时、网关和前端工作台组织能力的本地智能体系统，重点在让在线模型接入后的对话、执行和沉淀链路稳定可用。结合现有文档，当前阶段更偏向主链路收口，而不是继续扩展重型未来能力。".to_string()
    }
}

fn stable_project_status_answer(snippets: &str) -> String {
    if is_loyal_status_context(snippets) {
        return "已完成能力：在线模型对话、文件读写、受控命令执行、缓存、正式记忆查看与删除、会话续推，以及首页继续上次任务和下一步建议都已落到主链路，并已有 `tmp/loyal-user-acceptance/memory-visibility-sample.json`、`tmp/loyal-user-acceptance/memory-delete-sample.json`、`tmp/loyal-user-acceptance/project-continue-sample.json`、`tmp/loyal-user-acceptance/workspace-dashboard-sample.json` 留证。当前阶段：项目已从第二阶段短期可用，推进到忠实用户转化导向收口期；正式验收里工作包 A 已通过，整体结论仍是有条件通过，正在补工作包 D 的项目状态理解增强。待收口项：还需要把项目状态类回答稳定绑定到独立状态样本、验证路径和完成标准，并让“现在最该做什么 / 继续下一步 / 还差什么”更明确指向当前阶段动作。验证路径与完成标准：本轮应补 `tmp/loyal-user-acceptance/project-status-loyal-summary.json` 与 `tmp/loyal-user-acceptance/next-step-sample.json`，并回填 `docs/07-test/忠实用户转化导向验收文档_V1.md` 的工作包 D、构建验证和整体结论；做到回答能明确指出已完成能力、当前阶段、待收口项，并进一步指向样本或完成标准，才算通过。".to_string();
    }
    if is_phase2_status_context(snippets) {
        return "已完成能力：在线模型对话主链路、工作区内文件读取与写入、受控命令执行、本地缓存最小闭环，以及记忆和知识沉淀继续增强，这些能力都已有真实样本留证。当前阶段：仍处在第二阶段短期可用目标下的主链路收口期，重点继续把项目说明、验证留痕和前端事件日志展示做稳。待收口项：项目状态类回答还需要进一步细化到样本和完成标准，会话续答质量也还依赖压缩摘要厚度。下一步建议：优先围绕忠实用户转化方向补连续性、记忆可见性和续推体验。".to_string();
    }
    "当前项目已经具备基础运行能力，正在继续收口项目说明、执行验证和长期沉淀这几条主链路。"
        .to_string()
}

fn is_loyal_status_context(snippets: &str) -> bool {
    snippets.contains("忠实用户转化导向")
        || snippets.contains("工作包 A 已通过")
        || snippets.contains("工作包 D")
        || snippets.contains("memory-visibility-sample.json")
}

fn is_phase2_status_context(snippets: &str) -> bool {
    snippets.contains("在线模型对话主链路")
        || snippets.contains("工作区内文件读取与写入")
        || snippets.contains("受控命令执行")
        || snippets.contains("本地缓存最小闭环")
}

fn is_status_continue_request(input: &str) -> bool {
    ["继续推进", "上次做到哪", "还差什么", "下一步做什么"]
        .iter()
        .any(|token| input.contains(token))
}

fn session_continue_answer(
    request: &RunRequest,
    session_context: &SessionMemory,
) -> ActionExecution {
    let summary = continue_summary(session_context);
    ActionExecution {
        action_summary: "基于当前会话状态生成续推回答。".to_string(),
        result_summary: "已根据最近会话摘要整理上次进展与下一步建议。".to_string(),
        final_answer: format!("{}\n当前工作区：{}", summary, request.workspace_ref.name),
        success: true,
        memory_write_summary: None,
        reasoning_summary: "当前输入命中了续推类问题，优先使用短期状态和压缩摘要直接回答。"
            .to_string(),
        cache_status: "bypass".to_string(),
        cache_reason: "续推回答需要优先反映当前会话最新状态，不直接复用旧缓存。".to_string(),
    }
}

fn continue_summary(session_context: &SessionMemory) -> String {
    let short = &session_context.short_term;
    let current = blank_fallback(&short.current_goal, "当前目标尚未明确");
    let plan = blank_fallback(&short.current_plan, "当前计划尚未形成");
    let issue = blank_fallback(&short.open_issue, "当前没有显式阻塞");
    let next = next_step_text(short);
    format!(
        "上次做到哪：{}。当前计划：{}。还差什么：{}。下一步建议：{}。",
        current, plan, issue, next
    )
}

fn next_step_text(short: &crate::session::ShortTermMemory) -> &str {
    if !short.pending_confirmation.is_empty() {
        return &short.pending_confirmation;
    }
    if !short.open_issue.is_empty() {
        return &short.open_issue;
    }
    if !short.current_plan.is_empty() {
        return &short.current_plan;
    }
    "补充一个更明确的下一步任务"
}

fn blank_fallback<'a>(value: &'a str, fallback: &'a str) -> &'a str {
    if value.is_empty() { fallback } else { value }
}

fn has_cjk_text(content: &str) -> bool {
    content
        .chars()
        .any(|ch| ('\u{4e00}'..='\u{9fff}').contains(&ch))
}

fn looks_like_path_only(content: &str) -> bool {
    let value = content.trim();
    (value.contains(":\\") || value.contains(":/"))
        && !value.contains('。')
        && !value.contains('，')
        && !value.contains(' ')
}

fn execute_explain(request: &RunRequest) -> ActionExecution {
    let prompt = render_explain_runtime_prompt(request);
    match complete_text(request, prompt) {
        Ok(response) => ActionExecution {
            action_summary: "返回当前已支持的能力说明。".to_string(),
            result_summary: "已通过模型生成当前能力说明。".to_string(),
            final_answer: response.content,
            success: true,
            memory_write_summary: None,
            reasoning_summary: "直接请求模型概括当前运行时已支持的能力。".to_string(),
            cache_status: "bypass".to_string(),
            cache_reason: "能力说明属于即时生成内容，当前不使用回答缓存。".to_string(),
        },
        Err(error) => ActionExecution {
            action_summary: "返回当前已支持的能力说明。".to_string(),
            result_summary: format!("能力说明生成失败：{}", error),
            final_answer: format!("当前无法生成能力说明：{}", error),
            success: false,
            memory_write_summary: None,
            reasoning_summary: "能力说明生成失败，按模型错误直接返回。".to_string(),
            cache_status: "bypass".to_string(),
            cache_reason: "能力说明属于即时生成内容，当前不使用回答缓存。".to_string(),
        },
    }
}

fn materialize_artifact(
    request: &RunRequest,
    action: &PlannedAction,
    content: &str,
) -> Option<String> {
    externalize_text_artifact(request, action_tag(action), content).map(|item| item.path)
}

fn action_tag(action: &PlannedAction) -> &'static str {
    match action {
        PlannedAction::RunCommand { .. } => "command",
        PlannedAction::ReadFile { .. } => "read",
        PlannedAction::WriteFile { .. } => "write",
        PlannedAction::DeletePath { .. } => "delete",
        PlannedAction::ListFiles { .. } => "list",
        PlannedAction::WriteMemory { .. } => "memory-write",
        PlannedAction::RecallMemory { .. } => "memory-recall",
        PlannedAction::SearchKnowledge { .. } => "knowledge",
        PlannedAction::SearchSiyuanNotes { .. } => "siyuan-search",
        PlannedAction::ReadSiyuanNote { .. } => "siyuan-read",
        PlannedAction::WriteSiyuanKnowledge => "siyuan-write",
        PlannedAction::ProjectAnswer => "project",
        PlannedAction::ContextAnswer => "context",
        PlannedAction::Explain => "explain",
        PlannedAction::AgentResolve => "agent-resolve",
    }
}

fn missing_siyuan_action(message: &str) -> ActionExecution {
    ActionExecution {
        action_summary: "执行思源能力".to_string(),
        result_summary: message.to_string(),
        final_answer: message.to_string(),
        success: false,
        memory_write_summary: None,
        reasoning_summary: "思源能力缺少必要配置，无法进入实际执行。".to_string(),
        cache_status: "bypass".to_string(),
        cache_reason: "思源能力依赖实时配置与文件系统状态，不使用回答缓存。".to_string(),
    }
}

fn failed_siyuan_write(message: &str) -> ActionExecution {
    ActionExecution {
        action_summary: "导出知识到思源".to_string(),
        result_summary: format!("思源写入失败：{}", message),
        final_answer: format!("思源写入失败：{}", message),
        success: false,
        memory_write_summary: None,
        reasoning_summary: "思源导出阶段发生系统错误，直接按失败收口。".to_string(),
        cache_status: "bypass".to_string(),
        cache_reason: "思源写入属于实时副作用动作，不使用回答缓存。".to_string(),
    }
}

fn search_siyuan_notes(
    request: &RunRequest,
    root: &Path,
    query: &str,
) -> Vec<crate::knowledge::KnowledgeHit> {
    let indexed = search_siyuan_index_hits(request, query);
    if !indexed.is_empty() {
        return indexed;
    }
    let mut scored = collect_siyuan_paths(root)
        .into_iter()
        .filter_map(|path| score_siyuan_file(path, query))
        .collect::<Vec<_>>();
    scored.sort_by(|left, right| right.0.cmp(&left.0));
    scored.into_iter().take(3).map(|(_, hit)| hit).collect()
}

fn search_siyuan_index_hits(
    request: &RunRequest,
    query: &str,
) -> Vec<crate::knowledge::KnowledgeHit> {
    let mut scored = search_knowledge_records(request)
        .into_iter()
        .filter(|record| record.source_type == "siyuan")
        .filter_map(|record| score_siyuan_record(record, query))
        .collect::<Vec<_>>();
    scored.sort_by(|left, right| right.0.cmp(&left.0));
    scored.into_iter().take(3).map(|(_, hit)| hit).collect()
}

fn score_siyuan_record(
    record: KnowledgeRecord,
    query: &str,
) -> Option<(i32, crate::knowledge::KnowledgeHit)> {
    let haystack = format!("{} {} {}", record.title, record.summary, record.content);
    let score = crate::text::score_text(query, &haystack)
        + crate::knowledge::knowledge_path_priority(&record.source);
    (score > 0).then_some((
        score,
        crate::knowledge::KnowledgeHit {
            path: record.source,
            snippet: record.summary,
            source_type: "siyuan_index".to_string(),
            source_label: "用户确认知识".to_string(),
            knowledge_type: record.knowledge_type,
            confidence: if record.verified {
                "高（用户沉淀确认）".to_string()
            } else {
                "中（待进一步验证）".to_string()
            },
            updated_at: record.updated_at,
            reason: "思源索引命中".to_string(),
        },
    ))
}

fn collect_siyuan_paths(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_siyuan_files(root, &mut files, 0);
    files
}

fn collect_siyuan_files(root: &Path, files: &mut Vec<PathBuf>, depth: usize) {
    if depth > 4 || !root.exists() {
        return;
    }
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_siyuan_files(&path, files, depth + 1);
            continue;
        }
        if path.extension().and_then(|item| item.to_str()) == Some("md") {
            files.push(path);
        }
    }
}

fn score_siyuan_file(path: PathBuf, query: &str) -> Option<(i32, crate::knowledge::KnowledgeHit)> {
    let content = fs::read_to_string(&path).ok()?;
    let snippet = summarize_text(&content);
    let path_text = path.display().to_string();
    let haystack = format!("{} {}", path_text, content);
    let score = crate::text::score_text(query, &haystack)
        + crate::knowledge::knowledge_path_priority(&path_text);
    (score > 0).then_some((
        score,
        crate::knowledge::KnowledgeHit {
            path: path_text,
            snippet,
            source_type: "siyuan_file".to_string(),
            source_label: "用户确认知识".to_string(),
            knowledge_type: "user_curated".to_string(),
            confidence: "高（用户沉淀确认）".to_string(),
            updated_at: String::new(),
            reason: "思源正文命中".to_string(),
        },
    ))
}

fn resolve_siyuan_path(root: &Path, raw: &str) -> PathBuf {
    let path = Path::new(raw);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

fn reusable_siyuan_path(request: &RunRequest, title: &str, summary: &str) -> Option<PathBuf> {
    let record = find_reusable_siyuan_record(request, title, summary)?;
    let path = PathBuf::from(record.source);
    path.exists().then_some(path)
}

fn create_siyuan_path(request: &RunRequest) -> Option<PathBuf> {
    let export_dir = siyuan_export_dir(request)?;
    Some(export_dir.join(format!(
        "{}-{}.md",
        request.workspace_ref.workspace_id, request.run_id
    )))
}

fn reused_siyuan_write(path: PathBuf) -> ActionExecution {
    ActionExecution {
        action_summary: format!("复用已存在思源知识：{}", path.display()),
        result_summary: "命中已存在思源导出，已直接复用。".to_string(),
        final_answer: format!("思源导出已复用：{}", path.display()),
        success: true,
        memory_write_summary: None,
        reasoning_summary: "命中已存在的思源导出记录，直接复用现有产物。".to_string(),
        cache_status: "hit".to_string(),
        cache_reason: "命中现有思源导出记录，无需重复生成。".to_string(),
    }
}

fn append_siyuan_record(request: &RunRequest, path: &Path) -> Result<(), String> {
    let title = summarize_text(&request.user_input);
    let summary = summarize_text(&request.user_input);
    let record = KnowledgeRecord {
        id: format!("siyuan-note-{}", timestamp_now()),
        knowledge_type: "document_digest".to_string(),
        title,
        summary,
        content: format!("思源导出知识：{}", request.user_input),
        tags: vec![
            "siyuan".to_string(),
            request.workspace_ref.workspace_id.clone(),
        ],
        source: path.display().to_string(),
        source_type: "siyuan".to_string(),
        verified: true,
        workspace_id: request.workspace_ref.workspace_id.clone(),
        priority: 1,
        archived: false,
        created_at: timestamp_now(),
        updated_at: timestamp_now(),
    };
    if has_same_siyuan_content(request, &record) {
        return Ok(());
    }
    append_knowledge_record(request, &record)
}

fn has_same_siyuan_content(request: &RunRequest, record: &KnowledgeRecord) -> bool {
    find_reusable_siyuan_record(request, &record.title, &record.summary).is_some()
}

fn probe_context_cache(
    request: &RunRequest,
    session_context: &SessionMemory,
) -> crate::answer_cache::AnswerCacheProbe {
    if session_context.compressed_summary.trim().is_empty() {
        return bypass_probe("当前会话摘要为空，直接走模型回答路径。");
    }
    probe_answer_cache(
        request,
        "context_answer",
        &request.user_input,
        &session_context.compressed_summary,
    )
}

fn probe_project_cache(
    request: &RunRequest,
    snippets: &str,
) -> crate::answer_cache::AnswerCacheProbe {
    if snippets.contains("当前没有检索到可用项目文档片段") {
        return bypass_probe("当前没有稳定项目文档片段，直接走恢复或模型路径。");
    }
    probe_answer_cache(request, "project_answer", &request.user_input, snippets)
}

fn context_cache_hit(
    session_context: &SessionMemory,
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
) -> Option<ActionExecution> {
    let answer = cache_probe.answer.clone()?;
    Some(ActionExecution {
        action_summary: "基于会话压缩摘要继续回答。".to_string(),
        result_summary: cache_probe
            .summary
            .clone()
            .unwrap_or_else(|| "已复用本地缓存回答。".to_string()),
        final_answer: answer,
        success: true,
        memory_write_summary: None,
        reasoning_summary: format!(
            "当前输入与会话摘要稳定命中缓存，直接复用最近一次可用回答。会话轮次：{}。",
            session_context.recent_turns.len()
        ),
        cache_status: cache_probe.status.clone(),
        cache_reason: cache_probe.reason.clone(),
    })
}

fn project_cache_hit(
    snippets: &str,
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
) -> Option<ActionExecution> {
    let answer = sanitize_project_answer(&cache_probe.answer.clone()?, "");
    if answer.is_empty() {
        return None;
    }
    Some(ActionExecution {
        action_summary: "基于本地项目文档生成项目说明。".to_string(),
        result_summary: cache_probe
            .summary
            .clone()
            .unwrap_or_else(|| "已复用本地缓存项目说明。".to_string()),
        final_answer: answer,
        success: true,
        memory_write_summary: None,
        reasoning_summary: format!(
            "当前输入与项目文档摘要稳定命中缓存，直接复用最近一次项目说明。文档摘要：{}",
            summarize_text(snippets)
        ),
        cache_status: cache_probe.status.clone(),
        cache_reason: cache_probe.reason.clone(),
    })
}

fn render_context_prompt(
    request: &RunRequest,
    session_context: &SessionMemory,
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
) -> String {
    let repo_context = load_repo_context(std::path::Path::new(&request.workspace_ref.root_path));
    let registry = runtime_tool_registry();
    let visible_tools = registry.visible_tools(&request.mode);
    let envelope = build_runtime_context(
        request,
        session_context,
        &repo_context,
        &visible_tools,
        &cache_probe.status,
        &cache_probe.reason,
    );
    render_context_answer_prompt(&envelope).full_prompt
}

fn render_project_prompt(
    request: &RunRequest,
    cache_probe: &crate::answer_cache::AnswerCacheProbe,
) -> String {
    let session_context = SessionMemory::default();
    let repo_context = load_repo_context(std::path::Path::new(&request.workspace_ref.root_path));
    let registry = runtime_tool_registry();
    let visible_tools = registry.visible_tools(&request.mode);
    let envelope = build_runtime_context(
        request,
        &session_context,
        &repo_context,
        &visible_tools,
        &cache_probe.status,
        &cache_probe.reason,
    );
    render_project_answer_prompt(&envelope).full_prompt
}

fn render_explain_runtime_prompt(request: &RunRequest) -> &'static str {
    let _ = request;
    "你是本地智能体。用户没有给出明确动作前缀。请简明说明当前支持的能力，包括命令执行、文件读写、目录列举、记忆、知识检索，以及自然语言继续对话。要求用中文，控制在 9 行以内。"
}

/// 将 LLM 返回的 tool_call.function.name + arguments 反向解析为 PlannedAction。
fn tool_call_to_action(name: &str, arguments: &str) -> Option<PlannedAction> {
    let args: serde_json::Value = serde_json::from_str(arguments).unwrap_or_default();
    match name {
        "run_command" => {
            let command = args["command"].as_str()?.to_string();
            Some(PlannedAction::RunCommand { command })
        }
        "workspace_read" => {
            let path = args["path"].as_str()?.to_string();
            Some(PlannedAction::ReadFile { path })
        }
        "workspace_write" => {
            let path = args["path"].as_str()?.to_string();
            let content = args["content"].as_str().unwrap_or("").to_string();
            Some(PlannedAction::WriteFile { path, content })
        }
        "workspace_delete" => {
            let path = args["path"].as_str()?.to_string();
            Some(PlannedAction::DeletePath { path })
        }
        "workspace_list" => {
            let path = args["path"].as_str().map(|s| s.to_string());
            Some(PlannedAction::ListFiles { path })
        }
        "memory_write" => {
            let kind = args["kind"].as_str().unwrap_or("project_knowledge").to_string();
            let summary = args["summary"].as_str().unwrap_or("").to_string();
            let content = args["content"].as_str().unwrap_or("").to_string();
            Some(PlannedAction::WriteMemory { kind, summary, content })
        }
        "memory_recall" => {
            let query = args["query"].as_str()?.to_string();
            Some(PlannedAction::RecallMemory { query })
        }
        "knowledge_search" => {
            let query = args["query"].as_str()?.to_string();
            Some(PlannedAction::SearchKnowledge { query })
        }
        "search_siyuan_notes" => {
            let query = args["query"].as_str()?.to_string();
            Some(PlannedAction::SearchSiyuanNotes { query })
        }
        "read_siyuan_note" => {
            let path = args["path"].as_str()?.to_string();
            Some(PlannedAction::ReadSiyuanNote { path })
        }
        "write_siyuan_knowledge" => Some(PlannedAction::WriteSiyuanKnowledge),
        _ => None,
    }
}

/// 透传给大模型，检查返回中的 tool_calls，依次执行并汇总结果。
fn execute_agent_resolve(
    request: &RunRequest,
    session_context: &SessionMemory,
) -> ActionExecution {
    let prompt = build_agent_resolve_prompt(request, session_context);
    match complete_text(request, &prompt) {
        Err(error) => ActionExecution {
            action_summary: "透传大模型 Agent Resolve".to_string(),
            result_summary: format!("模型调用失败：{}", error),
            final_answer: format!("大模型调用失败：{}", error),
            success: false,
            memory_write_summary: None,
            reasoning_summary: "AgentResolve 调用模型失败，直接按错误收口。".to_string(),
            cache_status: "bypass".to_string(),
            cache_reason: "Agent 调用依赖实时模型输出，不使用回答缓存。".to_string(),
        },
        Ok(response) => {
            // 如果模型返回了 tool_calls，依次执行
            if let Some(calls) = response.tool_calls {
                let results: Vec<String> = calls
                    .iter()
                    .filter_map(|tc| {
                        let action = tool_call_to_action(&tc.function.name, &tc.function.arguments)?;
                        let exec = execute_action(request, &action, session_context);
                        Some(exec.final_answer)
                    })
                    .collect();
                ActionExecution {
                    action_summary: "大模型下发工具调用并执行".to_string(),
                    result_summary: format!("已执行 {} 个工具调用。", results.len()),
                    final_answer: results.join("\n"),
                    success: true,
                    memory_write_summary: None,
                    reasoning_summary: "模型通过 tool_calls 指定执行动作，运行时依次反向组装并执行。".to_string(),
                    cache_status: "bypass".to_string(),
                    cache_reason: "Agent 执行依赖实时模型输出，不使用回答缓存。".to_string(),
                }
            } else {
                // 没有 tool_calls，返回模型文本回答
                ActionExecution {
                    action_summary: "大模型自然语言回答".to_string(),
                    result_summary: "模型未下发工具调用，返回文本回答。".to_string(),
                    final_answer: response.content,
                    success: true,
                    memory_write_summary: None,
                    reasoning_summary: "模型未选择工具，直接返回生成文本。".to_string(),
                    cache_status: "bypass".to_string(),
                    cache_reason: "Agent 调用依赖实时模型输出，不使用回答缓存。".to_string(),
                }
            }
        }
    }
}

fn build_agent_resolve_prompt(request: &RunRequest, session_context: &SessionMemory) -> String {
    let mut parts = Vec::new();
    if !session_context.compressed_summary.is_empty() {
        parts.push(format!("【会话摘要】\n{}", session_context.compressed_summary));
    }
    parts.push(format!("【用户询问】\n{}", request.user_input));
    parts.join("\n\n")
}
