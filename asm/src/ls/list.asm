; f00-asm — directory enumeration, statx, filter, sort
BITS 64
DEFAULT REL
%include "syscalls.inc"

global list_path, sort_entries, dtype_to_flags, entry_reserve
global g_entries, g_entry_count, g_entry_cap, g_string_pool
extern arena_alloc, arena_reset
extern g_opts, g_opts2, g_exit, g_color, g_now_sec, g_sort
extern git_annotate
extern ignore_should_hide, ignore_load_files
extern archive_try_list, is_archive_path
extern parallel_stat_entries
extern strlen, strcmp, memcpy, memset
extern out_str, out_byte, out_flush

section .bss
alignb 64
; pointer table to Entry* (we store entries contiguously and build index)
g_entries:      resq 1              ; base of Entry array
g_entry_count:  resq 1
g_entry_cap:    resq 1
g_string_pool:  resq 1              ; not used separately — names in arena

; 256 KiB getdents buffer
alignb 64
dents_buf:      resb 262144
statx_buf:      resb STX_SIZEOF
path_join_buf:  resb 4096
readlink_buf:   resb 4096
name_cache:     resb 256            ; uid/gid decimal cache not needed first pass

section .rodata
dot:            db ".", 0
dotdot:         db "..", 0
slash:          db "/", 0
err_open:       db "f00: cannot open directory: ", 0
err_stat:       db "f00: cannot access: ", 0
nl:             db 10

section .text

; ------------------------------------------------------------
; dtype_to_flags(dil=d_type) -> al flags (EF_DIR|EF_LNK)
; ------------------------------------------------------------
dtype_to_flags:
    xor eax, eax
    cmp dil, DT_DIR
    jne .lnk
    or al, EF_DIR
    ret
.lnk:
    cmp dil, DT_LNK
    jne .done
    or al, EF_LNK
.done:
    ret

; ------------------------------------------------------------
; entry_reserve → rax = new Entry*
; Storage model:
;   g_entries → array of Entry* (pointer table, sorted in place)
;   each Entry is 64B from the arena (never relocated)
; ------------------------------------------------------------
entry_reserve:
    push rbx
    push r12
    ; grow pointer table if needed
    mov rax, [g_entry_count]
    cmp rax, [g_entry_cap]
    jb .have_tab
    mov rbx, [g_entry_cap]
    test rbx, rbx
    jnz .dbl
    mov rbx, 1024                   ; start roomy — avoid early growth
    jmp .alloc_tab
.dbl:
    add rbx, rbx
.alloc_tab:
    mov [g_entry_cap], rbx
    mov rdi, rbx
    shl rdi, 3                      ; *8 pointer bytes
    call arena_alloc
    mov rcx, [g_entries]
    test rcx, rcx
    jz .set_tab
    mov rsi, rcx
    mov rdi, rax
    mov rdx, [g_entry_count]
    shl rdx, 3
    push rax
    call memcpy
    pop rax
.set_tab:
    mov [g_entries], rax
.have_tab:
    ; allocate Entry body
    mov rdi, ENTRY_SIZE
    call arena_alloc
    mov r12, rax                    ; Entry*
    ; zero it
    mov rdi, rax
    xor sil, sil
    mov rdx, ENTRY_SIZE
    call memset
    ; store pointer
    mov rax, [g_entry_count]
    mov rbx, [g_entries]
    mov [rbx + rax*8], r12
    inc qword [g_entry_count]
    mov rax, r12
    pop r12
    pop rbx
    ret

