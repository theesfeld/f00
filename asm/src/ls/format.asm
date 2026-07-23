; f00-asm — short (columns) + long listing formatters, LS_COLORS subset
BITS 64
DEFAULT REL
%include "syscalls.inc"

global format_listing, color_init, emit_name_public, format_perms_into
extern g_entries, g_entry_count
extern g_opts, g_opts2, g_color, g_cols, g_tty, g_now_sec
extern out_byte, out_str, out_strn, out_u64, out_spaces, out_pad, out_write
extern human_size, u64_to_dec_buf, strlen, memcpy
extern icon_for_entry, icon_enabled
extern git_status_char
extern format_json, format_csv, format_tsv, format_tree
extern uid_to_name, gid_to_name
extern color_seq_for_entry, color_reset_seq
extern quote_emit_name
extern g_opts2
extern meta_paint

section .bss
align 64
widths:         resd 512            ; per-entry display widths (cap 512 shown fully)
name_tmp:       resb 64
perm_buf:       resb 16
human_buf:      resb 32
time_buf:       resb 32
uid_buf:        resb 16
gid_buf:        resb 16
size_buf:       resb 32

section .rodata
; ANSI
c_reset:        db 27, "[0m", 0
c_dir:          db 27, "[01;34m", 0
c_lnk:          db 27, "[01;36m", 0
c_exe:          db 27, "[01;32m", 0
c_fifo:         db 27, "[40;33m", 0
c_sock:         db 27, "[01;35m", 0
c_blk:          db 27, "[40;33;01m", 0
c_chr:          db 27, "[40;33;01m", 0
c_orhpan:       db 27, "[40;31;01m", 0
c_git_m:        db 27, "[33m", 0        ; modified / renamed
c_git_a:        db 27, "[32m", 0        ; added
c_git_d:        db 27, "[31m", 0        ; deleted / conflict
c_git_u:        db 27, "[31m", 0        ; untracked

; 4 bytes each: 3-letter month + NUL
months:
    db "Jan",0,"Feb",0,"Mar",0,"Apr",0
    db "May",0,"Jun",0,"Jul",0,"Aug",0
    db "Sep",0,"Oct",0,"Nov",0,"Dec",0

total_prefix:   db "total ", 0
arrow:          db " -> ", 0
space:          db " ", 0

section .text

color_init:
    ; g_color already decided by main from opts + tty
    ret

; paint_prefix(rdi=Entry*) → al=1 if ANSI emitted
paint_prefix:
    xor eax, eax
    cmp byte [g_color], 0
    je .done
    call color_seq_for_entry
    test al, al
    jz .done
    call out_str
    mov al, 1
.done:
    ret

paint_reset:
    call color_reset_seq
    jmp out_str

; git_paint_prefix(dil=git code) → al=1 if SGR emitted
; Uses eza-style: modified yellow, added green, deleted red
git_paint_prefix:
    push rbx
    mov bl, dil
    cmp byte [g_color], 0
    je .no
    cmp bl, GIT_MODIFIED
    je .mod
    cmp bl, GIT_RENAMED
    je .mod
    cmp bl, GIT_ADDED
    je .add
    cmp bl, GIT_DELETED
    je .del
    cmp bl, GIT_UNTRACKED
    je .unt
    cmp bl, GIT_CONFLICTED
    je .del
    jmp .no
.mod:
    lea rsi, [c_git_m]
    jmp .emit
.add:
    lea rsi, [c_git_a]
    jmp .emit
.del:
    lea rsi, [c_git_d]
    jmp .emit
.unt:
    lea rsi, [c_git_u]
.emit:
    call out_str
    mov al, 1
    pop rbx
    ret
.no:
    xor al, al
    pop rbx
    ret

