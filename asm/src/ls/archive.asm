; f00-asm — list zip / tar / tar.gz as virtual directory entries
BITS 64
DEFAULT REL
%include "syscalls.inc"

global archive_try_list, is_archive_path
extern arena_alloc, memcpy, strlen
extern g_entries, g_entry_count, g_entry_cap
extern entry_reserve_export
extern run_capture, find_in_path

; If entry_reserve not exported, we'll call local push_entry

section .bss
alignb 8
file_map:   resq 1
file_size:  resq 1
path_copy:  resb 4096
tar_buf:    resb 1<<20              ; 1 MiB gunzip capture / file

section .rodata
ext_zip:    db ".zip",0
ext_tar:    db ".tar",0
ext_tgz:    db ".tgz",0
ext_targz:  db ".tar.gz",0
gzip_name:  db "gzip",0
arg0:       db "gzip",0
arg1:       db "-dc",0
section .bss
argv_gz:    resq 4

section .text

; is_archive_path(rdi=path) → al kind: 0=no 1=zip 2=tar 3=tgz
is_archive_path:
    push rbx
    mov rbx, rdi
    call strlen
    mov rcx, rax
    cmp rcx, 4
    jb .no
    ; .zip
    lea rsi, [rbx + rcx - 4]
    cmp dword [rsi], 0x70697a2e
    je .zip
    cmp dword [rsi], 0x50495a2e
    je .zip
    cmp dword [rsi], 0x7261742e
    je .tar
    cmp dword [rsi], 0x7a67742e
    je .tgz
    cmp rcx, 7
    jb .no
    lea rsi, [rbx + rcx - 7]
    cmp byte [rsi], '.'
    jne .no
    cmp byte [rsi+1], 't'
    jne .no
    cmp byte [rsi+2], 'a'
    jne .no
    cmp byte [rsi+3], 'r'
    jne .no
    cmp byte [rsi+4], '.'
    jne .no
    cmp byte [rsi+5], 'g'
    jne .no
    cmp byte [rsi+6], 'z'
    jne .no
    mov al, 3
    pop rbx
    ret
.zip:
    mov al, 1
    pop rbx
    ret
.tar:
    mov al, 2
    pop rbx
    ret
.tgz:
    mov al, 3
    pop rbx
    ret
.no:
    xor al, al
    pop rbx
    ret

; archive_try_list(rdi=path) → eax 0 ok listed, -1 not archive, -2 error
; Populates g_entries
archive_try_list:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov r12, rdi
    call is_archive_path
    test al, al
    jz .not
    mov r13b, al
    mov qword [g_entry_count], 0

    ; mmap file
    mov rax, SYS_openat
    mov rdi, AT_FDCWD
    mov rsi, r12
    mov rdx, O_RDONLY | O_CLOEXEC
    xor r10, r10
    syscall
    cmp rax, -4096
    jae .err
    mov r14, rax                    ; fd
    ; size via lseek
    mov rax, SYS_lseek
    mov rdi, r14
    xor rsi, rsi
    mov rdx, 2                      ; SEEK_END
    syscall
    mov r15, rax                    ; size
    mov rax, SYS_lseek
    mov rdi, r14
    xor rsi, rsi
    xor rdx, rdx
    syscall
    test r15, r15
    jle .err_fd
    ; mmap
    mov rax, SYS_mmap
    xor rdi, rdi
    mov rsi, r15
    mov rdx, PROT_READ
    mov r10, MAP_PRIVATE
    mov r8, r14
    xor r9, r9
    syscall
    cmp rax, -4096
    jae .err_fd
    mov [file_map], rax
    mov [file_size], r15
    mov rdi, r14
    mov rax, SYS_close
    syscall

    cmp r13b, 1
    je .do_zip
    cmp r13b, 2
    je .do_tar
    ; tgz: gunzip to buffer then tar
    jmp .do_tgz

.do_zip:
    call list_zip
    jmp .unmap
.do_tar:
    mov rdi, [file_map]
    mov rsi, [file_size]
    call list_tar_mem
    jmp .unmap
.do_tgz:
    call list_tgz
.unmap:
    mov rax, SYS_munmap
    mov rdi, [file_map]
    mov rsi, [file_size]
    syscall
    xor eax, eax
    jmp .out
.not:
    mov eax, -1
    jmp .out
.err_fd:
    mov rdi, r14
    mov rax, SYS_close
    syscall
.err:
    mov eax, -2
.out:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    ret

;---- ZIP: find EOCD, walk central directory ----
list_zip:
    push rbx
    push r12
    push r13
    push r14
    mov r12, [file_map]
    mov r13, [file_size]
    ; search last 64k for EOCD sig PK\x05\x06
    mov rax, r13
    cmp rax, 22
    jb .done
    mov rcx, 65557
    cmp rax, rcx
    jb .sfrom0
    sub rax, rcx
    jmp .sbase
.sfrom0:
    xor rax, rax
.sbase:
    lea rbx, [r12 + r13 - 22]
.search:
    cmp rbx, r12
    jb .done
    ; sig 50 4b 05 06
    cmp dword [rbx], 0x06054b50
    je .eocd
    dec rbx
    jmp .search
.eocd:
    movzx r14d, word [rbx + 10]     ; total entries on disk
    mov eax, [rbx + 16]             ; CD offset
    cmp rax, r13
    jae .done
    lea r8, [r12 + rax]             ; CD ptr
    xor r9, r9
