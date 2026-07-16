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

/// Identity transform: copy input JSON to output (proves decorator wiring).
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
    if input_len > cap {
        return -2;
    }
    std::ptr::copy_nonoverlapping(input, output, input_len);
    *output_len = input_len;
    0
}