; ------------------------------------------------------------
; need_stat_p: returns ZF=1 if we can skip statx for this listing mode
; For short listings without -i/-s/-t/-S/-F/--color needing mode, skip.
; We always need type for colors/classify; d_type often enough.
; ------------------------------------------------------------
need_full_stat:
    mov eax, [g_opts]
    test eax, OPT_LONG | OPT_INODE | OPT_BLOCKS | OPT_TIME | OPT_SIZE | OPT_CLASSIFY | OPT_FILETYPE
    jnz .yes
    mov eax, [g_opts2]
    test eax, OPT2_JSON | OPT2_JSON_FULL | OPT2_CSV | OPT2_TSV | OPT2_GIT | OPT2_HYPER | OPT2_CONTEXT
    jnz .yes
    cmp byte [g_sort], SORT_SIZE
    je .yes
    cmp byte [g_sort], SORT_TIME
    je .yes
    ; icons may need exec bit
    extern icon_enabled
    call icon_enabled
    test al, al
    jnz .yes
    xor eax, eax
    ret
.yes:
    mov eax, 1
    ret

; ------------------------------------------------------------
; statx_fill(rdi=dirfd, rsi=name, rdx=Entry*)
; Fills metadata. Uses AT_SYMLINK_NOFOLLOW unless OPT_FOLLOW.
; ------------------------------------------------------------
statx_fill:
    ; rdi=dirfd, rsi=name, rdx=Entry*
    push rbx
    push r12
    push r13
    mov r12, rdx                    ; Entry*
    mov r13, rdi                    ; dirfd
    mov rbx, rsi                    ; name
    mov eax, [g_opts]
    mov edx, AT_SYMLINK_NOFOLLOW
    test eax, OPT_FOLLOW
    jz .flags_ok
    xor edx, edx
.flags_ok:
    ; statx(dirfd, path, flags, mask, buf)
    mov rax, SYS_statx
    mov rdi, r13
    mov rsi, rbx
    ; rdx = flags
    mov r10, STATX_BASIC_STATS
    lea r8, [statx_buf]
    syscall
    test rax, rax
    js .fail

    ; parse statx_buf into Entry
    lea rsi, [statx_buf]
    mov eax, [rsi + STX_MODE]
    mov [r12 + Entry.mode], eax
    mov eax, [rsi + STX_NLINK]
    mov [r12 + Entry.nlink], eax
    mov eax, [rsi + STX_UID]
    mov [r12 + Entry.uid], eax
    mov eax, [rsi + STX_GID]
    mov [r12 + Entry.gid], eax
    mov rax, [rsi + STX_SIZE]
    mov [r12 + Entry.size], rax
    mov rax, [rsi + STX_BLOCKS]
    mov [r12 + Entry.blocks], rax
    mov rax, [rsi + STX_INO]
    mov [r12 + Entry.ino], rax
    mov rax, [rsi + STX_MTIME_SEC]
    mov [r12 + Entry.mtime_sec], rax
    mov rax, [rsi + STX_ATIME_SEC]
    mov [r12 + Entry.atime_sec], rax
    mov rax, [rsi + STX_CTIME_SEC]
    mov [r12 + Entry.ctime_sec], rax
    mov rax, [rsi + STX_BTIME_SEC]
    mov [r12 + Entry.btime_sec], rax
    mov eax, [rsi + STX_RDEV_MAJOR]
    mov ecx, [rsi + STX_RDEV_MINOR]
    shl eax, 8
    or eax, ecx
    mov [r12 + Entry.rdev], eax

    ; flags from mode
    mov eax, [r12 + Entry.mode]
    mov ecx, eax
    and ecx, S_IFMT
    mov bl, [r12 + Entry.flags]
    and bl, ~(EF_DIR|EF_LNK|EF_EXEC)
    cmp ecx, S_IFDIR
    jne .notdir
    or bl, EF_DIR
    jmp .type_done
.notdir:
    cmp ecx, S_IFLNK
    jne .notlnk
    or bl, EF_LNK
    ; follow target once so icons see dir/exec (keep EF_LNK)
    push r12
    push r13
    push r14
    mov r14b, bl
    mov rsi, [r12 + Entry.name]
    mov rax, SYS_statx
    mov rdi, r13
    xor edx, edx                    ; follow symlinks
    mov r10, STATX_BASIC_STATS
    lea r8, [statx_buf]
    syscall
    mov bl, r14b
    test rax, rax
    js .lnk_follow_done
    lea rsi, [statx_buf]
    mov eax, [rsi + STX_MODE]
    mov ecx, eax
    and ecx, S_IFMT
    cmp ecx, S_IFDIR
    jne .lnk_notdir
    or bl, EF_DIR
    jmp .lnk_follow_done
