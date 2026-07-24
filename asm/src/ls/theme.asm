; f00tils — theme system for semantic color tokens
; Default "terminal" uses classic 16-color SGR so the *terminal palette* owns hues.
; Named themes (dracula, tokyo-night, catppuccin-*, monokai, …) set truecolor bodies.
; User themes: $XDG_CONFIG_HOME/f00/themes/<name>.theme  (body lines: path=1;36)
BITS 64
DEFAULT REL
%include "syscalls.inc"

global theme_init, theme_apply_name, theme_apply_default
global theme_list_print, theme_current_name
global g_theme_name
global theme_set_token_body

; token buffers live in util.asm BSS as c_path/c_num/… — we fill them
extern c_path, c_num, c_ok, c_err, c_hdr, c_dim, c_reset
extern c_banner, c_spin
extern strlen, strcmp, memcpy, memset
extern out_str, out_byte
extern g_envp
extern env_key_match

%define TOK_CAP 32

section .bss
alignb 8
g_theme_name:   resb 64
theme_path:     resb 1024
theme_filebuf:  resb 4096
line_tmp:       resb 256
body_tmp:       resb 48

section .rodata
; ── default: classic 16-color (inherits user terminal palette) ──
; bodies only (digits and ;); theme_set_token_body wraps ESC [ body m
def_path:   db "1;36",0          ; bold cyan → terminal cyan
def_num:    db "1;33",0          ; bold yellow
def_ok:     db "1;32",0
def_err:    db "1;31",0
def_hdr:    db "1;34",0
def_dim:    db "2",0
def_banner: db "1;36",0
def_spin:   db "1;36",0

name_terminal: db "terminal",0
name_f00:      db "f00",0

; Dracula
n_dracula: db "dracula",0
d_path: db "38;2;139;233;253",0
d_num:  db "38;2;241;250;140",0
d_ok:   db "38;2;80;250;123",0
d_err:  db "38;2;255;85;85",0
d_hdr:  db "38;2;189;147;249",0
d_dim:  db "2;38;2;98;114;164",0

; Tokyo Night
n_tokyo: db "tokyo-night",0
t_path: db "38;2;125;207;255",0
t_num:  db "38;2;224;175;104",0
t_ok:   db "38;2;158;206;106",0
t_err:  db "38;2;247;118;142",0
t_hdr:  db "38;2;187;154;247",0
t_dim:  db "2;38;2;86;95;137",0

n_tokyo_storm: db "tokyo-night-storm",0
ts_path: db "38;2;122;162;247",0
ts_num:  db "38;2;224;175;104",0
ts_ok:   db "38;2;158;206;106",0
ts_err:  db "38;2;247;118;142",0
ts_hdr:  db "38;2;187;154;247",0
ts_dim:  db "2;38;2;86;95;137",0

; Catppuccin Mocha / Latte
n_mocha: db "catppuccin-mocha",0
m_path: db "38;2;137;180;250",0
m_num:  db "38;2;249;226;175",0
m_ok:   db "38;2;166;227;161",0
m_err:  db "38;2;243;139;168",0
m_hdr:  db "38;2;203;166;247",0
m_dim:  db "2;38;2;108;112;134",0

n_latte: db "catppuccin-latte",0
l_path: db "38;2;30;102;245",0
l_num:  db "38;2;223;142;29",0
l_ok:   db "38;2;64;160;43",0
l_err:  db "38;2;210;15;57",0
l_hdr:  db "38;2;136;57;239",0
l_dim:  db "2;38;2;156;160;176",0

; Monokai / Monokai Pro-ish
n_monokai: db "monokai",0
k_path: db "38;2;102;217;239",0
k_num:  db "38;2;230;219;116",0
k_ok:   db "38;2;166;226;46",0
k_err:  db "38;2;249;38;114",0
k_hdr:  db "38;2;174;129;255",0
k_dim:  db "2;38;2;117;113;94",0

n_monokai_pro: db "monokai-pro",0
kp_path: db "38;2;120;220;232",0
kp_num:  db "38;2;255;216;102",0
kp_ok:   db "38;2;169;220;118",0
kp_err:  db "38;2;255;97;136",0
kp_hdr:  db "38;2;171;157;242",0
kp_dim:  db "2;38;2;147;146;147",0

; Nord
n_nord: db "nord",0
no_path: db "38;2;136;192;208",0
no_num:  db "38;2;235;203;139",0
no_ok:   db "38;2;163;190;140",0
no_err:  db "38;2;191;97;106",0
no_hdr:  db "38;2;129;161;193",0
no_dim:  db "2;38;2;76;86;106",0

; Gruvbox dark / light
n_gruvd: db "gruvbox-dark",0
gd_path: db "38;2;131;165;152",0
gd_num:  db "38;2;250;189;47",0
gd_ok:   db "38;2;184;187;38",0
gd_err:  db "38;2;251;73;52",0
gd_hdr:  db "38;2;211;134;155",0
gd_dim:  db "2;38;2;146;131;116",0

n_gruvl: db "gruvbox-light",0
gl_path: db "38;2;7;102;120",0
gl_num:  db "38;2;181;118;20",0
gl_ok:   db "38;2;121;116;14",0
gl_err:  db "38;2;157;0;6",0
gl_hdr:  db "38;2;143;63;113",0
gl_dim:  db "2;38;2;124;111;100",0

; Solarized
n_sold: db "solarized-dark",0
sd_path: db "38;2;38;139;210",0
sd_num:  db "38;2;181;137;0",0
sd_ok:   db "38;2;133;153;0",0
sd_err:  db "38;2;220;50;47",0
sd_hdr:  db "38;2;108;113;196",0
sd_dim:  db "2;38;2;88;110;117",0

n_soll: db "solarized-light",0
sl_path: db "38;2;38;139;210",0
sl_num:  db "38;2;181;137;0",0
sl_ok:   db "38;2;133;153;0",0
sl_err:  db "38;2;220;50;47",0
sl_hdr:  db "38;2;108;113;196",0
sl_dim:  db "2;38;2;147;161;161",0

; One Dark / One Light
n_oned: db "one-dark",0
od_path: db "38;2;97;175;239",0
od_num:  db "38;2;229;192;123",0
od_ok:   db "38;2;152;195;121",0
od_err:  db "38;2;224;108;117",0
od_hdr:  db "38;2;198;120;221",0
od_dim:  db "2;38;2;92;99;112",0

; Rose Pine
n_rosepine: db "rose-pine",0
rp_path: db "38;2;156;207;216",0
rp_num:  db "38;2;246;193;119",0
rp_ok:   db "38;2;49;116;143",0
rp_err:  db "38;2;235;111;146",0
rp_hdr:  db "38;2;196;167;231",0
rp_dim:  db "2;38;2;110;106;134",0

; table: name, path,num,ok,err,hdr,dim  (7 qwords each), 0 end
align 8
theme_table:
    dq name_terminal, def_path, def_num, def_ok, def_err, def_hdr, def_dim
    dq name_f00,      def_path, def_num, def_ok, def_err, def_hdr, def_dim
    dq n_dracula, d_path, d_num, d_ok, d_err, d_hdr, d_dim
    dq n_tokyo, t_path, t_num, t_ok, t_err, t_hdr, t_dim
    dq n_tokyo_storm, ts_path, ts_num, ts_ok, ts_err, ts_hdr, ts_dim
    dq n_mocha, m_path, m_num, m_ok, m_err, m_hdr, m_dim
    dq n_latte, l_path, l_num, l_ok, l_err, l_hdr, l_dim
    dq n_monokai, k_path, k_num, k_ok, k_err, k_hdr, k_dim
    dq n_monokai_pro, kp_path, kp_num, kp_ok, kp_err, kp_hdr, kp_dim
    dq n_nord, no_path, no_num, no_ok, no_err, no_hdr, no_dim
    dq n_gruvd, gd_path, gd_num, gd_ok, gd_err, gd_hdr, gd_dim
    dq n_gruvl, gl_path, gl_num, gl_ok, gl_err, gl_hdr, gl_dim
    dq n_sold, sd_path, sd_num, sd_ok, sd_err, sd_hdr, sd_dim
    dq n_soll, sl_path, sl_num, sl_ok, sl_err, sl_hdr, sl_dim
    dq n_oned, od_path, od_num, od_ok, od_err, od_hdr, od_dim
    dq n_rosepine, rp_path, rp_num, rp_ok, rp_err, rp_hdr, rp_dim
    dq 0

suf_themes:  db "/f00/themes/", 0
suf_dot_th:  db "/.config/f00/themes/", 0
ext_theme:   db ".theme", 0
env_xdg:     db "XDG_CONFIG_HOME", 0
env_home:    db "HOME", 0
env_theme:   db "F00_THEME", 0
tk_path:     db "path", 0
tk_num:      db "num", 0
tk_ok:       db "ok", 0
tk_err:      db "err", 0
tk_hdr:      db "hdr", 0
tk_dim:      db "dim", 0
tk_banner:   db "banner", 0
tk_spin:     db "spin", 0
nl:          db 10, 0
sp2:         db "  ", 0
star:        db " *", 0
hdr_builtin: db "Builtin themes:", 10, 0
hdr_user:    db "User themes (~/.config/f00/themes/):", 10, 0
msg_cur:     db "current: ", 0
sw:          db "██", 0
msg_note:    db 10, "swatches: path num ok err hdr dim", 10
             db "Default 'terminal'/'f00' use ANSI 16 colors (your palette).", 10
             db "Named themes use truecolor. User: ~/.config/f00/themes/<name>.theme", 10
             db "  path=1;36   or   path=38;2;R;G;B", 10
             db "Set: f00-config theme set <name>   (writes XDG config)", 10, 0
extern color_path, color_num, color_ok, color_err, color_hdr, color_dim, color_reset

section .text

; theme_init — seed default (terminal palette). Call after config_load.
; Caller applies g_cfg_theme then F00_THEME (env wins over config).
theme_init:
    push rbx
    ; always valid reset
    mov byte [c_reset], 27
    mov byte [c_reset+1], '['
    mov byte [c_reset+2], '0'
    mov byte [c_reset+3], 'm'
    mov byte [c_reset+4], 0
    call theme_apply_default
    pop rbx
    ret

; theme_apply_env — F00_THEME=name if set
; manual prefix match (env_key_match clobbers regs awkwardly)
global theme_apply_env
theme_apply_env:
    push rbx
    push r12
    push r13
    mov r12, [g_envp]
    test r12, r12
    jz .done
.elp:
    mov r13, [r12]
    test r13, r13
    jz .done
    ; compare prefix F00_THEME=
    lea rsi, [env_theme]
    mov rdi, r13
.cmp:
    mov al, [rsi]
    test al, al
    jz .endk
    cmp al, [rdi]
    jne .next
    inc rsi
    inc rdi
    jmp .cmp
.endk:
    cmp byte [rdi], '='
    jne .next
    inc rdi
    cmp byte [rdi], 0
    je .done
    call theme_apply_name
    jmp .done
.next:
    add r12, 8
    jmp .elp
.done:
    pop r13
    pop r12
    pop rbx
    ret

theme_apply_default:
    lea rdi, [name_terminal]
    jmp theme_apply_name

name_auto: db "auto", 0
env_colorfgbg: db "COLORFGBG", 0
n_auto_dark: db "catppuccin-mocha", 0
n_auto_light: db "catppuccin-latte", 0

; theme_apply_auto — COLORFGBG=fg;bg → dark if bg in 0..7 else light
; falls back to terminal if COLORFGBG missing
theme_apply_auto:
    push rbx
    push r12
    mov r12, [g_envp]
    test r12, r12
    jz .term
.elp:
    mov rdi, [r12]
    test rdi, rdi
    jz .term
    push rdi
    lea rsi, [env_colorfgbg]
    call env_key_match
    pop rdi
    test al, al
    jnz .got
    add r12, 8
    jmp .elp
.got:
    push rdi
    lea rdi, [env_colorfgbg]
    call strlen
    mov rcx, rax
    pop rdi
    add rdi, rcx
    cmp byte [rdi], '='
    jne .term
    inc rdi
    ; scan to ; then parse bg digit(s)
.sc:
    mov al, [rdi]
    test al, al
    jz .term
    cmp al, ';'
    je .bg
    inc rdi
    jmp .sc
.bg:
    inc rdi
    ; parse small int
    xor eax, eax
.pd:
    movzx ecx, byte [rdi]
    cmp cl, '0'
    jb .have
    cmp cl, '9'
    ja .have
    imul eax, eax, 10
    sub cl, '0'
    add eax, ecx
    inc rdi
    jmp .pd
.have:
    ; bg 0-7 → dark truecolor; 8-15 → light
    cmp eax, 8
    jae .light
    lea rdi, [n_auto_dark]
    call theme_apply_name
    jmp .out
.light:
    lea rdi, [n_auto_light]
    call theme_apply_name
    jmp .out
.term:
    call theme_apply_default
.out:
    ; record logical name "auto" for get/show
    lea rdi, [name_auto]
    call store_theme_name
    pop r12
    pop rbx
    mov eax, 1
    ret

; theme_apply_name(rdi=name cstr) → eax=1 ok, 0 miss
theme_apply_name:
    push rbx
    push r12
    push r13
    mov r12, rdi
    test r12, r12
    jz .fail
    cmp byte [r12], 0
    je .fail
    ; auto → COLORFGBG heuristic
    mov rdi, r12
    lea rsi, [name_auto]
    call strcmp
    test eax, eax
    jnz .builtins
    call theme_apply_auto
    jmp .out
.builtins:
    ; try builtins
    lea r13, [theme_table]
.blp:
    mov rdi, [r13]
    test rdi, rdi
    jz .try_user
    mov rsi, r12
    call strcmp
    test eax, eax
    jz .hit
    add r13, 56                     ; 7 * 8
    jmp .blp
.hit:
    mov rdi, [r13 + 8]
    lea rsi, [c_path]
    call theme_set_token_body_to
    mov rdi, [r13 + 16]
    lea rsi, [c_num]
    call theme_set_token_body_to
    mov rdi, [r13 + 24]
    lea rsi, [c_ok]
    call theme_set_token_body_to
    mov rdi, [r13 + 32]
    lea rsi, [c_err]
    call theme_set_token_body_to
    mov rdi, [r13 + 40]
    lea rsi, [c_hdr]
    call theme_set_token_body_to
    mov rdi, [r13 + 48]
    lea rsi, [c_dim]
    call theme_set_token_body_to
    ; banner/spin follow path by default
    mov rdi, [r13 + 8]
    lea rsi, [c_banner]
    call theme_set_token_body_to
    mov rdi, [r13 + 8]
    lea rsi, [c_spin]
    call theme_set_token_body_to
    mov rdi, r12
    call store_theme_name
    mov eax, 1
    jmp .out
.try_user:
    mov rdi, r12
    call theme_try_user_file
    test eax, eax
    jz .fail
    mov rdi, r12
    call store_theme_name
    mov eax, 1
    jmp .out
.fail:
    ; runtime soft-fallback: stay terminal, do not abort caller
    call theme_apply_default
    xor eax, eax
.out:
    pop r13
    pop r12
    pop rbx
    ret

; store name into g_theme_name
store_theme_name:
    push rsi
    lea rsi, [g_theme_name]
    xchg rdi, rsi
    ; rdi=dst rsi=src
    call strcpy_theme
    pop rsi
    ret

strcpy_theme:
    push rcx
    xor ecx, ecx
.lp:
    cmp ecx, 63
    jae .z
    mov al, [rsi + rcx]
    mov [rdi + rcx], al
    test al, al
    jz .d
    inc ecx
    jmp .lp
.z: mov byte [rdi + 63], 0
.d: pop rcx
    ret

; theme_set_token_body_to(rdi=body cstr "1;36", rsi=dest buffer)
; writes ESC [ body m NUL  into dest (cap body 24 → total ≤ 28 of 32)
theme_set_token_body_to:
    push rbx
    push r12
    push r13
    mov r12, rdi                    ; body
    mov r13, rsi                    ; dest
    mov byte [r13], 27
    mov byte [r13+1], '['
    ; copy body — keep length in ebx (memcpy clobbers rcx via cl)
    mov rdi, r12
    call strlen
    mov ebx, eax
    cmp ebx, 24
    jbe .oklen
    mov ebx, 24
.oklen:
    lea rdi, [r13+2]
    mov rsi, r12
    mov edx, ebx
    call memcpy
    mov byte [r13+2+rbx], 'm'
    mov byte [r13+3+rbx], 0
    pop r13
    pop r12
    pop rbx
    ret

; public alias
theme_set_token_body:
    ; rdi=body rsi=which: 0 path 1 num 2 ok 3 err 4 hdr 5 dim
    push rbx
    mov ebx, esi
    cmp ebx, 0
    jne .1
    lea rsi, [c_path]
    jmp .go
.1: cmp ebx, 1
    jne .2
    lea rsi, [c_num]
    jmp .go
.2: cmp ebx, 2
    jne .3
    lea rsi, [c_ok]
    jmp .go
.3: cmp ebx, 3
    jne .4
    lea rsi, [c_err]
    jmp .go
.4: cmp ebx, 4
    jne .5
    lea rsi, [c_hdr]
    jmp .go
.5: lea rsi, [c_dim]
.go:
    call theme_set_token_body_to
    pop rbx
    ret

; theme_try_user_file(rdi=name) → eax=1 if loaded
theme_try_user_file:
    push rbx
    push r12
    mov r12, rdi
    ; try XDG
    call env_get_xdg
    test rax, rax
    jz .home
    lea rdi, [theme_path]
    mov rsi, rax
    call strcpy_theme
    lea rdi, [theme_path]
    lea rsi, [suf_themes]
    call strcat_theme
    lea rdi, [theme_path]
    mov rsi, r12
    call strcat_theme
    lea rdi, [theme_path]
    lea rsi, [ext_theme]
    call strcat_theme
    lea rdi, [theme_path]
    call load_theme_file
    test eax, eax
    jnz .ok
.home:
    call env_get_home
    test rax, rax
    jz .no
    lea rdi, [theme_path]
    mov rsi, rax
    call strcpy_theme
    lea rdi, [theme_path]
    lea rsi, [suf_dot_th]
    call strcat_theme
    lea rdi, [theme_path]
    mov rsi, r12
    call strcat_theme
    lea rdi, [theme_path]
    lea rsi, [ext_theme]
    call strcat_theme
    lea rdi, [theme_path]
    call load_theme_file
    test eax, eax
    jnz .ok
.no:
    xor eax, eax
    jmp .out
.ok:
    mov eax, 1
.out:
    pop r12
    pop rbx
    ret

strcat_theme:
    push rbx
    push r12
    mov r12, rsi
    mov rbx, rdi
    call strlen
    lea rdi, [rbx + rax]
    mov rsi, r12
    call strcpy_theme
    pop r12
    pop rbx
    ret

env_get_xdg:
    push r12
    mov r12, [g_envp]
    test r12, r12
    jz .n
.lp:
    mov rdi, [r12]
    test rdi, rdi
    jz .n
    push rdi
    lea rsi, [env_xdg]
    call env_key_match
    pop rdi
    test al, al
    jnz .g
    add r12, 8
    jmp .lp
.g:
    push rdi
    lea rdi, [env_xdg]
    call strlen
    mov rcx, rax
    pop rdi
    add rdi, rcx
    cmp byte [rdi], '='
    jne .n
    inc rdi
    mov rax, rdi
    pop r12
    ret
.n: xor eax, eax
    pop r12
    ret

env_get_home:
    push r12
    mov r12, [g_envp]
    test r12, r12
    jz .n
.lp:
    mov rdi, [r12]
    test rdi, rdi
    jz .n
    push rdi
    lea rsi, [env_home]
    call env_key_match
    pop rdi
    test al, al
    jnz .g
    add r12, 8
    jmp .lp
.g:
    push rdi
    lea rdi, [env_home]
    call strlen
    mov rcx, rax
    pop rdi
    add rdi, rcx
    cmp byte [rdi], '='
    jne .n
    inc rdi
    mov rax, rdi
    pop r12
    ret
.n: xor eax, eax
    pop r12
    ret

; load_theme_file(rdi=path) → eax=1 if any key applied
load_theme_file:
    push rbx
    push r12
    push r13
    push r14
    mov r12, rdi
    mov rax, SYS_openat
    mov rdi, AT_FDCWD
    mov rsi, r12
    mov rdx, O_RDONLY
    xor r10, r10
    syscall
    cmp rax, -4096
    jae .fail
    mov r13, rax
    mov rax, SYS_read
    mov rdi, r13
    lea rsi, [theme_filebuf]
    mov rdx, 4095
    syscall
    mov r14, rax
    mov rdi, r13
    mov rax, SYS_close
    syscall
    test r14, r14
    jle .fail
    cmp r14, 4095
    jbe .oksz
    mov r14, 4095
.oksz:
    mov byte [theme_filebuf + r14], 0
    ; start from defaults so partial files work
    call theme_apply_default
    lea r12, [theme_filebuf]
    xor r13d, r13d                  ; applied count
.line:
    cmp byte [r12], 0
    je .done
    ; skip comments / blank
    mov al, [r12]
    cmp al, '#'
    je .skipl
    cmp al, ';'
    je .skipl
    cmp al, 10
    je .nl
    cmp al, 13
    je .nl
    ; parse key=val
    lea rdi, [line_tmp]
    mov rsi, r12
    xor ecx, ecx
.cp:
    mov al, [rsi]
    test al, al
    jz .eol
    cmp al, 10
    je .eol
    cmp al, 13
    je .eol
    cmp ecx, 250
    jae .eol
    mov [rdi + rcx], al
    inc ecx
    inc rsi
    jmp .cp
.eol:
    mov byte [line_tmp + rcx], 0
    ; advance r12 to next line
    mov r12, rsi
    call apply_theme_line
    test eax, eax
    jz .line
    inc r13d
    jmp .line
.skipl:
.nl:
    inc r12
    cmp byte [r12-1], 0
    je .done
    cmp byte [r12-1], 10
    je .line
    jmp .skipl
.done:
    test r13d, r13d
    jz .fail
    mov eax, 1
    jmp .out
.fail:
    xor eax, eax
.out:
    pop r14
    pop r13
    pop r12
    pop rbx
    ret

; apply_theme_line from line_tmp "key = body" → eax=1 if known key
apply_theme_line:
    push rbx
    push r12
    lea rbx, [line_tmp]
    ; strip spaces find =
    mov rdi, rbx
.sk:
    mov al, [rdi]
    cmp al, ' '
    je .s1
    cmp al, 9
    jne .k
.s1: inc rdi
    jmp .sk
.k:
    mov r12, rdi                    ; key start
.ke:
    mov al, [rdi]
    test al, al
    jz .no
    cmp al, '='
    je .eq
    cmp al, ' '
    je .sp
    cmp al, 9
    je .sp
    inc rdi
    jmp .ke
.sp:
    mov byte [rdi], 0
    inc rdi
    jmp .ke
.eq:
    mov byte [rdi], 0
    inc rdi
.vs:
    mov al, [rdi]
    cmp al, ' '
    je .v1
    cmp al, 9
    jne .val
.v1: inc rdi
    jmp .vs
.val:
    ; rdi = body, r12 = key
    mov rbx, rdi                    ; body
    mov rdi, r12
    lea rsi, [tk_path]
    call strcmp
    test eax, eax
    jnz .n1
    mov rdi, rbx
    lea rsi, [c_path]
    call theme_set_token_body_to
    ; also banner/spin
    mov rdi, rbx
    lea rsi, [c_banner]
    call theme_set_token_body_to
    mov rdi, rbx
    lea rsi, [c_spin]
    call theme_set_token_body_to
    jmp .yes
.n1:
    mov rdi, r12
    lea rsi, [tk_num]
    call strcmp
    test eax, eax
    jnz .n2
    mov rdi, rbx
    lea rsi, [c_num]
    call theme_set_token_body_to
    jmp .yes
.n2:
    mov rdi, r12
    lea rsi, [tk_ok]
    call strcmp
    test eax, eax
    jnz .n3
    mov rdi, rbx
    lea rsi, [c_ok]
    call theme_set_token_body_to
    jmp .yes
.n3:
    mov rdi, r12
    lea rsi, [tk_err]
    call strcmp
    test eax, eax
    jnz .n4
    mov rdi, rbx
    lea rsi, [c_err]
    call theme_set_token_body_to
    jmp .yes
.n4:
    mov rdi, r12
    lea rsi, [tk_hdr]
    call strcmp
    test eax, eax
    jnz .n5
    mov rdi, rbx
    lea rsi, [c_hdr]
    call theme_set_token_body_to
    jmp .yes
.n5:
    mov rdi, r12
    lea rsi, [tk_dim]
    call strcmp
    test eax, eax
    jnz .n6
    mov rdi, rbx
    lea rsi, [c_dim]
    call theme_set_token_body_to
    jmp .yes
.n6:
    mov rdi, r12
    lea rsi, [tk_banner]
    call strcmp
    test eax, eax
    jnz .n7
    mov rdi, rbx
    lea rsi, [c_banner]
    call theme_set_token_body_to
    jmp .yes
.n7:
    mov rdi, r12
    lea rsi, [tk_spin]
    call strcmp
    test eax, eax
    jnz .no
    mov rdi, rbx
    lea rsi, [c_spin]
    call theme_set_token_body_to
.yes:
    mov eax, 1
    jmp .out
.no:
    xor eax, eax
.out:
    pop r12
    pop rbx
    ret

; theme_list_print — gallery: name + token swatches + current mark
theme_list_print:
    push rbx
    push r12
    push r13
    push r14
    push r15
    ; save current theme name
    lea rsi, [g_theme_name]
    lea rdi, [body_tmp]
    call strcpy_theme_local
    lea rsi, [msg_cur]
    call out_str
    lea rsi, [g_theme_name]
    cmp byte [rsi], 0
    jne .cn
    lea rsi, [name_terminal]
.cn: call out_str
    lea rsi, [nl]
    call out_str
    lea rsi, [hdr_builtin]
    call out_str
    ; force color for swatches if TTY
    extern is_tty, g_color
    mov rdi, 1
    call is_tty
    test al, al
    jz .lp0
    mov byte [g_color], 1
.lp0:
    lea r12, [theme_table]
.lp:
    mov rdi, [r12]
    test rdi, rdi
    jz .rest
    ; apply theme for swatch (temporary)
    mov rdi, [r12]
    call theme_apply_name
    lea rsi, [sp2]
    call out_str
    mov rsi, [r12]
    call out_str
    lea rdi, [g_theme_name]
    mov rsi, [r12]
    ; mark if matches saved name in body_tmp
    lea rdi, [body_tmp]
    mov rsi, [r12]
    call strcmp
    test eax, eax
    jnz .nsw
    lea rsi, [star]
    call out_str
.nsw:
    lea rsi, [sp2]
    call out_str
    ; six swatches
    call color_path
    lea rsi, [sw]
    call out_str
    call color_reset
    call color_num
    lea rsi, [sw]
    call out_str
    call color_reset
    call color_ok
    lea rsi, [sw]
    call out_str
    call color_reset
    call color_err
    lea rsi, [sw]
    call out_str
    call color_reset
    call color_hdr
    lea rsi, [sw]
    call out_str
    call color_reset
    call color_dim
    lea rsi, [sw]
    call out_str
    call color_reset
    lea rsi, [nl]
    call out_str
    add r12, 56
    jmp .lp
.rest:
    ; restore previous theme
    lea rdi, [body_tmp]
    cmp byte [rdi], 0
    je .def
    call theme_apply_name
    jmp .note
.def:
    call theme_apply_default
.note:
    lea rsi, [msg_note]
    call out_str
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret

strcpy_theme_local:
    ; rdi=dst rsi=src max 47
    push rcx
    xor ecx, ecx
.lp:
    cmp ecx, 47
    jae .z
    mov al, [rsi+rcx]
    mov [rdi+rcx], al
    test al, al
    jz .d
    inc ecx
    jmp .lp
.z: mov byte [rdi+47], 0
.d: pop rcx
    ret

theme_current_name:
    lea rax, [g_theme_name]
    cmp byte [rax], 0
    jne .r
    lea rax, [name_terminal]
.r: ret

; theme_seed_user_dir(rdi=themes_dir absolute path) — write all builtins as .theme files
; eax = count written
global theme_seed_user_dir
theme_seed_user_dir:
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov r15, rdi                    ; base dir
    ; ensure dir exists
    mov rax, SYS_mkdir
    mov rdi, r15
    mov rsi, 0o755
    syscall
    lea r12, [theme_table]
    xor r14d, r14d
.lp:
    mov rdi, [r12]
    test rdi, rdi
    jz .done
    ; skip duplicate f00 (same as terminal bodies)
    lea rsi, [name_f00]
    call strcmp
    test eax, eax
    jz .nx
    ; build path: dir/name.theme into theme_path
    lea rdi, [theme_path]
    mov rsi, r15
    call strcpy_theme
    lea rdi, [theme_path]
    call strlen
    lea rdi, [theme_path + rax]
    mov byte [rdi], '/'
    inc rdi
    mov rsi, [r12]
    call strcpy_theme
    lea rdi, [theme_path]
    call strlen
    lea rdi, [theme_path + rax]
    lea rsi, [ext_theme]
    call strcpy_theme
    ; write file
    mov rax, SYS_openat
    mov rdi, AT_FDCWD
    lea rsi, [theme_path]
    mov rdx, O_WRONLY|O_CREAT|O_TRUNC
    mov r10, 0o644
    syscall
    cmp rax, -4096
    jae .nx
    mov r13, rax
    call seed_write_one
    mov rdi, r13
    mov rax, SYS_close
    syscall
    inc r14d
.nx:
    add r12, 56
    jmp .lp
.done:
    mov eax, r14d
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret

; seed_write_one: r13=fd, r12=theme row
seed_write_one:
    push rbx
    ; path
    lea rsi, [tk_path]
    mov rdi, [r12+8]
    call seed_write_kv
    lea rsi, [tk_num]
    mov rdi, [r12+16]
    call seed_write_kv
    lea rsi, [tk_ok]
    mov rdi, [r12+24]
    call seed_write_kv
    lea rsi, [tk_err]
    mov rdi, [r12+32]
    call seed_write_kv
    lea rsi, [tk_hdr]
    mov rdi, [r12+40]
    call seed_write_kv
    lea rsi, [tk_dim]
    mov rdi, [r12+48]
    call seed_write_kv
    pop rbx
    ret

; seed_write_kv(rsi=key, rdi=body) to fd r13
seed_write_kv:
    push rbx
    push r12
    mov rbx, rsi                    ; key
    mov r12, rdi                    ; body
    mov rdi, rbx
    call strlen
    mov rdx, rax
    mov rax, SYS_write
    mov rdi, r13
    mov rsi, rbx
    syscall
    lea rsi, [kv_eq]
    mov rdx, 3
    mov rax, SYS_write
    mov rdi, r13
    syscall
    mov rdi, r12
    call strlen
    mov rdx, rax
    mov rax, SYS_write
    mov rdi, r13
    mov rsi, r12
    syscall
    lea rsi, [nl]
    mov rdx, 1
    mov rax, SYS_write
    mov rdi, r13
    syscall
    pop r12
    pop rbx
    ret

kv_eq: db " = ", 0

; theme_count_builtins → eax
global theme_count_builtins
theme_count_builtins:
    push r12
    lea r12, [theme_table]
    xor eax, eax
.lp:
    cmp qword [r12], 0
    je .d
    inc eax
    add r12, 56
    jmp .lp
.d: pop r12
    ret

; theme_name_by_index(edi=0-based) → rax=name or 0
global theme_name_by_index
theme_name_by_index:
    push r12
    mov eax, edi
    lea r12, [theme_table]
.lp:
    cmp qword [r12], 0
    je .miss
    test eax, eax
    jz .hit
    dec eax
    add r12, 56
    jmp .lp
.hit:
    mov rax, [r12]
    pop r12
    ret
.miss:
    xor eax, eax
    pop r12
    ret
