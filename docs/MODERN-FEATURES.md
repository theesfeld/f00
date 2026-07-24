# Modern features decisions (surveys)

**f00tils** product law: when not `--core`, everything interactive is **modern**.

Always for every util: color (TTY), `--json`/`--csv` where structured, GNU flags, modern default, `--core` only for strict presentation.

Shared chrome (see [TERMINAL-UX.md](TERMINAL-UX.md)):

- Semantic color tokens
- Nerd Font **icons** via `icon_for_path` / `icon_for_entry` where paths appear
- **File headers** (`ui_file_header`) for multi-file text tools
- **Spinners** (`ui_spinner_*`) on stderr for longer multi-file work
- Help section chrome + progress bars

## f00-ls
Shipped: icons, git, tree, json/csv/tsv, TUI, archives, hyperlink, ignore-files, LS_COLORS, themes.

## f00-cat
**Chosen / shipping:** bat-class — colored multi-file headers with icons, line gutters, -vET markers colored, squeeze, `--json`/`--csv` summaries. Syntax highlight: progressive later.

## f00-head
**Chosen:** multi-file power + follow lite (headers, human units, json/csv, simple follow).

## f00-tail
**Chosen:** robust follow (-F style), headers, json/csv event/summary.

## f00-wc
**Chosen:** colored table + progress + full json/csv counts.

## f00-tee
**Chosen:** progress/bytes, multi-file status, json/csv result.

## f00-realpath / basename / dirname
**Chosen:** rich paths — color components, multi, json/csv abs/rel/exists.

## f00-env / printenv
**Chosen:** dotenv + color KEY=VAL, sort/filter, json/csv, diff mode.

## f00-yes / seq / nproc / tty / whoami / true / false
**Chosen:** consistent suite json/csv; yes suppresses infinite in machine mode.

## Tier 2 (locked)
- **cut:** CSV/TSV-native, colored columns, header support, json row objects
- **tr:** unicode-aware + map/dry-run + stats json/csv
- **sort:** parallel + progress + key debug color + json/csv
- **uniq:** groups + colored counts + json/csv
- **base64 + checksums:** hash suite + progress + parallel + BLAKE3 modern; GNU names + f00-hash
- **od/nl/fold/fmt/paste/join:** pretty TTY + structured json/csv

## Shipped multicall (0.14) — full coreutils surface

Pure ASM multicall (`asm/f00`, ~340KB static). All GNU coreutils names + `f00-*` hardlinks:

`ls cat head tail wc tee seq echo pwd sleep env printenv realpath readlink pathchk mktemp link unlink sync truncate mkdir rmdir chmod touch logname hostid cut tr sort uniq rev tac nl fold expand unexpand paste join comm fmt od split csplit shuf tsort pr ptx factor numfmt expr cp mv rm ln chown chgrp stat df du install mkfifo mknod shred dd dir vdir id groups uname arch date users who pinky uptime hostname nice nohup timeout kill test [ printf true false yes nproc tty whoami basename dirname md5sum sha1sum sha224sum sha256sum sha384sum sha512sum b2sum cksum sum base64 basenc dircolors chroot stty stdbuf runcon`

Suite standard on each: modern default, `--core`, `--json`/`--csv` where structured, `--help`/`--version`.

Depth varies by util (ls/cat/hash are deepest; some text/fs tools are solid common-case GNU subsets with modern flags). Iterative parity continues without leaving any util unshipped.