; indicator_char(rdi=Entry*) -> al char or 0
indicator_char:
    mov eax, [g_opts]
    test eax, OPT_CLASSIFY | OPT_SLASH
    jz .none
    test byte [rdi + Entry.flags], EF_DIR
    jnz .slash
    cmp byte [rdi + Entry.dtype], DT_DIR
    je .slash
    test eax, OPT_SLASH
    jnz .none                       ; -p only dirs
    ; -F classify
    test byte [rdi + Entry.flags], EF_LNK
    jnz .at
    cmp byte [rdi + Entry.dtype], DT_LNK
    je .at
    cmp byte [rdi + Entry.dtype], DT_FIFO
    je .pipe
    cmp byte [rdi + Entry.dtype], DT_SOCK
    je .eq
    test byte [rdi + Entry.flags], EF_EXEC
    jnz .star
    test byte [rdi + Entry.flags], EF_STATED
    jz .none
    mov eax, [rdi + Entry.mode]
    test eax, 0o111
    jnz .star
.none:
    xor al, al
    ret
.slash:
    mov al, '/'
    ret
.at:
    mov al, '@'
    ret
.star:
    mov al, '*'
    ret
.pipe:
    mov al, '|'
    ret
.eq:
    mov al, '='
    ret

; entry_disp_width(rdi=Entry*) -> eax display cells (name + chrome)
entry_disp_width:
    push rbx
    push r12
    mov rbx, rdi
    movzx r12d, word [rbx + Entry.namelen]
    mov rdi, rbx
    call indicator_char
    test al, al
    jz .noind
    inc r12d
.noind:
    mov eax, [g_opts]
    test eax, OPT_INODE
    jz .noino
    push r12
    mov rdi, [rbx + Entry.ino]
    lea rsi, [name_tmp]
    call u64_to_dec_buf
    pop r12
    add r12d, eax
    inc r12d                        ; space
.noino:
    mov eax, [g_opts]
    test eax, OPT_BLOCKS
    jz .noblk
    push r12
    mov rax, [rbx + Entry.blocks]
    add rax, 1
    shr rax, 1
    mov rdi, rax
    lea rsi, [name_tmp]
    call u64_to_dec_buf
    pop r12
    add r12d, eax
    inc r12d
.noblk:
    ; git status column: always 2 cells when enabled (char/space + pad)
    mov eax, [g_opts2]
    test eax, OPT2_GIT
    jz .nogitw
    test eax, OPT2_NO_GIT | OPT2_CORE
    jnz .nogitw
    add r12d, 2
.nogitw:
    ; icon: one terminal cell + trailing space (Nerd Fonts are typically width-1)
    mov rdi, rbx
    call icon_for_entry
    cmp byte [rsi], 0
    je .noiconw
    add r12d, 2
.noiconw:
    mov eax, r12d
    pop r12
    pop rbx
    ret

; emit_name(rdi=Entry*)
emit_name_public:
emit_name:
    push rbx
    push r12
    mov rbx, rdi
    mov eax, [g_opts]
    test eax, OPT_INODE
    jz .noino
    mov rdi, [rbx + Entry.ino]
    call out_u64
    mov dil, ' '
    call out_byte
.noino:
    mov eax, [g_opts]
    test eax, OPT_BLOCKS
    jz .noblk
    mov rax, [rbx + Entry.blocks]
    add rax, 1
    shr rax, 1
    mov rdi, rax
    call out_u64
    mov dil, ' '
    call out_byte
.noblk:
    mov eax, [g_opts2]
    test eax, OPT2_GIT
    jz .nogit
    test eax, OPT2_NO_GIT | OPT2_CORE
    jnz .nogit
    ; fixed-width git column (char or space) for eza-like alignment
    mov dil, [rbx + Entry.git]
    call git_status_char
    test al, al
    jnz .gitch
    mov al, ' '
.gitch:
    mov r12b, al
    ; color by status when color on (gm/ga/gd meta keys)
    cmp byte [g_color], 0
    je .gitplain
    mov dil, [rbx + Entry.git]
    call git_paint_prefix
    test al, al
    jz .gitplain
    mov dil, r12b
    call out_byte
    call paint_reset
    jmp .gitsp
