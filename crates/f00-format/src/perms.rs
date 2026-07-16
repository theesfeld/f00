use f00_core::{Entry, EntryKind, IndicatorStyle};

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
        EntryKind::Other => match mode & 0o170000 {
            0o140000 => 's',
            0o010000 => 'p',
            0o060000 => 'b',
            0o020000 => 'c',
            _ => '-',
        },
    };

    let mut s = String::with_capacity(10);
    s.push(file_type);
    // owner
    s.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    s.push(suid_bit(mode, 0o100, 0o4000));
    // group
    s.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    s.push(sgid_bit(mode, 0o010, 0o2000));
    // other
    s.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    s.push(sticky_bit(mode, 0o001, 0o1000));
    s
}

#[cfg(unix)]
fn suid_bit(mode: u32, exec: u32, special: u32) -> char {
    match (mode & exec != 0, mode & special != 0) {
        (true, true) => 's',
        (false, true) => 'S',
        (true, false) => 'x',
        (false, false) => '-',
    }
}

#[cfg(unix)]
fn sgid_bit(mode: u32, exec: u32, special: u32) -> char {
    suid_bit(mode, exec, special)
}

#[cfg(unix)]
fn sticky_bit(mode: u32, exec: u32, special: u32) -> char {
    match (mode & exec != 0, mode & special != 0) {
        (true, true) => 't',
        (false, true) => 'T',
        (true, false) => 'x',
        (false, false) => '-',
    }
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

/// Classification suffix like `ls -F` / `-p` / `--file-type`.
pub fn classify_suffix(entry: &Entry, style: IndicatorStyle) -> &'static str {
    if matches!(style, IndicatorStyle::None) || entry.is_dir_header {
        return "";
    }
    match entry.kind {
        EntryKind::Directory => "/",
        EntryKind::Symlink => {
            if matches!(style, IndicatorStyle::Slash) {
                ""
            } else {
                "@"
            }
        }
        EntryKind::File if is_executable(entry) => {
            if matches!(style, IndicatorStyle::Classify) {
                "*"
            } else {
                ""
            }
        }
        EntryKind::Other => {
            #[cfg(unix)]
            {
                match entry.mode & 0o170000 {
                    0o010000 if !matches!(style, IndicatorStyle::Slash) => "|", // fifo
                    0o140000 if !matches!(style, IndicatorStyle::Slash) => "=", // socket
                    _ => "",
                }
            }
            #[cfg(not(unix))]
            {
                ""
            }
        }
        _ => "",
    }
}

/// Backward-compatible helper used by older call sites.
pub fn classify_suffix_bool(entry: &Entry, enabled: bool) -> &'static str {
    classify_suffix(
        entry,
        if enabled {
            IndicatorStyle::Classify
        } else {
            IndicatorStyle::None
        },
    )
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
