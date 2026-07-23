; f00-asm — dual-pane TUI browser with mark/copy/move/delete (f00-tui parity)
BITS 64
DEFAULT REL
%include "syscalls.inc"

global tui_browse
extern list_path
extern g_entries, g_entry_count, g_opts, g_opts2, g_exit, g_tty, g_cols, g_color
extern out_init, out_flush, out_str, out_byte, out_u64
extern arena_alloc, memcpy, strlen
extern emit_name_public
extern get_winsize
extern color_path, color_num, color_reset

%define MAX_MARK 256
%define MAX_ENT  4096

section .bss
alignb 8
tios_orig: resb TIOS_SIZE
tios_raw:  resb TIOS_SIZE
; two panes
pane0_cwd: resb 4096
pane1_cwd: resb 4096
pane0_sel: resq 1
pane1_sel: resq 1
; snapshot of entries per pane: array of name pointers + flags (dir)
p0_count:  resq 1
p1_count:  resq 1
p0_names:  resq MAX_ENT
p1_names:  resq MAX_ENT
p0_dirs:   resb MAX_ENT
p1_dirs:   resb MAX_ENT
p0_mark:   resb MAX_ENT
p1_mark:   resb MAX_ENT
active:    resq 1                   ; 0 or 1
dual:      resb 1
confirm:   resb 1                   ; 0 none 1 copy 2 move 3 delete
keybuf:    resb 16
status:    resb 256
join_buf:  resb 8192
path_a:    resb 4096
path_b:    resb 4096

section .rodata
ansi_home:  db 27,"[H",0
ansi_clear: db 27,"[H",27,"[2J",0
ansi_el:    db 27,"[K",0            ; erase to end of line
ansi_rev:   db 27,"[7m",0
ansi_sgr0:  db 27,"[0m",0
ansi_dim:   db 27,"[2m",0
ansi_bold:  db 27,"[1m",0
ansi_hide:  db 27,"[?25l",0
ansi_show:  db 27,"[?25h",0
ansi_alt:   db 27,"[?1049h",0
ansi_alt0:  db 27,"[?1049l",0
hdr:        db "f00-tui",0
sp2:        db "  ",0
mid_dot:    db "  ·  ",0
slash_n:    db "/",0
items_lbl:  db " items",0
sel_lbl:    db "  sel ",0
; compact key help (status / chrome line)
keys_help:  db "j/k or arrows  enter  space mark  Tab pane  | dual  c/m/d  q",0
sep:        db " │ ",0
st_copy:    db " COPY → other pane?  y/n ",0
st_move:    db " MOVE → other pane?  y/n ",0
st_del:     db " DELETE marked/cursor?  y/n ",0
st_ok:      db "ok",0
dot:        db ".",0
nl:         db 10,0

section .text

tui_browse:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    push r14
    push r15
    ; start path
    test rdi, rdi
    jnz .has
    lea rdi, [dot]
.has:
    lea rsi, [pane0_cwd]
    call strcpy
    lea rsi, [pane1_cwd]
    lea rdi, [pane0_cwd]
    call strcpy
    mov qword [active], 0
    mov byte [dual], 1
    mov byte [confirm], 0
    mov qword [pane0_sel], 0
    mov qword [pane1_sel], 0

    or dword [g_opts], OPT_ALL | OPT_ONE
    and dword [g_opts2], ~OPT2_CORE
    or dword [g_opts2], OPT2_GIT
    ; interactive TUI always uses semantic chrome colors
    mov byte [g_color], 1
    mov byte [g_tty], 1

    call out_init
    call raw_on
    lea rsi, [ansi_alt]
    call out_str
    lea rsi, [ansi_hide]
    call out_str
    call out_flush
    call reload_active
    call reload_other
    call draw