.gitplain:
    mov dil, r12b
    call out_byte
.gitsp:
    mov dil, ' '
    call out_byte
.nogit:
    mov rdi, rbx
    call icon_for_entry
    cmp byte [rsi], 0
    je .noicon
    call out_str
    mov dil, ' '
    call out_byte
.noicon:
    mov rdi, rbx
    call paint_prefix
    mov r12b, al
    ; optional hyperlink open
    mov eax, [g_opts2]
    test eax, OPT2_HYPER
    jz .nohyp
    cmp byte [g_tty], 0
    je .nohyp
    call hyperlink_open
.nohyp:
    mov rsi, [rbx + Entry.name]
    movzx edx, word [rbx + Entry.namelen]
    call quote_emit_name
    mov eax, [g_opts2]
    test eax, OPT2_HYPER
    jz .nohyp2
    cmp byte [g_tty], 0
    je .nohyp2
    call hyperlink_close
.nohyp2:
    test r12b, r12b
    jz .noreset
    call paint_reset
.noreset:
    mov rdi, rbx
    call indicator_char
    test al, al
    jz .sym
    mov dil, al
    call out_byte
.sym:
    mov eax, [g_opts]
    test eax, OPT_LONG
    jz .done
    mov rsi, [rbx + Entry.target]
    test rsi, rsi
    jz .done
    lea rsi, [arrow_sym]
    call out_str
    mov rsi, [rbx + Entry.target]
    mov edx, [rbx + Entry.targetlen]
    call out_strn
.done:
    pop r12
    pop rbx
    ret

section .rodata
arrow_sym: db " -> ", 0
section .text

; ------------------------------------------------------------
; format_one_per_line
; ------------------------------------------------------------
format_one_per_line:
    push rbx
    push r12
    mov r12, [g_entries]
    xor rbx, rbx
.lp:
    cmp rbx, [g_entry_count]
    jae .done
    mov rdi, [r12 + rbx*8]
    call emit_name
    mov dil, 10
    call out_byte
    inc rbx
    jmp .lp
.done:
    pop r12
    pop rbx
    ret

; ------------------------------------------------------------
; format_commas (-m)
; ------------------------------------------------------------
format_commas:
    push rbx
    push r12
    mov r12, [g_entries]
    xor rbx, rbx
.lp:
    cmp rbx, [g_entry_count]
    jae .done
    test rbx, rbx
    jz .name
    lea rsi, [comma_sp]
    call out_str
.name:
    mov rdi, [r12 + rbx*8]
    call emit_name
    inc rbx
    jmp .lp
.done:
    mov dil, 10
    call out_byte
    pop r12
    pop rbx
    ret

section .rodata
comma_sp: db ", ", 0

section .text

; ------------------------------------------------------------
; format_columns — column-major like ls -C
; ------------------------------------------------------------
format_columns:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    push r14
    push r15

    mov r12, [g_entry_count]
    test r12, r12
    jz .done

    ; compute widths[i]
    cmp r12, 512
    jbe .cap_ok
    mov r12, 512                    ; safety cap for width array
.cap_ok:
    mov r13, [g_entries]
    xor rbx, rbx
    xor r14d, r14d                  ; max_width
.wloop:
    cmp rbx, r12
    jae .wdone
    mov rdi, [r13 + rbx*8]
    call entry_disp_width
    mov [widths + rbx*4], eax
    cmp eax, r14d
    jbe .wnext
    mov r14d, eax
.wnext:
    inc rbx
    jmp .wloop
.wdone:
    add r14d, 2                     ; subtle column gap (2 spaces)
    ; cols = max(1, term_cols / col_width)
    mov eax, [g_cols]
    test eax, eax
    jnz .have_cols
    mov eax, 80
.have_cols:
    xor edx, edx
    mov ecx, r14d
    test ecx, ecx
    jnz .div
    mov ecx, 1
.div:
    div ecx
    test eax, eax
    jnz .ncols
    mov eax, 1
