pub(crate) fn sanitize_answer(content: &str, fallback: &str) -> String {
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

pub(crate) fn is_answer_usable(content: &str) -> bool {
    !content.is_empty() && !content.contains('<') && !content.contains('>')
}