.loop:
    call read_key
    cmp al, 'q'
    je .quit
    cmp al, 3
    je .quit
    cmp byte [confirm], 0
    je .nav
    cmp al, 'y'
    je .do_yes
    cmp al, 'Y'
    je .do_yes
    cmp al, 'n'
    je .do_no
    cmp al, 'N'
    je .do_no
    cmp al, 27
    je .do_no
    jmp .loop
.nav:
    cmp al, 'j'
    je .down
    cmp al, 14
    je .down
    cmp al, 'k'
    je .up
    cmp al, 16
    je .up
    cmp al, 'g'
    je .top
    cmp al, 'G'
    je .bot
    cmp al, 'h'
    je .updir
    cmp al, 127
    je .updir
    cmp al, 8
    je .updir
    cmp al, 'l'
    je .enter
    cmp al, 10
    je .enter
    cmp al, 13
    je .enter
    cmp al, ' '
    je .mark
    cmp al, 9                       ; tab
    je .swpane
    cmp al, '|'
    je .tdual
    cmp al, '\'
    je .tdual
    cmp al, 'c'
    je .beg_copy
    cmp al, 'm'
    je .beg_move
    cmp al, 'd'
    je .beg_del
    jmp .loop

.down:
    call sel_ptr
    mov rcx, [rax]
    call cnt_ptr
    mov rdx, [rax]
    inc rcx
    cmp rcx, rdx
    jae .loop
    call sel_ptr
    mov [rax], rcx
    call draw
    jmp .loop
.up:
    call sel_ptr
    mov rcx, [rax]
    test rcx, rcx
    jz .loop
    dec rcx
    mov [rax], rcx
    call draw
    jmp .loop
.top:
    call sel_ptr
    mov qword [rax], 0
    call draw
    jmp .loop
.bot:
    call cnt_ptr
    mov rcx, [rax]
    test rcx, rcx
    jz .loop
    dec rcx
    call sel_ptr
    mov [rax], rcx
    call draw
    jmp .loop
.updir:
    call cwd_ptr
    mov rdi, rax
    call dirname
    call sel_ptr
    mov qword [rax], 0
    call reload_active
    call draw
    jmp .loop
.enter:
    call sel_ptr
    mov rcx, [rax]
    call cnt_ptr
    cmp rcx, [rax]
    jae .loop
    call names_ptr
    mov rsi, [rax + rcx*8]
    call dirs_ptr
    cmp byte [rax + rcx], 0
    je .loop
    call cwd_ptr
    mov rdi, rax
    call path_join
    call sel_ptr
    mov qword [rax], 0
    call reload_active
    call draw
    jmp .loop
.mark:
    call sel_ptr
    mov rcx, [rax]
    call cnt_ptr
    cmp rcx, [rax]
    jae .loop
    call marks_ptr
    xor byte [rax + rcx], 1
    call draw
    jmp .loop
.swpane:
    cmp byte [dual], 0
    je .loop
    xor qword [active], 1
    call draw
    jmp .loop
.tdual:
    xor byte [dual], 1
    call draw
    jmp .loop
.beg_copy:
    mov byte [confirm], 1
    call draw
    jmp .loop
.beg_move:
    mov byte [confirm], 2
    call draw
    jmp .loop
.beg_del:
    mov byte [confirm], 3
    call draw
    jmp .loop
.do_no:
    mov byte [confirm], 0
    call draw
    jmp .loop
.do_yes:
    movzx eax, byte [confirm]
    mov byte [confirm], 0
    cmp al, 1
    je .exec_copy
    cmp al, 2
    je .exec_move
    cmp al, 3
    je .exec_del
    jmp .loop
.exec_copy:
    call op_copy
    call reload_active
    call reload_other
    call draw
    jmp .loop
.exec_move:
    call op_move
    call reload_active
    call reload_other
    call draw
    jmp .loop
.exec_del:
    call op_delete
    call reload_active
    call draw
    jmp .loop

.quit:
    lea rsi, [ansi_show]
    call out_str
    lea rsi, [ansi_alt0]
    call out_str
    call out_flush
    call raw_off
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    ret

