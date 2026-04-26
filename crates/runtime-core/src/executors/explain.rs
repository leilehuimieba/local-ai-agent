use crate::contracts::RunRequest;
use crate::execution::ActionExecution;

const CACHE_REASON: &str = "能力说明属于即时生成内容，当前不使用回答缓存。";

pub(crate) fn execute_explain(request: &RunRequest) -> ActionExecution {
    let _ = request;
    ok_explain(render_static_capability_answer().to_string())
}

fn ok_explain(content: String) -> ActionExecution {
    ActionExecution::bypass_ok(
        "返回当前已支持的能力说明。".to_string(),
        "已通过模型生成当前能力说明。".to_string(),
        content,
        "使用本地静态模板返回当前运行时已支持的能力，避免能力说明依赖外部 provider。".to_string(),
        CACHE_REASON,
    )
}

fn render_static_capability_answer() -> &'static str {
    "你可以把我当成会推进本地项目的工作助手，而不只是聊天窗口。

你可以直接这样说：
1. 检查当前项目状态，并告诉我下一步该做什么。
2. 跑测试；如果失败，定位原因并给出最小修复。
3. 修改一个功能，但先说明影响范围和验证方式。
4. 准备上线前检查清单，执行后留下证据。
5. 继续上次任务，接着已完成的 change 往前推进。

我和通用竞品最大的区别：我会围绕你的本地工作区、项目文档、执行记录和验证证据持续推进；涉及文件修改、命令执行或高风险动作时，会尽量给出确认、影响范围和可回退线索。"
}
