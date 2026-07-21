//! Filename quoting styles matching GNU `ls` (`-b`, `-Q`, `-N`, `--quoting-style`).

use f00_core::QuotingStyle;

/// Quote / escape a display name according to `style`.
///
/// When `hide_control` is true and the style is literal/locale, nongraphic
/// characters are replaced with `?` (GNU `-q`).
pub fn quote_name(name: &str, style: QuotingStyle, hide_control: bool) -> String {
    match style {
        QuotingStyle::Literal => {
            if hide_control {
                hide_control_chars(name)
            } else {
                name.to_string()
            }
        }
        QuotingStyle::Locale => {
            // Locale style: escape nongraphic similarly to shell-escape without
            // forcing shell quotes when not needed; approximate as shell-escape.
            if needs_shell_escape(name) {
                shell_escape_quote(name, false)
            } else if hide_control {
                hide_control_chars(name)
            } else {
                name.to_string()
            }
        }
        QuotingStyle::Shell => shell_quote(name, false),
        QuotingStyle::ShellAlways => shell_quote(name, true),
        QuotingStyle::ShellEscape => shell_escape_quote(name, false),
        QuotingStyle::ShellEscapeAlways => shell_escape_quote(name, true),
        QuotingStyle::C => c_quote(name, true),
        QuotingStyle::Escape => c_quote(name, false),
    }
}

fn hide_control_chars(name: &str) -> String {
    name.chars()
        .map(|c| if is_nongraphic(c) { '?' } else { c })
        .collect()
}

fn is_nongraphic(c: char) -> bool {
    // GNU treats non-printable (including DEL) and non-space whitespace specially.
    c.is_control() || c == '\u{7f}'
}

fn is_shell_special(c: char) -> bool {
    // Note: `~` is only shell-special at the start of a word (home expansion).
    // Mid-name tildes (e.g. `file~` backups) must not force quotes — match GNU ls.
    matches!(
        c,
        ' ' | '\t'
            | '\n'
            | '\''
            | '"'
            | '\\'
            | '|'
            | '&'
            | ';'
            | '('
            | ')'
            | '<'
            | '>'
            | '$'
            | '`'
            | '!'
            | '*'
            | '?'
            | '['
            | ']'
            | '#'
            | '='
            | '%'
            | '{'
            | '}'
    ) || c == '\0'
}

fn needs_shell_quote(name: &str) -> bool {
    name.is_empty()
        || name.chars().any(is_shell_special)
        || name.starts_with('-')
        || name.starts_with('~')
}

fn needs_shell_escape(name: &str) -> bool {
    name.is_empty()
        || name.starts_with('~')
        || name
            .chars()
            .any(|c| is_shell_special(c) || is_nongraphic(c))
}

/// Single-quote shell style; `always` forces quotes.
fn shell_quote(name: &str, always: bool) -> String {
    if !always && !needs_shell_quote(name) {
        return name.to_string();
    }
    // 'foo'\''bar' for embedded single quotes.
    let mut out = String::with_capacity(name.len() + 2);
    out.push('\'');
    for c in name.chars() {
        if c == '\'' {
            out.push_str("'\\''");
        } else {
            out.push(c);
        }
    }
    out.push('\'');
    out
}

/// Shell quoting with `$''` C-style escapes for nongraphic chars.
fn shell_escape_quote(name: &str, always: bool) -> String {
    let has_nongraphic = name.chars().any(is_nongraphic);
    if has_nongraphic || (always && needs_shell_escape(name)) {
        return dollar_quote(name);
    }
    if always || needs_shell_quote(name) {
        return shell_quote(name, true);
    }
    name.to_string()
}

fn dollar_quote(name: &str) -> String {
    let mut out = String::from("$'");
    for c in name.chars() {
        push_c_escape(&mut out, c, true);
    }
    out.push('\'');
    out
}

/// C-style escapes; surround with `"` when `quoted` (`-Q` / style=c).
fn c_quote(name: &str, quoted: bool) -> String {
    let mut out = String::with_capacity(name.len() + 2);
    if quoted {
        out.push('"');
    }
    for c in name.chars() {
        push_c_escape(&mut out, c, quoted);
    }
    if quoted {
        out.push('"');
    }
    out
}

fn push_c_escape(out: &mut String, c: char, in_double_quotes: bool) {
    match c {
        '\\' => out.push_str("\\\\"),
        '\n' => out.push_str("\\n"),
        '\t' => out.push_str("\\t"),
        '\r' => out.push_str("\\r"),
        '\x08' => out.push_str("\\b"),
        '\x0c' => out.push_str("\\f"),
        '\x0b' => out.push_str("\\v"),
        '\0' => out.push_str("\\0"),
        // GNU `ls -b` / `--quoting-style=escape` escapes spaces (not only controls).
        ' ' if !in_double_quotes => out.push_str("\\ "),
        '"' if in_double_quotes => out.push_str("\\\""),
        '\'' if !in_double_quotes => {
            // Outside double quotes (escape style / $'') — escape single quote as \'.
            out.push_str("\\'");
        }
        c if is_nongraphic(c) => {
            let u = c as u32;
            if u <= 0xff {
                out.push_str(&format!("\\{u:03o}"));
            } else {
                out.push_str(&format!("\\u{u:04x}"));
            }
        }
        c => out.push(c),
    }
}

/// Apply quoting for display; used by formatters.
pub fn display_name(name: &str, style: QuotingStyle, hide_control: bool) -> String {
    quote_name(name, style, hide_control)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_passthrough() {
        assert_eq!(quote_name("hello", QuotingStyle::Literal, false), "hello");
    }

    #[test]
    fn hide_control_replaces() {
        assert_eq!(quote_name("a\nb", QuotingStyle::Literal, true), "a?b");
    }

    #[test]
    fn escape_style_c_escapes() {
        let q = quote_name("a\nb", QuotingStyle::Escape, false);
        assert_eq!(q, "a\\nb");
        let q2 = quote_name("x y", QuotingStyle::Escape, false);
        assert_eq!(q2, "x\\ y");
    }

    #[test]
    fn c_style_double_quotes() {
        let q = quote_name("hello", QuotingStyle::C, false);
        assert_eq!(q, "\"hello\"");
        let q2 = quote_name("a\"b", QuotingStyle::C, false);
        assert_eq!(q2, "\"a\\\"b\"");
        let q3 = quote_name("a\tb", QuotingStyle::C, false);
        assert_eq!(q3, "\"a\\tb\"");
    }

    #[test]
    fn shell_quotes_when_needed() {
        assert_eq!(quote_name("plain", QuotingStyle::Shell, false), "plain");
        let q = quote_name("a b", QuotingStyle::Shell, false);
        assert_eq!(q, "'a b'");
        let q2 = quote_name("it's", QuotingStyle::Shell, false);
        assert!(q2.starts_with('\''));
        assert!(q2.contains("\\'"));
    }

    #[test]
    fn shell_always_quotes() {
        assert_eq!(
            quote_name("plain", QuotingStyle::ShellAlways, false),
            "'plain'"
        );
    }

    #[test]
    fn shell_escape_nongraphic() {
        let q = quote_name("a\nb", QuotingStyle::ShellEscape, false);
        assert!(q.starts_with("$'"), "{q}");
        assert!(q.contains("\\n"), "{q}");
    }
}
