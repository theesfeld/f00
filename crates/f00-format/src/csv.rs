//! CSV / TSV output modes.

use f00_core::Entry;

use crate::perms::format_permissions;

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
    // header — aligned with rich JSON fields (flat columns)
    push_row(
        &mut out,
        sep,
        &[
            "name",
            "path",
            "kind",
            "size",
            "mode",
            "permissions",
            "readonly",
            "modified",
            "accessed",
            "changed",
            "created",
            "inode",
            "nlink",
            "blocks",
            "uid",
            "gid",
            "owner",
            "group",
            "author",
            "symlink_target",
            "context",
            "extension",
            "git_status",
            "depth",
        ],
    );
    for e in entries.iter().filter(|e| !e.is_dir_header) {
        let ts = |d: Option<chrono::DateTime<chrono::Local>>| {
            d.map(|x| x.to_rfc3339()).unwrap_or_default()
        };
        let target = e
            .symlink_target
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        let ext = e.extension().unwrap_or("").to_string();
        let fields = [
            e.name.clone(),
            e.path.display().to_string(),
            e.kind.as_str().to_string(),
            e.size.to_string(),
            format!("{:o}", e.mode),
            format_permissions(e),
            e.readonly.to_string(),
            ts(e.modified_datetime()),
            ts(e.accessed_datetime()),
            ts(e.changed_datetime()),
            ts(e.created_datetime()),
            e.inode.to_string(),
            e.nlink.to_string(),
            e.blocks.to_string(),
            e.uid.to_string(),
            e.gid.to_string(),
            e.owner.clone(),
            e.group.clone(),
            e.author.clone(),
            target,
            e.context.clone(),
            ext,
            e.git_status.as_str().to_string(),
            e.depth.to_string(),
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

fn push_csv_field(out: &mut String, f: &str) {
    if f.contains([',', '"', '\n', '\r']) {
        out.push('"');
        for c in f.chars() {
            if c == '"' {
                out.push('"');
            }
            out.push(c);
        }
        out.push('"');
    } else {
        out.push_str(f);
    }
}