.lnk_notdir:
    cmp ecx, S_IFREG
    jne .lnk_follow_done
    test eax, 0o111
    jz .lnk_follow_done
    or bl, EF_EXEC
.lnk_follow_done:
    pop r14
    pop r13
    pop r12
    jmp .type_done
.notlnk:
    cmp ecx, S_IFREG
    jne .type_done
    test eax, 0o111
    jz .type_done
    or bl, EF_EXEC
.type_done:
    or bl, EF_STATED
    mov [r12 + Entry.flags], bl

    ; readlink for symlinks when long/json
    test bl, EF_LNK
    jz .skip_rl
    mov eax, [g_opts]
    or eax, [g_opts2]
    test eax, eax
    ; always try for long
    mov eax, [g_opts]
    test eax, OPT_LONG
    jnz .do_rl
    mov eax, [g_opts2]
    test eax, OPT2_JSON | OPT2_JSON_FULL | OPT2_CSV | OPT2_TSV
    jz .skip_rl
.do_rl:
    ; readlinkat(dirfd, name, buf, 4095)
    push r12
    push r13
    push rbx
    mov rax, SYS_readlinkat
    mov rdi, r13
    mov rsi, rbx
    lea rdx, [readlink_buf]
    mov r10, 4095
    syscall
    pop rbx
    pop r13
    pop r12
    test rax, rax
    js .skip_rl
    mov r8, rax
    lea rdi, [rax + 1]
    push r8
    call arena_alloc
    pop r8
    mov rdi, rax
    lea rsi, [readlink_buf]
    mov rdx, r8
    push rax
    push r8
    call memcpy
    pop r8
    pop rax
    mov byte [rax + r8], 0
    mov [r12 + Entry.target], rax
    mov [r12 + Entry.targetlen], r8d
.skip_rl:

    ; SELinux context (-Z)
    mov eax, [g_opts2]
    test eax, OPT2_CONTEXT
    jz .skip_ctx
    mov rax, SYS_lgetxattr
    mov rdi, rbx                    ; path name relative - need full path
    ; use name relative to dirfd via /proc/self/fd is hard; use openat+fgetxattr
    ; For relative names with dirfd: construct nothing — use getxattr on joined path
    ; Simpler: skip if not absolute; use fgetxattr after openat O_PATH
    push r12
    push r13
    push rbx
    mov rax, SYS_openat
    mov rdi, r13
    mov rsi, rbx
    mov rdx, O_RDONLY | O_CLOEXEC | O_NOFOLLOW | O_PATH
    xor r10, r10
    syscall
    cmp rax, -4096
    jae .ctx_fail
    mov r8, rax
    sub rsp, 256
    mov rax, SYS_fgetxattr
    mov rdi, r8
    lea rsi, [xattr_selinux]
    mov rdx, rsp
    mov r10, 255
    syscall
    mov r9, rax
    push r8
    push r9
    mov rdi, r8
    mov rax, SYS_close
    syscall
    pop r9
    pop r8
    test r9, r9
    js .ctx_pop
    mov rdi, r9
    inc rdi
    push r9
    call arena_alloc
    pop r9
    mov rdi, rax
    mov rsi, rsp
    mov rdx, r9
    push rax
    call memcpy
    pop rax
    mov byte [rax + r9], 0
    mov [r12 + Entry.context], rax
.ctx_pop:
    add rsp, 256
.ctx_fail:
    pop rbx
    pop r13
    pop r12
.skip_ctx:

    ; dtype from mode for consistency
    cmp ecx, S_IFDIR
    jne .dt1
    mov byte [r12 + Entry.dtype], DT_DIR
    jmp .ok
