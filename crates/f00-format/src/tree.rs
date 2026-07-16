use f00_core::Entry;

use crate::color::Colorizer;
use crate::icons::icon_prefix;
use crate::perms::classify_suffix;

/// Render a basic tree from a flat recursive listing (headers + entries with depth).
///
/// Also works for a single non-recursive directory by treating all entries as depth 0.
pub fn format_tree(
    entries: &[Entry],
    colorizer: &Colorizer,
    icons: bool,
    classify: bool,
) -> String {
    // Collect non-header entries; use depth field when available.
    let items: Vec<&Entry> = entries.iter().filter(|e| !e.is_dir_header).collect();
    if items.is_empty() {
        return String::new();
    }

    // If all depths are 0, build tree structure from paths when possible.
    let use_depth = items.iter().any(|e| e.depth > 0);

    let mut out = String::new();
    if use_depth {
        format_tree_by_depth(&items, colorizer, icons, classify, &mut out);
    } else {
        // Flat tree of a single directory
        for (i, entry) in items.iter().enumerate() {
            let last = i + 1 == items.len();
            let branch = if last { "└── " } else { "├── " };
            let icon = icon_prefix(entry, icons);
            let suffix = classify_suffix(entry, classify);
            let plain = format!("{icon}{}{suffix}", entry.name);
            let name = colorizer.paint_name(entry, &plain);
            out.push_str(branch);
            out.push_str(&name);
            out.push('\n');
        }
    }
    out
}

fn format_tree_by_depth(
    items: &[&Entry],
    colorizer: &Colorizer,
    icons: bool,
    classify: bool,
    out: &mut String,
) {
    // Track which depths still have more siblings (for drawing │).
    // Precompute for each index whether it is the last among siblings at its depth
    // with the same parent path.
    for (i, entry) in items.iter().enumerate() {
        let depth = entry.depth.max(1); // root children are depth 1 in walkdir
        let level = depth - 1;

        // Determine if last among following items that share ancestor chain.
        let is_last = is_last_sibling(items, i);

        // Draw prefix for parent levels
        for d in 0..level {
            // Check if any ancestor at depth d+1 still has more siblings after us
            let ancestor_has_more = ancestor_continues(items, i, d + 1);
            if ancestor_has_more {
                out.push_str("│   ");
            } else {
                out.push_str("    ");
            }
        }

        out.push_str(if is_last { "└── " } else { "├── " });

        let icon = icon_prefix(entry, icons);
        let suffix = classify_suffix(entry, classify);
        let plain = format!("{icon}{}{suffix}", entry.name);
        let name = colorizer.paint_name(entry, &plain);
        out.push_str(&name);
        out.push('\n');
    }
}

fn is_last_sibling(items: &[&Entry], index: usize) -> bool {
    let depth = items[index].depth;
    let parent = items[index].path.parent().map(|p| p.to_path_buf());
    for next in items.iter().skip(index + 1) {
        if next.depth < depth {
            return true;
        }
        if next.depth == depth {
            let next_parent = next.path.parent().map(|p| p.to_path_buf());
            if next_parent == parent {
                return false;
            }
            // different parent at same depth means we've left the sibling group
            if next.depth == depth {
                return true;
            }
        }
    }
    true
}

fn ancestor_continues(items: &[&Entry], index: usize, ancestor_depth: usize) -> bool {
    // Find the ancestor entry at `ancestor_depth` for items[index], then see if
    // that ancestor has more siblings after this subtree.
    let path = &items[index].path;
    let ancestors: Vec<_> = path.ancestors().collect();
    // path.ancestors: self, parent, grandparent...
    // We need the path component at depth ancestor_depth from the walk root.
    // Simpler heuristic: look ahead for another entry at `ancestor_depth`
    // before any entry with depth < ancestor_depth.
    for next in items.iter().skip(index + 1) {
        if next.depth < ancestor_depth {
            return false;
        }
        if next.depth == ancestor_depth {
            return true;
        }
    }
    let _ = ancestors;
    false
}
