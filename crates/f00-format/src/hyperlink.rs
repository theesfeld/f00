//! OSC 8 hyperlinks for file names (`ls --hyperlink`).

use std::path::Path;

/// Wrap `text` in an OSC 8 hyperlink to `path` when enabled.
pub fn hyperlink_name(path: &Path, text: &str, enabled: bool) -> String {
    if !enabled {
        return text.to_string();
    }
    let uri = file_uri(path);
    // OSC 8: ESC ] 8 ; params ; URI ST  text  ESC ] 8 ;; ST
    format!("\x1b]8;;{uri}\x1b\\{text}\x1b]8;;\x1b\\")
}

fn file_uri(path: &Path) -> String {
    let abs = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map(|cwd| cwd.join(path))
            .unwrap_or_else(|_| path.to_path_buf())
    };
    let host = hostname();
    let mut uri = String::from("file://");
    if !host.is_empty() {
        uri.push_str(&host);
    }
    // Percent-encode path bytes.
    for b in abs.to_string_lossy().bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'/' | b'.' | b'-' | b'_' | b'~' => {
                uri.push(b as char);
            }
            _ => uri.push_str(&format!("%{b:02X}")),
        }
    }
    uri
}

fn hostname() -> String {
    std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn disabled_is_passthrough() {
        assert_eq!(hyperlink_name(Path::new("/tmp/x"), "x", false), "x");
    }

    #[test]
    fn enabled_wraps_osc8() {
        let s = hyperlink_name(&PathBuf::from("/tmp/x"), "x", true);
        assert!(s.contains("\x1b]8;;file://"), "{s:?}");
        assert!(s.contains("/tmp/x") || s.contains("%"), "{s:?}");
        assert!(s.ends_with("\x1b]8;;\x1b\\"), "{s:?}");
    }
}
