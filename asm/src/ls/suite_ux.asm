; f00 suite — terminal UI/UX design system (pure ASM)
; Expert console chrome: semantic color, columns, help sections, progress bars.
; Modern default ON when g_color; silent under --core.
BITS 64
DEFAULT REL
%include "syscalls.inc"

global ui_help_banner, ui_help_section, ui_help_footer, ui_help_print
global ui_pad_right, ui_pad_left_u64, ui_emit_bar
global ui_label, ui_value_path, ui_value_num, ui_value_ok, ui_value_err
global ui_kv_line, ui_rule, ui_bullet
global ui_color_use_pct

extern out_str, out_byte, out_u64, out_spaces, out_strn
extern color_reset, color_path, color_num, color_ok, color_err, color_hdr, color_dim
extern color_set
extern g_color, g_tty, g_cols
extern strlen
extern strcmp

section .rodata
s_core_sec:  db "Coreutils flags:", 0
s_mod_sec:   db "Modern flags:", 0
s_ex_sec:    db "Examples:", 0
s_footer:    db "f00 suite · pure assembly · MIT · https://f00.sh", 10, 0
s_rule:      db "────────────────────────────────────────────────────────────", 10, 0
c_sec:       db 27, "[1;35m", 0      ; bold magenta section
c_banner:    db 27, "[1;36m", 0
c_muted:     db 27, "[2;37m", 0
c_use_hi:    db 27, "[1;31m", 0      ; >90% red
c_use_mid:   db 27, "[1;33m", 0      ; >70% yellow
c_use_lo:    db 27, "[1;32m", 0      ; else green
c_bar_on:    db 27, "[32m", 0
c_bar_off:   db 27, "[2;37m", 0
bar_fill:    db "█", 0               ; UTF-8; fallback if needed
bar_empty:   db "░", 0
bar_fill_a:  db "#", 0
bar_empty_a: db "-", 0
bullet:      db "  · ", 0

section .bss
alignb 8
help_line:   resb 256

section .text

; ui_help_banner(rsi=title cstr) — colored title line + rule
ui_help_banner:
    push rsi
    cmp byte [g_color], 0
    je .plain
    lea rsi, [c_banner]
    call color_set
.plain:
    pop rsi
    call out_str
    mov dil, 10
    call out_byte
    call color_reset
    call ui_rule
    ret

; ui_help_section(rsi=section title like "Coreutils flags:")
ui_help_section:
    push rsi
    mov dil, 10
    call out_byte
    cmp byte [g_color], 0
    je .p
    lea rsi, [c_sec]
    call color_set
.p: pop rsi
    call out_str
    call color_reset
    mov dil, 10
    call out_byte
    ret

; ui_help_print(rsi=full help cstr)
; When g_color: colorize "Coreutils flags:" / "Modern flags:" / "Examples:" lines magenta.
; Otherwise plain out_str. Section titles must be whole lines ending with ':' before NL.
ui_help_print:
    cmp byte [g_color], 0
    je out_str
    push rbx
    push r12
    push r13
    mov r12, rsi                    ; cursor in help text
.line:
    mov al, [r12]
    test al, al
    jz .done
    ; copy one line into help_line (without NL)
    xor r13d, r13d
.cp:
    mov al, [r12]
    test al, al
    jz .eol
    cmp al, 10
    je .eol
    cmp r13d, 255
    jae .skipc
    mov [help_line + r13], al
    inc r13d
.skipc:
    inc r12
    jmp .cp
.eol:
    mov byte [help_line + r13], 0
    ; match section titles?
    lea rdi, [help_line]
    lea rsi, [s_core_sec]
    call strcmp
    test eax, eax
    jz .sec
    lea rdi, [help_line]
    lea rsi, [s_mod_sec]
    call strcmp
    test eax, eax
    jz .sec
    lea rdi, [help_line]
    lea rsi, [s_ex_sec]
    call strcmp
    test eax, eax
    jz .sec
    ; plain line
    lea rsi, [help_line]
    call out_str
    jmp .nl
.sec:
    lea rsi, [c_sec]
    call color_set
    lea rsi, [help_line]
    call out_str
    call color_reset
.nl:
    cmp byte [r12], 10
    jne .line                       ; EOF without NL
    mov dil, 10
    call out_byte
    inc r12
    jmp .line
.done:
    pop r13
    pop r12
    pop rbx
    ret

ui_help_footer:
    mov dil, 10
    call out_byte
    cmp byte [g_color], 0
    je .p
    lea rsi, [c_muted]
    call color_set
.p: lea rsi, [s_footer]
    call out_str
    jmp color_reset

ui_rule:
    cmp byte [g_color], 0
    je .p
    lea rsi, [c_muted]
    call color_set
.p: lea rsi, [s_rule]
    call out_str
    jmp color_reset

; ui_label(rsi=text) dim label
ui_label:
    push rsi
    call color_dim
    pop rsi
    call out_str
    jmp color_reset