.dt1:
    cmp ecx, S_IFLNK
    jne .dt2
    mov byte [r12 + Entry.dtype], DT_LNK
    jmp .ok
.dt2:
    cmp ecx, S_IFREG
    jne .dt3
    mov byte [r12 + Entry.dtype], DT_REG
    jmp .ok
.dt3:
    cmp ecx, S_IFIFO
    jne .dt4
    mov byte [r12 + Entry.dtype], DT_FIFO
    jmp .ok
.dt4:
    cmp ecx, S_IFCHR
    jne .dt5
    mov byte [r12 + Entry.dtype], DT_CHR
    jmp .ok
.dt5:
    cmp ecx, S_IFBLK
    jne .dt6
    mov byte [r12 + Entry.dtype], DT_BLK
    jmp .ok
.dt6:
    cmp ecx, S_IFSOCK
    jne .ok
    mov byte [r12 + Entry.dtype], DT_SOCK
.ok:
    xor eax, eax
    pop r13
    pop r12
    pop rbx
    ret
.fail:
    mov eax, -1
    pop r13
    pop r12
    pop rbx
    ret

; ------------------------------------------------------------
; should_show(rsi=name) -> al 1/0
; ------------------------------------------------------------
should_show:
    mov al, [rsi]
    cmp al, '.'
    jne .yes
    ; hidden
    mov eax, [g_opts]
    test eax, OPT_ALL
    jnz .yes
    test eax, OPT_ALMOST_ALL
    jnz .almost
    xor al, al
    ret
.almost:
    ; hide . and ..
    cmp byte [rsi], '.'
    jne .yes
    cmp byte [rsi+1], 0
    je .no
    cmp byte [rsi+1], '.'
    jne .yes
    cmp byte [rsi+2], 0
    je .no
.yes:
    mov al, 1
    ret
.no:
    xor al, al
    ret

; ------------------------------------------------------------
; list_directory(rdi = path NUL-terminated)
; Populates g_entries / g_entry_count for one directory.
; Does NOT reset arena (caller manages).
; ------------------------------------------------------------
list_directory:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    push r14
    push r15
    sub rsp, 8

    mov r15, rdi                    ; path

    ; openat(AT_FDCWD, path, O_RDONLY|O_DIRECTORY|O_CLOEXEC)
    mov rax, SYS_openat
    mov rdi, AT_FDCWD
    mov rsi, r15
    mov rdx, O_RDONLY | O_DIRECTORY | O_CLOEXEC
    xor r10, r10
    syscall
    cmp rax, -4096
    jae .open_fail
    mov r14, rax                    ; dirfd

    ; reset entry count (reuse Entry array capacity)
    mov qword [g_entry_count], 0

    ; load .gitignore / .f00ignore for this dir
    mov rdi, r15
    call ignore_load_files

    call need_full_stat
    mov r13, rax                    ; 1 if need stat

.read_loop:
    mov rax, SYS_getdents64
    mov rdi, r14
    lea rsi, [dents_buf]
    mov rdx, 262144
    syscall
    test rax, rax
    jz .done_read
    js .read_fail
    mov r12, rax                    ; bytes returned
    xor rbx, rbx                    ; offset into dents_buf