;---- helpers: active pane pointers ----
sel_ptr:
    cmp qword [active], 0
    jne .1
    lea rax, [pane0_sel]
    ret
.1: lea rax, [pane1_sel]
    ret
cnt_ptr:
    cmp qword [active], 0
    jne .1
    lea rax, [p0_count]
    ret
.1: lea rax, [p1_count]
    ret
names_ptr:
    cmp qword [active], 0
    jne .1
    lea rax, [p0_names]
    ret
.1: lea rax, [p1_names]
    ret
dirs_ptr:
    cmp qword [active], 0
    jne .1
    lea rax, [p0_dirs]
    ret
.1: lea rax, [p1_dirs]
    ret
marks_ptr:
    cmp qword [active], 0
    jne .1
    lea rax, [p0_mark]
    ret
.1: lea rax, [p1_mark]
    ret
cwd_ptr:
    cmp qword [active], 0
    jne .1
    lea rax, [pane0_cwd]
    ret
.1: lea rax, [pane1_cwd]
    ret
other_cwd:
    cmp qword [active], 0
    jne .0
    lea rax, [pane1_cwd]
    ret
.0: lea rax, [pane0_cwd]
    ret

reload_active:
    call cwd_ptr
    mov rdi, rax
    call snapshot_dir
    ret
reload_other:
    cmp byte [dual], 0
    je .r
    ; temporarily flip
    xor qword [active], 1
    call reload_active
    xor qword [active], 1
.r: ret

; snapshot_dir(rdi=cwd path) into active pane tables
snapshot_dir:
    push rbx
    push r12
    mov r12, rdi
    mov qword [g_entry_count], 0
    mov rdi, r12
    call list_path
    call cnt_ptr
    mov qword [rax], 0
    call marks_ptr
    mov rdi, rax
    xor esi, esi
    mov rdx, MAX_ENT
    call memset_local
    mov r8, [g_entries]
    xor ebx, ebx
.lp:
    cmp rbx, [g_entry_count]
    jae .done
    cmp rbx, MAX_ENT
    jae .done
    mov r9, [r8 + rbx*8]
    mov rsi, [r9 + Entry.name]
    call strdup
    mov r10, rax                    ; name copy
    call names_ptr
    mov [rax + rbx*8], r10
    call dirs_ptr
    mov cl, 0
    test byte [r9 + Entry.flags], EF_DIR
    jnz .isd
    cmp byte [r9 + Entry.dtype], DT_DIR
    jne .setd
.isd:
    mov cl, 1
.setd:
    mov [rax + rbx], cl
.nd:
    call cnt_ptr
    inc qword [rax]
    inc rbx
    jmp .lp
.done:
    pop r12
    pop rbx
    ret

strdup:
    push rsi
    mov rdi, rsi
    call strlen
    lea rdi, [rax+1]
    push rax
    call arena_alloc
    pop rdx
    pop rsi
    mov rdi, rax
    push rax
    call memcpy
    pop rax
    mov byte [rax + rdx], 0
    ret

memset_local:
    ; rdi dst sil val rdx len
.lp:
    test rdx, rdx
    jz .d
    mov [rdi], sil
    inc rdi
    dec rdx
    jmp .lp
.d: ret

; draw — single buffered frame (out_* → one flush) to avoid flicker
draw:
    call out_init
    lea rsi, [ansi_clear]
    call out_str
    call draw_status_bar
    call draw_keys_line
    cmp byte [confirm], 0
    je .list
    call draw_confirm
.list:
    cmp byte [dual], 0
    jne .dual_draw
    call draw_pane_full
    jmp .flush
.dual_draw:
    call draw_dual
.flush:
    ; ensure SGR clean at end of frame
    lea rsi, [ansi_sgr0]
    call out_str
    call out_flush
    ret

