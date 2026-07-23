; f00-asm — tree view (--tree / -R tree-style connectors)
BITS 64
DEFAULT REL
%include "syscalls.inc"

global format_tree
extern g_entries, g_entry_count
extern out_byte, out_str, out_strn
extern emit_name_public

section .rodata
t_branch:       db 0xe2, 0x94, 0x9c, 0xe2, 0x94, 0x80, 0xe2, 0x94, 0x80, 0x20, 0  ; ├── 
t_last:         db 0xe2, 0x94, 0x94, 0xe2, 0x94, 0x80, 0xe2, 0x94, 0x80, 0x20, 0  ; └── 
t_pipe:         db 0xe2, 0x94, 0x82, 0x20, 0x20, 0x20, 0                          ; │   
t_space:        db 0x20, 0x20, 0x20, 0x20, 0                                      ; four spaces

section .text

; Flat tree for non-recursive listing; recursive uses depth field
format_tree:
    push rbx
    push r12
    push r13
    mov r12, [g_entries]
    mov r13, [g_entry_count]
    test r13, r13
    jz .done
    ; detect if any depth > 0
    xor rbx, rbx
    xor eax, eax
.chk:
    cmp rbx, r13
    jae .chkdone
    mov rdi, [r12 + rbx*8]
    cmp byte [rdi + Entry.depth], 0
    je .chn
    mov eax, 1
.chn:
    inc rbx
    jmp .chk
.chkdone:
    test eax, eax
    jnz .by_depth
    ; flat
    xor rbx, rbx
.flat:
    cmp rbx, r13
    jae .done
    lea rax, [rbx + 1]
    cmp rax, r13
    je .flast
    lea rsi, [t_branch]
    jmp .fdraw
.flast:
    lea rsi, [t_last]
.fdraw:
    call out_str
    mov rdi, [r12 + rbx*8]
    call emit_name_public
    mov dil, 10
    call out_byte
    inc rbx
    jmp .flat

.by_depth:
    ; simple depth prefix with └── / ├── 
    xor rbx, rbx
.dlp:
    cmp rbx, r13
    jae .done
    mov r8, [r12 + rbx*8]
    movzx ecx, byte [r8 + Entry.depth]
    ; indent
.ind:
    test ecx, ecx
    jz .iname
    dec ecx
    lea rsi, [t_space]
    push rcx
    push r8
    call out_str
    pop r8
    pop rcx
    jmp .ind
.iname:
    ; last among same-depth lookahead
    movzx r9d, byte [r8 + Entry.depth]
    mov rax, rbx
    inc rax
    mov r10d, 1                     ; assume last
.look:
    cmp rax, r13
    jae .pick
    mov rdi, [r12 + rax*8]
    movzx edx, byte [rdi + Entry.depth]
    cmp edx, r9d
    jb .pick
    je .notlast
    inc rax
    jmp .look
.notlast:
    xor r10d, r10d
.pick:
    test r10d, r10d
    jz .br
    lea rsi, [t_last]
    jmp .dr
.br:
    lea rsi, [t_branch]
.dr:
    push r8
    call out_str
    pop rdi
    call emit_name_public
    mov dil, 10
    call out_byte
    inc rbx
    jmp .dlp

.done:
    pop r13
    pop r12
    pop rbx
    ret