.ncols:
    mov r15d, eax                   ; num columns
    ; rows = ceil(n / cols)
    mov eax, r12d
    add eax, r15d
    dec eax
    xor edx, edx
    div r15d
    mov ebx, eax                    ; num rows

    ; column-major: for row in 0..rows-1
    ;   for col in 0..cols-1
    ;     idx = col*rows + row
    xor r8d, r8d                    ; row
.row:
    cmp r8d, ebx
    jae .nl_done
    xor r9d, r9d                    ; col
.col:
    cmp r9d, r15d
    jae .row_end
    ; idx = col * rows + row
    mov eax, r9d
    mul ebx
    add eax, r8d
    cmp rax, r12
    jae .col_next
    ; emit entry
    push r8
    push r9
    push rbx
    push r12
    push r15
    mov r10, rax
    mov rdi, [r13 + rax*8]
    push r10
    call emit_name
    pop r10
    ; pad to col width unless last column with content
    mov eax, [widths + r10*4]
    mov ecx, r14d
    sub ecx, eax
    ; if next would be empty / last col, skip trailing pad somewhat
    pop r15
    pop r12
    pop rbx
    pop r9
    pop r8
    ; pad only if not last col
    mov eax, r9d
    inc eax
    cmp eax, r15d
    jae .col_next
    ; also if no more entries in later cols for this row, still pad for alignment
    push r8
    push r9
    mov ecx, r14d
    mov eax, [widths + r10*4]
    sub ecx, eax
    call out_spaces
    pop r9
    pop r8
.col_next:
    inc r9d
    jmp .col
.row_end:
    mov dil, 10
    call out_byte
    inc r8d
    jmp .row
.nl_done:
.done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    ret

; ------------------------------------------------------------
; format_long (-l)
; ------------------------------------------------------------
format_perms_into:
format_perms:
    ; rdi=Entry*, rsi=buf[11]
    push rbx
    mov rbx, rdi
    mov eax, [rbx + Entry.mode]
    mov ecx, eax
    and ecx, S_IFMT
    mov dl, '-'
    cmp ecx, S_IFDIR
    jne .1
    mov dl, 'd'
    jmp .t
.1: cmp ecx, S_IFLNK
    jne .2
    mov dl, 'l'
    jmp .t
.2: cmp ecx, S_IFIFO
    jne .3
    mov dl, 'p'
    jmp .t
.3: cmp ecx, S_IFSOCK
    jne .4
    mov dl, 's'
    jmp .t
.4: cmp ecx, S_IFBLK
    jne .5
    mov dl, 'b'
    jmp .t
.5: cmp ecx, S_IFCHR
    jne .t
    mov dl, 'c'
.t:
    mov [rsi], dl
    ; rwx bits
    mov ecx, eax
    lea rdi, [rsi + 1]
    mov r8d, 0o400
    mov r9d, 3                      ; three triples
.triple:
    ; r
    test ecx, r8d
    setnz dl
    mov al, '-'
    test dl, dl
    jz .nr
    mov al, 'r'
.nr: mov [rdi], al
    inc rdi
    shr r8d, 1
    ; w
    test ecx, r8d
    setnz dl
    mov al, '-'
    test dl, dl
    jz .nw
    mov al, 'w'
.nw: mov [rdi], al
    inc rdi
    shr r8d, 1
    ; x / special
    test ecx, r8d
    setnz dl
    ; special bit depends on which triple
    mov al, '-'
    test dl, dl
    jz .nx0
    mov al, 'x'
.nx0:
    mov [rdi], al
    inc rdi
    shr r8d, 1
    dec r9d
    jnz .triple
    ; fix suid/sgid/sticky on positions 3,6,9 (1-based in perms string)
    mov eax, [rbx + Entry.mode]
    ; suid
    test eax, S_ISUID
    jz .nosuid
    mov cl, [rsi+3]
    cmp cl, 'x'
    sete dl
    mov al, 'S'
    test dl, dl
    jz .s1
    mov al, 's'
