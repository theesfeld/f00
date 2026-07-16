//! Icons for listings and TUI.
//!
//! Uses **Nerd Font** code points (same approach as eza/lsd). Install a Nerd Font
//! in the terminal for correct glyphs; otherwise you may see tofu/boxes.
//! Disable with `--icons=never` or under `--gnu`.

use f00_core::{Entry, EntryKind};

/// Map an entry to a Nerd Font icon glyph.
pub fn icon_for(entry: &Entry) -> &'static str {
    if entry.is_dir_header {
        return "";
    }

    match entry.kind {
        EntryKind::Directory => return icon_for_dir(&entry.name),
        EntryKind::Symlink => return "\u{f0c1}", // fa-link
        EntryKind::Other => return "\u{f15b}",   // fa-file
        EntryKind::File => {}
    }

    #[cfg(unix)]
    if entry.mode & 0o111 != 0 && entry.extension().is_none() {
        return "\u{f471}"; // binary / chip
    }

    // Well-known basenames (Cargo.toml, Dockerfile, …) before extension map.
    let base = icon_for_basename(&entry.name);
    if base != "\u{f15b}" {
        return base;
    }

    if let Some(ext) = entry.extension().map(|e| e.to_ascii_lowercase()) {
        if let Some(ic) = icon_for_extension(&ext) {
            return ic;
        }
    }

    base
}

/// Special-case directory basenames (case-insensitive).
fn icon_for_dir(name: &str) -> &'static str {
    let lower = name.to_ascii_lowercase();
    let key = lower.trim_end_matches('/');

    match key {
        // ── XDG user dirs (eza-style specials) ─────────────────────────
        "desktop" => "\u{f108}",                         // desktop
        "documents" | "docs" | "document" => "\u{f15c}", // document
        "downloads" | "download" => "\u{f019}",          // download
        "music" | "audio" | "sounds" => "\u{f001}",      // music
        "pictures" | "photos" | "images" | "img" | "dcim" => "\u{f1c5}", // image
        "videos" | "movies" | "film" | "films" => "\u{f008}", // film
        "projects" | "project" | "code" | "src" | "source" | "workspace" | "workspaces" => {
            "\u{f1b2}" // cubes / projects
        }
        "public" | "www" | "site" | "sites" => "\u{f0ac}", // globe
        "templates" | "template" | "skel" => "\u{f1c9}",   // file-code
        "notes" | "note" | "notebook" | "notebooks" => "\u{f24a}", // sticky-note
        "nixos" | "nix" => "\u{f313}",                     // nixos
        "home" => "\u{f015}",                              // home
        // ── common project dirs ────────────────────────────────────────
        ".config" | "config" | "configuration" | "conf" | "settings" => "\u{f013}",
        ".git" => "\u{f1d3}",
        ".github" | ".gitlab" => "\u{f1d3}",
        "trash" | ".trash" => "\u{f1f8}",
        "bin" | "sbin" => "\u{f471}",
        "test" | "tests" | "spec" | "specs" | "__tests__" => "\u{f07c}", // folder-open
        "node_modules" | "target" | "build" | "dist" | "out" | "vendor" | "lib" | "libs"
        | "include" => "\u{f07b}",
        "applications" | "apps" => "\u{f108}",
        _ if key.starts_with('.') => "\u{e5fc}", // config/dot dir
        _ => "\u{f07b}",                         // generic folder
    }
}

