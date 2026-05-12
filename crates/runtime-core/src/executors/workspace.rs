use crate::contracts::RunRequest;
use crate::execution::ActionExecution;
use crate::paths::resolve_workspace_path;
use crate::planner::SearchOutputMode;
use crate::text::summarize_text;
use regex::Regex;
use std::fs;
use std::path::Path;

const CACHE_READ_REASON: &str = "文件读取结果依赖实时文件内容，不使用回答缓存。";
const CACHE_WRITE_REASON: &str = "文件写入属于实时副作用动作，不使用回答缓存。";
const CACHE_DELETE_REASON: &str = "删除属于实时副作用动作，不使用回答缓存。";
const CACHE_LIST_REASON: &str = "目录浏览依赖实时文件系统状态，不使用回答缓存。";
const CACHE_SEARCH_REASON: &str = "文件搜索依赖实时文件系统状态，不使用回答缓存。";
const SEARCH_MAX_DEPTH: u32 = 8;
const SEARCH_MAX_LINES: usize = 500;
const SEARCH_MAX_FILE_SIZE: u64 = 2 * 1024 * 1024; // 2MB
const SEARCH_MAX_RESULT_CHARS: usize = 8000;

const SKIP_DIRS: &[&str] = &[
    ".git", "target", "node_modules", "__pycache__", ".venv", "venv",
    ".idea", ".vscode", "dist", "build", ".next", ".nuxt", "vendor",
    ".cache", ".claude",
];

const TEXT_EXTENSIONS: &[&str] = &[
    "rs", "go", "js", "ts", "jsx", "tsx", "py", "java", "kt", "swift",
    "c", "cpp", "cc", "cxx", "h", "hpp", "rb", "php", "cs", "scala",
    "toml", "yaml", "yml", "json", "xml", "csv", "ini", "cfg", "conf",
    "md", "txt", "rst", "tex", "html", "htm", "css", "scss", "less",
    "sql", "sh", "bash", "zsh", "ps1", "bat", "cmake", "make", "dockerfile",
    "proto", "graphql", "vue", "svelte", "astro", "elm", "ex", "exs",
    "erl", "hrl", "clj", "cljs", "edn", "lua", "r", "jl", "dart",
    "zig", "nim", "v", "fs", "fsx", "ml", "mli", "hs", "lhs", "pl",
    "pm", "tcl", "groovy", "gradle", "lock",
];

pub(crate) fn execute_file_read(
    request: &RunRequest,
    path: &str,
    offset: Option<usize>,
    limit: Option<usize>,
) -> ActionExecution {
    let resolved = match resolve_path(request, "读取文件", path, CACHE_READ_REASON) {
        Ok(resolved) => resolved,
        Err(outcome) => return outcome,
    };
    if is_binary_file(&resolved) {
        return fail(
            format!("读取文件：{}", path),
            "目标文件为二进制文件，不支持文本读取。".to_string(),
            "目标文件为二进制文件，不支持文本读取。".to_string(),
            "二进制文件拒绝读取，返回错误。",
            CACHE_READ_REASON,
        );
    }
    match fs::read_to_string(&resolved) {
        Ok(content) => ok_file_read(&resolved, &content, offset, limit),
        Err(error) => fail(
            format!("读取文件：{}", path),
            format!("文件读取失败：{}", error),
            format!("文件读取失败：{}", error),
            "目标文件读取失败，按错误直接返回。",
            CACHE_READ_REASON,
        ),
    }
}