; status: dim chrome + cyan path + yellow count/sel
draw_status_bar:
    ; title (dim)
    lea rsi, [ansi_dim]
    call out_str
    lea rsi, [hdr]
    call out_str
    lea rsi, [sp2]
    call out_str
    lea rsi, [ansi_sgr0]
    call out_str
    ; path (cyan / path token)
    call color_path
    call cwd_ptr
    mov rsi, rax
    call out_str
    call color_reset
    ; mid-dot chrome
    lea rsi, [ansi_dim]
    call out_str
    lea rsi, [mid_dot]
    call out_str
    lea rsi, [ansi_sgr0]
    call out_str
    ; count (yellow)
    call color_num
    call cnt_ptr
    mov rdi, [rax]
    call out_u64
    call color_reset
    lea rsi, [ansi_dim]
    call out_str
    lea rsi, [items_lbl]
    call out_str
    lea rsi, [sel_lbl]
    call out_str
    lea rsi, [ansi_sgr0]
    call out_str
    call color_num
    call sel_ptr
    mov rdi, [rax]
    ; display 1-based index when non-empty
    call cnt_ptr
    cmp qword [rax], 0
    je .nosel
    inc rdi
.nosel:
    call out_u64
    call color_reset
    lea rsi, [ansi_dim]
    call out_str
    mov dil, '/'
    call out_byte
    lea rsi, [ansi_sgr0]
    call out_str
    call color_num
    call cnt_ptr
    mov rdi, [rax]
    call out_u64
    call color_reset
    ; dual pane secondary path hint
    cmp byte [dual], 0
    je .nl
    lea rsi, [ansi_dim]
    call out_str
    lea rsi, [mid_dot]
    call out_str
    lea rsi, [sep]
    call out_str
    lea rsi, [ansi_sgr0]
    call out_str
    call color_path
    call other_cwd
    mov rsi, rax
    call out_str
    call color_reset
.nl:
    lea rsi, [ansi_el]
    call out_str
    mov dil, 10
    call out_byte
    ret

draw_keys_line:
    lea rsi, [ansi_dim]
    call out_str
    lea rsi, [keys_help]
    call out_str
    lea rsi, [ansi_sgr0]
    call out_str
    lea rsi, [ansi_el]
    call out_str
    mov dil, 10
    call out_byte
    ret

draw_confirm:
    lea rsi, [ansi_rev]
    call out_str
    cmp byte [confirm], 1
    jne .c2
    lea rsi, [st_copy]
    jmp .cst
.c2: cmp byte [confirm], 2
    jne .c3
    lea rsi, [st_move]
    jmp .cst
.c3: lea rsi, [st_del]
.cst:
    call out_str
    lea rsi, [ansi_sgr0]
    call out_str
    lea rsi, [ansi_el]
    call out_str
    mov dil, 10
    call out_byte
    ret

draw_pane_full:
    call cnt_ptr
    mov r12, [rax]
    call names_ptr
    mov r13, rax
    call marks_ptr
    mov r14, rax
    call sel_ptr
    mov r15, [rax]
    xor rbx, rbx
.lp:
    cmp rbx, r12
    jae .d
    cmp rbx, r15
    jne .nr
    lea rsi, [ansi_rev]
    call out_str
.nr:
    ; mark column (dim when not selected)
    cmp rbx, r15
    je .mk
    lea rsi, [ansi_dim]
    call out_str
.mk:
    cmp byte [r14 + rbx], 0
    je .nm
    mov dil, '*'
    call out_byte
    jmp .nm2
.nm:
    mov dil, ' '
    call out_byte
.nm2:
    mov dil, ' '
    call out_byte
    cmp rbx, r15
    je .nm3
    lea rsi, [ansi_sgr0]
    call out_str
    lea rsi, [ansi_bold]
    call out_str
.nm3:
    mov rsi, [r13 + rbx*8]
    call out_str
    call dirs_ptr
    cmp byte [rax + rbx], 0
    je .nf
    mov dil, '/'
    call out_byte
