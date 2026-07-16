use f00_core::{Entry, EntryKind};

/// Render a 10-char permission string similar to `ls -l`.
///
/// On Unix uses mode bits; elsewhere falls back to a simplified string.
pub fn format_permissions(entry: &Entry) -> String {
    #[cfg(unix)]
    {
        format_unix(entry)
    }
    #[cfg(not(unix))]
    {
        format_fallback(entry)
    }
}

#[cfg(unix)]
fn format_unix(entry: &Entry) -> String {
    let mode = entry.mode;
    let file_type = match entry.kind {
        EntryKind::Directory => 'd',
        EntryKind::Symlink => 'l',
        EntryKind::File => '-',
        EntryKind::Other => {
            // socket, fifo, block, char — best effort from mode
            match mode & 0o170000 {
                0o140000 => 's',
                0o010000 => 'p',
                0o060000 => 'b',
                0o020000 => 'c',
                _ => '-',
            }
        }
    };

    let mut s = String::with_capacity(10);
    s.push(file_type);
    s.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o100 != 0 { 'x' } else { '-' });
    s.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o010 != 0 { 'x' } else { '-' });
    s.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o001 != 0 { 'x' } else { '-' });
    s
}

#[cfg(not(unix))]
fn format_fallback(entry: &Entry) -> String {
    let t = match entry.kind {
        EntryKind::Directory => 'd',
        EntryKind::Symlink => 'l',
        EntryKind::File => '-',
        EntryKind::Other => '-',
    };
    let w = if entry.readonly { '-' } else { 'w' };
    format!("{t}r{w}xr{w}xr{w}x")
}

/// Classification suffix like `ls -F`: `/` for dirs, `@` for symlinks, `*` for executables.
pub fn classify_suffix(entry: &Entry, enabled: bool) -> &'static str {
    if !enabled || entry.is_dir_header {
        return "";
    }
    match entry.kind {
        EntryKind::Directory => "/",
        EntryKind::Symlink => "@",
        EntryKind::File if is_executable(entry) => "*",
        _ => "",
    }
}

fn is_executable(entry: &Entry) -> bool {
    #[cfg(unix)]
    {
        entry.mode & 0o111 != 0
    }
    #[cfg(not(unix))]
    {
        let _ = entry;
        false
    }
}
