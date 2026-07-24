; f00-config — inspect / list / set themes and show config paths
BITS 64
DEFAULT REL
%include "syscalls.inc"

global config_main
extern out_init, out_flush, out_str, out_byte
extern g_exit, g_tty, g_color, g_envp
extern suite_runtime_init
extern is_tty, color_init_default
extern theme_list_print, theme_apply_name, theme_current_name, theme_init
extern g_theme_name, g_cfg_theme
extern strcmp, strlen
extern color_path, color_num, color_ok, color_err, color_hdr, color_dim, color_reset

section .rodata
usage:
    db "Usage: f00-config [COMMAND]", 10
    db 10
    db "Commands:", 10
    db "  (none) | show     Show current theme and token preview", 10
    db "  theme | themes    List builtin themes (and how to add user themes)", 10
    db "  theme get         Print current theme name", 10
    db "  theme set NAME    Apply theme for this process (and print config hint)", 10
    db "  paths             Print XDG config / themes directories", 10
    db 10
    db "Config (XDG):", 10
    db "  ~/.config/f00/config          theme = dracula", 10
    db "  ~/.config/f00/themes/*.theme  user theme files", 10
    db 10
    db "Env: F00_THEME=name  (overrides config)", 10
    db 10
    db "Default theme 'terminal' uses ANSI 16 colors so your terminal palette", 10
    db "owns the hues. Named themes (dracula, catppuccin-mocha, …) use truecolor.", 10
    db 10
    db "f00tils · pure assembly · MIT · https://f00.sh", 10, 0
v_cfg: db "f00-config (f00) 0.15.9", 10, "License: MIT · https://f00.sh", 10, 0
s_theme: db "theme", 0
s_themes: db "themes", 0
s_show: db "show", 0
s_get: db "get", 0
s_set: db "set", 0
s_list: db "list", 0
s_paths: db "paths", 0
s_help: db "help", 0
s_ver: db "version", 0
lbl_theme: db "theme: ", 0
lbl_preview: db 10, "token preview:", 10, 0
pv_path: db "  path   ", 0
pv_num:  db "  num    ", 0
pv_ok:   db "  ok     ", 0
pv_err:  db "  err    ", 0
pv_hdr:  db "  hdr    ", 0
pv_dim:  db "  dim    ", 0
sample:  db "sample text", 0
nl:      db 10, 0
hint_set:
    db 10, "To persist, add to ~/.config/f00/config:", 10
    db "  theme = ", 0
hint_end: db 10, 0
paths_msg:
    db "Config file:  $XDG_CONFIG_HOME/f00/config", 10
    db "              ~/.config/f00/config", 10
    db "User themes:  ~/.config/f00/themes/<name>.theme", 10
    db 10
    db "Theme file format (SGR body only):", 10
    db "  path = 1;36", 10
    db "  num  = 38;2;241;250;140", 10
    db "  ok = …  err = …  hdr = …  dim = …", 10
    db "  banner = …  spin = …", 10, 0
err_unknown: db "f00-config: unknown theme or command", 10, 0
err_need: db "f00-config: theme set requires a name", 10, 0

section .text

config_main:
    push rbx
    push r12
    push r13
    push r14
    mov r12, rdi
    mov r13, rsi
    ; tiny_init already ran suite_runtime_init
    mov r14, 1
    cmp r12, 1
    jle .show
    mov rdi, [r13+8]
    cmp byte [rdi], '-'
    jne .cmd
    ; --help / --version
    add rdi, 2
    lea rsi, [s_help]
    call strcmp
    test eax, eax
    jz .help
    lea rsi, [s_ver]
    mov rdi, [r13+8]
    add rdi, 2
    call strcmp
    test eax, eax
    jz .ver
.cmd:
    mov rdi, [r13+8]
    lea rsi, [s_show]
    call strcmp
    test eax, eax
    jz .show
    mov rdi, [r13+8]
    lea rsi, [s_theme]
    call strcmp
    test eax, eax
    jz .theme
    mov rdi, [r13+8]
    lea rsi, [s_themes]
    call strcmp
    test eax, eax
    jz .list
    mov rdi, [r13+8]
    lea rsi, [s_list]
    call strcmp
    test eax, eax
    jz .list
    mov rdi, [r13+8]
    lea rsi, [s_paths]
    call strcmp
    test eax, eax
    jz .paths
    mov rdi, [r13+8]
    lea rsi, [s_help]
    call strcmp
    test eax, eax
    jz .help
    jmp .bad
.theme:
    cmp r12, 2
    jle .list
    mov rdi, [r13+16]
    lea rsi, [s_list]
    call strcmp
    test eax, eax
    jz .list
    mov rdi, [r13+16]
    lea rsi, [s_get]
    call strcmp
    test eax, eax
    jz .get
    mov rdi, [r13+16]
    lea rsi, [s_set]
    call strcmp
    test eax, eax
    jnz .list
    cmp r12, 4
    jl .need
    mov rdi, [r13+24]
    call theme_apply_name
    test eax, eax
    jz .bad
    ; show + persist hint
    call .do_show
    lea rsi, [hint_set]
    call out_str
    mov rsi, [r13+24]
    call out_str
    lea rsi, [hint_end]
    call out_str
    jmp .exit
.list:
    call theme_list_print
    jmp .exit
.get:
    call theme_current_name
    mov rsi, rax
    call out_str
    mov dil, 10
    call out_byte
    jmp .exit
.show:
    call .do_show
    jmp .exit
.paths:
    lea rsi, [paths_msg]
    call out_str
    jmp .exit
.help:
    lea rsi, [usage]
    call out_str
    jmp .exit
.ver:
    lea rsi, [v_cfg]
    call out_str
    jmp .exit
.need:
    lea rsi, [err_need]
    call out_str
    mov dword [g_exit], 1
    jmp .exit
.bad:
    lea rsi, [err_unknown]
    call out_str
    mov dword [g_exit], 1
.exit:
    call out_flush
    pop r14
    pop r13
    pop r12
    pop rbx
    ret

.do_show:
    push rbx
    lea rsi, [lbl_theme]
    call out_str
    call theme_current_name
    mov rsi, rax
    call out_str
    lea rsi, [nl]
    call out_str
    lea rsi, [lbl_preview]
    call out_str
    ; force color for preview if TTY
    mov rdi, 1
    call is_tty
    test al, al
    jz .plain
    mov byte [g_color], 1
.plain:
    lea rsi, [pv_path]
    call out_str
    call color_path
    lea rsi, [sample]
    call out_str
    call color_reset
    lea rsi, [nl]
    call out_str
    lea rsi, [pv_num]
    call out_str
    call color_num
    lea rsi, [sample]
    call out_str
    call color_reset
    lea rsi, [nl]
    call out_str
    lea rsi, [pv_ok]
    call out_str
    call color_ok
    lea rsi, [sample]
    call out_str
    call color_reset
    lea rsi, [nl]
    call out_str
    lea rsi, [pv_err]
    call out_str
    call color_err
    lea rsi, [sample]
    call out_str
    call color_reset
    lea rsi, [nl]
    call out_str
    lea rsi, [pv_hdr]
    call out_str
    call color_hdr
    lea rsi, [sample]
    call out_str
    call color_reset
    lea rsi, [nl]
    call out_str
    lea rsi, [pv_dim]
    call out_str
    call color_dim
    lea rsi, [sample]
    call out_str
    call color_reset
    lea rsi, [nl]
    call out_str
    pop rbx
    ret