ui_value_path:
    push rsi
    call color_path
    pop rsi
    call out_str
    jmp color_reset

ui_value_num:
    ; rdi = u64
    push rdi
    call color_num
    pop rdi
    call out_u64
    jmp color_reset

ui_value_ok:
    push rsi
    call color_ok
    pop rsi
    call out_str
    jmp color_reset

ui_value_err:
    push rsi
    call color_err
    pop rsi
    call out_str
    jmp color_reset

; ui_kv_line(rdi=key cstr, rsi=value cstr) — "key: value" modern
ui_kv_line:
    push rbx
    push r12
    mov rbx, rdi
    mov r12, rsi
    call color_dim
    mov rsi, rbx
    call out_str
    mov dil, ':'
    call out_byte
    mov dil, ' '
    call out_byte
    call color_reset
    call color_path
    mov rsi, r12
    call out_str
    call color_reset
    mov dil, 10
    call out_byte
    pop r12
    pop rbx
    ret

; ui_pad_right(rsi=str, ecx=width) — print str then pad spaces to width
ui_pad_right:
    push rbx
    push r12
    push r13
    mov r12, rsi
    mov ebx, ecx                    ; target width
    mov rdi, r12
    call strlen
    mov r13d, eax                   ; len (preserved across out_str)
    mov rsi, r12
    call out_str
    mov ecx, ebx
    sub ecx, r13d
    jg .pad
    ; always leave at least one column gap when overflow
    mov ecx, 1
.pad:
    call out_spaces
    pop r13
    pop r12
    pop rbx
    ret

; ui_pad_left_u64(rdi=val, ecx=width)
ui_pad_left_u64:
    push rbx
    push r12
    push r13
    mov r12, rdi
    mov ebx, ecx
    ; measure digits
    mov rax, r12
    xor r13d, r13d
    test rax, rax
    jnz .cnt
    mov r13d, 1
    jmp .pad
.cnt:
    inc r13d
    xor rdx, rdx
    mov rcx, 10
    div rcx
    test rax, rax
    jnz .cnt
.pad:
    mov ecx, ebx
    sub ecx, r13d
    jle .num
    call out_spaces
.num:
    call color_num
    mov rdi, r12
    call out_u64
    call color_reset
    pop r13
    pop r12
    pop rbx
    ret

; ui_emit_bar(edi=pct 0-100, esi=width cells)
; modern TTY: colored unicode bar; core/no-color: #---- ascii
ui_emit_bar:
    push rbx
    push r12
    push r13
    push r14
    mov r12d, edi                   ; pct
    cmp r12d, 100
    jbe .okp
    mov r12d, 100
.okp:
    mov r13d, esi                   ; width
    test r13d, r13d
    jnz .w
    mov r13d, 10
.w:
    ; filled = pct * width / 100
    mov eax, r12d
    mul r13d
    mov ecx, 100
    xor edx, edx
    div ecx
    mov r14d, eax                   ; filled
    cmp byte [g_color], 0
    je .ascii
    ; color by severity
    cmp r12d, 90
    jae .hi
    cmp r12d, 70
    jae .mid
    lea rsi, [c_use_lo]
    jmp .col
.hi: lea rsi, [c_use_hi]
    jmp .col
.mid: lea rsi, [c_use_mid]
.col:
    call color_set
    xor ebx, ebx
.fill:
    cmp ebx, r14d
    jae .empty
    lea rsi, [bar_fill]
    call out_str
    inc ebx
    jmp .fill
.empty:
    call color_reset
    lea rsi, [c_bar_off]
    call color_set
.e2:
    cmp ebx, r13d
    jae .done
    lea rsi, [bar_empty]
    call out_str
    inc ebx
    jmp .e2
.ascii:
    xor ebx, ebx
.af:
    cmp ebx, r14d
    jae .ae
    mov dil, '#'
    call out_byte
    inc ebx
    jmp .af
.ae:
    cmp ebx, r13d
    jae .done
    mov dil, '-'
    call out_byte
    inc ebx
    jmp .ae
.done:
    call color_reset
    pop r14
    pop r13
    pop r12
    pop rbx
    ret

; ui_color_use_pct(edi=pct) — print "NN%" with severity color
ui_color_use_pct:
    push rbx
    mov ebx, edi
    cmp ebx, 100
    jbe .p
    mov ebx, 100
.p: cmp byte [g_color], 0
    je .n
    cmp ebx, 90
    jae .hi
    cmp ebx, 70
    jae .mid
    lea rsi, [c_use_lo]
    jmp .c
.hi: lea rsi, [c_use_hi]
    jmp .c
.mid: lea rsi, [c_use_mid]
.c: call color_set
.n: mov edi, ebx
    call out_u64
    mov dil, '%'
    call out_byte
    call color_reset
    pop rbx
    ret

ui_bullet:
    cmp byte [g_color], 0
    je .p
    call color_dim
.p: lea rsi, [bullet]
    call out_str
    jmp color_reset