.nf:
    lea rsi, [ansi_sgr0]
    call out_str
    lea rsi, [ansi_el]
    call out_str
    mov dil, 10
    call out_byte
    inc rbx
    jmp .lp
.d: ret

draw_dual:
    ; left = pane0, right = pane1; reverse video only on active pane selection
    mov r12, [p0_count]
    mov r13, [p1_count]
    mov rax, r12
    cmp rax, r13
    jae .mx
    mov rax, r13
.mx:
    mov r14, rax                    ; rows
    xor rbx, rbx
.row:
    cmp rbx, r14
    jae .d
    ; ---- left (pane0) ----
    cmp rbx, [p0_count]
    jae .padL
    xor r15, r15                    ; flag: selected
    cmp qword [active], 0
    jne .l1
    cmp rbx, [pane0_sel]
    jne .l1
    mov r15, 1
    lea rsi, [ansi_rev]
    call out_str
.l1:
    test r15, r15
    jnz .lm
    lea rsi, [ansi_dim]
    call out_str
.lm:
    cmp byte [p0_mark + rbx], 0
    je .l2
    mov dil, '*'
    call out_byte
    jmp .l3
.l2:
    mov dil, ' '
    call out_byte
.l3:
    test r15, r15
    jnz .l3b
    lea rsi, [ansi_sgr0]
    call out_str
    lea rsi, [ansi_bold]
    call out_str
.l3b:
    mov rsi, [p0_names + rbx*8]
    call out_str
    cmp byte [p0_dirs + rbx], 0
    je .l4
    mov dil, '/'
    call out_byte
.l4:
    lea rsi, [ansi_sgr0]
    call out_str
.padL:
    ; dim separator chrome
    lea rsi, [ansi_dim]
    call out_str
    lea rsi, [sep]
    call out_str
    lea rsi, [ansi_sgr0]
    call out_str
    ; ---- right (pane1) ----
    cmp rbx, [p1_count]
    jae .nl
    xor r15, r15
    cmp qword [active], 1
    jne .r1
    cmp rbx, [pane1_sel]
    jne .r1
    mov r15, 1
    lea rsi, [ansi_rev]
    call out_str
.r1:
    test r15, r15
    jnz .rm
    lea rsi, [ansi_dim]
    call out_str
.rm:
    cmp byte [p1_mark + rbx], 0
    je .r2
    mov dil, '*'
    call out_byte
    jmp .r3
.r2:
    mov dil, ' '
    call out_byte
.r3:
    test r15, r15
    jnz .r3b
    lea rsi, [ansi_sgr0]
    call out_str
    lea rsi, [ansi_bold]
    call out_str
.r3b:
    mov rsi, [p1_names + rbx*8]
    call out_str
    cmp byte [p1_dirs + rbx], 0
    je .r4
    mov dil, '/'
    call out_byte
.r4:
    lea rsi, [ansi_sgr0]
    call out_str
.nl:
    lea rsi, [ansi_el]
    call out_str
    mov dil, 10
    call out_byte
    inc rbx
    jmp .row
.d: ret

;---- file ops ----
; build full path of marked or cursor into path_a, dest dir path_b
op_copy:
    call other_cwd
    lea rdi, [path_b]
    mov rsi, rax
    call strcpy
    call iter_selected_copy
    ret
op_move:
    call other_cwd
    lea rdi, [path_b]
    mov rsi, rax
    call strcpy
    call iter_selected_move
    ret
op_delete:
    call iter_selected_del
    ret

iter_selected_copy:
    push rbx
    call cnt_ptr
    mov r12, [rax]
    call marks_ptr
    mov r13, rax
    call names_ptr
    mov r14, rax
    xor ebx, ebx
    xor r15, r15                    ; any marked?
.chk:
    cmp rbx, r12
    jae .chd
    cmp byte [r13 + rbx], 0
    je .cn
    mov r15, 1
.cn: inc rbx
    jmp .chk
.chd:
    xor ebx, ebx
