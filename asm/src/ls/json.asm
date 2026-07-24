; f00-asm — JSON / CSV / TSV output (f00 -j / --json-full / --csv / --tsv)
BITS 64
DEFAULT REL
%include "syscalls.inc"

global format_json, format_csv, format_tsv
extern plugins_decorate_json
extern g_entries, g_entry_count, g_opts2, g_color, g_json_core, g_tty
extern out_byte, out_str, out_strn, out_u64, out_pad, out_spaces
extern git_status_str
extern u64_to_dec_buf, strlen
extern color_reset, color_path, color_num, color_dim, color_hdr, color_ok

section .rodata
j_open:         db "[", 0
j_close:        db "]", 10, 0
j_obj_o:        db "{", 0
j_obj_c:        db "}", 0
j_comma:        db ",", 0
j_colon:        db ":", 0
j_nl:           db 10, 0
j_sp:           db " ", 0
; pretty modern: one object per line under array
j_nl_ind:       db 10, "  ", 0
j_name:         db '"name"', 0
j_path:         db '"path"', 0
j_kind:         db '"kind"', 0
j_size:         db '"size"', 0
j_mode:         db '"mode"', 0
j_mode_oct:     db '"mode_octal"', 0
j_perm:         db '"permissions"', 0
j_ino:          db '"inode"', 0
j_nlink:        db '"nlink"', 0
j_blocks:       db '"blocks"', 0
j_uid:          db '"uid"', 0
j_gid:          db '"gid"', 0
j_mtime:        db '"modified_unix"', 0
j_git:          db '"git_status"', 0
j_depth:        db '"depth"', 0
j_target:       db '"symlink_target"', 0
j_null:         db "null", 0
k_file:         db '"file"', 0
k_dir:          db '"directory"', 0
k_lnk:          db '"symlink"', 0
k_other:        db '"other"', 0

; ANSI for machine formats (modern only)
c_key:          db 27, "[36m", 0       ; cyan keys
c_str:          db 27, "[32m", 0       ; green strings
c_num:          db 27, "[33m", 0       ; yellow numbers
c_pun:          db 27, "[2;37m", 0     ; dim punct
c_rst:          db 27, "[0m", 0
c_hdr:          db 27, "[1;36m", 0     ; bold cyan table header
j_nl2:          db 10, "  ", 0
j_nl4:          db 10, "    ", 0
j_comma_nl4:    db ",", 10, "    ", 0
j_close_obj_nl: db 10, "  }", 0
j_open_arr_nl:  db "[", 10, 0
j_close_arr_nl: db 10, "]", 10, 0

; table header labels (modern CSV view)
th_name:  db "name", 0
th_path:  db "path", 0
th_kind:  db "kind", 0
th_size:  db "size", 0
th_mode:  db "mode", 0
th_ino:   db "inode", 0
th_nlink: db "nlink", 0
th_blk:   db "blocks", 0
th_uid:   db "uid", 0
th_gid:   db "gid", 0
th_git:   db "git", 0
th_depth: db "depth", 0

csv_hdr:        db "name,path,kind,size,mode,inode,nlink,blocks,uid,gid,git_status,depth", 10, 0
tsv_hdr:        db "name", 9, "path", 9, "kind", 9, "size", 9, "mode", 9, "inode", 9
                db "nlink", 9, "blocks", 9, "uid", 9, "gid", 9, "git_status", 9, "depth", 10, 0

section .bss
numbuf:         resb 32
permbuf:        resb 16
tw:             resd 12             ; table column widths
tcell:          resb 64             ; scratch for numeric cell

section .text

; chrome when modern (!--core) and (color on OR TTY)
; Machine pipelines: use --core for plain JSON/CSV.
j_chrome:
    cmp dword [g_json_core], 0
    jne .no
    cmp byte [g_color], 0
    jne .yes
    cmp byte [g_tty], 0
    je .no
.yes:
    mov al, 1
    ret
.no:
    xor al, al
    ret

