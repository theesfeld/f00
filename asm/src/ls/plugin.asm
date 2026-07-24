; f00-asm — plugin host: discover & list .so plugins (ABI v1)
; Full in-process ELF relocate is optional; we record plugins and pass-through decorate.
BITS 64
DEFAULT REL
%include "syscalls.inc"

global plugins_init, plugins_list, plugins_decorate_json
extern arena_alloc, strlen
extern out_str, out_byte, out_flush
extern g_envp

%define MAX_PLUGINS 32

section .bss
alignb 8
plug_n:    resq 1
plug_path: resq MAX_PLUGINS
plug_name: resq MAX_PLUGINS
dir_ents:  resb 65536
name_buf:  resb 1024
home_buf:  resb 512

section .rodata
msg_hdr:  db "plugins:",10,0
msg_none: db "(no plugins loaded)",10,0
; XDG only — never ~/.f00
suf_xdg:  db "/f00/plugins",0
suf_home: db "/.config/f00/plugins",0
env_xdg:  db "XDG_CONFIG_HOME=",0
env_home: db "HOME=",0
env_key:  db "F00_PLUGIN_DIR=",0
slash:    db "/",0

section .text

; Plugin search (XDG Base Directory):
;   1) $XDG_CONFIG_HOME/f00/plugins
;   2) $HOME/.config/f00/plugins
;   3) $F00_PLUGIN_DIR
plugins_init:
    push r12
    mov r12, rdi                    ; envp
    mov [g_envp], r12
    mov qword [plug_n], 0
    ; 1) XDG_CONFIG_HOME/f00/plugins
    lea rsi, [env_xdg]
    mov ecx, 16                     ; len "XDG_CONFIG_HOME="
    call env_get_pfx
    test rax, rax
    jz .home_cfg
    lea rdi, [home_buf]
    mov rsi, rax
    call strcpy
    lea rdi, [home_buf]
    lea rsi, [suf_xdg]
    call strcat
    lea rdi, [home_buf]
    call scan_dir
.home_cfg:
    ; 2) $HOME/.config/f00/plugins
    lea rsi, [env_home]
    mov ecx, 5
    call env_get_pfx
    test rax, rax
    jz .env
    lea rdi, [home_buf]
    mov rsi, rax
    call strcpy
    lea rdi, [home_buf]
    lea rsi, [suf_home]
    call strcat
    lea rdi, [home_buf]
    call scan_dir
.env:
    ; 3) F00_PLUGIN_DIR
    call get_plugin_dir
    test rax, rax
    jz .done
    mov rdi, rax
    call scan_dir
.done:
    pop r12
    ret

; env_get_pfx(rsi=prefix cstr, ecx=prefix len) → rax value or 0
env_get_pfx:
    push rbx
    push r12
    push r13
    mov r12, rsi
    mov r13d, ecx
    mov rdi, [g_envp]
    test rdi, rdi
    jz .no
.lp:
    mov rbx, [rdi]
    test rbx, rbx
    jz .no
    push rdi
    mov rdi, rbx
    mov rsi, r12
    mov ecx, r13d
    call pfx
    pop rdi
    test al, al
    jnz .y
    add rdi, 8
    jmp .lp
.y: mov rsi, [rdi]
    mov eax, r13d
    lea rax, [rsi+rax]
    pop r13
    pop r12
    pop rbx
    ret
.no:
    xor eax, eax
    pop r13
    pop r12
    pop rbx
    ret

get_plugin_dir:
    lea rsi, [env_key]
    mov ecx, 15                     ; "F00_PLUGIN_DIR="
    call env_get_pfx
    ret

pfx:
.lp:
    test ecx, ecx
    jz .y
    mov al, [rdi]
    cmp al, [rsi]
    jne .n
    inc rdi
    inc rsi
    dec ecx
    jmp .lp
.y: mov al, 1
    ret
.n: xor al, al
    ret

scan_dir:
    push rbx
    push r12
    push r13
    mov r12, rdi
    mov rax, SYS_openat
    mov rdi, AT_FDCWD
    mov rsi, r12
    mov rdx, O_RDONLY|O_DIRECTORY|O_CLOEXEC
    xor r10, r10
    syscall
    cmp rax, -4096
    jae .out
    mov r13, rax
.rd:
    mov rax, SYS_getdents64
    mov rdi, r13
    lea rsi, [dir_ents]
    mov rdx, 65536
    syscall
    test rax, rax
    jle .cl
    mov r8, rax
    xor ebx, ebx
