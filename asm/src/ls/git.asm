; f00-asm — git status annotation via `git status --porcelain` (like f00-git)
BITS 64
DEFAULT REL
%include "syscalls.inc"

global git_annotate, git_status_char, git_status_str
extern run_capture, find_in_path, arena_alloc
extern g_entries, g_entry_count, g_opts2
extern strlen, strcmp, memcpy, memset
extern arena_alloc

section .bss
alignb 8
git_map_keys:   resq 4096           ; name pointers
git_map_vals:   resb 4096           ; status codes
git_map_count:  resq 1
git_repo:       resq 1
argv_git:       resq 8
git_path:       resq 1

section .rodata
git_name:       db "git", 0
arg0:           db "git", 0
arg1:           db "status", 0
arg2:           db "--porcelain", 0
arg3:           db "--ignored=no", 0
arg4:           db "-uall", 0
str_clean:      db "clean", 0
str_mod:        db "modified", 0
str_add:        db "added", 0
str_del:        db "deleted", 0
str_ren:        db "renamed", 0
str_unt:        db "untracked", 0
str_ign:        db "ignored", 0
str_con:        db "conflicted", 0
str_unk:        db "unknown", 0
dot_git:        db ".git", 0

section .text

; git_status_char(dil=code) → al ASCII or 0 if clean
git_status_char:
    movzx eax, dil
    cmp al, GIT_MODIFIED
    je .M
    cmp al, GIT_ADDED
    je .A
    cmp al, GIT_DELETED
    je .D
    cmp al, GIT_RENAMED
    je .R
    cmp al, GIT_UNTRACKED
    je .Q
    cmp al, GIT_IGNORED
    je .I
    cmp al, GIT_CONFLICTED
    je .U
    cmp al, GIT_UNKNOWN
    je .sp
    xor al, al
    ret
.M: mov al, 'M'
    ret
.A: mov al, 'A'
    ret
.D: mov al, 'D'
    ret
.R: mov al, 'R'
    ret
.Q: mov al, '?'
    ret
.I: mov al, '!'
    ret
.U: mov al, 'U'
    ret
.sp: mov al, ' '
    ret

; git_status_str(dil=code) → rsi pointer to string
git_status_str:
    movzx eax, dil
    cmp al, GIT_MODIFIED
    je .m
    cmp al, GIT_ADDED
    je .a
    cmp al, GIT_DELETED
    je .d
    cmp al, GIT_RENAMED
    je .r
    cmp al, GIT_UNTRACKED
    je .u
    cmp al, GIT_IGNORED
    je .i
    cmp al, GIT_CONFLICTED
    je .c
    cmp al, GIT_UNKNOWN
    je .k
    lea rsi, [str_clean]
    ret
.m: lea rsi, [str_mod]
    ret
.a: lea rsi, [str_add]
    ret
.d: lea rsi, [str_del]
    ret
.r: lea rsi, [str_ren]
    ret
.u: lea rsi, [str_unt]
    ret
.i: lea rsi, [str_ign]
    ret
.c: lea rsi, [str_con]
    ret
.k: lea rsi, [str_unk]
    ret

; parse_xy(dil=X, sil=Y) → al code
parse_xy:
    mov al, sil
    call .one
    cmp al, GIT_CLEAN
    jne .done
    mov al, dil
    call .one
.done:
    ret
.one:
    cmp al, 'M'
    je .mod
    cmp al, 'A'
    je .add
    cmp al, 'D'
    je .del
    cmp al, 'R'
    je .ren
    cmp al, 'C'
    je .ren
    cmp al, 'U'
    je .con
    cmp al, '?'
    je .unt
    cmp al, '!'
    je .ign
    mov al, GIT_CLEAN
    ret
.mod: mov al, GIT_MODIFIED
    ret
.add: mov al, GIT_ADDED
    ret
.del: mov al, GIT_DELETED
    ret
