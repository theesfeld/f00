; f00-asm — uid/gid → names from /etc/passwd and /etc/group (no NSS)
BITS 64
DEFAULT REL
%include "syscalls.inc"

global names_init, uid_to_name, gid_to_name
extern arena_alloc, memcpy, strlen, u64_to_dec_buf

section .bss
alignb 8
uid_ids:        resd 512
uid_names:      resq 512
gid_ids:        resd 512
gid_names:      resq 512
file_buf:       resb 262144
name_tmp:       resb 32
num_scratch:    resb 32

section .rodata
path_passwd:    db "/etc/passwd", 0
path_group:     db "/etc/group", 0

section .text

names_init:
    push rbx
    push r12
    push r13
    mov ecx, 512
    lea rdi, [uid_ids]
    mov eax, 0xffffffff
    rep stosd
    mov ecx, 512
    lea rdi, [gid_ids]
    mov eax, 0xffffffff
    rep stosd
    lea rdi, [path_passwd]
    lea r12, [uid_ids]
    lea r13, [uid_names]
    call load_db
    lea rdi, [path_group]
    lea r12, [gid_ids]
    lea r13, [gid_names]
    call load_db
    pop r13
    pop r12
    pop rbx
    ret

; rdi=path, r12=ids*, r13=names*
load_db:
    push rbx
    push r14
    push r15
    mov rax, SYS_openat
    mov rsi, rdi
    mov rdi, AT_FDCWD
    mov rdx, O_RDONLY | O_CLOEXEC
    xor r10, r10
    syscall
    cmp rax, -4096
    jae .out
    mov r14, rax
    mov rax, SYS_read
    mov rdi, r14
    lea rsi, [file_buf]
    mov rdx, 262143
    syscall
    mov r15, rax
    mov rdi, r14
    mov rax, SYS_close
    syscall
    test r15, r15
    jle .out
    cmp r15, 262143
    jbe .lenok
    mov r15, 262143
.lenok:
    mov byte [file_buf + r15], 0
    xor ebx, ebx                    ; offset
.line:
    cmp rbx, r15
    jae .out
    ; skip empty
    cmp byte [file_buf + rbx], 10
    jne .parse
    inc rbx
    jmp .line
.parse:
    ; name start
    lea r8, [file_buf + rbx]        ; name ptr
    mov r9, rbx                     ; name start off
.findcol1:
    cmp rbx, r15
    jae .out
    cmp byte [file_buf + rbx], ':'
    je .col1
    cmp byte [file_buf + rbx], 10
    je .skipline
    inc rbx
    jmp .findcol1
.col1:
    mov r10, rbx
    sub r10, r9                     ; name len
    inc rbx                         ; skip :
    ; skip to second :
.sk2:
    cmp rbx, r15
    jae .out
    cmp byte [file_buf + rbx], ':'
    je .col2
    cmp byte [file_buf + rbx], 10
    je .skipline
    inc rbx
    jmp .sk2
.col2:
    inc rbx
    ; parse id
    xor r11, r11
.dig:
    cmp rbx, r15
    jae .gotid
    movzx eax, byte [file_buf + rbx]
    cmp al, '0'
    jb .gotid
    cmp al, '9'
    ja .gotid
    imul r11, 10
    sub al, '0'
    add r11, rax
    inc rbx
    jmp .dig
.gotid:
    ; copy name
    push r8
    push r10
    push r11
    lea rdi, [r10 + 1]
    call arena_alloc
    mov rdi, rax
    pop r11
    pop r10
    pop r8
    push r11
    push rax
    mov rsi, r8
    mov rdx, r10
    call memcpy
    pop rax
    pop r11
    mov byte [rax + r10], 0
    ; hash insert
    mov ecx, r11d
    and ecx, 511
    mov edx, 512
.probe:
    cmp dword [r12 + rcx*4], 0xffffffff
    je .put
    cmp dword [r12 + rcx*4], r11d
    je .put
    inc ecx
    and ecx, 511
    dec edx
    jnz .probe
    jmp .skipline
.put:
    mov [r12 + rcx*4], r11d
    mov [r13 + rcx*8], rax
.skipline:
    cmp rbx, r15
    jae .out
    cmp byte [file_buf + rbx], 10
    je .nl
    inc rbx
    jmp .skipline
.nl:
    inc rbx
    jmp .line
.out:
    pop r15
    pop r14
    pop rbx
    ret

; edi=uid → rsi name
uid_to_name:
    mov r8d, edi
    mov ecx, r8d
    and ecx, 511
    mov edx, 512
.lp:
    cmp dword [uid_ids + rcx*4], 0xffffffff
    je .num
    cmp dword [uid_ids + rcx*4], r8d
    je .hit
    inc ecx
    and ecx, 511
    dec edx
    jnz .lp
.num:
    mov edi, r8d
    lea rsi, [name_tmp]
    call u64_to_dec_buf
    mov byte [name_tmp + rax], 0
    lea rsi, [name_tmp]
    ret
.hit:
    mov rsi, [uid_names + rcx*8]
    ret

gid_to_name:
    mov r8d, edi
    mov ecx, r8d
    and ecx, 511
    mov edx, 512
.lp:
    cmp dword [gid_ids + rcx*4], 0xffffffff
    je .num
    cmp dword [gid_ids + rcx*4], r8d
    je .hit
    inc ecx
    and ecx, 511
    dec edx
    jnz .lp
.num:
    mov edi, r8d
    lea rsi, [name_tmp]
    call u64_to_dec_buf
    mov byte [name_tmp + rax], 0
    lea rsi, [name_tmp]
    ret
.hit:
    mov rsi, [gid_names + rcx*8]
    ret