fn icon_for_extension(ext: &str) -> Option<&'static str> {
    Some(match ext {
        // languages
        "rs" => "\u{e7a8}",
        "go" => "\u{e626}",
        "py" | "pyi" | "pyc" | "pyw" => "\u{e73c}",
        "js" | "mjs" | "cjs" | "jsx" => "\u{e781}",
        "ts" | "tsx" | "mts" | "cts" => "\u{e628}",
        "c" | "h" => "\u{e61e}",
        "cc" | "cpp" | "cxx" | "hpp" | "hxx" | "hh" => "\u{e61d}",
        "java" | "jar" | "class" | "kt" | "kts" => "\u{e738}",
        "rb" | "erb" => "\u{e739}",
        "php" => "\u{e73d}",
        "lua" => "\u{e620}",
        "vim" | "vimrc" => "\u{e62b}",
        "sh" | "bash" | "zsh" | "fish" | "ps1" | "bat" | "cmd" => "\u{f489}",
        // web / data
        "html" | "htm" | "xhtml" => "\u{e736}",
        "css" | "scss" | "sass" | "less" => "\u{e749}",
        "json" | "jsonc" | "json5" => "\u{e60b}",
        "toml" => "\u{e615}",
        "yaml" | "yml" => "\u{e6a8}",
        "xml" | "xsl" | "xsd" => "\u{f121}",
        "csv" | "tsv" => "\u{f0ce}",
        "md" | "markdown" | "mdx" | "rst" | "txt" | "text" | "log" => "\u{f48a}",
        // media
        "png" | "jpg" | "jpeg" | "gif" | "webp" | "ico" | "bmp" | "tiff" | "heic" | "avif" => {
            "\u{f1c5}"
        }
        "svg" | "svgz" => "\u{e698}",
        "mp3" | "wav" | "flac" | "ogg" | "m4a" | "aac" | "wma" | "opus" => "\u{f028}",
        "mp4" | "mkv" | "webm" | "mov" | "avi" | "m4v" | "wmv" => "\u{f008}",
        // docs / packages
        "pdf" => "\u{f1c1}",
        "epub" | "mobi" => "\u{f02d}",
        "zip" | "tar" | "gz" | "tgz" | "bz2" | "xz" | "7z" | "rar" | "zst" | "lz4" => "\u{f1c6}",
        "ttf" | "otf" | "woff" | "woff2" | "eot" => "\u{f031}",
        "sql" | "db" | "sqlite" | "sqlite3" => "\u{f1c0}",
        "pem" | "crt" | "cer" | "key" | "pub" | "gpg" | "asc" => "\u{f084}",
        "lock" => "\u{f023}",
        "exe" | "dll" | "so" | "dylib" | "o" | "a" | "bin" => "\u{f471}",
        "dockerignore" => "\u{f308}",
        _ => return None,
    })
}

fn icon_for_basename(name: &str) -> &'static str {
    match name.to_ascii_lowercase().as_str() {
        "cargo.toml" | "cargo.lock" => "\u{e7a8}",
        "go.mod" | "go.sum" => "\u{e626}",
        "package.json" | "package-lock.json" | "yarn.lock" | "pnpm-lock.yaml" => "\u{e781}",
        "dockerfile"
        | "containerfile"
        | "compose.yaml"
        | "compose.yml"
        | "docker-compose.yml"
        | "docker-compose.yaml" => "\u{f308}",
        "makefile" | "gnumakefile" | "cmakelists.txt" => "\u{f0ad}",
        "license" | "license.md" | "license.txt" | "copying" | "copying.txt" => "\u{f2c2}",
        "readme" | "readme.md" | "readme.txt" | "readme.rst" => "\u{f02d}",
        "changelog" | "changelog.md" | "changes" | "history.md" => "\u{f48a}",
        ".gitignore" | ".gitattributes" | ".gitmodules" => "\u{f1d3}",
        ".editorconfig" | ".prettierrc" | ".eslintrc" | ".eslintrc.js" | ".eslintrc.cjs" => {
            "\u{e615}"
        }
        "flake.nix" | "flake.lock" | "default.nix" | "shell.nix" | "configuration.nix" => {
            "\u{f313}"
        }
        ".bashrc" | ".zshrc" | ".profile" | ".bash_profile" | ".zprofile" => "\u{f489}",
        "justfile" | "taskfile.yml" | "taskfile.yaml" => "\u{f0ad}",
        _ => "\u{f15b}",
    }
}