.lp:
    cmp rbx, r12
    jae .done
    test r15, r15
    jz .cursor
    cmp byte [r13 + rbx], 0
    je .n
    jmp .do
.cursor:
    call sel_ptr
    cmp rbx, [rax]
    jne .n
.do:
    ; path_a = cwd/name
    call cwd_ptr
    lea rdi, [path_a]
    mov rsi, rax
    call strcpy
    lea rdi, [path_a]
    mov rsi, [r14 + rbx*8]
    call path_join
    ; dest = path_b/name
    lea rdi, [join_buf]
    lea rsi, [path_b]
    call strcpy
    lea rdi, [join_buf]
    mov rsi, [r14 + rbx*8]
    call path_join
    lea rdi, [path_a]
    lea rsi, [join_buf]
    call file_copy
.n: inc rbx
    jmp .lp
.done:
    pop rbx
    ret

iter_selected_move:
    push rbx
    call cnt_ptr
    mov r12, [rax]
    call marks_ptr
    mov r13, rax
    call names_ptr
    mov r14, rax
    xor ebx, ebx
    xor r15, r15
.chk:
    cmp rbx, r12
    jae .go
    cmp byte [r13+rbx], 0
    je .c
    mov r15, 1
.c: inc rbx
    jmp .chk
.go:
    xor ebx, ebx
.lp:
    cmp rbx, r12
    jae .d
    test r15, r15
    jz .cur
    cmp byte [r13+rbx], 0
    je .n
    jmp .do
.cur:
    call sel_ptr
    cmp rbx, [rax]
    jne .n
.do:
    call cwd_ptr
    lea rdi, [path_a]
    mov rsi, rax
    call strcpy
    lea rdi, [path_a]
    mov rsi, [r14+rbx*8]
    call path_join
    lea rdi, [join_buf]
    lea rsi, [path_b]
    call strcpy
    lea rdi, [join_buf]
    mov rsi, [r14+rbx*8]
    call path_join
    lea rdi, [path_a]
    lea rsi, [join_buf]
    call file_move
.n: inc rbx
    jmp .lp
.d: pop rbx
    ret

iter_selected_del:
    push rbx
    call cnt_ptr
    mov r12, [rax]
    call marks_ptr
    mov r13, rax
    call names_ptr
    mov r14, rax
    xor ebx, ebx
    xor r15, r15
.chk:
    cmp rbx, r12
    jae .go
    cmp byte [r13+rbx],0
    je .c
    mov r15,1
.c: inc rbx
    jmp .chk
.go:
    xor ebx, ebx
.lp:
    cmp rbx, r12
    jae .d
    test r15, r15
    jz .cur
    cmp byte [r13+rbx],0
    je .n
    jmp .do
.cur:
    call sel_ptr
    cmp rbx,[rax]
    jne .n
.do:
    call cwd_ptr
    lea rdi,[path_a]
    mov rsi,rax
    call strcpy
    lea rdi,[path_a]
    mov rsi,[r14+rbx*8]
    call path_join
    lea rdi,[path_a]
    call file_unlink
.n: inc rbx
    jmp .lp
.d: pop rbx
    ret

file_copy:
    ; rdi=src rsi=dst — read/write loop
    push rbx
    push r12
    push r13
    mov r12, rdi
    mov r13, rsi
    mov rax, SYS_openat
    mov rdi, AT_FDCWD
    mov rsi, r12
    mov rdx, O_RDONLY|O_CLOEXEC
    xor r10,r10
    syscall
    cmp rax,-4096
    jae .fail
    mov rbx, rax
    mov rax, SYS_openat
    mov rdi, AT_FDCWD
    mov rsi, r13
    mov rdx, O_WRONLY|O_CREAT|O_TRUNC|O_CLOEXEC
    mov r10, 0o644
    syscall
    cmp rax,-4096
    jae .failc
    mov r12, rax
