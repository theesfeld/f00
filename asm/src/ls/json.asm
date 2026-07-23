; f00-asm — JSON / CSV / TSV output (f00 -j / --json-full / --csv / --tsv)
BITS 64
DEFAULT REL
%include "syscalls.inc"

global format_json, format_csv, format_tsv
extern plugins_decorate_json
extern g_entries, g_entry_count, g_opts2
extern out_byte, out_str, out_strn, out_u64
extern git_status_str
extern u64_to_dec_buf, strlen

section .rodata
j_open:         db "[", 0
j_close:        db "]", 10, 0
j_obj_o:        db "{", 0
j_obj_c:        db "}", 0
j_comma:        db ",", 0
j_colon:        db ":", 0
j_nl:           db 10, 0
j_name:         db '"name":', 0
j_path:         db '"path":', 0
j_kind:         db '"kind":', 0
j_size:         db '"size":', 0
j_mode:         db '"mode":', 0
j_mode_oct:     db '"mode_octal":', 0
j_perm:         db '"permissions":', 0
j_ino:          db '"inode":', 0
j_nlink:        db '"nlink":', 0
j_blocks:       db '"blocks":', 0
j_uid:          db '"uid":', 0
j_gid:          db '"gid":', 0
j_mtime:        db '"modified_unix":', 0
j_git:          db '"git_status":', 0
j_depth:        db '"depth":', 0
j_target:       db '"symlink_target":', 0
j_null:         db "null", 0
k_file:         db '"file"', 0
k_dir:          db '"directory"', 0
k_lnk:          db '"symlink"', 0
k_other:        db '"other"', 0

csv_hdr:        db "name,path,kind,size,mode,inode,nlink,blocks,uid,gid,git_status,depth", 10, 0
tsv_hdr:        db "name", 9, "path", 9, "kind", 9, "size", 9, "mode", 9, "inode", 9
                db "nlink", 9, "blocks", 9, "uid", 9, "gid", 9, "git_status", 9, "depth", 10, 0

section .bss
numbuf:         resb 32
permbuf:        resb 16

section .text

; emit JSON string escaped
json_str:
    ; rsi=ptr, rdx=len
    push rbx
    push r12
    push r13
    mov r12, rsi
    mov r13, rdx
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
    call out_str
    mov r12, [g_entries]
    xor rbx, rbx
.lp:
    cmp rbx, [g_entry_count]
    jae .end
    test rbx, rbx
    jz .obj
    lea rsi, [j_comma]
    call out_str
.obj:
    lea rsi, [j_obj_o]
    call out_str
    mov r13, [r12 + rbx*8]

    lea rsi, [j_name]
    call out_str
    mov rsi, [r13 + Entry.name]
    movzx edx, word [r13 + Entry.namelen]
    call json_str
    lea rsi, [j_comma]
    call out_str

    lea rsi, [j_path]
    call out_str
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
    call out_str

    lea rsi, [j_kind]
    call out_str
    mov rdi, r13
    call kind_str
    call out_str
    lea rsi, [j_comma]
    call out_str

    lea rsi, [j_size]
    call out_str
    mov rdi, [r13 + Entry.size]
    call out_u64
    lea rsi, [j_comma]
    call out_str

    lea rsi, [j_mode_oct]
    call out_str
    mov dil, '"'
    call out_byte
    mov edi, [r13 + Entry.mode]
    call out_oct
    mov dil, '"'
    call out_byte
    lea rsi, [j_comma]
    call out_str

    lea rsi, [j_ino]
    call out_str
    mov rdi, [r13 + Entry.ino]
    call out_u64
    lea rsi, [j_comma]
    call out_str

    lea rsi, [j_nlink]
    call out_str
    mov edi, [r13 + Entry.nlink]
    call out_u64
    lea rsi, [j_comma]
    call out_str

    lea rsi, [j_blocks]
    call out_str
    mov rdi, [r13 + Entry.blocks]
    call out_u64
    lea rsi, [j_comma]
    call out_str

    lea rsi, [j_uid]
    call out_str
    mov edi, [r13 + Entry.uid]
    call out_u64
    lea rsi, [j_comma]
    call out_str

    lea rsi, [j_gid]
    call out_str
    mov edi, [r13 + Entry.gid]
    call out_u64
    lea rsi, [j_comma]
    call out_str

    lea rsi, [j_mtime]
    call out_str
    mov rdi, [r13 + Entry.mtime_sec]
    call out_u64
    lea rsi, [j_comma]
    call out_str

    lea rsi, [j_git]
    call out_str
    mov dil, [r13 + Entry.git]
    call git_status_str
    ; quote the status string
    mov rdi, rsi
    call strlen
    mov rdx, rax
    call json_str
    lea rsi, [j_comma]
    call out_str

    lea rsi, [j_depth]
    call out_str
    movzx edi, byte [r13 + Entry.depth]
    call out_u64

    ; full json extras
    mov eax, [g_opts2]
    test eax, OPT2_JSON_FULL
    jz .close_obj
    lea rsi, [j_comma]
    call out_str
    lea rsi, [j_target]
    call out_str
    mov rsi, [r13 + Entry.target]
    test rsi, rsi
    jz .tnull
    mov edi, [r13 + Entry.targetlen]
    mov edx, edi
    call json_str
    jmp .close_obj
.tnull:
    lea rsi, [j_null]
    call out_str

.close_obj:
    lea rsi, [j_obj_c]
    call out_str
    inc rbx
    jmp .lp
.end:
    lea rsi, [j_close]
    call out_str
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
