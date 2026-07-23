; f00-asm — F00_COLORS / EZA_COLORS / EXA_COLORS for long-list chrome
BITS 64
DEFAULT REL
%include "syscalls.inc"

global meta_init, meta_paint
extern g_color
extern arena_alloc
extern out_str

section .bss
alignb 8
meta_n:    resq 1
meta_keys: resd 64
meta_vals: resq 64
seq_buf:   resb 48

section .rodata
reset:  db 27,"[0m",0
k_f00:  db "F00_COLORS=",0
k_eza:  db "EZA_COLORS=",0
k_exa:  db "EXA_COLORS=",0
s_31:   db "31",0
s_32:   db "32",0
s_33:   db "33",0
s_34:   db "34",0

section .text

;------------------------------------------------------------
; meta_init(rdi=envp)
;------------------------------------------------------------
meta_init:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    push r14
    mov r12, rdi
    mov qword [meta_n], 0

    mov r8d, 0x6e73                 ; 'sn' little-endian s=0x73 n=0x6e → "sn" as word 's'|'n'<<8 = 0x6e73
    lea rsi, [s_32]
    call store_kv
    mov r8d, 0x7575                 ; uu
    lea rsi, [s_33]
    call store_kv
    mov r8d, 0x7567                 ; gu
    lea rsi, [s_33]
    call store_kv
    mov r8d, 0x6164                 ; da
    lea rsi, [s_34]
    call store_kv
    mov r8d, 0x6d67                 ; gm
    lea rsi, [s_33]
    call store_kv
    mov r8d, 0x6167                 ; ga
    lea rsi, [s_32]
    call store_kv
    mov r8d, 0x6467                 ; gd
    lea rsi, [s_31]
    call store_kv

    test r12, r12
    jz .done
.scan:
    mov r13, [r12]
    test r13, r13
    jz .done
    mov rdi, r13
    lea rsi, [k_f00]
    call starts_with_11
    test al, al
    jnz .use
    mov rdi, r13
    lea rsi, [k_eza]
    call starts_with_11
    test al, al
    jnz .use
    mov rdi, r13
    lea rsi, [k_exa]
    call starts_with_11
    test al, al
    jnz .use
    add r12, 8
    jmp .scan
.use:
    lea r14, [r13 + 11]
    call parse_map
.done:
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    ret

starts_with_11:
    mov ecx, 11
.lp:
    mov al, [rdi]
    cmp al, [rsi]
    jne .no
    inc rdi
    inc rsi
    dec ecx
    jnz .lp
    mov al, 1
    ret
.no:
    xor al, al
    ret

; store_kv: r8d=key (2 chars), rsi=NUL val
store_kv:
    push rbx
    push r12
    push r13
    mov r12d, r8d
    mov r13, rsi
    ; strlen
    xor ecx, ecx
.slen:
    cmp byte [r13 + rcx], 0
    je .slen_done
    inc ecx
    jmp .slen
.slen_done:
    lea rdi, [rcx + 1]
    push rcx
    call arena_alloc
    pop rcx
    mov rbx, rax                    ; dest base
    xor edx, edx
.cpy:
    cmp edx, ecx
    jae .term
    mov al, [r13 + rdx]
    mov [rbx + rdx], al
    inc edx
    jmp .cpy
.term:
    mov byte [rbx + rcx], 0
    ; upsert
    mov rcx, [meta_n]
    xor edx, edx
.find:
    cmp rdx, rcx
    jae .add
    cmp dword [meta_keys + rdx*4], r12d
    je .upd
    inc edx
    jmp .find
.upd:
    mov [meta_vals + rdx*8], rbx
    jmp .out
.add:
    cmp rcx, 64
    jae .out
    mov [meta_keys + rcx*4], r12d
    mov [meta_vals + rcx*8], rbx
    inc qword [meta_n]
.out:
    pop r13
    pop r12
    pop rbx
    ret

; parse_map: r14 → "ab=val:cd=val:"
parse_map:
.item:
    mov al, [r14]
    test al, al
    jz .ret
    cmp al, ':'
    jne .key
    inc r14
    jmp .item
.key:
    cmp byte [r14+2], '='
    jne .skip
    movzx r8d, word [r14]
    add r14, 3
    mov rsi, r14
.v:
    mov al, [r14]
    test al, al
    jz .set
    cmp al, ':'
    je .set
    inc r14
    jmp .v
.set:
    ; temporarily NUL-terminate val
    mov bl, [r14]
    mov byte [r14], 0
    push rbx
    push r14
    call store_kv
    pop r14
    pop rbx
    mov [r14], bl
    jmp .item
.skip:
    inc r14
    jmp .item
.ret:
    ret

; meta_paint(rsi=text, r8d=key) — paint with meta color if enabled
meta_paint:
    cmp byte [g_color], 0
    je .plain
    mov rcx, [meta_n]
    xor edx, edx
.find:
    cmp rdx, rcx
    jae .plain
    cmp dword [meta_keys + rdx*4], r8d
    je .hit
    inc edx
    jmp .find
.hit:
    mov r9, [meta_vals + rdx*8]
    lea rdi, [seq_buf]
    mov byte [rdi], 27
    mov byte [rdi+1], '['
    add rdi, 2
.cp:
    mov al, [r9]
    test al, al
    jz .endc
    mov [rdi], al
    inc r9
    inc rdi
    jmp .cp
.endc:
    mov byte [rdi], 'm'
    mov byte [rdi+1], 0
    push rsi
    lea rsi, [seq_buf]
    call out_str
    pop rsi
    call out_str
    lea rsi, [reset]
    jmp out_str
.plain:
    jmp out_str