.s1: mov [rsi+3], al
.nosuid:
    test eax, S_ISGID
    jz .nosgid
    mov cl, [rsi+6]
    cmp cl, 'x'
    sete dl
    mov al, 'S'
    test dl, dl
    jz .s2
    mov al, 's'
.s2: mov [rsi+6], al
.nosgid:
    test eax, S_ISVTX
    jz .nostick
    mov cl, [rsi+9]
    cmp cl, 'x'
    sete dl
    mov al, 'T'
    test dl, dl
    jz .s3
    mov al, 't'
.s3: mov [rsi+9], al
.nostick:
    mov byte [rsi+10], 0
    pop rbx
    ret

; format_mtime(rdi=epoch_sec) → time_buf "Mon DD HH:MM" or "Mon DD  YYYY"
; UTC civil date via Howard Hinnant days_from_civil inverse (no libc)
format_mtime:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov r12, rdi                    ; epoch

    mov rax, r12
    mov rcx, 86400
    cqo
    idiv rcx                        ; rax=days since epoch (may be neg), rdx=sod
    ; force non-negative sec-of-day
    mov r13, rdx
    test r13, r13
    jns .sod_ok
    add r13, 86400
    dec rax
.sod_ok:
    ; z = days_since_epoch + 719468  (1970-01-01 → civil)
    lea r8, [rax + 719468]          ; z
    ; era = floor(z / 146097)
    mov rax, r8
    mov rcx, 146097
    cqo
    idiv rcx
    mov r9, rax                     ; era
    mov r10, rdx                    ; doe (0..146096)
    ; yoe = (doe - doe/1460 + doe/36524 - doe/146096) / 365
    mov rax, r10
    mov rcx, 1460
    xor rdx, rdx
    div rcx
    mov r11, rax                    ; doe/1460
    mov rax, r10
    mov rcx, 36524
    xor rdx, rdx
    div rcx
    sub r11, rax                    ; doe/1460 - doe/36524  (Hinnant: -doe/1460 +doe/36524)
    ; correct Hinnant: yoe = (doe - doe/1460 + doe/36524 - doe/146096) / 365
    ; recompute cleanly
    mov rax, r10
    mov rcx, 1460
    xor rdx, rdx
    div rcx
    mov rbx, rax                    ; a = doe/1460
    mov rax, r10
    mov rcx, 36524
    xor rdx, rdx
    div rcx
    mov r11, rax                    ; b = doe/36524
    mov rax, r10
    mov rcx, 146096
    xor rdx, rdx
    div rcx                         ; c = doe/146096
    ; t = doe - a + b - c
    mov rdx, r10
    sub rdx, rbx
    add rdx, r11
    sub rdx, rax
    mov rax, rdx
    mov rcx, 365
    xor rdx, rdx
    div rcx
    mov r14, rax                    ; yoe 0..399
    ; doy = doe - (365*yoe + yoe/4 - yoe/100)
    mov rax, r14
    mov rcx, 365
    mul rcx
    mov rbx, rax
    mov rax, r14
    shr rax, 2                      ; yoe/4
    add rbx, rax
    mov rax, r14
    mov rcx, 100
    xor rdx, rdx
    div rcx
    sub rbx, rax
    mov rax, r10
    sub rax, rbx
    mov r15, rax                    ; doy 0..365
    ; year = yoe + era*400
    mov rax, r9
    mov rcx, 400
    mul rcx
    add rax, r14
    mov r9, rax                     ; year (may +1 after month)
    ; mp = (5*doy + 2)/153
    mov rax, r15
    mov rcx, 5
    mul rcx
    add rax, 2
    mov rcx, 153
    xor rdx, rdx
    div rcx
    mov r10, rax                    ; mp 0..11
    ; day = doy - (153*mp+2)/5 + 1
    mov rax, r10
    mov rcx, 153
    mul rcx
    add rax, 2
    mov rcx, 5
    xor rdx, rdx
    div rcx
    mov rcx, r15
    sub rcx, rax
    inc rcx
    mov r11, rcx                    ; day 1..31
    ; month = mp < 10 ? mp+3 : mp-9
    mov rax, r10
    cmp rax, 10
    jb .m_early
    sub rax, 9
    jmp .m_set