.dent:
    cmp rbx, r8
    jae .rd
    lea r9, [dir_ents+rbx]
    movzx r10d, word [r9+16]        ; reclen (keep in r10)
    test r10d, r10d
    jz .cl
    lea r11, [r9+19]                ; d_name
    ; skip . and ..
    cmp byte [r11], '.'
    jne .nameok
    cmp byte [r11+1], 0
    je .nd
    cmp byte [r11+1], '.'
    jne .nameok
    cmp byte [r11+2], 0
    je .nd
.nameok:
    mov rdi, r11
    call strlen
    cmp rax, 3
    jb .nd
    cmp byte [r11+rax-3], '.'
    jne .nd
    cmp byte [r11+rax-2], 's'
    jne .nd
    cmp byte [r11+rax-1], 'o'
    jne .nd
    mov rdx, [plug_n]
    cmp rdx, MAX_PLUGINS
    jae .nd
    ; name_buf = dir + / + name
    lea rdi, [name_buf]
    mov rsi, r12
    call strcpy
    lea rdi, [name_buf]
    lea rsi, [slash]
    call strcat
    lea rdi, [name_buf]
    mov rsi, r11
    call strcat
    ; skip duplicates
    mov rcx, [plug_n]
    xor edx, edx
.dup:
    cmp rdx, rcx
    jae .add
    mov rdi, [plug_path+rdx*8]
    lea rsi, [name_buf]
    call streq
    test al, al
    jnz .nd
    inc edx
    jmp .dup
.add:
    lea rdi, [name_buf]
    call strdup
    mov rcx, [plug_n]
    mov [plug_path+rcx*8], rax
    mov rdi, r11
    call strdup
    mov rcx, [plug_n]
    mov [plug_name+rcx*8], rax
    inc qword [plug_n]
.nd:
    add rbx, r10
    jmp .dent
.cl:
    mov rdi, r13
    mov rax, SYS_close
    syscall
.out:
    pop r13
    pop r12
    pop rbx
    ret

strdup:
    push rsi
    mov rsi, rdi
    call strlen
    lea rdi, [rax+1]
    push rax
    push rsi
    call arena_alloc
    pop rsi
    pop rdx
    mov rdi, rax
    push rax
.cp:
    mov al, [rsi]
    mov [rdi], al
    test al, al
    jz .d
    inc rsi
    inc rdi
    jmp .cp
.d: pop rax
    pop rsi
    ret

basename_dup:
    push rbx
    mov rbx, rdi
    call strlen
    lea rsi, [rbx+rax]
.b:
    cmp rsi, rbx
    jbe .f
    dec rsi
    cmp byte [rsi], '/'
    jne .b
    inc rsi
    mov rdi, rsi
    call strdup
    pop rbx
    ret
.f:
    mov rdi, rbx
    call strdup
    pop rbx
    ret

strcpy:
.lp:
    mov al, [rsi]
    mov [rdi], al
    test al, al
    jz .d
    inc rsi
    inc rdi
    jmp .lp
.d: ret
strcat:
    push rsi
    mov rsi, rdi
    call strlen
    pop rsi
    add rdi, rax
    jmp strcpy

streq:
.lp:
    mov al, [rdi]
    mov cl, [rsi]
    cmp al, cl
    jne .n
    test al, al
    jz .y
    inc rdi
    inc rsi
    jmp .lp
.y: mov al, 1
    ret
.n: xor al, al
    ret

plugins_list:
    push rbx
    lea rsi, [msg_hdr]
    call out_str
    cmp qword [plug_n], 0
    jne .lp
    lea rsi, [msg_none]
    call out_str
    jmp .fl
.lp:
    xor ebx, ebx
.l:
    cmp rbx, [plug_n]
    jae .fl
    mov dil, ' '
    call out_byte
    mov dil, '-'
    call out_byte
    mov dil, ' '
    call out_byte
    mov rsi, [plug_name+rbx*8]
    call out_str
    mov dil, ' '
    call out_byte
    mov dil, '('
    call out_byte
    mov rsi, [plug_path+rbx*8]
    call out_str
    mov dil, ')'
    call out_byte
    mov dil, 10
    call out_byte
    inc rbx
    jmp .l
.fl:
    call out_flush
    pop rbx
    ret

; passthrough
plugins_decorate_json:
    mov rax, rsi
    ret
