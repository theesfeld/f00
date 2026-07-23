; f00-asm — filename quoting (-b -Q -N --quoting-style, -q)
BITS 64
DEFAULT REL
%include "syscalls.inc"

global quote_emit_name
extern g_quoting, g_opts2
extern out_byte, out_strn

section .text

; quote_emit_name(rsi=name, edx=len)
; Emits name with active quoting style
quote_emit_name:
    push rbx
    push r12
    push r13
    mov r12, rsi
    mov r13d, edx
    movzx eax, byte [g_quoting]
    cmp al, QUOTE_C
    je .cstyle
    cmp al, QUOTE_ESCAPE
    je .esc
    cmp al, QUOTE_SHELL
    je .shell
    ; literal + optional hide control
    mov eax, [g_opts2]
    test eax, OPT2_HIDE_CTRL
    jnz .hide
    mov rsi, r12
    mov edx, r13d
    call out_strn
    jmp .done
.hide:
    xor ebx, ebx
.hl:
    cmp ebx, r13d
    jae .done
    mov al, [r12 + rbx]
    cmp al, 32
    jb .qmark
    cmp al, 127
    je .qmark
    mov dil, al
    call out_byte
    jmp .hn
.qmark:
    mov dil, '?'
    call out_byte
.hn:
    inc ebx
    jmp .hl

.cstyle:
    mov dil, '"'
    call out_byte
    xor ebx, ebx
.cl:
    cmp ebx, r13d
    jae .cend
    mov al, [r12 + rbx]
    cmp al, '"'
    je .cq
    cmp al, '\'
    je .cb
    cmp al, 10
    je .cn
    cmp al, 9
    je .ct
    cmp al, 32
    jb .chex
    mov dil, al
    call out_byte
    jmp .cnx
.cq:
    mov dil, '\'
    call out_byte
    mov dil, '"'
    call out_byte
    jmp .cnx
.cb:
    mov dil, '\'
    call out_byte
    mov dil, '\'
    call out_byte
    jmp .cnx
.cn:
    mov dil, '\'
    call out_byte
    mov dil, 'n'
    call out_byte
    jmp .cnx
.ct:
    mov dil, '\'
    call out_byte
    mov dil, 't'
    call out_byte
    jmp .cnx
.chex:
    mov dil, '\'
    call out_byte
    mov dil, 'x'
    call out_byte
    mov ah, al
    shr al, 4
    call hex
    mov dil, al
    call out_byte
    mov al, ah
    and al, 15
    call hex
    mov dil, al
    call out_byte
.cnx:
    inc ebx
    jmp .cl
.cend:
    mov dil, '"'
    call out_byte
    jmp .done

.esc:
    ; C escapes without quotes
    xor ebx, ebx
.el:
    cmp ebx, r13d
    jae .done
    mov al, [r12 + rbx]
    cmp al, 32
    jb .ee
    cmp al, '\'
    je .eb
    cmp al, 127
    jae .ee
    mov dil, al
    call out_byte
    jmp .en
.eb:
    mov dil, '\'
    call out_byte
    mov dil, '\'
    call out_byte
    jmp .en
.ee:
    mov dil, '\'
    call out_byte
    cmp al, 10
    jne .e1
    mov dil, 'n'
    call out_byte
    jmp .en
.e1:
    cmp al, 9
    jne .e2
    mov dil, 't'
    call out_byte
    jmp .en
.e2:
    mov dil, 'x'
    call out_byte
    mov ah, al
    shr al, 4
    call hex
    mov dil, al
    call out_byte
    mov al, ah
    and al, 15
    call hex
    mov dil, al
    call out_byte
.en:
    inc ebx
    jmp .el

.shell:
    ; simple: always single-quote, escape ' as '\''
    mov dil, "'"
    call out_byte
    xor ebx, ebx
.sl:
    cmp ebx, r13d
    jae .send
    mov al, [r12 + rbx]
    cmp al, "'"
    jne .so
    ; '\''
    mov dil, "'"
    call out_byte
    mov dil, '\'
    call out_byte
    mov dil, "'"
    call out_byte
    mov dil, "'"
    call out_byte
    jmp .sn
.so:
    mov dil, al
    call out_byte
.sn:
    inc ebx
    jmp .sl
.send:
    mov dil, "'"
    call out_byte
.done:
    pop r13
    pop r12
    pop rbx
    ret

hex:
    cmp al, 10
    jb .d
    add al, 'a' - 10
    ret
.d:
    add al, '0'
    ret
