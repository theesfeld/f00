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

/// Disk blocks for `ls -s` (usually 1024-byte "kibibytes" display of 512-unit st_blocks/2).
/// GNU `ls -s` prints allocated size in 1024-byte blocks by default (or 512 with `POSIXLY_CORRECT`).
pub fn block_display(blocks_512: u64) -> u64 {
    // Convert 512-byte units to 1K blocks, rounding up.
    blocks_512.div_ceil(2)
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
}