.m_early:
    add rax, 3
.m_set:
    mov r10, rax                    ; month 1..12
    cmp r10, 2
    ja .y_ok
    inc r9
.y_ok:
    ; hour/min from r13
    mov rax, r13
    mov rcx, 3600
    xor rdx, rdx
    div rcx
    mov r8, rax                     ; hour
    mov rax, rdx
    mov rcx, 60
    xor rdx, rdx
    div rcx
    mov r13, rax                    ; minute

    ; clamp month 1..12
    mov eax, r10d
    cmp eax, 1
    jb .bad
    cmp eax, 12
    jbe .month_ok
.bad:
    ; fallback epoch string
    lea rdi, [time_buf]
    mov dword [rdi], '???'
    mov byte [rdi+3], 0
    jmp .out
.month_ok:
    lea rsi, [months]
    dec eax
    shl eax, 2                      ; *4
    add rsi, rax
    lea rdi, [time_buf]
    mov eax, [rsi]
    mov [rdi], eax                  ; 3 letters + NUL from table
    mov byte [rdi+3], ' '
    add rdi, 4
    ; day, space-padded width 2
    mov eax, r11d
    cmp eax, 10
    jae .d2
    mov byte [rdi], ' '
    inc rdi
    add al, '0'
    mov [rdi], al
    inc rdi
    jmp .daydone
.d2:
    mov ecx, 10
    xor edx, edx
    div ecx
    add al, '0'
    mov [rdi], al
    add dl, '0'
    mov [rdi+1], dl
    add rdi, 2
.daydone:
    mov byte [rdi], ' '
    inc rdi
    ; recent?
    mov rax, [g_now_sec]
    sub rax, r12
    mov rcx, rax
    sar rcx, 63
    xor rax, rcx
    sub rax, rcx
    cmp rax, 15552000
    ja .year
    ; HH:MM
    mov eax, r8d
    mov ecx, 10
    xor edx, edx
    div ecx
    add al, '0'
    add dl, '0'
    mov [rdi], al
    mov [rdi+1], dl
    mov byte [rdi+2], ':'
    add rdi, 3
    mov eax, r13d
    mov ecx, 10
    xor edx, edx
    div ecx
    add al, '0'
    add dl, '0'
    mov [rdi], al
    mov [rdi+1], dl
    mov byte [rdi+2], 0
    jmp .out
.year:
    mov byte [rdi], ' '
    inc rdi
    mov rax, r9
    mov ecx, 1000
    xor edx, edx
    div ecx
    add al, '0'
    mov [rdi], al
    mov rax, rdx
    mov ecx, 100
    xor edx, edx
    div ecx
    add al, '0'
    mov [rdi+1], al
    mov rax, rdx
    mov ecx, 10
    xor edx, edx
    div ecx
    add al, '0'
    add dl, '0'
    mov [rdi+2], al
    mov [rdi+3], dl
    mov byte [rdi+4], 0
.out:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret

format_long:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    push r14
    push r15

    mov r12, [g_entries]
    mov r13, [g_entry_count]

    ; total blocks (512-byte units / 2 = 1K for GNU default)
    xor r14, r14
    xor rbx, rbx
.sum:
    cmp rbx, r13
    jae .sumdone
    mov rax, [r12 + rbx*8]
    mov rax, [rax + Entry.blocks]
    add r14, rax
    inc rbx
    jmp .sum
.sumdone:
    lea rsi, [total_prefix]
    call out_str
    mov rax, r14
    add rax, 1
    shr rax, 1
    mov rdi, rax
    call out_u64
    mov dil, 10
    call out_byte

    ; compute column widths: nlink, owner, group, size
    xor r8d, r8d                    ; w_nlink
    xor r9d, r9d                    ; w_owner
    xor r10d, r10d                  ; w_group
    xor r11d, r11d                  ; w_size
    xor rbx, rbx
