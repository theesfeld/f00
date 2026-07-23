; f00-asm — shell completions (--generate-completions SHELL)
BITS 64
DEFAULT REL
%include "syscalls.inc"

global generate_completions
extern out_str, out_flush
extern strcmp

section .rodata
nm_bash: db "bash",0
nm_zsh:  db "zsh",0
nm_fish: db "fish",0

bash_script:
    db 35," bash completion for f00",10
    db "_f00() {",10
    db "  local cur opts",10
    db "  cur=${COMP_WORDS[COMP_CWORD]}",10
    db "  opts=",34,"-a -A -l -1 -C -h -r -t -S -R -d -F -i -s -j --json --csv --tsv --tree --core --git --icons --browse --color --help --version",34,10
    db "  mapfile -t COMPREPLY < <(compgen -W ",34,"$opts",34," -- ",34,"$cur",34,")",10
    db "  [[ $cur != -* ]] && mapfile -t -O ${#COMPREPLY[@]} COMPREPLY < <(compgen -f -- ",34,"$cur",34,")",10
    db "}",10
    db "complete -F _f00 f00",10,0

zsh_script:
    db "#compdef f00",10
    db "# zsh completion for f00 (assembly port)",10
    db "_f00() {",10
    db "  _arguments \\",10
    db "    ",34,"-a[all]",34," \\",10
    db "    ",34,"-l[long]",34," \\",10
    db "    ",34,"-j[json]",34," \\",10
    db "    ",34,"--tree[tree]",34," \\",10
    db "    ",34,"--git[git]",34," \\",10
    db "    ",34,"--browse[tui]",34," \\",10
    db "    ",34,"*:file:_files",34,10
    db "}",10
    db "_f00",34,"$@",34,10,0

fish_script:
    db "complete -c f00 -s a -l all",10
    db "complete -c f00 -s l",10
    db "complete -c f00 -s j -l json",10
    db "complete -c f00 -l tree",10
    db "complete -c f00 -l git",10
    db "complete -c f00 -l icons",10
    db "complete -c f00 -l browse",10
    db "complete -c f00 -l gnu",10,0

unk: db "f00: unknown shell (use bash, zsh, or fish)",10,0

section .text
generate_completions:
    push rbx
    mov rbx, rdi
    mov rdi, rbx
    lea rsi, [nm_bash]
    call strcmp
    test eax, eax
    jnz .z
    lea rsi, [bash_script]
    jmp .emit
.z:
    mov rdi, rbx
    lea rsi, [nm_zsh]
    call strcmp
    test eax, eax
    jnz .f
    lea rsi, [zsh_script]
    jmp .emit
.f:
    mov rdi, rbx
    lea rsi, [nm_fish]
    call strcmp
    test eax, eax
    jnz .bad
    lea rsi, [fish_script]
    jmp .emit
.bad:
    lea rsi, [unk]
.emit:
    call out_str
    call out_flush
    pop rbx
    ret