/// Prefix for display: icon + space, or empty when icons disabled.
pub fn icon_prefix(entry: &Entry, enabled: bool) -> String {
    if !enabled || entry.is_dir_header {
        return String::new();
    }
    format!("{} ", icon_for(entry))
}

#[cfg(test)]
mod tests {
    use super::*;
    use f00_core::{Entry, EntryKind, GitStatus};
    use std::path::PathBuf;

    fn ent(name: &str, kind: EntryKind) -> Entry {
        Entry {
            path: PathBuf::from(name),
            name: name.into(),
            kind,
            size: 0,
            modified: None,
            created: None,
            accessed: None,
            changed: None,
            mode: if kind == EntryKind::Directory {
                0o755
            } else {
                0o644
            },
            readonly: false,
            symlink_target: None,
            depth: 0,
            git_status: GitStatus::Clean,
            is_dir_header: false,
            nlink: 1,
            uid: 0,
            gid: 0,
            inode: 0,
            blocks: 0,
            owner: String::new(),
            group: String::new(),
            author: String::new(),
            context: String::new(),
        }
    }

    #[test]
    fn special_dirs_differ_from_generic_folder() {
        let desk = icon_for(&ent("Desktop", EntryKind::Directory));
        let dl = icon_for(&ent("Downloads", EntryKind::Directory));
        let music = icon_for(&ent("Music", EntryKind::Directory));
        let pics = icon_for(&ent("Pictures", EntryKind::Directory));
        let vids = icon_for(&ent("Videos", EntryKind::Directory));
        let projects = icon_for(&ent("Projects", EntryKind::Directory));
        let gen = icon_for(&ent("random-dir", EntryKind::Directory));
        assert_ne!(desk, gen);
        assert_ne!(dl, gen);
        assert_ne!(desk, dl);
        assert_ne!(music, pics);
        assert_ne!(vids, projects);
        assert_eq!(desk, "\u{f108}");
        assert_eq!(dl, "\u{f019}");
        assert_eq!(music, "\u{f001}");
        assert_eq!(pics, "\u{f1c5}");
        assert_eq!(vids, "\u{f008}");
        assert_eq!(projects, "\u{f1b2}");
        assert_eq!(gen, "\u{f07b}");
    }

    #[test]
    fn file_ext_icons() {
        assert_eq!(icon_for(&ent("main.rs", EntryKind::File)), "\u{e7a8}");
        assert_eq!(icon_for(&ent("app.py", EntryKind::File)), "\u{e73c}");
        assert_eq!(icon_for(&ent("pic.png", EntryKind::File)), "\u{f1c5}");
        assert_eq!(icon_for(&ent("song.mp3", EntryKind::File)), "\u{f028}");
        assert_eq!(icon_for(&ent("clip.mp4", EntryKind::File)), "\u{f008}");
        assert_eq!(icon_for(&ent("archive.zip", EntryKind::File)), "\u{f1c6}");
    }

    #[test]
    fn basename_icons() {
        assert_eq!(icon_for(&ent("Cargo.toml", EntryKind::File)), "\u{e7a8}");
        assert_eq!(icon_for(&ent("Dockerfile", EntryKind::File)), "\u{f308}");
        assert_eq!(icon_for(&ent("README.md", EntryKind::File)), "\u{f02d}");
        assert_eq!(icon_for(&ent("flake.nix", EntryKind::File)), "\u{f313}");
    }

    #[test]
    fn case_insensitive_dirs() {
        assert_eq!(
            icon_for(&ent("MUSIC", EntryKind::Directory)),
            icon_for(&ent("music", EntryKind::Directory))
        );
    }

    #[test]
    fn icon_prefix_empty_when_disabled() {
        let e = ent("Desktop", EntryKind::Directory);
        assert!(icon_prefix(&e, false).is_empty());
        assert!(icon_prefix(&e, true).starts_with('\u{f108}'));
    }
}