.dent_loop:
    cmp rbx, r12
    jae .read_loop

    lea r8, [dents_buf + rbx]       ; dirent*
    movzx r9d, word [r8 + D64_RECLEN]
    lea r10, [r8 + D64_NAME]        ; name

    ; filter
    mov rsi, r10
    push r8
    push r9
    push r10
    call should_show
    pop r10
    pop r9
    pop r8
    test al, al
    jz .next_dent
    push r8
    push r9
    push r10
    mov rsi, r10
    call ignore_should_hide
    pop r10
    pop r9
    pop r8
    test al, al
    jnz .next_dent

    ; copy name into arena
    mov rdi, r10
    push r8
    push r9
    push r10
    call strlen
    mov r11, rax                    ; namelen
    lea rdi, [rax + 1]
    call arena_alloc
    mov rsi, r10
    mov rdi, rax
    mov rdx, r11
    push rax
    push r11
    call memcpy
    pop r11
    pop rax
    mov byte [rax + r11], 0
    mov rsi, rax                    ; name in arena
    pop r10
    pop r9
    pop r8

    ; reserve entry
    push rsi
    push r8
    push r9
    push r11
    call entry_reserve
    mov rdi, rax                    ; Entry*
    pop r11
    pop r9
    pop r8
    pop rsi

    mov [rdi + Entry.name], rsi
    mov [rdi + Entry.namelen], r11w
    mov [rdi + Entry.path], rsi
    mov [rdi + Entry.pathlen], r11d
    mov byte [rdi + Entry.git], 0
    mov byte [rdi + Entry.depth], 0
    mov qword [rdi + Entry.target], 0
    mov dword [rdi + Entry.targetlen], 0
    mov qword [rdi + Entry.context], 0
    mov al, [r8 + D64_TYPE]
    mov [rdi + Entry.dtype], al
    push rdi
    mov dil, al
    call dtype_to_flags
    pop rdi
    mov [rdi + Entry.flags], al
    mov rax, [r8 + D64_INO]
    mov [rdi + Entry.ino], rax
    ; zero rest
    mov qword [rdi + Entry.size], 0
    mov qword [rdi + Entry.mtime_sec], 0
    mov qword [rdi + Entry.blocks], 0
    mov dword [rdi + Entry.mode], 0
    mov dword [rdi + Entry.nlink], 0
    mov dword [rdi + Entry.uid], 0
    mov dword [rdi + Entry.gid], 0
    mov dword [rdi + Entry.rdev], 0

    ; optional statx
    test r13, r13
    jz .next_dent
    ; also stat if dtype unknown
    cmp byte [rdi + Entry.dtype], DT_UNKNOWN
    je .do_stat
    test r13, r13
    jz .next_dent
.do_stat:
    push r8
    push r9
    push r12
    push r13
    push r14
    mov rsi, [rdi + Entry.name]
    mov rdx, rdi
    mov rdi, r14                    ; dirfd
    call statx_fill
    pop r14
    pop r13
    pop r12
    pop r9
    pop r8
    ; ignore stat fail for individual entries (minor error)
    test eax, eax
    jns .next_dent
    mov dword [g_exit], 1

.next_dent:
    add rbx, r9
    jmp .dent_loop

.done_read:
    mov rax, SYS_close
    mov rdi, r14
    syscall

    ; if color and not full stat, still may want dtype-only colors — OK
    ; if classify and no full stat done for exec — need_full_stat covers

    ; optional parallel re-stat for any unstated (noop if all stated)
    mov rdi, r14
    call parallel_stat_entries

    ; sort
    call sort_entries

    ; git annotate when enabled
    mov rdi, r15
    call git_annotate

    xor eax, eax
    add rsp, 8
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    ret

.open_fail:
    ; try as file: single-entry listing via AT_FDCWD stat
    mov rdi, r15
    call list_single
    add rsp, 8
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    ret

.read_fail:
    mov dword [g_exit], 1
    mov rax, SYS_close
    mov rdi, r14
    syscall
    mov eax, -1
    add rsp, 8
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    ret

; list_single(rdi=path): one entry for a file or -d
list_single:
    push rbx
    push r12
    mov r12, rdi

    mov qword [g_entry_count], 0
    call entry_reserve
    mov rbx, rax

    ; basename for display: find last /
    mov rdi, r12
    call strlen
    mov rcx, rax
    mov rsi, r12
    lea rdi, [r12 + rcx]
.find:
    cmp rdi, r12
    jbe .base
    dec rdi
    cmp byte [rdi], '/'
    jne .find
    inc rdi
    mov rsi, rdi
