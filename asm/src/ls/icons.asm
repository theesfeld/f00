; f00tils — icons: default single-width Unicode glyphs; emoji/nerd/ascii opt-in
; --icons=auto|glyph|emoji|nerd|ascii|never
BITS 64
DEFAULT REL
%include "syscalls.inc"

global icon_for_entry, icon_for_path, icon_enabled, icon_set_style_from_str
global icon_disp_cells
extern g_opts2, g_tty, g_icons_when, g_icons_style, g_color
extern strlen

; icon kind indices (must match tables)
%define IK_FOLDER     0
%define IK_FOLDER_CFG 1
%define IK_FILE       2
%define IK_LINK       3
%define IK_EXEC       4
%define IK_RS         5
%define IK_PY         6
%define IK_JS         7
%define IK_TS         8
%define IK_C          9
%define IK_CPP        10
%define IK_GO         11
%define IK_MD         12
%define IK_JSON       13
%define IK_TOML       14
%define IK_YML        15
%define IK_SH         16
%define IK_IMG        17
%define IK_ZIP        18
%define IK_GIT        19
%define IK_HTML       20
%define IK_CSS        21
%define IK_PDF        22
%define IK_LOCK       23
%define IK_SRC        24
%define IK_CFG        25
%define IK_COUNT      26

section .rodata
; ── glyph (default): single-width Unicode, no emoji, no Nerd Font ──
; ▸ U+25B8
gl_folder:      db 0xe2, 0x96, 0xb8, 0
; ▹ U+25B9
gl_folder_cfg:  db 0xe2, 0x96, 0xb9, 0
; · U+00B7
gl_file:        db 0xc2, 0xb7, 0
; ↪ U+21AA
gl_link:        db 0xe2, 0x86, 0xaa, 0
; ★ U+2605
gl_exec:        db 0xe2, 0x98, 0x85, 0
; λ U+03BB  (source-ish)
gl_rs:          db 0xce, 0xbb, 0
gl_py:          db 0xce, 0xbb, 0
gl_js:          db 0xce, 0xbb, 0
gl_ts:          db 0xce, 0xbb, 0
gl_c:           db 0xce, 0xbb, 0
gl_cpp:         db 0xce, 0xbb, 0
gl_go:          db 0xce, 0xbb, 0
; ¶ U+00B6
gl_md:          db 0xc2, 0xb6, 0
; ≡ U+2261  (structured data)
gl_json:        db 0xe2, 0x89, 0xa1, 0
gl_toml:        db 0xe2, 0x89, 0xa1, 0
gl_yml:         db 0xe2, 0x89, 0xa1, 0
; $
gl_sh:          db '$', 0
; ▣ U+25A3
gl_img:         db 0xe2, 0x96, 0xa3, 0
; ▤ U+25A4
gl_zip:         db 0xe2, 0x96, 0xa4, 0
; ⌥ U+2325  (branch-ish)
gl_git:         db 0xe2, 0x8c, 0xa5, 0
; @
gl_html:        db '@', 0
; ~
gl_css:         db '~', 0
; § U+00A7
gl_pdf:         db 0xc2, 0xa7, 0
; ⊘ U+2298
gl_lock:        db 0xe2, 0x8a, 0x98, 0
; ◇ U+25C7
gl_src:         db 0xe2, 0x97, 0x87, 0
; ∗ U+2217
gl_cfg:         db 0xe2, 0x88, 0x97, 0