j_pun:                                  ; dim punctuation cstr
    call j_chrome
    test al, al
    jz .p
    push rsi
    lea rsi, [c_pun]
    call out_str
    pop rsi
    call out_str
    lea rsi, [c_rst]
    jmp out_str
.p: jmp out_str

j_key:                                  ; key cstr (quoted) + ": "
    call j_chrome
    test al, al
    jz .plain
    push rsi
    lea rsi, [c_key]
    call out_str
    pop rsi
    call out_str
    lea rsi, [c_rst]
    call out_str
    lea rsi, [c_pun]
    call out_str
    mov dil, ':'
    call out_byte
    lea rsi, [c_rst]
    call out_str
    lea rsi, [j_sp]
    jmp out_str
.plain:
    call out_str
    mov dil, ':'
    jmp out_byte

j_num_u64:                              ; yellow number rdi
    call j_chrome
    test al, al
    jz .p
    push rdi
    lea rsi, [c_num]
    call out_str
    pop rdi
    call out_u64
    lea rsi, [c_rst]
    jmp out_str
.p: jmp out_u64

; emit JSON string escaped (green when chrome)
json_str:
    ; rsi=ptr, rdx=len
    push rbx
    push r12
    push r13
    mov r12, rsi
    mov r13, rdx
    call j_chrome
    test al, al
    jz .q0
    lea rsi, [c_str]
    call out_str
.q0:
    mov dil, '"'
    call out_byte
    xor rbx, rbx
.lp:
    cmp rbx, r13
    jae .end
    mov al, [r12 + rbx]
    cmp al, '"'
    je .esc
    cmp al, '\'
    je .escb
    cmp al, 32
    jb .u
    mov dil, al
    call out_byte
    jmp .n
.esc:
    mov dil, '\'
    call out_byte
    mov dil, '"'
    call out_byte
    jmp .n
.escb:
    mov dil, '\'
    call out_byte
    mov dil, '\'
    call out_byte
    jmp .n
.u:
    ; \u00XX
    push rax
    lea rsi, [hex_u]
    call out_str
    pop rax
    mov ah, al
    shr al, 4
    call hexdig
    mov dil, al
    call out_byte
    mov al, ah
    and al, 15
    call hexdig
    mov dil, al
    call out_byte
.n:
    inc rbx
    jmp .lp
.end:
    mov dil, '"'
    call out_byte
    call j_chrome
    test al, al
    jz .done
    lea rsi, [c_rst]
    call out_str
.done:
    pop r13
    pop r12
    pop rbx
    ret

hexdig:
    cmp al, 10
    jb .d
    add al, 'a' - 10
    ret
.d:
    add al, '0'
    ret

section .rodata
hex_u: db "\u00", 0

section .text

kind_str:
    ; rdi=Entry* → rsi static json kind
    test byte [rdi + Entry.flags], EF_DIR
    jnz .d
    cmp byte [rdi + Entry.dtype], DT_DIR
    je .d
    test byte [rdi + Entry.flags], EF_LNK
    jnz .l
    cmp byte [rdi + Entry.dtype], DT_LNK
    je .l
    cmp byte [rdi + Entry.dtype], DT_REG
    je .f
    lea rsi, [k_other]
    ret
.d: lea rsi, [k_dir]
    ret
.l: lea rsi, [k_lnk]
    ret
.f: lea rsi, [k_file]
    ret

format_perms_simple:
    ; rdi=Entry*, rsi=buf
    extern format_perms_into
    jmp format_perms_into

; j_field_sep — comma between fields (pretty: ",\n    ")
j_field_sep:
    call j_chrome
    test al, al
    jz .c
    lea rsi, [j_comma_nl4]
    jmp j_pun
.c: lea rsi, [j_comma]
    jmp j_pun

; j_obj_begin — "{" or "{\n    "
j_obj_begin:
    call j_chrome
    test al, al
    jz .c
    lea rsi, [j_obj_o]
    call j_pun
    lea rsi, [j_nl4]
    jmp out_str
.c: lea rsi, [j_obj_o]
    jmp j_pun