.base:
    ; rsi = basename start
    mov rdi, rsi
    push rsi
    call strlen
    mov r11, rax
    lea rdi, [rax + 1]
    call arena_alloc
    pop rsi
    mov rdi, rax
    mov rdx, r11
    push rax
    push r11
    call memcpy
    pop r11
    pop rax
    mov byte [rax + r11], 0
    mov [rbx + Entry.name], rax
    mov [rbx + Entry.namelen], r11w

    mov rdi, AT_FDCWD
    mov rsi, r12
    mov rdx, rbx
    call statx_fill
    test eax, eax
    jns .ok
    mov dword [g_exit], 1
.ok:
    pop r12
    pop rbx
    ret

; ------------------------------------------------------------
; list_path(rdi=path): top-level entry point for one path arg
; Handles -d, files, directories. Recursive -R handled by main.
; ------------------------------------------------------------
list_path:
    push rbx
    mov rbx, rdi

    mov eax, [g_opts]
    test eax, OPT_DIRECTORY
    jz .normal
    call list_single
    pop rbx
    ret
.normal:
    ; archives as virtual dirs unless --core
    mov eax, [g_opts2]
    test eax, OPT2_CORE
    jnz .dironly
    mov rdi, rbx
    call is_archive_path
    test al, al
    jz .dironly
    mov rdi, rbx
    call archive_try_list
    test eax, eax
    jnz .dironly
    call sort_entries
    pop rbx
    ret
.dironly:
    mov rdi, rbx
    call list_directory
    ; if open failed, try archive again
    test eax, eax
    jns .ok
    mov rdi, rbx
    call archive_try_list
.ok:
    pop rbx
    ret

; ------------------------------------------------------------
; sort_entries — shell sort on Entry* pointer table (8-byte swaps)
; ------------------------------------------------------------
sort_entries:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov eax, [g_opts]
    test eax, OPT_NOSORT
    jnz .maybe_rev
    mov r12, [g_entries]            ; Entry** table
    mov r13, [g_entry_count]
    cmp r13, 2
    jb .maybe_rev

    mov r14, r13
    shr r14, 1                      ; gap
.gap:
    test r14, r14
    jz .maybe_rev
    mov rbx, r14                    ; i
.outer:
    cmp rbx, r13
    jae .next_gap
    mov r15, [r12 + rbx*8]          ; key ptr
    mov rcx, rbx                    ; j (use stack for j across calls)
    push rcx
.inner:
    mov rcx, [rsp]
    cmp rcx, r14
    jb .place
    mov rax, rcx
    sub rax, r14
    mov rsi, [r12 + rax*8]          ; entries[j-gap]
    mov rdi, r15                    ; key
    push r15
    call cmp_entries
    pop r15
    test eax, eax
    jns .place
    mov rcx, [rsp]
    mov rax, rcx
    sub rax, r14
    mov rdx, [r12 + rax*8]
    mov [r12 + rcx*8], rdx
    sub rcx, r14
    mov [rsp], rcx
    jmp .inner
.place:
    pop rcx
    mov [r12 + rcx*8], r15
    inc rbx
    jmp .outer
.next_gap:
    shr r14, 1
    jmp .gap

.maybe_rev:
    mov eax, [g_opts]
    test eax, OPT_REVERSE
    jz .done
    call reverse_entries
.done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret

reverse_entries:
    mov rcx, [g_entry_count]
    cmp rcx, 2
    jb .done
    mov rdi, [g_entries]
    xor eax, eax
    dec rcx
.lp:
    cmp rax, rcx
    jae .done
    mov r8, [rdi + rax*8]
    mov r9, [rdi + rcx*8]
    mov [rdi + rax*8], r9
    mov [rdi + rcx*8], r8
    inc rax
    dec rcx
    jmp .lp
.done:
    ret

; cmp_entries(rdi=a, rsi=b) -> eax <0,0,>0  (a compared to b)
cmp_entries:
    push rbx
    push r12
    push r13
    mov r12, rdi
    mov r13, rsi
    mov eax, [g_opts]

    test eax, OPT_DIRS_FIRST
    jz .by_key
    mov bl, [r12 + Entry.flags]
    mov cl, [r13 + Entry.flags]
    and bl, EF_DIR
    and cl, EF_DIR
    cmp bl, cl
    je .by_key
    test bl, bl
    jnz .lt
    jmp .gt