.wlp:
    cmp rbx, r13
    jae .wdone
    mov r15, [r12 + rbx*8]

    mov edi, [r15 + Entry.nlink]
    lea rsi, [name_tmp]
    push r8
    push r9
    push r10
    push r11
    call u64_to_dec_buf
    pop r11
    pop r10
    pop r9
    pop r8
    cmp eax, r8d
    jbe .wn
    mov r8d, eax
.wn:
    ; owner name width
    mov edi, [r15 + Entry.uid]
    push r8
    push r9
    push r10
    push r11
    call uid_to_name
    mov rdi, rsi
    call strlen
    pop r11
    pop r10
    pop r9
    pop r8
    cmp eax, r9d
    jbe .wo
    mov r9d, eax
.wo:
    mov edi, [r15 + Entry.gid]
    push r8
    push r9
    push r10
    push r11
    call gid_to_name
    mov rdi, rsi
    call strlen
    pop r11
    pop r10
    pop r9
    pop r8
    cmp eax, r10d
    jbe .wg
    mov r10d, eax
.wg:
    mov eax, [g_opts]
    test eax, OPT_HUMAN | OPT_SI
    jz .rawsz
    mov rdi, [r15 + Entry.size]
    lea rsi, [human_buf]
    mov edx, 0
    test eax, OPT_SI
    jz .hu
    mov edx, 1
.hu:
    push r8
    push r9
    push r10
    push r11
    call human_size
    pop r11
    pop r10
    pop r9
    pop r8
    jmp .szc
.rawsz:
    mov rdi, [r15 + Entry.size]
    lea rsi, [name_tmp]
    push r8
    push r9
    push r10
    push r11
    call u64_to_dec_buf
    pop r11
    pop r10
    pop r9
    pop r8
.szc:
    cmp eax, r11d
    jbe .ws
    mov r11d, eax
.ws:
    inc rbx
    jmp .wlp
.wdone:
    ; save widths
    mov dword [w_nlink], r8d
    mov dword [w_owner], r9d
    mov dword [w_group], r10d
    mov dword [w_size], r11d

    xor rbx, rbx
.lines:
    cmp rbx, r13
    jae .done
    mov r15, [r12 + rbx*8]

    ; perms
    mov rdi, r15
    lea rsi, [perm_buf]
    call format_perms
    lea rsi, [perm_buf]
    call out_str
    mov dil, ' '
    call out_byte

    ; nlink right-aligned
    mov edi, [r15 + Entry.nlink]
    lea rsi, [name_tmp]
    call u64_to_dec_buf
    mov edx, eax
    mov ecx, [w_nlink]
    call out_pad
    mov edi, [r15 + Entry.nlink]
    call out_u64
    mov dil, ' '
    call out_byte

    ; owner
    mov eax, [g_opts]
    test eax, OPT_NO_OWNER
    jnz .skip_owner
    test eax, OPT_NUMERIC
    jnz .own_num
    mov edi, [r15 + Entry.uid]
    call uid_to_name
    push rsi
    mov rdi, rsi
    call strlen
    mov edx, eax
    mov ecx, [w_owner]
    pop rsi
    push rax
    mov r8d, 0x7575                 ; uu
    call meta_paint
    pop rax
    mov edx, eax
    mov ecx, [w_owner]
    call out_pad
    jmp .own_sp
.own_num:
    mov edi, [r15 + Entry.uid]
    lea rsi, [uid_buf]
    call u64_to_dec_buf
    mov edx, eax
    mov ecx, [w_owner]
    call out_pad
    mov edi, [r15 + Entry.uid]
    call out_u64
.own_sp:
    mov dil, ' '
    call out_byte