pub(crate) fn execute_file_write(
    request: &RunRequest,
    path: &str,
    content: &str,
    write_mode: Option<&str>,
) -> ActionExecution {
    let resolved = match resolve_path(request, "写入文件", path, CACHE_WRITE_REASON) {
        Ok(resolved) => resolved,
        Err(outcome) => return outcome,
    };
    if let Err(error) = ensure_parent_dir(&resolved) {
        return fail_create_parent_dir(&resolved, &error.to_string(), CACHE_WRITE_REASON);
    }
    let is_append = write_mode.is_some_and(|m| m.eq_ignore_ascii_case("append"));
    let write_result = if is_append {
        append_to_file(&resolved, content)
    } else {
        fs::write(&resolved, content)
    };
    if let Err(error) = write_result {
        return fail_write_file(&resolved, &error.to_string(), CACHE_WRITE_REASON);
    }
    if is_append {
        ok_file_append(&resolved, content)
    } else {
        ok_file_write(&resolved, content)
    }
}

fn append_to_file(resolved: &Path, content: &str) -> Result<(), std::io::Error> {
    use std::io::Write;
    let mut file = fs::OpenOptions::new().append(true).create(true).open(resolved)?;
    file.write_all(content.as_bytes())
}

pub(crate) fn execute_delete_path(request: &RunRequest, path: &str) -> ActionExecution {
    let resolved = match resolve_path(request, "删除路径", path, CACHE_DELETE_REASON) {
        Ok(resolved) => resolved,
        Err(outcome) => return outcome,
    };
    let result = if resolved.is_dir() {
        fs::remove_dir_all(&resolved)
    } else {
        fs::remove_file(&resolved)
    };
    match result {
        Ok(()) => ok_file_delete(&resolved),
        Err(error) => fail(
            format!("删除路径：{}", resolved.display()),
            format!("删除失败：{}", error),
            format!("删除失败：{}", error),
            "删除阶段失败，直接按系统错误返回。",
            CACHE_DELETE_REASON,
        ),
    }
}

pub(crate) fn execute_list_files(
    request: &RunRequest,
    path: Option<&str>,
    recursive: bool,
    file_glob: Option<&str>,
) -> ActionExecution {
    let base_path = path.unwrap_or(".");
    let Ok(resolved) = resolve_workspace_path(&request.workspace_ref.root_path, base_path) else {
        return invalid_path("列出目录", base_path, "目标路径越界或解析失败", CACHE_LIST_REASON);
    };
    match fs::read_dir(&resolved) {
        Ok(entries) => {
            if recursive {
                ok_list_tree(&resolved, file_glob)
            } else {
                ok_list_flat(&resolved, entries, file_glob)
            }
        }
        Err(error) => fail(
            format!("列出目录：{}", resolved.display()),
            format!("目录列举失败：{}", error),
            format!("目录列举失败：{}", error),
            "目录读取失败，按系统错误直接返回。",
            CACHE_LIST_REASON,
        ),
    }
}

struct DirEntry {
    name: String,
    is_dir: bool,
    size: u64,
}

