pub(crate) fn summarize_text(text: &str) -> String {
    let compact = text.trim().replace('\r', "");
    let shortened: String = compact.chars().take(240).collect();
    if compact.chars().count() > 240 {
        format!("{}...", shortened)
    } else if shortened.is_empty() {
        "无可显示内容。".to_string()
    } else {
        shortened
    }
}

pub(crate) fn extract_snippet(content: &str, query: &str) -> String {
    let lower_content = content.to_lowercase();
    let lower_query = query.trim().to_lowercase();
    if !lower_query.is_empty() {
        if let Some(index) = lower_content.find(&lower_query) {
            let prefix_chars = lower_content[..index].chars().count();
            let query_chars = lower_query.chars().count();
            let start_char = prefix_chars.saturating_sub(80);
            let end_char = usize::min(prefix_chars + query_chars + 120, content.chars().count());
            let snippet = content
                .chars()
                .skip(start_char)
                .take(end_char.saturating_sub(start_char))
                .collect::<String>();
            return summarize_text(&snippet);
        }
    }
    summarize_text(content)
}

pub(crate) fn score_text(query: &str, text: &str) -> i32 {
    let query = query.trim().to_lowercase();
    let haystack = text.to_lowercase();
    if query.is_empty() {
        return 1;
    }

    let mut score = 0;
    if haystack.contains(&query) {
        score += 40;
    }

    for token in query
        .split(|c: char| c.is_whitespace() || matches!(c, ':' | ',' | '|' | '/' | '\\'))
        .filter(|token| !token.is_empty())
    {
        if haystack.contains(token) {
            score += 12;
        }
    }

    score
}
