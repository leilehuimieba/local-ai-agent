pub(crate) fn contains_sensitive_text(text: &str) -> bool {
    contains_secret_marker(text)
        || contains_email(text)
        || contains_cn_mobile(text)
        || contains_cn_id_card(text)
}

pub(crate) fn redact_sensitive_text(text: &str) -> String {
    let mut output = String::new();
    let mut token = String::new();
    for ch in text.chars() {
        if redaction_delimiter(ch) {
            output.push_str(&redact_token(&token));
            token.clear();
            output.push(ch);
            continue;
        }
        token.push(ch);
    }
    output.push_str(&redact_token(&token));
    output
}

pub(crate) fn contains_private_marker(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    [
        "[private]",
        "privacy:private",
        "private=true",
        "private_content",
        "仅自己可见",
    ]
    .iter()
    .any(|marker| lower.contains(marker))
}

fn contains_secret_marker(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    let markers = [
        "authorization: bearer",
        "api_key",
        "token=",
        "password=",
        "secret=",
        "sk-",
        "ak-",
    ];
    markers.iter().any(|marker| lower.contains(marker))
}

fn redaction_delimiter(ch: char) -> bool {
    ch.is_whitespace()
        || matches!(
            ch,
            '"' | '\'' | '<' | '>' | '(' | ')' | '[' | ']' | '{' | '}' | ',' | ';'
        )
}

fn redact_token(token: &str) -> String {
    if token.is_empty() {
        return String::new();
    }
    if is_secret_token(token) || is_email_candidate(token) || is_cn_mobile_token(token) {
        return "[REDACTED]".to_string();
    }
    if is_cn_id_candidate(token) {
        return "[REDACTED]".to_string();
    }
    token.to_string()
}

fn is_secret_token(token: &str) -> bool {
    let lower = token.to_ascii_lowercase();
    lower.contains("authorization:")
        || lower.contains("api_key")
        || lower.contains("token=")
        || lower.contains("password=")
        || lower.contains("secret=")
        || lower.starts_with("sk-")
        || lower.starts_with("ak-")
}

fn is_cn_mobile_token(token: &str) -> bool {
    let digits: String = token.chars().filter(|ch| ch.is_ascii_digit()).collect();
    is_cn_mobile_digits(&digits)
}

fn contains_email(text: &str) -> bool {
    text.split(email_delimiter).any(is_email_candidate)
}

fn email_delimiter(ch: char) -> bool {
    ch.is_whitespace()
        || matches!(
            ch,
            '"' | '\'' | '<' | '>' | '(' | ')' | '[' | ']' | '{' | '}' | ',' | ';'
        )
}

fn is_email_candidate(token: &str) -> bool {
    let token = token.trim_matches(|ch: char| matches!(ch, '.' | ':' | '!' | '?'));
    if token.len() < 6 || token.len() > 120 || token.contains("..") {
        return false;
    }
    let Some((local, domain)) = token.split_once('@') else {
        return false;
    };
    if local.is_empty() || domain.is_empty() || !domain.contains('.') {
        return false;
    }
    local.chars().all(is_email_local_char) && domain.chars().all(is_email_domain_char)
}

fn is_email_local_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '%' | '+' | '-')
}

fn is_email_domain_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-')
}

fn contains_cn_mobile(text: &str) -> bool {
    digit_sequences(text)
        .into_iter()
        .any(|digits| is_cn_mobile_digits(&digits))
}

fn digit_sequences(text: &str) -> Vec<String> {
    let mut runs = Vec::new();
    let mut current = String::new();
    for ch in text.chars() {
        if ch.is_ascii_digit() {
            current.push(ch);
            continue;
        }
        if !current.is_empty() {
            runs.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        runs.push(current);
    }
    runs
}

fn is_cn_mobile_digits(digits: &str) -> bool {
    if digits.len() == 13 && digits.starts_with("86") {
        return is_cn_mobile_11(&digits[2..]);
    }
    is_cn_mobile_11(digits)
}

fn is_cn_mobile_11(digits: &str) -> bool {
    if digits.len() != 11 {
        return false;
    }
    let bytes = digits.as_bytes();
    bytes[0] == b'1' && (b'3'..=b'9').contains(&bytes[1])
}

fn contains_cn_id_card(text: &str) -> bool {
    text.split(|ch: char| !ch.is_ascii_alphanumeric())
        .any(is_cn_id_candidate)
}

fn is_cn_id_candidate(token: &str) -> bool {
    if token.len() != 18 {
        return false;
    }
    let prefix = &token[..17];
    let last = token.as_bytes()[17];
    prefix.as_bytes().iter().all(|item| item.is_ascii_digit())
        && (last.is_ascii_digit() || last == b'X' || last == b'x')
}

#[cfg(test)]
mod tests {
    use super::{contains_private_marker, contains_sensitive_text, redact_sensitive_text};

    #[test]
    fn detects_secret_markers() {
        assert!(contains_sensitive_text(
            "authorization: bearer sk-test-token"
        ));
    }

    #[test]
    fn detects_email_patterns() {
        assert!(contains_sensitive_text(
            "请联系 test.user+ops@example.com 获取详情"
        ));
    }

    #[test]
    fn detects_cn_mobile_patterns() {
        assert!(contains_sensitive_text("手机号 13800138000 已绑定"));
        assert!(contains_sensitive_text("紧急联系 +86 13800138000"));
    }

    #[test]
    fn detects_cn_id_patterns() {
        assert!(contains_sensitive_text("身份证号 11010519491231002X"));
    }

    #[test]
    fn ignores_normal_text() {
        assert!(!contains_sensitive_text(
            "这是一条正常学习计划总结，没有隐私字段"
        ));
    }

    #[test]
    fn redacts_sensitive_tokens() {
        let text = "token=abc123 和邮箱 test@example.com";
        let redacted = redact_sensitive_text(text);
        assert!(redacted.contains("[REDACTED]"));
        assert!(!redacted.contains("abc123"));
        assert!(!redacted.contains("test@example.com"));
    }

    #[test]
    fn detects_private_markers() {
        assert!(contains_private_marker("[PRIVATE] 仅自己可见"));
        assert!(contains_private_marker("privacy:private"));
    }
}
