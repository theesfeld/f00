//! Example f00 plugin for ABI smoke tests.

use std::os::raw::c_char;

/// SAFETY: required export for host ABI version check.
#[no_mangle]
pub extern "C" fn f00_plugin_abi_version() -> u32 {
    f00_plugin::ABI_VERSION
}

/// SAFETY: returns pointer to static C string.
#[no_mangle]
pub extern "C" fn f00_plugin_name() -> *const c_char {
    static NAME: &[u8] = b"hello\0";
    NAME.as_ptr() as *const c_char
}

/// Decorate: for each entry, set `display_name` to `name` prefixed with `· ` (middle-dot space).
/// Identity-preserving for paths; host applies `display_name` to the listing.
///
/// # Safety
///
/// - `input` must be valid for reads of `input_len` bytes (or null only if len is 0).
/// - `output` must be valid for writes of at least `*output_len` bytes.
/// - `output_len` must be non-null and initialized with the capacity of `output`.
#[no_mangle]
pub unsafe extern "C" fn f00_plugin_on_entries_json(
    input: *const u8,
    input_len: usize,
    output: *mut u8,
    output_len: *mut usize,
) -> i32 {
    if input.is_null() || output.is_null() || output_len.is_null() {
        return -1;
    }
    let cap = *output_len;
    let slice = std::slice::from_raw_parts(input, input_len);
    let Ok(text) = std::str::from_utf8(slice) else {
        return -3;
    };
    // Minimal transform without pulling serde into the example: if JSON array, rewrite
    // "name":"X" display by injecting display_name after each name field when absent.
    // Fallback: pass-through.
    let out = decorate_json_naive(text);
    let bytes = out.as_bytes();
    if bytes.len() > cap {
        return -2;
    }
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), output, bytes.len());
    *output_len = bytes.len();
    0
}

/// Naive decorate: inject `"display_name":"· <name>"` after each `"name":"..."` when missing.
fn decorate_json_naive(input: &str) -> String {
    // Fast path pass-through if not an array of objects.
    if !input.trim_start().starts_with('[') {
        return input.to_string();
    }
    let mut out = String::with_capacity(input.len() + 64);
    let mut rest = input;
    while let Some(idx) = rest.find("\"name\"") {
        out.push_str(&rest[..idx]);
        // copy "name"
        let after_key = &rest[idx..];
        // find : "value"
        if let Some(colon) = after_key.find(':') {
            let after_colon = after_key[colon + 1..].trim_start();
            if let Some(stripped) = after_colon.strip_prefix('"') {
                if let Some(end) = stripped.find('"') {
                    let name = &stripped[..end];
                    let consumed = idx + colon + 1 + (after_colon.len() - stripped.len()) + end + 1;
                    out.push_str(&rest[idx..consumed]);
                    // only inject if display_name not already present in the next ~80 chars
                    let lookahead = rest
                        .get(consumed..consumed.saturating_add(120))
                        .unwrap_or("");
                    if !lookahead.contains("display_name") {
                        out.push_str(",\"display_name\":\"· ");
                        out.push_str(name);
                        out.push('"');
                    }
                    rest = &rest[consumed..];
                    continue;
                }
            }
        }
        // failed to parse this name; copy one char and continue
        out.push(rest.chars().next().unwrap_or(' '));
        rest = &rest[rest.chars().next().map(|c| c.len_utf8()).unwrap_or(1)..];
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decorate_injects_display_name() {
        let input = r#"[{"name":"a.txt","path":"/t/a.txt","kind":"file","size":1,"depth":0,"is_dir_header":false}]"#;
        let out = decorate_json_naive(input);
        assert!(out.contains("display_name"), "{out}");
        assert!(out.contains("· a.txt"), "{out}");
    }
}