; ── emoji (opt-in) ────────────────────────────────────────────────
em_folder:      db 0xf0, 0x9f, 0x93, 0x81, 0          ; 📁
em_folder_cfg:  db 0xf0, 0x9f, 0x93, 0x82, 0          ; 📂
em_file:        db 0xf0, 0x9f, 0x93, 0x84, 0          ; 📄
em_link:        db 0xe2, 0x86, 0xaa, 0                  ; ↪ (not 🔗)
em_exec:        db 0xe2, 0x9a, 0xa1, 0                  ; ⚡
em_rs:          db 0xf0, 0x9f, 0xa6, 0x80, 0            ; 🦀
em_py:          db 0xf0, 0x9f, 0x90, 0x8d, 0            ; 🐍
em_js:          db 0xf0, 0x9f, 0x92, 0xbb, 0            ; 💻
em_ts:          db 0xf0, 0x9f, 0x92, 0xbb, 0
em_c:           db 0xf0, 0x9f, 0x93, 0x9d, 0            ; 📝
em_cpp:         db 0xf0, 0x9f, 0x93, 0x9d, 0
em_go:          db 0xf0, 0x9f, 0x90, 0xb5, 0            ; 🐯 (legacy)
em_md:          db 0xf0, 0x9f, 0x93, 0x91, 0            ; 📑
em_json:        db 0xf0, 0x9f, 0x93, 0x8b, 0            ; 📋
em_toml:        db 0xf0, 0x9f, 0x93, 0x8b, 0
em_yml:         db 0xf0, 0x9f, 0x93, 0x8b, 0
em_sh:          db 0xf0, 0x9f, 0x90, 0xa7, 0            ; 🐧
em_img:         db 0xf0, 0x9f, 0x96, 0xbc, 0xef, 0xb8, 0x8f, 0
em_zip:         db 0xf0, 0x9f, 0x93, 0xa6, 0            ; 📦
em_git:         db 0xf0, 0x9f, 0x8c, 0xbf, 0            ; 🌿
em_html:        db 0xf0, 0x9f, 0x8c, 0x90, 0            ; 🌐
em_css:         db 0xf0, 0x9f, 0x8e, 0xa8, 0            ; 🎨
em_pdf:         db 0xf0, 0x9f, 0x93, 0x84, 0
em_lock:        db 0xf0, 0x9f, 0x94, 0x92, 0            ; 🔒
em_src:         db 0xf0, 0x9f, 0x93, 0x81, 0
em_cfg:         db 0xe2, 0x9a, 0x99, 0                  ; ⚙ (no VS16)

; ── Nerd Font PUA (opt-in) ────────────────────────────────────────
nf_folder:      db 0xef, 0x81, 0xbb, 0
nf_folder_cfg:  db 0xee, 0x97, 0xbc, 0
nf_file:        db 0xef, 0x85, 0x9b, 0
nf_link:        db 0xef, 0x83, 0x81, 0
nf_exec:        db 0xef, 0x91, 0xb1, 0
nf_rs:          db 0xee, 0x9e, 0xa8, 0
nf_py:          db 0xee, 0x9c, 0xbc, 0
nf_js:          db 0xee, 0x9e, 0x81, 0
nf_ts:          db 0xee, 0x98, 0xa8, 0
nf_c:           db 0xee, 0x98, 0x9e, 0
nf_cpp:         db 0xee, 0x98, 0x9d, 0
nf_go:          db 0xee, 0x98, 0xa6, 0
nf_md:          db 0xef, 0x92, 0x8a, 0
nf_json:        db 0xee, 0x98, 0x8b, 0
nf_toml:        db 0xee, 0x98, 0x95, 0
nf_yml:         db 0xee, 0x9a, 0xa8, 0
nf_sh:          db 0xef, 0x92, 0x89, 0
nf_img:         db 0xef, 0x87, 0x85, 0
nf_zip:         db 0xef, 0x87, 0x86, 0
nf_git:         db 0xef, 0x87, 0x93, 0
nf_html:        db 0xee, 0x9c, 0xb6, 0
nf_css:         db 0xee, 0x9d, 0x89, 0
nf_pdf:         db 0xef, 0x87, 0x81, 0
nf_lock:        db 0xef, 0x80, 0xa3, 0
nf_src:         db 0xef, 0x86, 0xb2, 0
nf_cfg:         db 0xef, 0x80, 0x93, 0

; ── ASCII (always works) ──────────────────────────────────────────
as_folder:      db "[D]", 0
as_folder_cfg:  db "[.]", 0
as_file:        db "[F]", 0
as_link:        db "[L]", 0
as_exec:        db "[*]", 0
as_rs:          db "[rs]", 0
as_py:          db "[py]", 0
as_js:          db "[js]", 0
as_ts:          db "[ts]", 0
as_c:           db "[c]", 0
as_cpp:         db "[c+]", 0
as_go:          db "[go]", 0
as_md:          db "[md]", 0
as_json:        db "[{}]", 0
as_toml:        db "[tm]", 0
as_yml:         db "[ym]", 0
as_sh:          db "[sh]", 0
as_img:         db "[im]", 0
as_zip:         db "[z]", 0
as_git:         db "[g]", 0
as_html:        db "[ht]", 0
as_css:         db "[cs]", 0
as_pdf:         db "[pd]", 0
as_lock:        db "[lk]", 0
as_src:         db "[src]", 0
as_cfg:         db "[cfg]", 0