fn collect_entries(
    entries: fs::ReadDir,
    file_glob: Option<&str>,
) -> Vec<DirEntry> {
    let mut result: Vec<DirEntry> = Vec::new();
    for entry in entries.flatten() {
        let file_type = entry.file_type().ok();
        let is_dir = file_type.as_ref().is_some_and(|t| t.is_dir());
        let name = entry.file_name().to_string_lossy().to_string();
        if let Some(glob) = file_glob {
            if !is_dir && !glob_matches(glob, &name) {
                continue;
            }
        }
        let size = if is_dir {
            0
        } else {
            entry.metadata().map(|m| m.len()).unwrap_or(0)
        };
        result.push(DirEntry { name, is_dir, size });
    }
    result.sort_by(|a, b| {
        b.is_dir
            .cmp(&a.is_dir)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    result
}

fn ok_list_flat(resolved: &Path, entries: fs::ReadDir, file_glob: Option<&str>) -> ActionExecution {
    let items = collect_entries(entries, file_glob);
    let count = items.len();
    let output = format_entries_compact(&items);
    ok(
        format!("列出目录：{}", resolved.display()),
        format!("目录列举成功，共 {} 个条目。", count),
        format!("目录内容：{}\n{}", resolved.display(), output),
        "读取目标目录条目，按类型分组、大小标注。",
        CACHE_LIST_REASON,
    )
}

fn ok_list_tree(resolved: &Path, file_glob: Option<&str>) -> ActionExecution {
    let mut output = String::new();
    let count = build_tree_output(resolved, resolved, "", 0, file_glob, &mut output);
    ok(
        format!("递归列出目录：{}", resolved.display()),
        format!("目录树列举成功，共 {} 个条目。", count),
        format!("目录树：{}\n{}", resolved.display(), output),
        "递归遍历目录树，显示层级结构与文件大小。",
        CACHE_LIST_REASON,
    )
}

fn build_tree_output(
    root: &Path,
    dir: &Path,
    prefix: &str,
    depth: u32,
    file_glob: Option<&str>,
    output: &mut String,
) -> usize {
    if depth > 4 {
        return 0;
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return 0;
    };
    let items = collect_entries(entries, None);
    let mut count = 0usize;
    for (i, item) in items.iter().enumerate() {
        if item.is_dir && SKIP_DIRS.iter().any(|skip| item.name.eq_ignore_ascii_case(skip)) {
            continue;
        }
        if !item.is_dir {
            if let Some(glob) = file_glob {
                if !glob_matches(glob, &item.name) {
                    continue;
                }
            }
        }
        let is_last = i + 1 == items.len();
        let branch = if is_last { "└── " } else { "├── " };
        let child_prefix = if is_last { "    " } else { "│   " };
        let size_label = if item.is_dir {
            String::new()
        } else {
            format!(" ({})", format_size(item.size))
        };
        let dir_marker = if item.is_dir { "/" } else { "" };
        output.push_str(&format!(
            "{}{}{}{}{}\n",
            prefix, branch, item.name, dir_marker, size_label
        ));
        count += 1;
        if item.is_dir {
            let child_path = dir.join(&item.name);
            let new_prefix = format!("{prefix}{child_prefix}");
            count += build_tree_output(root, &child_path, &new_prefix, depth + 1, file_glob, output);
        }
    }
    count
}

fn format_entries_compact(items: &[DirEntry]) -> String {
    let mut parts: Vec<String> = Vec::new();
    for item in items.iter().take(60) {
        if item.is_dir {
            parts.push(format!("{}/", item.name));
        } else {
            parts.push(format!("{} ({})", item.name, format_size(item.size)));
        }
    }
    if items.len() > 60 {
        parts.push(format!("... 及其他 {} 个条目", items.len() - 60));
    }
    if parts.is_empty() {
        "目录为空。".to_string()
    } else {
        parts.join("\n")
    }
}

fn format_size(bytes: u64) -> String {
    if bytes >= 1024 * 1024 {
        format!("{:.1}MB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes >= 1024 {
        format!("{:.1}KB", bytes as f64 / 1024.0)
    } else {
        format!("{}B", bytes)
    }
}

pub(crate) fn execute_file_search(
    request: &RunRequest,
    query: &str,
    path: Option<&str>,
    context_lines: u32,
    file_glob: Option<&str>,
    output_mode: &SearchOutputMode,
) -> ActionExecution {
    let base_path = path.unwrap_or(".");
    let Ok(resolved) = resolve_workspace_path(&request.workspace_ref.root_path, base_path) else {
        return invalid_path("搜索文件", base_path, "目标路径越界或解析失败", CACHE_SEARCH_REASON);
    };
    if !resolved.exists() {
        return fail(
            format!("搜索文件：{}", resolved.display()),
            "目标路径不存在。".to_string(),
            "目标路径不存在，请检查路径后重试。".to_string(),
            "目标路径不存在，按错误直接返回。",
            CACHE_SEARCH_REASON,
        );
    }
    let pattern = compile_search_pattern(query);
    let mut results: Vec<SearchResult> = Vec::new();
    let mut total_match_count = 0usize;
    let mut total_line_count = 0usize;
    let mut file_count = 0usize;
    search_dir(
        &resolved,
        &resolved,
        &pattern,
        0,
        context_lines,
        file_glob,
        output_mode,
        &mut results,
        &mut total_match_count,
        &mut total_line_count,
        &mut file_count,
    );
    match output_mode {
        SearchOutputMode::FilesWithMatches => format_files_with_matches(&resolved, &results),
        SearchOutputMode::Count => format_count_results(&resolved, &results, total_match_count),
        SearchOutputMode::Content => {
            format_content_results(&resolved, query, &results, total_match_count, total_line_count)
        }
    }
}

struct SearchResult {
    rel_path: String,
    matches: Vec<MatchLine>,
    match_count: usize,
}

struct MatchLine {
    line_number: usize,
    text: String,
    is_context: bool,
}

fn search_dir(
    root: &Path,
    dir: &Path,
    pattern: &SearchPattern,
    depth: u32,
    context_lines: u32,
    file_glob: Option<&str>,
    output_mode: &SearchOutputMode,
    results: &mut Vec<SearchResult>,
    total_match_count: &mut usize,
    total_line_count: &mut usize,
    file_count: &mut usize,
) {
    if depth > SEARCH_MAX_DEPTH {
        return;
    }
    let limit_hit = match output_mode {
        SearchOutputMode::Content => *total_line_count >= SEARCH_MAX_LINES,
        SearchOutputMode::FilesWithMatches | SearchOutputMode::Count => {
            results.len() >= 200
        }
    };
    if limit_hit {
        return;
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let limit_hit = match output_mode {
            SearchOutputMode::Content => *total_line_count >= SEARCH_MAX_LINES,
            SearchOutputMode::FilesWithMatches | SearchOutputMode::Count => {
                results.len() >= 200
            }
        };
        if limit_hit {
            return;
        }
        let file_type = entry.file_type().ok();
        if file_type.as_ref().is_some_and(|t| t.is_dir()) {
            let dir_name = entry.file_name();
            if SKIP_DIRS.iter().any(|skip| dir_name.eq_ignore_ascii_case(skip)) {
                continue;
            }
            search_dir(
                root,
                &entry.path(),
                pattern,
                depth + 1,
                context_lines,
                file_glob,
                output_mode,
                results,
                total_match_count,
                total_line_count,
                file_count,
            );
        } else if file_type.as_ref().is_some_and(|t| t.is_file()) {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            if let Some(glob) = file_glob {
                if !glob_matches(glob, &file_name_str) {
                    continue;
                }
            }
            if !is_text_file(&file_name_str) {
                continue;
            }
            let entry_path = entry.path();
            if is_large_or_binary(&entry_path) {
                continue;
            }
            *file_count += 1;
            if let Some(result) = search_file_content(
                &entry_path,
                root,
                pattern,
                context_lines,
                output_mode,
                total_line_count,
            ) {
                *total_match_count += result.match_count;
                results.push(result);
            }
        }
    }
}

enum SearchPattern {
    Regex(Regex),
    Literal(String),
}

fn compile_search_pattern(query: &str) -> SearchPattern {
    if query.len() <= 2 {
        return SearchPattern::Literal(query.to_lowercase());
    }
    let escaped = regex::escape(query);
    let pattern_str = if escaped != query { escaped } else { query.to_string() };
    match Regex::new(&format!("(?i){pattern_str}")) {
        Ok(re) => SearchPattern::Regex(re),
        Err(_) => SearchPattern::Literal(query.to_lowercase()),
    }
}

fn search_file_content(
    path: &Path,
    root: &Path,
    pattern: &SearchPattern,
    context_lines: u32,
    output_mode: &SearchOutputMode,
    total_line_count: &mut usize,
) -> Option<SearchResult> {
    let content = fs::read_to_string(path).ok()?;
    let rel_path = path.strip_prefix(root).unwrap_or(path).display().to_string();

    if matches!(output_mode, SearchOutputMode::FilesWithMatches) {
        return search_file_has_match(&content, pattern).then(|| SearchResult {
            rel_path,
            matches: Vec::new(),
            match_count: 1,
        });
    }

    if matches!(output_mode, SearchOutputMode::Count) {
        let count = count_matches(&content, pattern);
        return (count > 0).then_some(SearchResult {
            rel_path,
            matches: Vec::new(),
            match_count: count,
        });
    }

    let lines: Vec<&str> = content.lines().collect();
    let context = context_lines as usize;
    let mut matches: Vec<MatchLine> = Vec::new();
    let mut match_count = 0usize;
    let mut pending_context: Vec<(usize, &str)> = Vec::with_capacity(context);

    for (idx, line) in lines.iter().enumerate() {
        if *total_line_count >= SEARCH_MAX_LINES {
            break;
        }
        let is_match = match_pattern(pattern, line);
        if is_match {
            match_count += 1;
            for (ctx_idx, ctx_line) in &pending_context {
                matches.push(MatchLine {
                    line_number: ctx_idx + 1,
                    text: ctx_line.to_string(),
                    is_context: true,
                });
                *total_line_count += 1;
            }
            pending_context.clear();
            matches.push(MatchLine {
                line_number: idx + 1,
                text: line.to_string(),
                is_context: false,
            });
            *total_line_count += 1;
            if context > 0 {
                let after_start = idx + 1;
                let after_end = usize::min(idx + 1 + context, lines.len());
                for after_idx in after_start..after_end {
                    if *total_line_count >= SEARCH_MAX_LINES {
                        break;
                    }
                    if match_pattern(pattern, lines[after_idx]) {
                        break;
                    }
                    matches.push(MatchLine {
                        line_number: after_idx + 1,
                        text: lines[after_idx].to_string(),
                        is_context: true,
                    });
                    *total_line_count += 1;
                }
            }
        } else if context > 0 {
            add_context_line(&mut pending_context, context, idx, line);
        }
    }
    if matches.is_empty() {
        None
    } else {
        Some(SearchResult {
            rel_path,
            matches,
            match_count,
        })
    }
}

fn search_file_has_match(content: &str, pattern: &SearchPattern) -> bool {
    content.lines().any(|line| match_pattern(pattern, line))
}

fn count_matches(content: &str, pattern: &SearchPattern) -> usize {
    content.lines().filter(|line| match_pattern(pattern, line)).count()
}

fn match_pattern(pattern: &SearchPattern, line: &str) -> bool {
    match pattern {
        SearchPattern::Regex(re) => re.is_match(line),
        SearchPattern::Literal(lit) => line.to_lowercase().contains(lit.as_str()),
    }
}

fn add_context_line<'a>(
    pending: &mut Vec<(usize, &'a str)>,
    context: usize,
    idx: usize,
    line: &'a str,
) {
    pending.push((idx, line));
    if pending.len() > context {
        pending.remove(0);
    }
}

fn is_text_file(name: &str) -> bool {
    let Some(ext) = name.rsplit('.').next().map(|e| e.to_lowercase()) else {
        return true;
    };
    if ext == name {
        return true;
    }
    TEXT_EXTENSIONS.iter().any(|te| te.eq_ignore_ascii_case(&ext))
}

fn is_large_or_binary(path: &Path) -> bool {
    let Ok(meta) = fs::metadata(path) else {
        return true;
    };
    if meta.len() > SEARCH_MAX_FILE_SIZE {
        return true;
    }
    is_binary_file(path)
}

fn is_binary_file(path: &Path) -> bool {
    use std::io::Read;
    let Ok(mut file) = fs::File::open(path) else {
        return true;
    };
    let mut buf = [0u8; 8192];
    let Ok(n) = file.read(&mut buf) else {
        return true;
    };
    let chunk = &buf[..n];
    if chunk.is_empty() {
        return false;
    }
    chunk[..usize::min(512, n)].iter().any(|&b| b == 0)
}

fn glob_matches(glob: &str, name: &str) -> bool {
    let pattern = glob_to_regex(glob);
    Regex::new(&format!("(?i)^{pattern}$"))
        .is_ok_and(|re| re.is_match(name))
}

fn glob_to_regex(glob: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = glob.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '*' => {
                if i + 1 < chars.len() && chars[i + 1] == '*' {
                    i += 1;
                    out.push_str(".*");
                } else {
                    out.push_str("[^/\\\\]*");
                }
            }
            '?' => out.push_str("[^/\\\\]"),
            '.' | '+' | '(' | ')' | '|' | '^' | '$' | '{' | '}' | '[' | ']' | '\\' => {
                out.push('\\');
                out.push(chars[i]);
            }
            c => out.push(c),
        }
        i += 1;
    }
    out
}

fn format_content_results(
    resolved: &Path,
    query: &str,
    results: &[SearchResult],
    total_match_count: usize,
    total_line_count: usize,
) -> ActionExecution {
    if results.is_empty() {
        return ok(
            format!("搜索文件：{}", resolved.display()),
            format!("未找到包含 \"{}\" 的文件。", query),
            format!("在 {} 中搜索 \"{}\"：未找到匹配结果。", resolved.display(), query),
            "递归搜索目标目录中文件内容，未发现匹配。",
            CACHE_SEARCH_REASON,
        );
    }
    let mut output = String::new();
    for result in results {
        if !output.is_empty() {
            output.push_str("\n---\n");
        }
        output.push_str(&result.rel_path);
        output.push_str(":\n");
        for m in &result.matches {
            let marker = if m.is_context { "  " } else { "> " };
            output.push_str(&format!("{}{}: {}\n", marker, m.line_number, m.text.trim_end()));
        }
    }
    let truncated = if output.chars().count() > SEARCH_MAX_RESULT_CHARS {
        let truncated: String = output.chars().take(SEARCH_MAX_RESULT_CHARS).collect();
        format!("{truncated}\n...[截断，达到输出上限]")
    } else {
        output
    };
    let file_count = results.len();
    ok(
        format!("搜索文件：{}", resolved.display()),
        format!(
            "在 {} 个文件中找到 {} 条匹配。",
            file_count, total_match_count
        ),
        format!(
            "搜索 \"{}\" 在 {}（{} 个文件，{} 条匹配，{} 行输出）：\n{}",
            query,
            resolved.display(),
            file_count,
            total_match_count,
            total_line_count,
            summarize_text(&truncated),
        ),
        "递归搜索目标目录，支持正则、glob 过滤、上下文行。",
        CACHE_SEARCH_REASON,
    )
}

fn format_files_with_matches(resolved: &Path, results: &[SearchResult]) -> ActionExecution {
    if results.is_empty() {
        return ok(
            format!("搜索文件：{}", resolved.display()),
            "未找到匹配的文件。".to_string(),
            format!("在 {} 中搜索：未找到匹配结果。", resolved.display()),
            "递归搜索目标目录，未发现包含匹配内容的文件。",
            CACHE_SEARCH_REASON,
        );
    }
    let paths: Vec<String> = results.iter().map(|r| r.rel_path.clone()).collect();
    let count = paths.len();
    ok(
        format!("搜索文件：{}", resolved.display()),
        format!("找到 {} 个匹配的文件。", count),
        format!(
            "在 {} 中搜索，{} 个文件匹配：\n{}",
            resolved.display(),
            count,
            paths.join("\n")
        ),
        "递归搜索目标目录，返回匹配文件路径列表。",
        CACHE_SEARCH_REASON,
    )
}

fn format_count_results(
    resolved: &Path,
    results: &[SearchResult],
    total_match_count: usize,
) -> ActionExecution {
    if results.is_empty() {
        return ok(
            format!("搜索文件：{}", resolved.display()),
            "未找到匹配项。".to_string(),
            format!("在 {} 中搜索：未找到匹配结果。", resolved.display()),
            "递归搜索目标目录，未发现匹配项。",
            CACHE_SEARCH_REASON,
        );
    }
    let lines: Vec<String> = results
        .iter()
        .map(|r| format!("{}: {} matches", r.rel_path, r.match_count))
        .collect();
    ok(
        format!("搜索文件：{}", resolved.display()),
        format!(
            "在 {} 个文件中找到 {} 条匹配。",
            results.len(),
            total_match_count
        ),
        format!(
            "在 {} 中搜索，按文件统计匹配数（{} 个文件，{} 条匹配）：\n{}",
            resolved.display(),
            results.len(),
            total_match_count,
            lines.join("\n")
        ),
        "递归搜索目标目录，返回匹配计数。",
        CACHE_SEARCH_REASON,
    )
}

fn ensure_parent_dir(resolved: &Path) -> Result<(), std::io::Error> {
    let Some(parent) = resolved.parent() else {
        return Ok(());
    };
    fs::create_dir_all(parent)
}

fn invalid_path(action: &str, path: &str, summary: &str, cache_reason: &str) -> ActionExecution {
    fail(
        format!("{}：{}", action, path),
        summary.to_string(),
        summary.to_string(),
        summary,
        cache_reason,
    )
}

#[allow(clippy::result_large_err)]
fn resolve_path(
    request: &RunRequest,
    action: &str,
    path: &str,
    cache_reason: &str,
) -> Result<std::path::PathBuf, ActionExecution> {
    let normalized = normalize_explicit_path(path);
    if let Some(reason) = invalid_explicit_path_reason(&normalized) {
        return Err(invalid_path(action, &normalized, reason, cache_reason));
    }
    resolve_workspace_path(&request.workspace_ref.root_path, &normalized)
        .map_err(|_| invalid_path(action, &normalized, "目标路径越界或解析失败", cache_reason))
}

fn normalize_explicit_path(path: &str) -> String {
    path.trim().replace('\\', "/")
}

fn invalid_explicit_path_reason(path: &str) -> Option<&'static str> {
    if path.is_empty() {
        return Some("目标路径为空，请提供可读取的文件路径。");
    }
    if has_encoding_placeholder(path) {
        return Some("目标路径包含 `?`，疑似发生编码丢失；请改用 ASCII 路径或先 list 目录后复制路径重试。");
    }
    let bad = ['*', '"', '<', '>', '|', '\0'];
    path.chars()
        .any(|ch| bad.contains(&ch))
        .then_some("目标路径包含非法字符；请使用标准文件路径（不要包含 * \" < > |）。")
}