; j_obj_end — "}" or "\n  }"
j_obj_end:
    call j_chrome
    test al, al
    jz .c
    lea rsi, [j_close_obj_nl]
    jmp j_pun
.c: lea rsi, [j_obj_c]
    jmp j_pun

format_json:
    push rbx
    push r12
    push r13
    call j_chrome
    test al, al
    jz .flat_open
    lea rsi, [j_open_arr_nl]
    call j_pun
    jmp .start
.flat_open:
    lea rsi, [j_open]
    call j_pun
.start:
    mov r12, [g_entries]
    xor rbx, rbx
.lp:
    cmp rbx, [g_entry_count]
    jae .end
    test rbx, rbx
    jz .obj
    lea rsi, [j_comma]
    call j_pun
    call j_chrome
    test al, al
    jz .obj
    lea rsi, [j_nl2]
    call out_str
.obj:
    call j_chrome
    test al, al
    jz .brace
    ; pretty: first needs "  " (after "[\n"); later already have "  " from j_nl2
    test rbx, rbx
    jnz .brace_only
    mov dil, ' '
    call out_byte
    mov dil, ' '
    call out_byte
.brace_only:
    lea rsi, [c_pun]
    call out_str
    mov dil, '{'
    call out_byte
    lea rsi, [c_rst]
    call out_str
    lea rsi, [j_nl4]
    call out_str
    jmp .fields
.brace:
    lea rsi, [j_obj_o]
    call j_pun
.fields:
    mov r13, [r12 + rbx*8]

    lea rsi, [j_name]
    call j_key
    mov rsi, [r13 + Entry.name]
    movzx edx, word [r13 + Entry.namelen]
    call json_str
    call j_field_sep

    lea rsi, [j_path]
    call j_key
    mov rsi, [r13 + Entry.path]
    test rsi, rsi
    jnz .ph
    mov rsi, [r13 + Entry.name]
.ph:
    mov rdi, rsi
    call strlen
    mov rdx, rax
    call json_str
    call j_field_sep

    lea rsi, [j_kind]
    call j_key
    mov rdi, r13
    call kind_str
    call j_chrome
    test al, al
    jz .kplain
    push rsi
    lea rsi, [c_str]
    call out_str
    pop rsi
    call out_str
    lea rsi, [c_rst]
    call out_str
    jmp .kdone
.kplain:
    call out_str
.kdone:
    call j_field_sep

    lea rsi, [j_size]
    call j_key
    mov rdi, [r13 + Entry.size]
    call j_num_u64
    call j_field_sep

    lea rsi, [j_mode_oct]
    call j_key
    call j_chrome
    test al, al
    jz .mq
    lea rsi, [c_str]
    call out_str
.mq:
    mov dil, '"'
    call out_byte
    mov edi, [r13 + Entry.mode]
    call out_oct
    mov dil, '"'
    call out_byte
    call j_chrome
    test al, al
    jz .mq2
    lea rsi, [c_rst]
    call out_str
.mq2:
    call j_field_sep

    lea rsi, [j_ino]
    call j_key
    mov rdi, [r13 + Entry.ino]
    call j_num_u64
    call j_field_sep

    lea rsi, [j_nlink]
    call j_key
    mov edi, [r13 + Entry.nlink]
    call j_num_u64
    call j_field_sep

    lea rsi, [j_blocks]
    call j_key
    mov rdi, [r13 + Entry.blocks]
    call j_num_u64
    call j_field_sep

    lea rsi, [j_uid]
    call j_key
    mov edi, [r13 + Entry.uid]
    call j_num_u64
    call j_field_sep

    lea rsi, [j_gid]
    call j_key
    mov edi, [r13 + Entry.gid]
    call j_num_u64
    call j_field_sep

    lea rsi, [j_mtime]
    call j_key
    mov rdi, [r13 + Entry.mtime_sec]
    call j_num_u64
    call j_field_sep

    lea rsi, [j_git]
    call j_key
    mov dil, [r13 + Entry.git]
    call git_status_str
    mov rdi, rsi
    call strlen
    mov rdx, rax
    call json_str
    call j_field_sep

    lea rsi, [j_depth]
    call j_key
    movzx edi, byte [r13 + Entry.depth]
    call j_num_u64

    mov eax, [g_opts2]
    test eax, OPT2_JSON_FULL
    jz .close_obj
    call j_field_sep
    lea rsi, [j_target]
    call j_key
    mov rsi, [r13 + Entry.target]
    test rsi, rsi
    jz .tnull
    mov edi, [r13 + Entry.targetlen]
    mov edx, edi
    call json_str
    jmp .close_obj