empty_icon:     db 0

align 8
; pointer tables: glyph / emoji / nerd / ascii
tbl_glyph:
    dq gl_folder, gl_folder_cfg, gl_file, gl_link, gl_exec
    dq gl_rs, gl_py, gl_js, gl_ts, gl_c, gl_cpp, gl_go
    dq gl_md, gl_json, gl_toml, gl_yml, gl_sh, gl_img, gl_zip
    dq gl_git, gl_html, gl_css, gl_pdf, gl_lock, gl_src, gl_cfg
tbl_emoji:
    dq em_folder, em_folder_cfg, em_file, em_link, em_exec
    dq em_rs, em_py, em_js, em_ts, em_c, em_cpp, em_go
    dq em_md, em_json, em_toml, em_yml, em_sh, em_img, em_zip
    dq em_git, em_html, em_css, em_pdf, em_lock, em_src, em_cfg
tbl_nerd:
    dq nf_folder, nf_folder_cfg, nf_file, nf_link, nf_exec
    dq nf_rs, nf_py, nf_js, nf_ts, nf_c, nf_cpp, nf_go
    dq nf_md, nf_json, nf_toml, nf_yml, nf_sh, nf_img, nf_zip
    dq nf_git, nf_html, nf_css, nf_pdf, nf_lock, nf_src, nf_cfg
tbl_ascii:
    dq as_folder, as_folder_cfg, as_file, as_link, as_exec
    dq as_rs, as_py, as_js, as_ts, as_c, as_cpp, as_go
    dq as_md, as_json, as_toml, as_yml, as_sh, as_img, as_zip
    dq as_git, as_html, as_css, as_pdf, as_lock, as_src, as_cfg

ext_rs:         db "rs", 0
ext_py:         db "py", 0
ext_js:         db "js", 0
ext_ts:         db "ts", 0
ext_tsx:        db "tsx", 0
ext_jsx:        db "jsx", 0
ext_c:          db "c", 0
ext_h:          db "h", 0
ext_cpp:        db "cpp", 0
ext_cc:         db "cc", 0
ext_go:         db "go", 0
ext_md:         db "md", 0
ext_json:       db "json", 0
ext_toml:       db "toml", 0
ext_yml:        db "yml", 0
ext_yaml:       db "yaml", 0
ext_sh:         db "sh", 0
ext_bash:       db "bash", 0
ext_zsh:        db "zsh", 0
ext_png:        db "png", 0
ext_jpg:        db "jpg", 0
ext_jpeg:       db "jpeg", 0
ext_gif:        db "gif", 0
ext_svg:        db "svg", 0
ext_zip:        db "zip", 0
ext_tar:        db "tar", 0
ext_gz:         db "gz", 0
ext_html:       db "html", 0
ext_css:        db "css", 0
ext_pdf:        db "pdf", 0
ext_lock:       db "lock", 0
ext_asm:        db "asm", 0
ext_s:          db "s", 0
ext_txt:        db "txt", 0
ext_so:         db "so", 0

bn_git:         db ".git", 0
bn_github:      db ".github", 0
bn_src:         db "src", 0
bn_build:       db "build", 0
bn_target:      db "target", 0
bn_node:        db "node_modules", 0
bn_config:      db ".config", 0
bn_dockerfile:  db "Dockerfile", 0
bn_makefile:    db "Makefile", 0
bn_cargo:       db "Cargo.toml", 0

s_auto:         db "auto", 0
s_glyph:        db "glyph", 0
s_glyphs:       db "glyphs", 0
s_unicode:      db "unicode", 0
s_emoji:        db "emoji", 0
s_nerd:         db "nerd", 0
s_ascii:        db "ascii", 0
s_never:        db "never", 0
s_always:       db "always", 0
s_on:           db "on", 0
s_off:          db "off", 0

