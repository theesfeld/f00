use f00_core::{Entry, IndicatorStyle};

use crate::color::Colorizer;
use crate::icons::icon_prefix;
use crate::perms::classify_suffix;

/// Render a tree from a flat recursive listing (entries with depth).
///
/// Also works for a single non-recursive directory by treating all entries as depth 0.
///
/// Rendering is **O(n)** in the number of entries (precomputed last-sibling flags).
pub fn format_tree(
    entries: &[Entry],
    colorizer: &Colorizer,
    icons: bool,
    indicator: IndicatorStyle,
) -> String {
    let items: Vec<&Entry> = entries.iter().filter(|e| !e.is_dir_header).collect();
    if items.is_empty() {
        return String::new();
    }

    let use_depth = items.iter().any(|e| e.depth > 0);
    let mut out = String::with_capacity(items.len().saturating_mul(48));

    if !use_depth {
        for (i, entry) in items.iter().enumerate() {
            let last = i + 1 == items.len();
            out.push_str(if last { "└── " } else { "├── " });
            push_name(&mut out, entry, colorizer, icons, indicator);
            out.push('\n');
        }
        return out;
    }

    format_tree_by_depth(&items, colorizer, icons, indicator, &mut out);
    out
}

fn push_name(
    out: &mut String,
    entry: &Entry,
    colorizer: &Colorizer,
    icons: bool,
    indicator: IndicatorStyle,
) {
    let icon = icon_prefix(entry, icons);
    let suffix = classify_suffix(entry, indicator);
    let plain = format!("{icon}{}{suffix}", entry.name);
    out.push_str(&colorizer.paint_name(entry, &plain));
}

/// Preorder depths (1 = root children). Compute `is_last` and draw connectors in O(n).
fn format_tree_by_depth(
    items: &[&Entry],
    colorizer: &Colorizer,
    icons: bool,
    indicator: IndicatorStyle,
    out: &mut String,
) {
    let n = items.len();
    // Walkdir depths are typically 1+ for listed nodes; treat 0 as 1 for safety.
    let depths: Vec<usize> = items.iter().map(|e| e.depth.max(1)).collect();

    // is_last[i] == true if items[i] is the last among its siblings (same parent in preorder).
    let is_last = precompute_is_last(&depths);

    // stack of indices of ancestors (by depth level: stack[0] is depth-1 node, …)
    let mut stack: Vec<usize> = Vec::with_capacity(16);

    for i in 0..n {
        let d = depths[i];
        // Pop finished subtrees: keep only ancestors strictly above `d`.
        while !stack.is_empty() && depths[*stack.last().unwrap()] >= d {
            stack.pop();
        }

        // Vertical bars for each ancestor that still has more siblings after this node.
        for &anc in &stack {
            if is_last[anc] {
                out.push_str("    ");
            } else {
                out.push_str("│   ");
            }
        }

        out.push_str(if is_last[i] {
            "└── "
        } else {
            "├── "
        });
        push_name(out, items[i], colorizer, icons, indicator);
        out.push('\n');

        stack.push(i);
    }
}

/// For a preorder sequence of depths, mark each index as last among siblings.
///
/// Sibling group: consecutive nodes that share the same parent in the preorder walk
/// (same depth, with no intervening shallower node).
fn precompute_is_last(depths: &[usize]) -> Vec<bool> {
    let n = depths.len();
    let mut is_last = vec![true; n];
    if n == 0 {
        return is_last;
    }

    // last_at[d] = most recent index at depth d that may still get another sibling
    let max_d = depths.iter().copied().max().unwrap_or(1);
    let mut last_at: Vec<Option<usize>> = vec![None; max_d + 2];

    for (i, &d) in depths.iter().enumerate() {
        // Closing any open nodes deeper than d: they are last in their groups already.
        for slot in last_at.iter_mut().skip(d + 1) {
            *slot = None;
        }
        // Previous node at this depth (same parent, preorder) is not last — we are its sibling.
        if let Some(prev) = last_at[d] {
            is_last[prev] = false;
        }
        last_at[d] = Some(i);
    }
    is_last
}

#[cfg(test)]
mod tests {
    use super::*;
    use f00_core::{Entry, EntryKind, GitStatus};
    use std::path::PathBuf;

    fn e_path(path: &str, depth: usize) -> Entry {
        let p = PathBuf::from(path);
        let name = p.file_name().unwrap().to_string_lossy().into_owned();
        Entry {
            path: p,
            name,
            kind: EntryKind::File,
            size: 0,
            modified: None,
            created: None,
            accessed: None,
            changed: None,
            mode: 0o644,
            readonly: false,
            symlink_target: None,
            depth,
            git_status: GitStatus::Clean,
            is_dir_header: false,
            nlink: 1,
            uid: 0,
            gid: 0,
            inode: 0,
            blocks: 0,
            dev: 0,
            rdev: 0,
            blksize: 0,
            owner: String::new(),
            group: String::new(),
            author: String::new(),
            context: String::new(),
        }
    }

    #[test]
    fn precompute_siblings() {
        // a(1), b(2), g(3), f(2) — classic tree preorder
        let d = vec![1, 2, 3, 2];
        let last = precompute_is_last(&d);
        assert!(last[0]); // a only root child
        assert!(!last[1]); // b then f
        assert!(last[2]); // g only child of b
        assert!(last[3]); // f last under a
    }

    #[test]
    fn tree_connectors_correct() {
        let entries = vec![
            e_path("/r/a", 1),
            e_path("/r/a/b", 2),
            e_path("/r/a/b/g", 3),
            e_path("/r/a/f", 2),
        ];
        // force directory kinds for a,b
        let mut entries = entries;
        entries[0].kind = EntryKind::Directory;
        entries[1].kind = EntryKind::Directory;

        let colorizer = Colorizer::new(false);
        let out = format_tree(&entries, &colorizer, false, IndicatorStyle::None);
        // Expect classic tree:
        // └── a
        //     ├── b
        //     │   └── g
        //     └── f
        assert!(out.contains("└── a\n"), "{out}");
        assert!(out.contains("├── b\n"), "{out}");
        assert!(out.contains("│   └── g\n"), "{out}");
        assert!(out.contains("└── f\n"), "{out}");
    }
}