.tnull:
    call j_chrome
    test al, al
    jz .tn
    lea rsi, [c_num]
    call out_str
    lea rsi, [j_null]
    call out_str
    lea rsi, [c_rst]
    call out_str
    jmp .close_obj
.tn:
    lea rsi, [j_null]
    call out_str

.close_obj:
    call j_obj_end
    inc rbx
    jmp .lp
.end:
    call j_chrome
    test al, al
    jz .ec
    lea rsi, [j_close_arr_nl]
    call j_pun
    jmp .done
.ec:
    lea rsi, [j_close]
    call j_pun
.done:
    pop r13
    pop r12
    pop rbx
    ret

out_oct:
    ; edi = value, print octal without leading zeros (or 0)
    push rbx
    mov eax, edi
    lea rsi, [numbuf + 16]
    mov byte [rsi], 0
    test eax, eax
    jnz .lp
    dec rsi
    mov byte [rsi], '0'
    jmp .emit
.lp:
    mov ecx, eax
    and ecx, 7
    add cl, '0'
    dec rsi
    mov [rsi], cl
    shr eax, 3
    jnz .lp
.emit:
    call out_str
    pop rbx
    ret

format_csv:
    call j_chrome
    test al, al
    jnz format_table_view          ; modern → pretty table
    push rbx
    push r12
    lea rsi, [csv_hdr]
    call out_str
    mov r12, [g_entries]
    xor rbx, rbx
.lp:
    cmp rbx, [g_entry_count]
    jae .done
    mov rdi, [r12 + rbx*8]
    mov sil, ','
    call emit_row
    inc rbx
    jmp .lp
.done:
    pop r12
    pop rbx
    ret

format_tsv:
    call j_chrome
    test al, al
    jnz format_table_view
    push rbx
    push r12
    lea rsi, [tsv_hdr]
    call out_str
    mov r12, [g_entries]
    xor rbx, rbx
.lp:
    cmp rbx, [g_entry_count]
    jae .done
    mov rdi, [r12 + rbx*8]
    mov sil, 9
    call emit_row
    inc rbx
    jmp .lp
.done:
    pop r12
    pop rbx
    ret

; ── modern CSV/TSV: aligned colored table ─────────────────────────
; columns: name path kind size mode inode nlink blocks uid gid git depth
%define NCOL 12

; col_max(eax=col, ecx=len) — update tw[col] if larger
col_max:
    cmp ecx, [tw + rax*4]
    jbe .r
    mov [tw + rax*4], ecx
.r: ret

; kind_raw(rdi=Entry*) → rsi cstr
kind_raw:
    test byte [rdi + Entry.flags], EF_DIR
    jnz .d
    cmp byte [rdi + Entry.dtype], DT_DIR
    je .d
    test byte [rdi + Entry.flags], EF_LNK
    jnz .l
    cmp byte [rdi + Entry.dtype], DT_LNK
    je .l
    lea rsi, [raw_file]
    ret
.d: lea rsi, [raw_dir]
    ret
.l: lea rsi, [raw_lnk]
    ret

format_table_view:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    push r14
    push r15
    ; zero widths, seed with header label lengths
    xor ecx, ecx
