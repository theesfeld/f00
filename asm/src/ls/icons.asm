; f00-asm — Nerd Font icons (subset of f00-format icons.rs)
BITS 64
DEFAULT REL
%include "syscalls.inc"

global icon_for_entry, icon_for_path, icon_enabled
extern g_opts2, g_tty, g_icons_when, g_color
extern strlen

section .rodata
; UTF-8 Nerd Font glyphs
ic_folder:      db 0xef, 0x81, 0xbb, 0          ; f07b
ic_folder_cfg:  db 0xee, 0x97, 0xbc, 0          ; e5fc
ic_file:        db 0xef, 0x85, 0x9b, 0          ; f15b
ic_link:        db 0xef, 0x83, 0x81, 0          ; f0c1
ic_exec:        db 0xef, 0x91, 0xb1, 0          ; f471
ic_rs:          db 0xee, 0x9e, 0xa8, 0          ; e7a8
ic_py:          db 0xee, 0x9c, 0xbc, 0          ; e73c
ic_js:          db 0xee, 0x9e, 0x81, 0          ; e781
ic_ts:          db 0xee, 0x98, 0xa8, 0          ; e628
ic_c:           db 0xee, 0x98, 0x9e, 0          ; e61e
ic_cpp:         db 0xee, 0x98, 0x9d, 0          ; e61d
ic_go:          db 0xee, 0x98, 0xa6, 0          ; e626
ic_md:          db 0xef, 0x92, 0x8a, 0          ; f48a
ic_json:        db 0xee, 0x98, 0x8b, 0          ; e60b
ic_toml:        db 0xee, 0x98, 0x95, 0          ; e615
ic_yml:         db 0xee, 0x9a, 0xa8, 0          ; e6a8
ic_sh:          db 0xef, 0x92, 0x89, 0          ; f489
ic_img:         db 0xef, 0x87, 0x85, 0          ; f1c5
ic_zip:         db 0xef, 0x87, 0x86, 0          ; f1c6
ic_git:         db 0xef, 0x87, 0x93, 0          ; f1d3
ic_html:        db 0xee, 0x9c, 0xb6, 0          ; e736
ic_css:         db 0xee, 0x9d, 0x89, 0          ; e749
ic_pdf:         db 0xef, 0x87, 0x81, 0          ; f1c1
ic_lock:        db 0xef, 0x80, 0xa3, 0          ; f023
ic_src:         db 0xef, 0x86, 0xb2, 0          ; f1b2
ic_cfg:         db 0xef, 0x80, 0x93, 0          ; f013
ic_space:       db " ", 0

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

section .text

icon_enabled:
    ; al = 1 if icons on (suite-wide modern chrome)
    movzx eax, byte [g_icons_when]
    cmp al, ICONS_ALWAYS
    je .yes
    cmp al, ICONS_NEVER
    je .no
    ; auto: modern TTY only — off under --core, NO_ICONS, g_color=0, non-TTY
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

; icon_for_entry(rdi=Entry*) → rsi = UTF-8 glyph (static), or empty
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
    test byte [rbx + Entry.flags], EF_LNK
    jnz .lnk
    cmp byte [rbx + Entry.dtype], DT_LNK
    je .lnk
    ; basename specials
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
    ; executable?
    test byte [rbx + Entry.flags], EF_EXEC
    jnz .maybe_exec
    ; extension
    mov rdi, [rbx + Entry.name]
    call find_ext_lc
    test rax, rax
    jz .file
    mov rdi, rax
    call map_ext
    test rsi, rsi
    jnz .done
.file:
    lea rsi, [ic_file]
    jmp .done
.maybe_exec:
    mov rdi, [rbx + Entry.name]
    call find_ext_lc
    test rax, rax
    jnz .file_check_ext
    lea rsi, [ic_exec]
    jmp .done
.file_check_ext:
    mov rdi, rax
    call map_ext
    test rsi, rsi
    jnz .done
    lea rsi, [ic_exec]
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
    lea rsi, [ic_folder_cfg]
    jmp .done
.ic_folder:
    lea rsi, [ic_folder]
    jmp .done
.ic_git:
    lea rsi, [ic_git]
    jmp .done
.ic_src:
    lea rsi, [ic_src]
    jmp .done