fn has_encoding_placeholder(path: &str) -> bool {
    path.chars()
        .any(|ch| ch == '?' || ch == '？' || ch == '\u{FFFD}' || ch.is_control())
}

fn ok_file_read(
    resolved: &Path,
    content: &str,
    offset: Option<usize>,
    limit: Option<usize>,
) -> ActionExecution {
    let total_lines = content.lines().count();
    let selected = select_lines(content, offset, limit);
    let display = if offset.is_some() || limit.is_some() {
        let start = offset.unwrap_or(1);
        let end = start + selected.lines().count().saturating_sub(1);
        if total_lines > 0 {
            format!(
                "文件读取完成（行 {}-{}/{}）：{}\n{}",
                start,
                end,
                total_lines,
                resolved.display(),
                selected,
            )
        } else {
            format!("文件读取完成：{}\n{}", resolved.display(), selected)
        }
    } else {
        format!("文件读取完成：{}\n{}", resolved.display(), summarize_text(content))
    };
    let summary = if selected.lines().count() < total_lines {
        let trunc_total = content.chars().count();
        format!(
            "文件读取成功，显示 {}/{} 行（文件共 {} 字符）。",
            selected.lines().count(),
            total_lines,
            trunc_total,
        )
    } else {
        format!("文件读取成功，{} 行。", total_lines)
    };
    ok(
        format!("读取文件：{}", resolved.display()),
        summary,
        display,
        "读取目标文件，支持行范围与二进制检测。",
        CACHE_READ_REASON,
    )
}