.zw:
    mov dword [tw + rcx*4], 0
    inc ecx
    cmp ecx, NCOL
    jb .zw
    lea rsi, [th_name]
    call strlen
    mov eax, 0
    mov ecx, eax
    xchg eax, ecx
    ; manual seed headers
    mov dword [tw+0*4], 4    ; name
    mov dword [tw+1*4], 4    ; path
    mov dword [tw+2*4], 4    ; kind (min "file")
    mov dword [tw+3*4], 4    ; size
    mov dword [tw+4*4], 4    ; mode
    mov dword [tw+5*4], 5    ; inode
    mov dword [tw+6*4], 5    ; nlink
    mov dword [tw+7*4], 6    ; blocks
    mov dword [tw+8*4], 3    ; uid
    mov dword [tw+9*4], 3    ; gid
    mov dword [tw+10*4], 3   ; git
    mov dword [tw+11*4], 5   ; depth

    mov r12, [g_entries]
    mov r13, [g_entry_count]
    xor rbx, rbx
.wlp:
    cmp rbx, r13
    jae .whdr
    mov r14, [r12 + rbx*8]
    ; 0 name
    movzx ecx, word [r14 + Entry.namelen]
    mov eax, 0
    call col_max
    ; 1 path
    mov rsi, [r14 + Entry.path]
    test rsi, rsi
    jnz .wp
    mov rsi, [r14 + Entry.name]
.wp: mov rdi, rsi
    call strlen
    mov ecx, eax
    mov eax, 1
    call col_max
    ; 2 kind
    mov rdi, r14
    call kind_raw
    mov rdi, rsi
    call strlen
    mov ecx, eax
    mov eax, 2
    call col_max
    ; 3 size
    mov rdi, [r14 + Entry.size]
    lea rsi, [tcell]
    call u64_to_dec_buf
    mov ecx, eax
    mov eax, 3
    call col_max
    ; 4 mode octal as string
    mov edi, [r14 + Entry.mode]
    push rbx
    call mode_oct_len
    pop rbx
    mov ecx, eax
    mov eax, 4
    call col_max
    ; 5 ino
    mov rdi, [r14 + Entry.ino]
    lea rsi, [tcell]
    call u64_to_dec_buf
    mov ecx, eax
    mov eax, 5
    call col_max
    ; 6 nlink
    mov edi, [r14 + Entry.nlink]
    lea rsi, [tcell]
    call u64_to_dec_buf
    mov ecx, eax
    mov eax, 6
    call col_max
    ; 7 blocks
    mov rdi, [r14 + Entry.blocks]
    lea rsi, [tcell]
    call u64_to_dec_buf
    mov ecx, eax
    mov eax, 7
    call col_max
    ; 8 uid
    mov edi, [r14 + Entry.uid]
    lea rsi, [tcell]
    call u64_to_dec_buf
    mov ecx, eax
    mov eax, 8
    call col_max
    ; 9 gid
    mov edi, [r14 + Entry.gid]
    lea rsi, [tcell]
    call u64_to_dec_buf
    mov ecx, eax
    mov eax, 9
    call col_max
    ; 10 git
    mov dil, [r14 + Entry.git]
    call git_status_str
    mov rdi, rsi
    call strlen
    mov ecx, eax
    mov eax, 10
    call col_max
    ; 11 depth
    movzx edi, byte [r14 + Entry.depth]
    lea rsi, [tcell]
    call u64_to_dec_buf
    mov ecx, eax
    mov eax, 11
    call col_max
    inc rbx
    jmp .wlp

.whdr:
    ; header row
    lea rsi, [c_hdr]
    call out_str
    lea rsi, [th_name]
    mov eax, 0
    call t_emit_left
    lea rsi, [th_path]
    mov eax, 1
    call t_emit_left
    lea rsi, [th_kind]
    mov eax, 2
    call t_emit_left
    lea rsi, [th_size]
    mov eax, 3
    call t_emit_right_lab
    lea rsi, [th_mode]
    mov eax, 4
    call t_emit_right_lab
    lea rsi, [th_ino]
    mov eax, 5
    call t_emit_right_lab
    lea rsi, [th_nlink]
    mov eax, 6
    call t_emit_right_lab
    lea rsi, [th_blk]
    mov eax, 7
    call t_emit_right_lab
    lea rsi, [th_uid]
    mov eax, 8
    call t_emit_right_lab
    lea rsi, [th_gid]
    mov eax, 9
    call t_emit_right_lab
    lea rsi, [th_git]
    mov eax, 10
    call t_emit_left
    lea rsi, [th_depth]
    mov eax, 11
    call t_emit_right_lab
    lea rsi, [c_rst]
    call out_str
    mov dil, 10
    call out_byte

    ; rows
    xor rbx, rbx