.by_key:
    movzx ecx, byte [g_sort]
    cmp cl, SORT_TIME
    je .by_time
    cmp cl, SORT_SIZE
    je .by_size
    cmp cl, SORT_EXT
    je .by_ext
    cmp cl, SORT_VERSION
    je .by_version
    cmp cl, SORT_NONE
    je .eq0
    test eax, OPT_TIME
    jnz .by_time
    test eax, OPT_SIZE
    jnz .by_size
    test eax, OPT_EXT
    jnz .by_ext
.by_name:
    mov rdi, [r12 + Entry.name]
    mov rsi, [r13 + Entry.name]
    call strcmp
    jmp .done

.by_time:
    mov rax, [r12 + Entry.mtime_sec]
    mov rcx, [r13 + Entry.mtime_sec]
    cmp rax, rcx
    jb .gt
    ja .lt
    jmp .by_name

.by_size:
    mov rax, [r12 + Entry.size]
    mov rcx, [r13 + Entry.size]
    cmp rax, rcx
    jb .gt
    ja .lt
    jmp .by_name

.by_ext:
    mov rdi, [r12 + Entry.name]
    call find_ext
    mov rbx, rax
    mov rdi, [r13 + Entry.name]
    call find_ext
    mov rdi, rbx
    mov rsi, rax
    call strcmp
    test eax, eax
    jnz .done
    jmp .by_name

.by_version:
    mov rdi, [r12 + Entry.name]
    mov rsi, [r13 + Entry.name]
    call strverscmp
    jmp .done

.lt:
    mov eax, -1
    jmp .done
.gt:
    mov eax, 1
.eq0:
    xor eax, eax
.done:
    pop r13
    pop r12
    pop rbx
    ret

; find_ext(rdi=name) -> rax ptr to extension or empty string
find_ext:
    push rdi
    call strlen
    pop rdi
    lea rsi, [rdi + rax]
.lp:
    cmp rsi, rdi
    jbe .empty
    dec rsi
    cmp byte [rsi], '.'
    je .found
    cmp byte [rsi], '/'
    je .empty
    jmp .lp
.found:
    lea rax, [rsi + 1]
    ret
.empty:
    lea rax, [empty_str]
    ret


; strverscmp-like (GNU): digit runs compared numerically
strverscmp:
    push rbx
    push r12
    push r13
    mov r12, rdi
    mov r13, rsi
.lp:
    movzx eax, byte [r12]
    movzx ecx, byte [r13]
    cmp al, cl
    jne .diff
    test al, al
    jz .eq
    ; if both digits, compare runs
    cmp al, '0'
    jb .adv
    cmp al, '9'
    ja .adv
    ; digit run
    xor r8, r8
    xor r9, r9
.d1:
    movzx eax, byte [r12]
    cmp al, '0'
    jb .d1e
    cmp al, '9'
    ja .d1e
    imul r8, 10
    sub al, '0'
    add r8, rax
    inc r12
    jmp .d1
.d1e:
.d2:
    movzx eax, byte [r13]
    cmp al, '0'
    jb .d2e
    cmp al, '9'
    ja .d2e
    imul r9, 10
    sub al, '0'
    add r9, rax
    inc r13
    jmp .d2
.d2e:
    cmp r8, r9
    jb .lt
    ja .gt
    jmp .lp
.adv:
    inc r12
    inc r13
    jmp .lp
.diff:
    ; if both in digit, handled above
    movzx eax, al
    movzx ecx, cl
    sub eax, ecx
    jmp .out
.eq:
    xor eax, eax
    jmp .out
.lt:
    mov eax, -1
    jmp .out
.gt:
    mov eax, 1
.out:
    pop r13
    pop r12
    pop rbx
    ret

section .rodata
empty_str: db 0

section .rodata
xattr_selinux: db "security.selinux", 0
