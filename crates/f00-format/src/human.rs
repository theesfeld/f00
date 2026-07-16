use f00_core::BlockSize;

/// Format a byte size with binary units (1024) like GNU `ls -h`.
pub fn human_size(bytes: u64) -> String {
    human_size_base(bytes, 1024)
}

/// Format a byte size with SI units (1000) like GNU `ls --si`.
pub fn human_size_si(bytes: u64) -> String {
    human_size_base(bytes, 1000)
}

fn human_size_base(bytes: u64, base: u64) -> String {
    const UNITS: [&str; 6] = ["B", "K", "M", "G", "T", "P"];
    if bytes < base {
        return format!("{bytes}");
    }

    let mut value = bytes as f64;
    let mut unit = 0;
    let base_f = base as f64;
    while value >= base_f && unit < UNITS.len() - 1 {
        value /= base_f;
        unit += 1;
    }

    if value >= 10.0 || unit == 0 {
        format!("{:.0}{}", value, UNITS[unit])
    } else {
        format!("{:.1}{}", value, UNITS[unit])
    }
}

/// Disk blocks for `ls -s`.
///
/// `blocks_512` is `st_blocks` (512-byte units). `unit_bytes` is the display
/// unit size (default 1024 for kibibytes; 512 with POSIXLY_CORRECT / custom
/// `--block-size`).
pub fn block_display(blocks_512: u64) -> u64 {
    block_display_with_unit(blocks_512, 1024)
}

/// Convert 512-byte units to `unit_bytes`-sized blocks, rounding up.
pub fn block_display_with_unit(blocks_512: u64, unit_bytes: u64) -> u64 {
    let unit = unit_bytes.max(1);
    let bytes = blocks_512.saturating_mul(512);
    bytes.div_ceil(unit)
}

/// Format a size field according to block-size / human flags.
pub fn format_size_bytes(bytes: u64, block_size: BlockSize, human: bool, si: bool) -> String {
    if human || matches!(block_size, BlockSize::HumanBinary) {
        return if si || matches!(block_size, BlockSize::HumanSi) {
            human_size_si(bytes)
        } else {
            human_size(bytes)
        };
    }
    if si || matches!(block_size, BlockSize::HumanSi) {
        return human_size_si(bytes);
    }
    match block_size {
        BlockSize::Bytes(1) | BlockSize::Bytes(0) => bytes.to_string(),
        BlockSize::Bytes(unit) => {
            let n = bytes.div_ceil(unit.max(1));
            n.to_string()
        }
        BlockSize::HumanBinary => human_size(bytes),
        BlockSize::HumanSi => human_size_si(bytes),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_sizes_are_plain() {
        assert_eq!(human_size(0), "0");
        assert_eq!(human_size(999), "999");
    }

    #[test]
    fn kilobytes() {
        assert_eq!(human_size(1024), "1.0K");
        assert_eq!(human_size(1536), "1.5K");
        assert_eq!(human_size(10 * 1024), "10K");
    }

    #[test]
    fn si_uses_1000() {
        assert_eq!(human_size_si(1000), "1.0K");
        assert_eq!(human_size_si(1000 * 1000), "1.0M");
    }

    #[test]
    fn block_display_rounds() {
        assert_eq!(block_display(1), 1);
        assert_eq!(block_display(2), 1);
        assert_eq!(block_display(3), 2);
    }

    #[test]
    fn block_display_custom_unit() {
        // 2 * 512 = 1024 bytes → 2 blocks of 512
        assert_eq!(block_display_with_unit(2, 512), 2);
        // 2 * 512 = 1024 → 1 block of 1024
        assert_eq!(block_display_with_unit(2, 1024), 1);
    }
}