.rlp:
    cmp rbx, r13
    jae .rdone
    mov r14, [r12 + rbx*8]
    ; name (path color)
    lea rsi, [c_str]
    call out_str
    mov rsi, [r14 + Entry.name]
    movzx edx, word [r14 + Entry.namelen]
    mov eax, 0
    call t_emit_strn_left
    lea rsi, [c_rst]
    call out_str
    ; path
    mov rsi, [r14 + Entry.path]
    test rsi, rsi
    jnz .rp
    mov rsi, [r14 + Entry.name]
.rp: mov rdi, rsi
    call strlen
    mov edx, eax
    mov eax, 1
    call t_emit_strn_left
    ; kind
    mov rdi, r14
    call kind_raw
    mov rdi, rsi
    call strlen
    mov edx, eax
    mov eax, 2
    call t_emit_strn_left
    ; size num
    lea rsi, [c_num]
    call out_str
    mov rdi, [r14 + Entry.size]
    mov eax, 3
    call t_emit_u64_right
    lea rsi, [c_rst]
    call out_str
    ; mode
    mov edi, [r14 + Entry.mode]
    mov eax, 4
    call t_emit_oct_right
    ; ino
    lea rsi, [c_num]
    call out_str
    mov rdi, [r14 + Entry.ino]
    mov eax, 5
    call t_emit_u64_right
    lea rsi, [c_rst]
    call out_str
    mov edi, [r14 + Entry.nlink]
    mov eax, 6
    call t_emit_u64_right
    mov rdi, [r14 + Entry.blocks]
    mov eax, 7
    call t_emit_u64_right
    mov edi, [r14 + Entry.uid]
    mov eax, 8
    call t_emit_u64_right
    mov edi, [r14 + Entry.gid]
    mov eax, 9
    call t_emit_u64_right
    mov dil, [r14 + Entry.git]
    call git_status_str
    mov rdi, rsi
    call strlen
    mov edx, eax
    mov eax, 10
    call t_emit_strn_left
    movzx edi, byte [r14 + Entry.depth]
    mov eax, 11
    call t_emit_u64_right
    mov dil, 10
    call out_byte
    inc rbx
    jmp .rlp
.rdone:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    ret

; t_emit_strn_left(rsi, edx=len, eax=col) — left text + pad + 2-space gutter
t_emit_strn_left:
    push rbx
    push r12
    mov ebx, eax
    mov r12d, edx
    call out_strn
    mov ecx, [tw + rbx*4]
    sub ecx, r12d
    jle .g
.sp: mov dil, ' '
    call out_byte
    dec ecx
    jg .sp
.g:  mov dil, ' '
    call out_byte
    mov dil, ' '
    call out_byte
    pop r12
    pop rbx
    ret

; t_emit_left(rsi=cstr, eax=col)
t_emit_left:
    push rbx
    push r12
    mov ebx, eax
    mov r12, rsi
    mov rdi, rsi
    call strlen
    mov edx, eax
    mov rsi, r12
    mov eax, ebx
    call t_emit_strn_left
    pop r12
    pop rbx
    ret

; t_emit_right_lab(rsi=cstr, eax=col) — right-align label in column
t_emit_right_lab:
    push rbx
    push r12
    push r13
    mov ebx, eax
    mov r12, rsi
    mov rdi, rsi
    call strlen
    mov r13d, eax
    mov ecx, [tw + rbx*4]
    mov edx, r13d
    call out_pad
    mov rsi, r12
    mov edx, r13d
    call out_strn
    mov dil, ' '
    call out_byte
    mov dil, ' '
    call out_byte
    pop r13
    pop r12
    pop rbx
    ret