.ic_cfg:
    lea rsi, [ic_cfg]
    jmp .done
.ic_toml:
    lea rsi, [ic_toml]
    jmp .done
.lnk:
    lea rsi, [ic_link]
    jmp .done
.none:
    lea rsi, [empty_icon]
.done:
    pop rbx
    ret

; icon_for_path(rdi=path cstr) → rsi UTF-8 glyph (static) or empty
; Basename + extension map; used by cat headers, hash paths, etc.
icon_for_path:
    push rbx
    push r12
    mov r12, rdi
    call icon_enabled
    test al, al
    jz .none
    ; basename = after last '/'
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
    ; special basenames
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
    call map_ext
    test rsi, rsi
    jnz .done
.file:
    lea rsi, [ic_file]
    jmp .done
.ic_cfg:
    lea rsi, [ic_cfg]
    jmp .done
.ic_toml:
    lea rsi, [ic_toml]
    jmp .done
.none:
    lea rsi, [empty_icon]
.done:
    pop r12
    pop rbx
    ret

section .rodata
empty_icon: db 0

section .text

; streq(rdi, rsi) → al 1/0 case-sensitive
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

; find_ext_lc(rdi=name) → rax ptr to extension in scratch or 0
section .bss
ext_scratch: resb 32

section .text
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
    ; copy lowercased to ext_scratch
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

; map_ext(rdi=ext) → rsi icon or 0
map_ext:
    push rbx
    mov rbx, rdi
    mov rdi, rbx
    lea rsi, [ext_rs]
    call streq
    test al, al
    jz .1
    lea rsi, [ic_rs]
    jmp .ok
.1: mov rdi, rbx
    lea rsi, [ext_py]
    call streq
    test al, al
    jz .2
    lea rsi, [ic_py]
    jmp .ok
.2: mov rdi, rbx
    lea rsi, [ext_js]
    call streq
    test al, al
    jz .3
    lea rsi, [ic_js]
    jmp .ok
.3: mov rdi, rbx
    lea rsi, [ext_ts]
    call streq
    test al, al
    jz .4
    lea rsi, [ic_ts]
    jmp .ok
.4: mov rdi, rbx
    lea rsi, [ext_asm]
    call streq
    test al, al
    jz .5
    lea rsi, [ic_c]
    jmp .ok
.5: mov rdi, rbx
    lea rsi, [ext_c]
    call streq
    test al, al
    jz .6
    lea rsi, [ic_c]
    jmp .ok
.6: mov rdi, rbx
    lea rsi, [ext_go]
    call streq
    test al, al
    jz .7
    lea rsi, [ic_go]
    jmp .ok
.7: mov rdi, rbx
    lea rsi, [ext_md]
    call streq
    test al, al
    jz .8
    lea rsi, [ic_md]
    jmp .ok
.8: mov rdi, rbx
    lea rsi, [ext_json]
    call streq
    test al, al
    jz .9
    lea rsi, [ic_json]
    jmp .ok
.9: mov rdi, rbx
    lea rsi, [ext_toml]
    call streq
    test al, al
    jz .10
    lea rsi, [ic_toml]
    jmp .ok
.10: mov rdi, rbx
    lea rsi, [ext_yml]
    call streq
    test al, al
    jz .11
    lea rsi, [ic_yml]
    jmp .ok
.11: mov rdi, rbx
    lea rsi, [ext_sh]
    call streq
    test al, al
    jz .12
    lea rsi, [ic_sh]
    jmp .ok
.12: mov rdi, rbx
    lea rsi, [ext_png]
    call streq
    test al, al
    jz .13
    lea rsi, [ic_img]
    jmp .ok
.13: mov rdi, rbx
    lea rsi, [ext_zip]
    call streq
    test al, al
    jz .14
    lea rsi, [ic_zip]
    jmp .ok
.14: mov rdi, rbx
    lea rsi, [ext_html]
    call streq
    test al, al
    jz .15
    lea rsi, [ic_html]
    jmp .ok
.15: mov rdi, rbx
    lea rsi, [ext_so]
    call streq
    test al, al
    jz .no
    lea rsi, [ic_exec]
    jmp .ok
.no:
    xor esi, esi
    pop rbx
    ret
.ok:
    pop rbx
    ret