.ren: mov al, GIT_RENAMED
    ret
.con: mov al, GIT_CONFLICTED
    ret
.unt: mov al, GIT_UNTRACKED
    ret
.ign: mov al, GIT_IGNORED
    ret

; find_repo_root(rdi=start path) → rax root or 0; walks up looking for .git
find_repo_root:
    push rbx
    push r12
    push r13
    mov r12, rdi
    ; copy path to stack buffer
    call strlen
    cmp rax, 4000
    ja .no
    sub rsp, 4096
    mov rdi, rsp
    mov rsi, r12
    mov rdx, rax
    mov r13, rax
    call memcpy
    mov byte [rsp + r13], 0
    mov rbx, rsp
.loop:
    ; check rbx/.git via access
    mov rdi, r13
    add rdi, 6
    ; build path in path_join area — use end of stack
    lea rdi, [rsp + 2048]
    mov rsi, rbx
    mov rdx, r13
    call memcpy
    ; if path not ending with /, add
    cmp r13, 0
    je .no_pop
    cmp byte [rbx + r13 - 1], '/'
    je .addgit
    mov byte [rsp + 2048 + r13], '/'
    inc r13
    mov byte [rsp + 2048 + r13], 0
    dec r13
    ; wait we messed r13. rebuild simply:
.addgit:
    ; append .git
    lea rdi, [rsp + 2048]
    call strlen
    mov rcx, rax
    lea rdi, [rsp + 2048 + rcx]
    cmp byte [rsp + 2048 + rcx - 1], '/'
    je .ag
    mov byte [rdi], '/'
    inc rdi
.ag:
    mov dword [rdi], '.git'
    mov byte [rdi+4], 0
    lea rdi, [rsp + 2048]
    xor rsi, rsi
    mov rax, SYS_access
    syscall
    test rax, rax
    jz .found
    ; strip last component of rbx
    mov rdi, rbx
    call strlen
    mov r13, rax
    test r13, r13
    jz .no_pop
.trim:
    cmp r13, 1
    jbe .no_pop
    dec r13
    cmp byte [rbx + r13], '/'
    jne .trim
    test r13, r13
    jz .no_pop
    mov byte [rbx + r13], 0
    jmp .loop
.found:
    ; return copy of rbx in arena
    mov rdi, rbx
    call strlen
    mov r13, rax
    lea rdi, [rax + 1]
    call arena_alloc
    mov rdi, rax
    mov rsi, rbx
    mov rdx, r13
    push rax
    call memcpy
    pop rax
    mov byte [rax + r13], 0
    add rsp, 4096
    pop r13
    pop r12
    pop rbx
    ret
.no_pop:
    add rsp, 4096
.no:
    xor eax, eax
    pop r13
    pop r12
    pop rbx
    ret

; git_annotate(rdi=listing root path)
; Loads porcelain into map; stamps Entry.git for each entry by basename/path
git_annotate:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    push r14
    push r15

    mov r12, rdi
    mov qword [git_map_count], 0

    ; skip if OPT2_NO_GIT or not OPT2_GIT
    mov eax, [g_opts2]
    test eax, OPT2_NO_GIT
    jnz .done
    test eax, OPT2_GIT
    jz .done

    mov rdi, r12
    call find_repo_root
    test rax, rax
    jz .done
    mov [git_repo], rax
    mov r14, rax

    ; find git binary
    lea rdi, [git_name]
    call find_in_path
    test rax, rax
    jz .done
    mov [git_path], rax

    ; argv
    lea rax, [arg0]
    mov [argv_git], rax
    lea rax, [arg1]
    mov [argv_git+8], rax
    lea rax, [arg2]
    mov [argv_git+16], rax
    lea rax, [arg3]
    mov [argv_git+24], rax
    lea rax, [arg4]
    mov [argv_git+32], rax
    mov qword [argv_git+40], 0

    mov rdi, [git_path]
    lea rsi, [argv_git]
    mov rdx, r14
    call run_capture
    test rax, rax
    jz .done
    mov r15, rax                    ; porcelain text
    mov r13, rdx                    ; len

    ; parse lines
    mov rbx, r15