mode_oct_len:
    push rbx
    mov eax, edi
    xor ebx, ebx
    test eax, eax
    jnz .lp
    mov eax, 1
    pop rbx
    ret
.lp:
    inc ebx
    shr eax, 3
    jnz .lp
    mov eax, ebx
    pop rbx
    ret

; t_emit_u64_right(rdi=u64, eax=col)
t_emit_u64_right:
    push rbx
    push r12
    push r13
    mov r12d, eax
    lea rsi, [tcell]
    call u64_to_dec_buf
    mov r13d, eax
    mov edx, eax
    mov ecx, [tw + r12*4]
    call out_pad
    lea rsi, [tcell]
    mov edx, r13d
    call out_strn
    mov dil, ' '
    call out_byte
    mov dil, ' '
    call out_byte
    pop r13
    pop r12
    pop rbx
    ret

; t_emit_oct_right(edi=mode, eax=col)
t_emit_oct_right:
    push rbx
    push r12
    push r13
    mov r12d, eax
    mov r13d, edi
    call mode_oct_len
    mov edx, eax
    mov ecx, [tw + r12*4]
    call out_pad
    mov edi, r13d
    call out_oct
    mov dil, ' '
    call out_byte
    mov dil, ' '
    call out_byte
    pop r13
    pop r12
    pop rbx
    ret

; emit_row(rdi=Entry*, sil=sep)
emit_row:
    push rbx
    push r12
    push r13
    mov r12, rdi
    mov r13b, sil
    ; name
    mov rsi, [r12 + Entry.name]
    movzx edx, word [r12 + Entry.namelen]
    call out_strn
    mov dil, r13b
    call out_byte
    ; path
    mov rsi, [r12 + Entry.path]
    test rsi, rsi
    jnz .p
    mov rsi, [r12 + Entry.name]
.p:
    call out_str
    mov dil, r13b
    call out_byte
    ; kind
    mov rdi, r12
    call kind_str
    ; strip quotes from kind_str json form - emit raw
    test byte [r12 + Entry.flags], EF_DIR
    jnz .kd
    cmp byte [r12 + Entry.dtype], DT_DIR
    je .kd
    test byte [r12 + Entry.flags], EF_LNK
    jnz .kl
    cmp byte [r12 + Entry.dtype], DT_LNK
    je .kl
    lea rsi, [raw_file]
    jmp .ke
.kd: lea rsi, [raw_dir]
    jmp .ke
.kl: lea rsi, [raw_lnk]
.ke:
    call out_str
    mov dil, r13b
    call out_byte
    mov rdi, [r12 + Entry.size]
    call out_u64
    mov dil, r13b
    call out_byte
    mov edi, [r12 + Entry.mode]
    call out_oct
    mov dil, r13b
    call out_byte
    mov rdi, [r12 + Entry.ino]
    call out_u64
    mov dil, r13b
    call out_byte
    mov edi, [r12 + Entry.nlink]
    call out_u64
    mov dil, r13b
    call out_byte
    mov rdi, [r12 + Entry.blocks]
    call out_u64
    mov dil, r13b
    call out_byte
    mov edi, [r12 + Entry.uid]
    call out_u64
    mov dil, r13b
    call out_byte
    mov edi, [r12 + Entry.gid]
    call out_u64
    mov dil, r13b
    call out_byte
    mov dil, [r12 + Entry.git]
    call git_status_str
    call out_str
    mov dil, r13b
    call out_byte
    movzx edi, byte [r12 + Entry.depth]
    call out_u64
    mov dil, 10
    call out_byte
    pop r13
    pop r12
    pop rbx
    ret

section .rodata
raw_file: db "file", 0
raw_dir:  db "directory", 0
raw_lnk:  db "symlink", 0
