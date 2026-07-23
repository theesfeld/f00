; f00-asm — run external commands (git status, update helpers) via fork/execve
BITS 64
DEFAULT REL
%include "syscalls.inc"

global run_capture, find_in_path
extern arena_alloc, memcpy, strlen, memset
extern g_exit

section .bss
alignb 8
pipe_fds:       resd 2
capture_buf:    resb 1<<20          ; 1 MiB capture cap for porcelain / updates
path_join:      resb 4096

section .rodata
env_path_key:   db "PATH=", 0
slash_bin:      db "/bin/", 0
slash_usr_bin:  db "/usr/bin/", 0

section .text

; ------------------------------------------------------------
; find_in_path(rdi=name) → rax=full path in arena or 0
; Tries /usr/bin/name and /bin/name (no libc env walk for speed/reliability)
; ------------------------------------------------------------
find_in_path:
    push rbx
    push r12
    mov r12, rdi
    call strlen
    mov rbx, rax
    ; /usr/bin/ + name
    lea rdi, [rbx + 10]
    call arena_alloc
    mov rdi, rax
    lea rsi, [slash_usr_bin]
    mov rdx, 9
    push rax
    call memcpy
    pop rax
    lea rdi, [rax + 9]
    mov rsi, r12
    mov rdx, rbx
    push rax
    call memcpy
    pop rax
    mov byte [rax + 9 + rbx], 0
    ; access(F_OK)
    push rax
    mov rdi, rax
    mov rsi, 0                      ; F_OK
    mov rax, SYS_access
    syscall
    pop rdi
    test rax, rax
    jz .found
    ; /bin/
    lea rdi, [rbx + 6]
    call arena_alloc
    mov rdi, rax
    lea rsi, [slash_bin]
    mov rdx, 5
    push rax
    call memcpy
    pop rax
    lea rdi, [rax + 5]
    mov rsi, r12
    mov rdx, rbx
    push rax
    call memcpy
    pop rax
    mov byte [rax + 5 + rbx], 0
    push rax
    mov rdi, rax
    xor rsi, rsi
    mov rax, SYS_access
    syscall
    pop rdi
    test rax, rax
    jz .found
    xor eax, eax
    pop r12
    pop rbx
    ret
.found:
    mov rax, rdi
    pop r12
    pop rbx
    ret

; ------------------------------------------------------------
; run_capture:
;   rdi = path to executable (absolute)
;   rsi = argv** (NULL-terminated), argv[0] = name
;   rdx = cwd or 0 to keep
; → rax = pointer to NUL-terminated stdout in capture_buf
;   rdx = length
;   on failure rax=0
; ------------------------------------------------------------
run_capture:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov r12, rdi                    ; exe path
    mov r13, rsi                    ; argv
    mov r14, rdx                    ; cwd

    ; pipe2(fds, O_CLOEXEC)
    lea rdi, [pipe_fds]
    mov rsi, O_CLOEXEC
    mov rax, SYS_pipe2
    syscall
    test rax, rax
    jnz .fail

    ; fork
    mov rax, SYS_fork
    syscall
    test rax, rax
    js .fail
    jz .child
    ; parent
    mov r15, rax                    ; child pid
    ; close write end
    mov edi, [pipe_fds + 4]
    mov rax, SYS_close
    syscall
    ; read all from read end into capture_buf
    xor r12, r12                    ; total (reuse r12)
.read:
    cmp r12, (1<<20) - 1
    jae .read_done
    mov rax, SYS_read
    mov edi, [pipe_fds]
    lea rsi, [capture_buf + r12]
    mov rdx, (1<<20) - 1
    sub rdx, r12
    syscall
    test rax, rax
    jle .read_done
    add r12, rax
    jmp .read
.read_done:
    mov byte [capture_buf + r12], 0
    mov edi, [pipe_fds]
    mov rax, SYS_close
    syscall
    ; wait4
    sub rsp, 16
    mov rdi, r15
    mov rsi, rsp
    xor rdx, rdx
    xor r10, r10
    mov rax, SYS_wait4
    syscall
    add rsp, 16
    lea rax, [capture_buf]
    mov rdx, r12
    jmp .out

.child:
    ; close read end
    mov edi, [pipe_fds]
    mov rax, SYS_close
    syscall
    ; dup2 write → stdout
    mov edi, [pipe_fds + 4]
    mov rsi, 1
    mov rax, SYS_dup2
    syscall
    ; close write
    mov edi, [pipe_fds + 4]
    mov rax, SYS_close
    syscall
    ; optional chdir
    test r14, r14
    jz .exec
    mov rdi, r14
    mov rax, SYS_chdir
    syscall
.exec:
    ; execve(path, argv, envp=NULL → empty env is bad; use environ from stack?
    ; Kernel needs envp; pass empty list {NULL} loses PATH but we use absolute path.
    sub rsp, 8
    mov qword [rsp], 0
    mov rdi, r12
    mov rsi, r13
    mov rdx, rsp                    ; envp = [NULL]
    mov rax, SYS_execve
    syscall
    ; exec failed
    mov rdi, 127
    mov rax, SYS_exit
    syscall

.fail:
    xor eax, eax
    xor edx, edx
.out:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    ret
