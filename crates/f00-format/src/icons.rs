use f00_core::{Entry, EntryKind};

/// Map an entry to a simple emoji icon (extension / kind based).
pub fn icon_for(entry: &Entry) -> &'static str {
    if entry.is_dir_header {
        return "";
    }

    match entry.kind {
        EntryKind::Directory => return "📁",
        EntryKind::Symlink => return "🔗",
        EntryKind::Other => return "📄",
        EntryKind::File => {}
    }

    match entry.extension().map(|e| e.to_ascii_lowercase()).as_deref() {
        Some("rs") => "🦀",
        Some("go") => "🐹",
        Some("py") => "🐍",
        Some("js" | "mjs" | "cjs") => "🟨",
        Some("ts" | "tsx") => "🔷",
        Some("json") => "📋",
        Some("toml" | "yaml" | "yml" | "ini" | "cfg") => "⚙️",
        Some("md" | "markdown" | "txt" | "rst") => "📝",
        Some("html" | "htm" | "css" | "scss") => "🌐",
        Some("png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" | "ico") => "🖼️",
        Some("mp3" | "wav" | "flac" | "ogg" | "m4a") => "🎵",
        Some("mp4" | "mkv" | "webm" | "mov" | "avi") => "🎬",
        Some("zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar") => "📦",
        Some("pdf") => "📕",
        Some("sh" | "bash" | "zsh" | "fish") => "💻",
        Some("lock") => "🔒",
        Some("git") => "🌱",
        _ => {
            // Common bare names
            match entry.name.as_str() {
                "Cargo.toml" | "Cargo.lock" => "📦",
                "Makefile" | "Dockerfile" => "🛠️",
                "LICENSE" | "LICENSE.md" | "COPYING" => "📜",
                "README" | "README.md" => "📖",
                _ => "📄",
            }
        }
    }
}

/// Prefix for display: icon + space, or empty when icons disabled.
pub fn icon_prefix(entry: &Entry, enabled: bool) -> String {
    if !enabled || entry.is_dir_header {
        return String::new();
    }
    format!("{} ", icon_for(entry))
}