.skip_owner:

    ; group
    mov eax, [g_opts]
    test eax, OPT_NO_GROUP
    jnz .skip_group
    test eax, OPT_NUMERIC
    jnz .grp_num
    mov edi, [r15 + Entry.gid]
    call gid_to_name
    push rsi
    mov rdi, rsi
    call strlen
    mov edx, eax
    mov ecx, [w_group]
    pop rsi
    push rax
    mov r8d, 0x7567                 ; gu
    call meta_paint
    pop rax
    mov edx, eax
    mov ecx, [w_group]
    call out_pad
    jmp .grp_sp
.grp_num:
    mov edi, [r15 + Entry.gid]
    lea rsi, [gid_buf]
    call u64_to_dec_buf
    mov edx, eax
    mov ecx, [w_group]
    call out_pad
    mov edi, [r15 + Entry.gid]
    call out_u64
.grp_sp:
    mov dil, ' '
    call out_byte
.skip_group:

    ; size right-aligned
    mov eax, [g_opts]
    test eax, OPT_HUMAN | OPT_SI
    jz .szraw
    mov rdi, [r15 + Entry.size]
    lea rsi, [human_buf]
    xor edx, edx
    test eax, OPT_SI
    jz .szh
    mov edx, 1
.szh:
    call human_size
    mov edx, eax
    mov ecx, [w_size]
    call out_pad
    lea rsi, [human_buf]
    mov edx, eax
    ; human_size returned len in eax but we overwrote — recompute
    mov rdi, [r15 + Entry.size]
    lea rsi, [human_buf]
    xor edx, edx
    mov eax, [g_opts]
    test eax, OPT_SI
    jz .szh2
    mov edx, 1
.szh2:
    call human_size
    lea rsi, [human_buf]
    mov edx, eax
    call out_strn
    jmp .szdone
.szraw:
    mov rdi, [r15 + Entry.size]
    lea rsi, [name_tmp]
    call u64_to_dec_buf
    mov edx, eax
    mov ecx, [w_size]
    call out_pad
    mov rdi, [r15 + Entry.size]
    call out_u64
.szdone:
    mov dil, ' '
    call out_byte

    ; time (date chrome via F00_COLORS da=)
    mov rdi, [r15 + Entry.mtime_sec]
    call format_mtime
    lea rsi, [time_buf]
    mov r8d, 0x6164                 ; da
    call meta_paint
    mov dil, ' '
    call out_byte

    ; name
    mov rdi, r15
    call emit_name
    mov dil, 10
    call out_byte

    inc rbx
    jmp .lines
.done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    ret

section .bss
w_nlink: resd 1
w_owner: resd 1
w_group: resd 1
w_size:  resd 1

section .text

; ------------------------------------------------------------
; format_listing — dispatch
; ------------------------------------------------------------

; OSC 8 hyperlink helpers
hyperlink_open:
    push rbx
    lea rsi, [osc8_open]
    call out_str
    lea rsi, [file_scheme]
    call out_str
    mov rsi, [rbx + Entry.path]
    test rsi, rsi
    jnz .p
    mov rsi, [rbx + Entry.name]
.p:
    call out_str
    lea rsi, [osc8_mid]
    call out_str
    pop rbx
    ret

hyperlink_close:
    lea rsi, [osc8_close]
    jmp out_str

section .rodata
osc8_open: db 27, "]8;;", 0
file_scheme: db "file://", 0
osc8_mid: db 27, "\\", 0
osc8_close: db 27, "]8;;", 27, "\\", 0
section .text

format_listing:
    mov eax, [g_opts2]
    test eax, OPT2_JSON | OPT2_JSON_FULL
    jnz format_json
    test eax, OPT2_CSV
    jnz format_csv
    test eax, OPT2_TSV
    jnz format_tsv
    mov eax, [g_opts]
    test eax, OPT_TREE
    jnz format_tree
    test eax, OPT_LONG
    jnz format_long
    test eax, OPT_COMMA
    jnz format_commas
    test eax, OPT_ONE | OPT_ZERO
    jnz format_one_per_line
    cmp byte [g_tty], 0
    je format_one_per_line
    jmp format_columns
