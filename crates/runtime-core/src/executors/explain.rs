use crate::contracts::RunRequest;
use crate::execution::ActionExecution;
use crate::llm::complete_text;

const CACHE_REASON: &str = "能力说明属于即时生成内容，当前不使用回答缓存。";

pub(crate) fn execute_explain(request: &RunRequest) -> ActionExecution {
    let prompt = render_explain_runtime_prompt(request);
    match complete_text(request, prompt) {
        Ok(response) => ok_explain(response.content),
        Err(error) => fail_explain(&error),
    }
}

fn ok_explain(content: String) -> ActionExecution {
    ActionExecution::bypass_ok(
        "返回当前已支持的能力说明。".to_string(),
        "已通过模型生成当前能力说明。".to_string(),
        content,
        "直接请求模型概括当前运行时已支持的能力。".to_string(),
        CACHE_REASON,
    )
}

fn fail_explain(error: &str) -> ActionExecution {
    ActionExecution::bypass_fail(
        "返回当前已支持的能力说明。".to_string(),
        format!("能力说明生成失败：{}", error),
        format!("当前无法生成能力说明：{}", error),
        "能力说明生成失败，按模型错误直接返回。".to_string(),
        CACHE_REASON,
    )
}

fn render_explain_runtime_prompt(request: &RunRequest) -> &'static str {
    let _ = request;
    "你是本地智能体。用户没有给出明确动作前缀。请简明说明当前支持的能力，包括命令执行、文件读写、目录列举、记忆、知识检索，以及自然语言继续对话。要求用中文，控制在 9 行以内。"
}