section .bss
ext_scratch: resb 32

section .text

; icon_resolve(eax=kind) → rsi glyph
icon_resolve:
    cmp eax, IK_COUNT
    jae .empty
    movzx ecx, byte [g_icons_style]
    cmp cl, ICONS_STYLE_EMOJI
    je .emoji
    cmp cl, ICONS_STYLE_NERD
    je .nerd
    cmp cl, ICONS_STYLE_ASCII
    je .ascii
    ; default / glyph
    lea rdx, [tbl_glyph]
    jmp .pick
.emoji:
    lea rdx, [tbl_emoji]
    jmp .pick
.nerd:
    lea rdx, [tbl_nerd]
    jmp .pick
.ascii:
    lea rdx, [tbl_ascii]
.pick:
    mov rsi, [rdx + rax*8]
    ret
.empty:
    lea rsi, [empty_icon]
    ret

; icon_disp_cells(rsi=glyph cstr) → eax terminal cells (no trailing space)
; glyph/nerd ≈ 1, emoji ≈ 2, ascii = strlen
icon_disp_cells:
    cmp byte [rsi], 0
    je .z
    movzx eax, byte [g_icons_style]
    cmp al, ICONS_STYLE_ASCII
    je .ascii
    cmp al, ICONS_STYLE_EMOJI
    je .emoji
    mov eax, 1
    ret
.emoji:
    mov eax, 2
    ret
.ascii:
    push rsi
    mov rdi, rsi
    call strlen
    pop rsi
    ret
.z:
    xor eax, eax
    ret

icon_enabled:
    movzx eax, byte [g_icons_when]
    cmp al, ICONS_ALWAYS
    je .yes
    cmp al, ICONS_NEVER
    je .no
    mov eax, [g_opts2]
    test eax, OPT2_NO_ICONS | OPT2_CORE
    jnz .no
    cmp byte [g_color], 0
    je .no
    cmp byte [g_tty], 0
    je .no
.yes:
    mov al, 1
    ret
.no:
    xor al, al
    ret

; icon_set_style_from_str(rdi=cstr)
; auto|glyph|glyphs|unicode|emoji|nerd|ascii|never|always|on|off
icon_set_style_from_str:
    push rbx
    mov rbx, rdi
    lea rsi, [s_never]
    call streq
    test al, al
    jnz .never
    mov rdi, rbx
    lea rsi, [s_off]
    call streq
    test al, al
    jnz .never
    mov rdi, rbx
    lea rsi, [s_ascii]
    call streq
    test al, al
    jnz .ascii
    mov rdi, rbx
    lea rsi, [s_nerd]
    call streq
    test al, al
    jnz .nerd
    mov rdi, rbx
    lea rsi, [s_emoji]
    call streq
    test al, al
    jnz .emoji
    mov rdi, rbx
    lea rsi, [s_glyph]
    call streq
    test al, al
    jnz .glyph
    mov rdi, rbx
    lea rsi, [s_glyphs]
    call streq
    test al, al
    jnz .glyph
    mov rdi, rbx
    lea rsi, [s_unicode]
    call streq
    test al, al
    jnz .glyph
    mov rdi, rbx
    lea rsi, [s_always]
    call streq
    test al, al
    jnz .always
    mov rdi, rbx
    lea rsi, [s_on]
    call streq
    test al, al
    jnz .always
    mov rdi, rbx
    lea rsi, [s_auto]
    call streq
    test al, al
    jnz .auto
    xor al, al
    pop rbx
    ret
.never:
    mov byte [g_icons_when], ICONS_NEVER
    mov al, 1
    pop rbx
    ret
.ascii:
    mov byte [g_icons_when], ICONS_ALWAYS
    mov byte [g_icons_style], ICONS_STYLE_ASCII
    mov al, 1
    pop rbx
    ret
.nerd:
    mov byte [g_icons_when], ICONS_ALWAYS
    mov byte [g_icons_style], ICONS_STYLE_NERD
    mov al, 1
    pop rbx
    ret
.emoji:
    mov byte [g_icons_when], ICONS_ALWAYS
    mov byte [g_icons_style], ICONS_STYLE_EMOJI
    mov al, 1
    pop rbx
    ret