fn select_lines(content: &str, offset: Option<usize>, limit: Option<usize>) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let start = offset.map(|o| o.saturating_sub(1)).unwrap_or(0);
    let start = usize::min(start, lines.len());
    let end = match limit {
        Some(l) => usize::min(start + l, lines.len()),
        None => lines.len(),
    };
    lines[start..end].join("\n")
}

fn ok_file_append(resolved: &Path, content: &str) -> ActionExecution {
    let count = content.chars().count();
    ok(
        format!("追加写入：{}", resolved.display()),
        format!("文件追加成功，共追加 {} 个字符。", count),
        format!(
            "文件追加完成：{}\n追加字符数：{}\n内容摘要：{}",
            resolved.display(),
            count,
            summarize_text(content),
        ),
        "先校验工作区路径，再追加写入目标文件。",
        CACHE_WRITE_REASON,
    )
}

fn ok_file_write(resolved: &Path, content: &str) -> ActionExecution {
    let count = content.chars().count();
    let summary = summarize_text(content);
    ok(
        format!("写入文件：{}", resolved.display()),
        format!("文件写入成功，共写入 {} 个字符。", count),
        format!(
            "文件写入完成：{}\n写入字符数：{}\n内容摘要：{}",
            resolved.display(),
            count,
            summary
        ),
        "先校验工作区路径，再直接写入目标文件并返回摘要。",
        CACHE_WRITE_REASON,
    )
}

