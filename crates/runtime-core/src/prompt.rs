use crate::context_builder::RuntimeContextEnvelope;

#[derive(Clone, Debug)]
pub(crate) struct PromptRenderResult {
    pub full_prompt: String,
}

pub(crate) fn render_context_answer_prompt(
    envelope: &RuntimeContextEnvelope,
) -> PromptRenderResult {
    render_prompt(
        envelope,
        "请基于会话上下文直接回答用户当前输入。",
        concat!(
            "只输出纯中文自然语言；",
            "禁止输出任何 tool_call、minimax:tool_call、workspace_read 等工具调用字样；",
            "禁止输出任何 XML、HTML、Markdown 标签、尖括号包裹内容或标签式动作；",
            "禁止输出代码块、协议块、伪指令、伪动作计划；",
            "不能写“我先查看/读取/调用”，只能基于已有上下文直接回答；",
            "如信息不足，只能说明基于当前上下文可知的内容。"
        ),
    )
}

pub(crate) fn render_project_answer_prompt(
    envelope: &RuntimeContextEnvelope,
) -> PromptRenderResult {
    render_prompt(
        envelope,
        &project_task_hint(&envelope.dynamic_block.user_input),
        concat!(
            "只根据项目上下文回答；",
            "只输出纯中文自然语言；",
            "控制在 3 句话以内；",
            "禁止输出任何 tool_call、minimax:tool_call、workspace_read 等工具调用字样；",
            "禁止输出任何 XML、HTML、Markdown 标签、尖括号包裹内容或标签式动作；",
            "禁止输出代码块、协议块、伪指令、伪动作计划；",
            "不能写“我先查看/读取文件”，必须直接给出项目说明；",
            "不要输出能力清单；",
            "不要以“我来/我先/我将”开头；",
            "如果用户在问当前进度，优先回答已完成能力、当前阶段和剩余缺口；",
            "优先使用“已完成、当前阶段、待收口”这类表述；",
            "如上下文已包含真实样本、验收文档或完成标准，要优先点出对应样本路径、验证路径或完成标准；",
            "不要把项目说成还停留在纯需求阶段。"
        ),
    )
}

pub(crate) fn render_agent_resolve_prompt(user_input: &str, session_summary: &str) -> String {
    let summary = if session_summary.trim().is_empty() {
        "当前会话还没有可复用的压缩摘要。"
    } else {
        session_summary
    };
    format!(
        concat!(
            "你是本地智能体，负责在当前工作区内完成真实任务。\n\n",
            "会话摘要：{}\n\n",
            "用户请求：{}\n",
            "执行要求：可以分步调用工具，但不要只停在读取、检索或观察；",
            "如果用户要求生成文件、写回结果或保存产物，完成前必须实际调用写入类工具；",
            "只有在任务真正完成后，才输出最终中文结果，并尽量点明产物路径。"
        ),
        summary, user_input
    )
}

fn project_task_hint(user_input: &str) -> String {
    if is_status_question(user_input) {
        "请直接说明当前项目已做到什么程度、为什么这样判断、下一步做什么，并尽量落到真实样本、验证路径或完成标准。".to_string()
    } else {
        "请直接说明当前项目是做什么的、当前主目标是什么。".to_string()
    }
}

fn is_status_question(user_input: &str) -> bool {
    [
        "做到什么程度",
        "进度",
        "当前阶段",
        "还差什么",
        "继续下一步",
        "现在最该做什么",
        "下一步做什么",
    ]
    .iter()
    .any(|token| user_input.contains(token))
}

fn render_prompt(
    envelope: &RuntimeContextEnvelope,
    task_hint: &str,
    answer_style: &str,
) -> PromptRenderResult {
    let static_prompt = render_static_prompt(envelope);
    let project_prompt = render_project_prompt(envelope);
    let dynamic_prompt = render_dynamic_prompt(envelope, task_hint, answer_style);
    let full_prompt = join_prompt_parts(&static_prompt, &project_prompt, &dynamic_prompt);
    PromptRenderResult { full_prompt }
}

fn render_static_prompt(envelope: &RuntimeContextEnvelope) -> String {
    format!(
        "{}\n{}",
        envelope.static_block.role_prompt, envelope.static_block.mode_prompt
    )
}

fn render_project_prompt(envelope: &RuntimeContextEnvelope) -> String {
    format!(
        "工作区：{}\n仓库摘要：{}\n说明文件摘要：{}",
        envelope.project_block.workspace_root,
        envelope.project_block.repo_summary,
        envelope.project_block.doc_summary
    )
}

fn render_dynamic_prompt(
    envelope: &RuntimeContextEnvelope,
    task_hint: &str,
    answer_style: &str,
) -> String {
    format!(
        "任务意图：{}\n当前阶段：{}\n调度原因：{}\n用户输入：{}\n会话摘要：{}\n记忆摘要：{}\n知识摘要：{}\n可见工具：{}\n交接提示：{}\n回答要求：{}",
        task_hint,
        envelope.dynamic_block.phase_label,
        envelope.dynamic_block.selection_reason,
        envelope.dynamic_block.user_input,
        envelope.dynamic_block.session_summary,
        envelope.dynamic_block.memory_digest,
        envelope.dynamic_block.knowledge_digest,
        envelope.dynamic_block.tool_preview,
        envelope.dynamic_block.artifact_hint,
        answer_style
    )
}

fn join_prompt_parts(static_prompt: &str, project_prompt: &str, dynamic_prompt: &str) -> String {
    [static_prompt, project_prompt, dynamic_prompt].join("\n\n")
}