.glyph:
    mov byte [g_icons_when], ICONS_ALWAYS
    mov byte [g_icons_style], ICONS_STYLE_GLYPH
    mov al, 1
    pop rbx
    ret
.always:
    mov byte [g_icons_when], ICONS_ALWAYS
    ; keep current style (default glyph)
    mov al, 1
    pop rbx
    ret
.auto:
    mov byte [g_icons_when], ICONS_AUTO
    mov byte [g_icons_style], ICONS_STYLE_GLYPH
    mov al, 1
    pop rbx
    ret

; icon_for_entry(rdi=Entry*) → rsi glyph
; Order: dir → name specials → exec (incl. followed symlink targets) →
;        extension → pure link → file
icon_for_entry:
    push rbx
    mov rbx, rdi
    call icon_enabled
    test al, al
    jz .none
    test byte [rbx + Entry.flags], EF_DIR
    jnz .dir
    cmp byte [rbx + Entry.dtype], DT_DIR
    je .dir

    mov rdi, [rbx + Entry.name]
    lea rsi, [bn_dockerfile]
    call streq
    test al, al
    jnz .ic_cfg
    mov rdi, [rbx + Entry.name]
    lea rsi, [bn_makefile]
    call streq
    test al, al
    jnz .ic_cfg
    mov rdi, [rbx + Entry.name]
    lea rsi, [bn_cargo]
    call streq
    test al, al
    jnz .ic_toml

    test byte [rbx + Entry.flags], EF_EXEC
    jnz .maybe_exec

    mov rdi, [rbx + Entry.name]
    call find_ext_lc
    test rax, rax
    jz .maybe_lnk
    mov rdi, rax
    call map_ext_kind
    cmp eax, -1
    je .maybe_lnk
    call icon_resolve
    jmp .done

.maybe_lnk:
    test byte [rbx + Entry.flags], EF_LNK
    jnz .lnk
    cmp byte [rbx + Entry.dtype], DT_LNK
    je .lnk
.file:
    mov eax, IK_FILE
    call icon_resolve
    jmp .done

.maybe_exec:
    mov rdi, [rbx + Entry.name]
    call find_ext_lc
    test rax, rax
    jnz .file_check_ext
    mov eax, IK_EXEC
    call icon_resolve
    jmp .done
.file_check_ext:
    mov rdi, rax
    call map_ext_kind
    cmp eax, -1
    je .ex
    call icon_resolve
    jmp .done
.ex:
    mov eax, IK_EXEC
    call icon_resolve
    jmp .done

.dir:
    mov rdi, [rbx + Entry.name]
    lea rsi, [bn_git]
    call streq
    test al, al
    jnz .ic_git
    mov rdi, [rbx + Entry.name]
    lea rsi, [bn_github]
    call streq
    test al, al
    jnz .ic_git
    mov rdi, [rbx + Entry.name]
    lea rsi, [bn_src]
    call streq
    test al, al
    jnz .ic_src
    mov rdi, [rbx + Entry.name]
    lea rsi, [bn_build]
    call streq
    test al, al
    jnz .ic_folder
    mov rdi, [rbx + Entry.name]
    lea rsi, [bn_target]
    call streq
    test al, al
    jnz .ic_folder
    mov rdi, [rbx + Entry.name]
    lea rsi, [bn_config]
    call streq
    test al, al
    jnz .ic_cfg
    mov rsi, [rbx + Entry.name]
    cmp byte [rsi], '.'
    jne .ic_folder
    mov eax, IK_FOLDER_CFG
    call icon_resolve
    jmp .done
.ic_folder:
    mov eax, IK_FOLDER
    call icon_resolve
    jmp .done
.ic_git:
    mov eax, IK_GIT
    call icon_resolve
    jmp .done
.ic_src:
    mov eax, IK_SRC
    call icon_resolve
    jmp .done
.ic_cfg:
    mov eax, IK_CFG
    call icon_resolve
    jmp .done
.ic_toml:
    mov eax, IK_TOML
    call icon_resolve
    jmp .done
.lnk:
    mov eax, IK_LINK
    call icon_resolve
    jmp .done
.none:
    lea rsi, [empty_icon]
