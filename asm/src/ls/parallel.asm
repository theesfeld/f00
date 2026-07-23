; f00-asm — parallel/batch stat hook
; Inline statx in readdir remains the fast path. This entry is reserved for
; optional multi-worker mode; currently a safe no-op to avoid clone races.
BITS 64
DEFAULT REL
%include "syscalls.inc"

global parallel_stat_entries
extern g_entries, g_entry_count

section .text

; parallel_stat_entries(rdi=dirfd) — no-op (entries already stated in list path)
parallel_stat_entries:
    xor eax, eax
    ret
