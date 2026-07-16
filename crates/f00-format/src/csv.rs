//! CSV / TSV output modes.

use f00_core::Entry;

/// Format entries as CSV (header + rows).
pub fn format_csv(entries: &[Entry]) -> String {
    format_delimited(entries, ',')
}

/// Format entries as TSV (header + rows).
pub fn format_tsv(entries: &[Entry]) -> String {
    format_delimited(entries, '\t')
}

fn format_delimited(entries: &[Entry], sep: char) -> String {
    let mut out = String::new();
    // header
    push_row(
        &mut out,
        sep,
        &[
            "name",
            "path",
            "kind",
            "size",
            "modified",
            "mode",
            "inode",
            "uid",
            "gid",
            "nlink",
            "symlink_target",
            "context",
        ],
    );
    for e in entries.iter().filter(|e| !e.is_dir_header) {
        let modified = e
            .modified_datetime()
            .map(|d| d.to_rfc3339())
            .unwrap_or_default();
        let target = e
            .symlink_target
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        let fields = [
            e.name.clone(),
            e.path.display().to_string(),
            e.kind.as_str().to_string(),
            e.size.to_string(),
            modified,
            format!("{:o}", e.mode),
            e.inode.to_string(),
            e.uid.to_string(),
            e.gid.to_string(),
            e.nlink.to_string(),
            target,
            e.context.clone(),
        ];
        let refs: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();
        push_row(&mut out, sep, &refs);
    }
    out
}

fn push_row(out: &mut String, sep: char, fields: &[&str]) {
    for (i, f) in fields.iter().enumerate() {
        if i > 0 {
            out.push(sep);
        }
        if sep == ',' {
            push_csv_field(out, f);
        } else {
            // TSV: escape tabs/newlines lightly
            for c in f.chars() {
                match c {
                    '\t' => out.push_str("\\t"),
                    '\n' => out.push_str("\\n"),
                    '\r' => out.push_str("\\r"),
                    _ => out.push(c),
                }
            }
        }
    }
    out.push('\n');
}

fn push_csv_field(out: &mut String, field: &str) {
    if field.contains([',', '"', '\n', '\r']) {
        out.push('"');
        for c in field.chars() {
            if c == '"' {
                out.push_str("\"\"");
            } else {
                out.push(c);
            }
        }
        out.push('"');
    } else {
        out.push_str(field);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use f00_core::{Entry, EntryKind, GitStatus};
    use std::path::PathBuf;

    fn ent(name: &str) -> Entry {
        Entry {
            path: PathBuf::from(name),
            name: name.into(),
            kind: EntryKind::File,
            size: 3,
            modified: None,
            created: None,
            accessed: None,
            changed: None,
            mode: 0o644,
            readonly: false,
            symlink_target: None,
            depth: 0,
            git_status: GitStatus::Clean,
            is_dir_header: false,
            nlink: 1,
            uid: 0,
            gid: 0,
            inode: 1,
            blocks: 0,
            owner: "u".into(),
            group: "g".into(),
            author: "u".into(),
            context: String::new(),
        }
    }

    #[test]
    fn csv_has_header_and_row() {
        let s = format_csv(&[ent("a.txt")]);
        assert!(s.starts_with("name,path,"));
        assert!(s.contains("a.txt"));
    }

    #[test]
    fn tsv_uses_tabs() {
        let s = format_tsv(&[ent("b")]);
        assert!(s.contains('\t'));
        assert!(s.contains("b"));
    }
}