.done:
    pop rbx
    ret

icon_for_path:
    push rbx
    push r12
    mov r12, rdi
    call icon_enabled
    test al, al
    jz .none
    mov rdi, r12
    call strlen
    lea rbx, [r12 + rax]
.scan:
    cmp rbx, r12
    jbe .base
    dec rbx
    cmp byte [rbx], '/'
    jne .scan
    inc rbx
.base:
    mov rdi, rbx
    lea rsi, [bn_makefile]
    call streq
    test al, al
    jnz .ic_cfg
    mov rdi, rbx
    lea rsi, [bn_dockerfile]
    call streq
    test al, al
    jnz .ic_cfg
    mov rdi, rbx
    lea rsi, [bn_cargo]
    call streq
    test al, al
    jnz .ic_toml
    mov rdi, rbx
    call find_ext_lc
    test rax, rax
    jz .file
    mov rdi, rax
    call map_ext_kind
    cmp eax, -1
    je .file
    call icon_resolve
    jmp .done
.file:
    mov eax, IK_FILE
    call icon_resolve
    jmp .done
.ic_cfg:
    mov eax, IK_CFG
    call icon_resolve
    jmp .done
.ic_toml:
    mov eax, IK_TOML
    call icon_resolve
    jmp .done
.none:
    lea rsi, [empty_icon]
.done:
    pop r12
    pop rbx
    ret

streq:
.lp:
    mov al, [rdi]
    mov cl, [rsi]
    cmp al, cl
    jne .no
    test al, al
    jz .yes
    inc rdi
    inc rsi
    jmp .lp
.yes:
    mov al, 1
    ret
.no:
    xor al, al
    ret

find_ext_lc:
    push rbx
    mov rbx, rdi
    call strlen
    lea rsi, [rbx + rax]
.lp:
    cmp rsi, rbx
    jbe .no
    dec rsi
    cmp byte [rsi], '.'
    je .yes
    cmp byte [rsi], '/'
    je .no
    jmp .lp
.yes:
    inc rsi
    lea rdi, [ext_scratch]
    xor ecx, ecx
.cp:
    mov al, [rsi]
    test al, al
    jz .end
    cmp al, 'A'
    jb .st
    cmp al, 'Z'
    ja .st
    add al, 32
.st:
    mov [rdi], al
    inc rsi
    inc rdi
    inc ecx
    cmp ecx, 30
    jb .cp
.end:
    mov byte [rdi], 0
    lea rax, [ext_scratch]
    pop rbx
    ret
.no:
    xor eax, eax
    pop rbx
    ret

; map_ext_kind(rdi=ext) → eax kind or -1
map_ext_kind:
    push rbx
    mov rbx, rdi
    mov rdi, rbx
    lea rsi, [ext_rs]
    call streq
    test al, al
    jz .1
    mov eax, IK_RS
    jmp .ok
.1: mov rdi, rbx
    lea rsi, [ext_py]
    call streq
    test al, al
    jz .2
    mov eax, IK_PY
    jmp .ok
.2: mov rdi, rbx
    lea rsi, [ext_js]
    call streq
    test al, al
    jz .3
    mov eax, IK_JS
    jmp .ok
.3: mov rdi, rbx
    lea rsi, [ext_ts]
    call streq
    test al, al
    jz .3b
    mov eax, IK_TS
    jmp .ok
.3b: mov rdi, rbx
    lea rsi, [ext_tsx]
    call streq
    test al, al
    jz .3c
    mov eax, IK_TS
    jmp .ok
.3c: mov rdi, rbx
    lea rsi, [ext_jsx]
    call streq
    test al, al
    jz .4
    mov eax, IK_JS
    jmp .ok
.4: mov rdi, rbx
    lea rsi, [ext_asm]
    call streq
    test al, al
    jz .4b
    mov eax, IK_C
    jmp .ok
.4b: mov rdi, rbx
    lea rsi, [ext_s]
    call streq
    test al, al
    jz .5
    mov eax, IK_C
    jmp .ok
.5: mov rdi, rbx
    lea rsi, [ext_c]
    call streq
    test al, al
    jz .5b
    mov eax, IK_C
    jmp .ok