fn ok_file_delete(resolved: &Path) -> ActionExecution {
    ok(
        format!("删除路径：{}", resolved.display()),
        "目标路径已删除。".to_string(),
        format!("删除完成：{}", resolved.display()),
        "按目标类型执行删除，并将删除结果直接回传。",
        CACHE_DELETE_REASON,
    )
}

fn fail_create_parent_dir(resolved: &Path, error: &str, cache_reason: &str) -> ActionExecution {
    fail(
        format!("写入文件：{}", resolved.display()),
        format!("目录创建失败：{}", error),
        format!("写入前创建目录失败：{}", error),
        "写入前置目录创建失败，未进入文件写入阶段。",
        cache_reason,
    )
}

fn fail_write_file(resolved: &Path, error: &str, cache_reason: &str) -> ActionExecution {
    fail(
        format!("写入文件：{}", resolved.display()),
        format!("文件写入失败：{}", error),
        format!("文件写入失败：{}", error),
        "文件写入过程中出现系统错误，直接按失败收口。",
        cache_reason,
    )
}

fn ok(
    action_summary: String,
    result_summary: String,
    final_answer: String,
    reasoning_summary: &str,
    cache_reason: &str,
) -> ActionExecution {
    ActionExecution::bypass_ok(
        action_summary,
        result_summary,
        final_answer,
        reasoning_summary.to_string(),
        cache_reason,
    )
}

fn fail(
    action_summary: String,
    result_summary: String,
    final_answer: String,
    reasoning_summary: &str,
    cache_reason: &str,
) -> ActionExecution {
    ActionExecution::bypass_fail(
        action_summary,
        result_summary,
        final_answer,
        reasoning_summary.to_string(),
        cache_reason,
    )
}

#[cfg(test)]
mod tests {
    use super::{invalid_explicit_path_reason, normalize_explicit_path};

    #[test]
    fn normalizes_explicit_path_slashes_and_spaces() {
        let value = normalize_explicit_path("  docs\\README.md  ");
        assert_eq!(value, "docs/README.md");
    }

    #[test]
    fn rejects_question_mark_path_hint() {
        let reason = invalid_explicit_path_reason("docs/???.md");
        assert!(reason.is_some());
    }

    #[test]
    fn rejects_replacement_char_path_hint() {
        let reason = invalid_explicit_path_reason("docs/\u{FFFD}\u{FFFD}.md");
        assert!(reason.is_some());
    }
}