.rd:
    mov rax, SYS_read
    mov rdi, rbx
    lea rsi, [join_buf+4096]
    mov rdx, 4096
    syscall
    test rax, rax
    jle .done
    mov rdx, rax
    mov rax, SYS_write
    mov rdi, r12
    lea rsi, [join_buf+4096]
    syscall
    jmp .rd
.done:
    mov rdi, rbx
    mov rax, SYS_close
    syscall
    mov rdi, r12
    mov rax, SYS_close
    syscall
    pop r13
    pop r12
    pop rbx
    ret
.failc:
    mov rdi, rbx
    mov rax, SYS_close
    syscall
.fail:
    pop r13
    pop r12
    pop rbx
    ret

file_move:
    ; try renameat
    push rbx
    mov r8, rdi
    mov r9, rsi
    mov rax, SYS_renameat
    mov rdi, AT_FDCWD
    mov rsi, r8
    mov rdx, AT_FDCWD
    mov r10, r9
    syscall
    test rax, rax
    jz .ok
    ; fallback copy+unlink
    mov rdi, r8
    mov rsi, r9
    call file_copy
    mov rdi, r8
    call file_unlink
.ok:
    pop rbx
    ret

file_unlink:
    mov rax, SYS_unlinkat
    mov rsi, rdi
    mov rdi, AT_FDCWD
    xor rdx, rdx
    syscall
    ret

;---- path utils ----
strcpy:
.lp:
    mov al,[rsi]
    mov [rdi],al
    test al,al
    jz .d
    inc rsi
    inc rdi
    jmp .lp
.d: ret

path_join:
    ; rdi=buf with path, rsi=name
    push rbx
    push r12
    mov rbx, rdi
    mov r12, rsi
    call strlen
    mov rcx, rax
    cmp rcx,1
    jne .sl
    cmp byte [rbx],'/'
    je .cat
.sl:
    mov byte [rbx+rcx],'/'
    inc rcx
.cat:
    lea rdi,[rbx+rcx]
    mov rsi,r12
    call strcpy
    pop r12
    pop rbx
    ret

dirname:
    mov rsi, rdi
    call strlen
    mov rcx, rax
.tr:
    cmp rcx,1
    jbe .root
    dec rcx
    cmp byte [rdi+rcx],'/'
    jne .tr
    test rcx,rcx
    jz .root
    mov byte [rdi+rcx],0
    ret
.root:
    mov word [rdi],'/'
    mov byte [rdi+1],0
    ret

raw_on:
    mov rax, SYS_ioctl
    xor rdi, rdi
    mov rsi, TCGETS
    lea rdx, [tios_orig]
    syscall
    lea rdi, [tios_raw]
    lea rsi, [tios_orig]
    mov rdx, TIOS_SIZE
    call memcpy
    mov eax, [tios_raw+TIOS_LFLAG]
    and eax, ~(ICANON|ECHO)
    mov [tios_raw+TIOS_LFLAG], eax
    mov byte [tios_raw+TIOS_CC+VMIN],1
    mov byte [tios_raw+TIOS_CC+VTIME],0
    mov rax, SYS_ioctl
    xor rdi,rdi
    mov rsi, TCSETS
    lea rdx,[tios_raw]
    syscall
    ret
raw_off:
    mov rax, SYS_ioctl
    xor rdi,rdi
    mov rsi, TCSETS
    lea rdx,[tios_orig]
    syscall
    ret
read_key:
    mov rax, SYS_read
    xor rdi,rdi
    lea rsi,[keybuf]
    mov rdx,8
    syscall
    test rax,rax
    jle .z
    mov al,[keybuf]
    cmp al,27
    jne .d
    cmp rax,3
    jb .d
    cmp byte [keybuf+1],'['
    jne .d
    cmp byte [keybuf+2],'A'
    jne .b
    mov al,16
    ret
.b: cmp byte [keybuf+2],'B'
    jne .d
    mov al,14
.d: ret
.z: xor al,al
    ret