.parse_line:
    cmp rbx, r15
    jb .apply
    mov rax, r15
    add rax, r13
    cmp rbx, rax
    jae .apply
    ; find end of line
    mov rdi, rbx
.find_nl:
    cmp rdi, rax
    jae .eol
    cmp byte [rdi], 10
    je .eol
    inc rdi
    jmp .find_nl
.eol:
    mov rcx, rdi
    sub rcx, rbx                    ; line len
    cmp rcx, 3
    jb .next_line
    ; XY space path
    mov dil, [rbx]
    mov sil, [rbx+1]
    call parse_xy
    mov r8b, al                     ; status
    lea rsi, [rbx + 3]
    ; strip rename " -> "
    push rdi
    mov rdi, rsi
    push rcx
    ; find line end
    mov rcx, rdi
    ; path length until NL or " -> "
    mov rdx, rbx
    add rdx, [rsp]                  ; not right
    pop rcx
    pop rdi
    ; simpler: path is from rbx+3 to end of line, stop at " -> "
    lea r9, [rbx + 3]
    mov r10, rdi                    ; line end
    ; trim quotes and spaces
    cmp byte [r9], '"'
    jne .path_ok
    inc r9
.path_ok:
    mov r11, r9
.scanp:
    cmp r11, r10
    jae .gotp
    cmp byte [r11], 10
    je .gotp
    cmp dword [r11], ' -> '
    je .gotp
    ; check " ->" 
    cmp byte [r11], ' '
    jne .contp
    cmp byte [r11+1], '-'
    je .gotp
.contp:
    inc r11
    jmp .scanp
.gotp:
    ; r9..r11 is path; take basename for map key
    mov rax, r11
    dec rax
.findbase:
    cmp rax, r9
    jb .base0
    cmp byte [rax], '/'
    je .base1
    dec rax
    jmp .findbase
.base1:
    inc rax
    mov r9, rax
.base0:
    mov rcx, r11
    sub rcx, r9
    jle .next_line
    cmp byte [r9 + rcx - 1], '"'
    jne .nok
    dec rcx
.nok:
    test rcx, rcx
    jle .next_line
    ; store key copy
    push r8
    lea rdi, [rcx + 1]
    call arena_alloc
    mov rdi, rax
    mov rsi, r9
    mov rdx, rcx
    push rax
    push rcx
    call memcpy
    pop rcx
    pop rax
    mov byte [rax + rcx], 0
    pop r8
    mov rdx, [git_map_count]
    cmp rdx, 4096
    jae .next_line
    mov [git_map_keys + rdx*8], rax
    mov [git_map_vals + rdx], r8b
    inc qword [git_map_count]

.next_line:
    mov rbx, rdi
    cmp byte [rbx], 10
    jne .parse_line
    inc rbx
    jmp .parse_line

.apply:
    ; annotate entries
    mov r12, [g_entries]
    xor rbx, rbx
.ae:
    cmp rbx, [g_entry_count]
    jae .done
    mov r14, [r12 + rbx*8]
    mov byte [r14 + Entry.git], GIT_CLEAN
    mov rsi, [r14 + Entry.name]
    ; lookup
    xor r8, r8
.lk:
    cmp r8, [git_map_count]
    jae .ae_next
    mov rdi, [git_map_keys + r8*8]
    push r8
    push rsi
    call strcmp
    pop rsi
    pop r8
    test eax, eax
    jnz .lk_next
    mov al, [git_map_vals + r8]
    mov [r14 + Entry.git], al
    jmp .ae_next
.lk_next:
    inc r8
    jmp .lk
.ae_next:
    inc rbx
    jmp .ae

.done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    ret