.5b: mov rdi, rbx
    lea rsi, [ext_h]
    call streq
    test al, al
    jz .6
    mov eax, IK_C
    jmp .ok
.6: mov rdi, rbx
    lea rsi, [ext_cpp]
    call streq
    test al, al
    jz .6b
    mov eax, IK_CPP
    jmp .ok
.6b: mov rdi, rbx
    lea rsi, [ext_cc]
    call streq
    test al, al
    jz .7
    mov eax, IK_CPP
    jmp .ok
.7: mov rdi, rbx
    lea rsi, [ext_go]
    call streq
    test al, al
    jz .8
    mov eax, IK_GO
    jmp .ok
.8: mov rdi, rbx
    lea rsi, [ext_md]
    call streq
    test al, al
    jz .9
    mov eax, IK_MD
    jmp .ok
.9: mov rdi, rbx
    lea rsi, [ext_json]
    call streq
    test al, al
    jz .10
    mov eax, IK_JSON
    jmp .ok
.10: mov rdi, rbx
    lea rsi, [ext_toml]
    call streq
    test al, al
    jz .11
    mov eax, IK_TOML
    jmp .ok
.11: mov rdi, rbx
    lea rsi, [ext_yml]
    call streq
    test al, al
    jz .11b
    mov eax, IK_YML
    jmp .ok
.11b: mov rdi, rbx
    lea rsi, [ext_yaml]
    call streq
    test al, al
    jz .12
    mov eax, IK_YML
    jmp .ok
.12: mov rdi, rbx
    lea rsi, [ext_sh]
    call streq
    test al, al
    jz .12b
    mov eax, IK_SH
    jmp .ok
.12b: mov rdi, rbx
    lea rsi, [ext_bash]
    call streq
    test al, al
    jz .12c
    mov eax, IK_SH
    jmp .ok
.12c: mov rdi, rbx
    lea rsi, [ext_zsh]
    call streq
    test al, al
    jz .13
    mov eax, IK_SH
    jmp .ok
.13: mov rdi, rbx
    lea rsi, [ext_png]
    call streq
    test al, al
    jz .13b
    mov eax, IK_IMG
    jmp .ok
.13b: mov rdi, rbx
    lea rsi, [ext_jpg]
    call streq
    test al, al
    jz .13c
    mov eax, IK_IMG
    jmp .ok
.13c: mov rdi, rbx
    lea rsi, [ext_jpeg]
    call streq
    test al, al
    jz .13d
    mov eax, IK_IMG
    jmp .ok
.13d: mov rdi, rbx
    lea rsi, [ext_gif]
    call streq
    test al, al
    jz .13e
    mov eax, IK_IMG
    jmp .ok
.13e: mov rdi, rbx
    lea rsi, [ext_svg]
    call streq
    test al, al
    jz .14
    mov eax, IK_IMG
    jmp .ok
.14: mov rdi, rbx
    lea rsi, [ext_zip]
    call streq
    test al, al
    jz .14b
    mov eax, IK_ZIP
    jmp .ok
.14b: mov rdi, rbx
    lea rsi, [ext_tar]
    call streq
    test al, al
    jz .14c
    mov eax, IK_ZIP
    jmp .ok
.14c: mov rdi, rbx
    lea rsi, [ext_gz]
    call streq
    test al, al
    jz .15
    mov eax, IK_ZIP
    jmp .ok
.15: mov rdi, rbx
    lea rsi, [ext_html]
    call streq
    test al, al
    jz .16
    mov eax, IK_HTML
    jmp .ok
.16: mov rdi, rbx
    lea rsi, [ext_css]
    call streq
    test al, al
    jz .17
    mov eax, IK_CSS
    jmp .ok
.17: mov rdi, rbx
    lea rsi, [ext_pdf]
    call streq
    test al, al
    jz .18
    mov eax, IK_PDF
    jmp .ok
.18: mov rdi, rbx
    lea rsi, [ext_lock]
    call streq
    test al, al
    jz .19
    mov eax, IK_LOCK
    jmp .ok
.19: mov rdi, rbx
    lea rsi, [ext_so]
    call streq
    test al, al
    jz .no
    mov eax, IK_EXEC
    jmp .ok
.no:
    mov eax, -1
.ok:
    pop rbx
    ret
