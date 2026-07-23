; f00-asm — -I/--hide patterns + --ignore-files (.gitignore / .f00ignore)
BITS 64
DEFAULT REL
%include "syscalls.inc"

global ignore_init, ignore_add_pattern, ignore_should_hide, ignore_load_files
extern arena_alloc, memcpy, strlen
extern g_opts, g_opts2

section .bss
alignb 8
pat_count:  resq 1
pat_ptrs:   resq 64                 ; glob patterns
pat_hide:   resb 64                 ; 1 = --hide (only without -a), 0 = -I always
ign_count:  resq 1
ign_ptrs:   resq 512                ; from ignore files
file_buf:   resb 65536

section .rodata
f_gitignore: db ".gitignore", 0
f_f00ignore: db ".f00ignore", 0

section .text

ignore_init:
    mov qword [pat_count], 0
    mov qword [ign_count], 0
    ret

; ignore_add_pattern(rdi=pattern string, sil=is_hide)
ignore_add_pattern:
    push rbx
    push r12
    mov r12d, esi
    mov rbx, rdi
    call strlen
    mov rdx, rax
    lea rdi, [rax + 1]
    push rdx
    call arena_alloc
    pop rdx
    mov rdi, rax
    mov rsi, rbx
    push rax
    push rdx
    call memcpy
    pop rdx
    pop rax
    mov byte [rax + rdx], 0
    mov rcx, [pat_count]
    cmp rcx, 64
    jae .done
    mov [pat_ptrs + rcx*8], rax
    mov [pat_hide + rcx], r12b
    inc qword [pat_count]
.done:
    pop r12
    pop rbx
    ret

; ignore_load_files(rdi=dir path) — if OPT2_IGN_FILES
ignore_load_files:
    mov eax, [g_opts2]
    test eax, OPT2_IGN_FILES
    jz .ret
    test eax, OPT2_NO_IGNORE
    jnz .ret
    push rbx
    mov rbx, rdi
    lea rsi, [f_gitignore]
    call load_one
    mov rdi, rbx
    lea rsi, [f_f00ignore]
    call load_one
    pop rbx
.ret:
    ret

; rdi=dir, rsi=filename
load_one:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    sub rsp, 4096
    ; join path
    mov r12, rdi
    mov r13, rsi
    mov rdi, rsp
    mov rsi, r12
    call strcpy
    mov rdi, rsp
    call strlen
    lea rdi, [rsp + rax]
    cmp rax, 1
    je .chkroot
    cmp byte [rsp + rax - 1], '/'
    je .cat
    mov byte [rdi], '/'
    inc rdi
    jmp .cat
.chkroot:
    cmp byte [rsp], '/'
    je .cat
    mov byte [rdi], '/'
    inc rdi
.cat:
    mov rsi, r13
    call strcpy

    mov rax, SYS_openat
    mov rdi, AT_FDCWD
    mov rsi, rsp
    mov rdx, O_RDONLY | O_CLOEXEC
    xor r10, r10
    syscall
    cmp rax, -4096
    jae .out
    mov r12, rax
    mov rax, SYS_read
    mov rdi, r12
    lea rsi, [file_buf]
    mov rdx, 65535
    syscall
    mov r13, rax
    mov rdi, r12
    mov rax, SYS_close
    syscall
    test r13, r13
    jle .out
    cmp r13, 65535
    jbe .ok
    mov r13, 65535
.ok:
    mov byte [file_buf + r13], 0
    xor ebx, ebx
.line:
    cmp rbx, r13
    jae .out
    cmp byte [file_buf + rbx], 10
    jne .p
    inc rbx
    jmp .line
.p:
    cmp byte [file_buf + rbx], '#'
    je .skipline
    cmp byte [file_buf + rbx], '!'
    je .skipline                    ; no negation support
    ; pattern until NL
    lea r8, [file_buf + rbx]
    mov r9, rbx
.sl:
    cmp rbx, r13
    jae .take
    cmp byte [file_buf + rbx], 10
    je .take
    inc rbx
    jmp .sl
.take:
    mov r10, rbx
    sub r10, r9
    jle .skipline
    ; trim trailing spaces
.trim:
    test r10, r10
    jz .skipline
    mov al, [r8 + r10 - 1]
    cmp al, ' '
    je .tr
    cmp al, 13
    jne .store
.tr:
    dec r10
    jmp .trim
.store:
    lea rdi, [r10 + 1]
    push r8
    push r10
    call arena_alloc
    pop r10
    pop r8
    mov rdi, rax
    mov rsi, r8
    mov rdx, r10
    push rax
    call memcpy
    pop rax
    mov byte [rax + r10], 0
    mov rcx, [ign_count]
    cmp rcx, 512
    jae .skipline
    mov [ign_ptrs + rcx*8], rax
    inc qword [ign_count]
.skipline:
    cmp rbx, r13
    jae .out
    cmp byte [file_buf + rbx], 10
    je .nl
    inc rbx
    jmp .skipline
.nl:
    inc rbx
    jmp .line
.out:
    add rsp, 4096
    pop r13
    pop r12
    pop rbx
    pop rbp
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

; ignore_should_hide(rsi=name) → al 1=hide
ignore_should_hide:
    push rbx
    push r12
    push r13
    mov r12, rsi
    ; backups ~
    mov eax, [g_opts]
    test eax, OPT_IGN_BACKUP
    jz .pats
    mov rdi, r12
    call strlen
    test rax, rax
    jz .pats
    cmp byte [r12 + rax - 1], '~'
    jne .pats
    mov al, 1
    jmp .done
.pats:
    xor ebx, ebx
.pl:
    cmp rbx, [pat_count]
    jae .ignfiles
    ; hide patterns skipped if -a/-A
    cmp byte [pat_hide + rbx], 0
    je .match
    mov eax, [g_opts]
    test eax, OPT_ALL | OPT_ALMOST_ALL
    jnz .pn
.match:
    mov rdi, r12
    mov rsi, [pat_ptrs + rbx*8]
    call glob_match
    test al, al
    jnz .yes
.pn:
    inc rbx
    jmp .pl
.ignfiles:
    xor ebx, ebx
.il:
    cmp rbx, [ign_count]
    jae .no
    mov rdi, r12
    mov rsi, [ign_ptrs + rbx*8]
    call glob_match
    test al, al
    jnz .yes
    inc rbx
    jmp .il
.no:
    xor al, al
    jmp .done
.yes:
    mov al, 1
.done:
    pop r13
    pop r12
    pop rbx
    ret

; glob_match(rdi=name, rsi=pattern) — supports * and ?
glob_match:
    push rbx
    push r12
    push r13
    mov r12, rdi
    mov r13, rsi
.lp:
    mov al, [r13]
    test al, al
    jz .pend
    cmp al, '*'
    je .star
    cmp al, '?'
    je .q
    cmp al, [r12]
    jne .no
    inc r12
    inc r13
    jmp .lp
.q:
    cmp byte [r12], 0
    je .no
    inc r12
    inc r13
    jmp .lp
.star:
    inc r13
    cmp byte [r13], 0
    je .yes                         ; trailing *
.try:
    push r12
    push r13
    mov rdi, r12
    mov rsi, r13
    call glob_match
    pop r13
    pop r12
    test al, al
    jnz .yes
    cmp byte [r12], 0
    je .no
    inc r12
    jmp .try
.pend:
    cmp byte [r12], 0
    je .yes
.no:
    xor al, al
    jmp .out
.yes:
    mov al, 1
.out:
    pop r13
    pop r12
    pop rbx
    ret