.ent:
    cmp r9, r14
    jae .done
    cmp r9, 100000
    jae .done
    cmp dword [r8], 0x02014b50
    jne .done
    movzx eax, word [r8 + 28]       ; name len
    movzx ecx, word [r8 + 30]       ; extra
    movzx edx, word [r8 + 32]       ; comment
    mov r10d, [r8 + 24]             ; uncompressed size
    lea r11, [r8 + 46]              ; name
    ; is dir if name ends with /
    xor r15d, r15d
    test eax, eax
    jz .push
    cmp byte [r11 + rax - 1], '/'
    jne .push
    mov r15d, 1
.push:
    push r8
    push r9
    push r14
    push rax
    push r10
    push r15
    push r11
    ; push_entry(name, namelen, size, is_dir)
    mov rdi, r11
    mov rsi, rax
    mov rdx, r10
    mov ecx, r15d
    call push_synthetic
    pop r11
    pop r15
    pop r10
    pop rax
    pop r14
    pop r9
    pop r8
    ; next CD record
    add r8, 46
    add r8, rax
    add r8, rcx
    add r8, rdx
    inc r9
    jmp .ent
.done:
    pop r14
    pop r13
    pop r12
    pop rbx
    ret

;---- TAR ustar in memory rdi=buf rsi=len ----
list_tar_mem:
    push rbx
    push r12
    push r13
    push r14
    mov r12, rdi
    mov r13, rsi
    xor r14, r14                    ; offset
.loop:
    lea rax, [r14 + 512]
    cmp rax, r13
    ja .done
    lea rbx, [r12 + r14]
    ; empty block ends archive
    cmp byte [rbx], 0
    je .done
    ; name at 0 (100 bytes)
    ; size octal at 124 (12 bytes)
    lea rdi, [rbx + 124]
    call parse_octal
    mov r8, rax                     ; size
    ; typeflag at 156
    mov cl, [rbx + 156]
    xor r9d, r9d
    cmp cl, '5'
    je .isdir
    cmp cl, 'D'
    je .isdir
    jmp .notdir
.isdir:
    mov r9d, 1
.notdir:
    ; name length
    mov rdi, rbx
    call strnlen100
    mov rsi, rax
    mov rdi, rbx
    mov rdx, r8
    mov ecx, r9d
    push r14
    push r8
    call push_synthetic
    pop r8
    pop r14
    ; advance: 512 + ceil(size/512)*512
    mov rax, r8
    add rax, 511
    and rax, ~511
    add rax, 512
    add r14, rax
    cmp r14, r13
    jb .loop
.done:
    pop r14
    pop r13
    pop r12
    pop rbx
    ret

strnlen100:
    xor eax, eax
.lp:
    cmp eax, 100
    jae .d
    cmp byte [rdi + rax], 0
    je .d
    inc eax
    jmp .lp
.d: ret

parse_octal:
    ; rdi → up to 12 octal digits
    xor eax, eax
    xor ecx, ecx
.lp:
    cmp ecx, 12
    jae .d
    movzx edx, byte [rdi + rcx]
    cmp dl, '0'
    jb .d
    cmp dl, '7'
    ja .d
    shl rax, 3
    sub dl, '0'
    add rax, rdx
    inc ecx
    jmp .lp
.d: ret

; tar.gz via gzip -dc
list_tgz:
    push rbx
    ; need path string for gzip - need argv with path
    lea rdi, [gzip_name]
    call find_in_path
    test rax, rax
    jz .fail
    mov rbx, rax
    lea rax, [arg0]
    mov [argv_gz], rax
    lea rax, [arg1]
    mov [argv_gz+8], rax
    mov [argv_gz+16], r12           ; path
    mov qword [argv_gz+24], 0
    mov rdi, rbx
    lea rsi, [argv_gz]
    xor rdx, rdx
    call run_capture
    test rax, rax
    jz .fail
    ; rax=buf rdx=len — copy to tar_buf if needed
    cmp rdx, 1<<20
    ja .fail
    mov rdi, rax
    mov rsi, rdx
    call list_tar_mem
    pop rbx
    ret
.fail:
    pop rbx
    ret

; push_synthetic(rdi=name, rsi=namelen, rdx=size, ecx=is_dir)
extern entry_reserve
push_synthetic:
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov r12, rdi
    mov r13, rsi
    mov r14, rdx
    mov r15d, ecx
    lea rdi, [r13 + 1]
    call arena_alloc
    mov rbx, rax
    mov rdi, rax
    mov rsi, r12
    mov rdx, r13
    call memcpy
    mov byte [rbx + r13], 0
    call entry_reserve
    mov r8, rax                     ; Entry*
    mov [r8 + Entry.name], rbx
    mov [r8 + Entry.namelen], r13w
    mov [r8 + Entry.path], rbx
    mov [r8 + Entry.pathlen], r13d
    mov [r8 + Entry.size], r14
    test r15d, r15d
    jz .file
    mov byte [r8 + Entry.dtype], DT_DIR
    mov byte [r8 + Entry.flags], EF_DIR | EF_STATED
    mov dword [r8 + Entry.mode], 0o40755
    jmp .fin
.file:
    mov byte [r8 + Entry.dtype], DT_REG
    mov byte [r8 + Entry.flags], EF_STATED
    mov dword [r8 + Entry.mode], 0o100644
.fin:
    mov dword [r8 + Entry.nlink], 1
    mov rax, r14
    add rax, 511
    shr rax, 9                      ; 512-byte blocks
    mov [r8 + Entry.blocks], rax
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    ret
