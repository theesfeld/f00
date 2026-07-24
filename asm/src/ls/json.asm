; f00-asm — JSON / CSV / TSV output (f00 -j / --json-full / --csv / --tsv)
BITS 64
DEFAULT REL
%include "syscalls.inc"

global format_json, format_csv, format_tsv
extern plugins_decorate_json
extern g_entries, g_entry_count, g_opts2, g_color, g_json_core
extern out_byte, out_str, out_strn, out_u64
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

csv_hdr:        db "name,path,kind,size,mode,inode,nlink,blocks,uid,gid,git_status,depth", 10, 0
tsv_hdr:        db "name", 9, "path", 9, "kind", 9, "size", 9, "mode", 9, "inode", 9
                db "nlink", 9, "blocks", 9, "uid", 9, "gid", 9, "git_status", 9, "depth", 10, 0

section .bss
numbuf:         resb 32
permbuf:        resb 16

section .text

; chrome when modern color && not --core
j_chrome:
    cmp byte [g_color], 0
    je .no
    cmp dword [g_json_core], 0
    jne .no
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

format_json:
    push rbx
    push r12
    push r13
    lea rsi, [j_open]
    call j_pun
    mov r12, [g_entries]
    xor rbx, rbx
.lp:
    cmp rbx, [g_entry_count]
    jae .end
    test rbx, rbx
    jz .nl0
    lea rsi, [j_comma]
    call j_pun
.nl0:
    ; pretty: newline + indent between objects when chrome
    call j_chrome
    test al, al
    jz .obj
    lea rsi, [j_nl_ind]
    call out_str
.obj:
    lea rsi, [j_obj_o]
    call j_pun
    mov r13, [r12 + rbx*8]

    lea rsi, [j_name]
    call j_key
    mov rsi, [r13 + Entry.name]
    movzx edx, word [r13 + Entry.namelen]
    call json_str
    lea rsi, [j_comma]
    call j_pun

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
    lea rsi, [j_comma]
    call j_pun

    lea rsi, [j_kind]
    call j_key
    mov rdi, r13
    call kind_str
    ; kind already quoted; color as string
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
    lea rsi, [j_comma]
    call j_pun

    lea rsi, [j_size]
    call j_key
    mov rdi, [r13 + Entry.size]
    call j_num_u64
    lea rsi, [j_comma]
    call j_pun

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
    lea rsi, [j_comma]
    call j_pun

    lea rsi, [j_ino]
    call j_key
    mov rdi, [r13 + Entry.ino]
    call j_num_u64
    lea rsi, [j_comma]
    call j_pun

    lea rsi, [j_nlink]
    call j_key
    mov edi, [r13 + Entry.nlink]
    call j_num_u64
    lea rsi, [j_comma]
    call j_pun

    lea rsi, [j_blocks]
    call j_key
    mov rdi, [r13 + Entry.blocks]
    call j_num_u64
    lea rsi, [j_comma]
    call j_pun

    lea rsi, [j_uid]
    call j_key
    mov edi, [r13 + Entry.uid]
    call j_num_u64
    lea rsi, [j_comma]
    call j_pun

    lea rsi, [j_gid]
    call j_key
    mov edi, [r13 + Entry.gid]
    call j_num_u64
    lea rsi, [j_comma]
    call j_pun

    lea rsi, [j_mtime]
    call j_key
    mov rdi, [r13 + Entry.mtime_sec]
    call j_num_u64
    lea rsi, [j_comma]
    call j_pun

    lea rsi, [j_git]
    call j_key
    mov dil, [r13 + Entry.git]
    call git_status_str
    mov rdi, rsi
    call strlen
    mov rdx, rax
    call json_str
    lea rsi, [j_comma]
    call j_pun

    lea rsi, [j_depth]
    call j_key
    movzx edi, byte [r13 + Entry.depth]
    call j_num_u64

    mov eax, [g_opts2]
    test eax, OPT2_JSON_FULL
    jz .close_obj
    lea rsi, [j_comma]
    call j_pun
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
    lea rsi, [j_obj_c]
    call j_pun
    inc rbx
    jmp .lp
.end:
    call j_chrome
    test al, al
    jz .ec
    mov dil, 10
    call out_byte
.ec:
    lea rsi, [j_close]
    call j_pun
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
    push rbx
    push r12
    ; modern: cyan header row
    call j_chrome
    test al, al
    jz .hdr
    lea rsi, [c_key]
    call out_str
.hdr:
    lea rsi, [csv_hdr]
    call out_str
    call j_chrome
    test al, al
    jz .body
    lea rsi, [c_rst]
    call out_str
.body:
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
    push rbx
    push r12
    call j_chrome
    test al, al
    jz .hdr
    lea rsi, [c_key]
    call out_str
.hdr:
    lea rsi, [tsv_hdr]
    call out_str
    call j_chrome
    test al, al
    jz .body
    lea rsi, [c_rst]
    call out_str
.body:
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
