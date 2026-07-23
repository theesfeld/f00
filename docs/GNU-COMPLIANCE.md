# GNU coreutils flag compliance (f00 pure ASM)

Source of truth for GNU flags: `/tmp/f00_gnu_help/*.txt` (coreutils 9.x `--help`), plus `mktemp --help`.

Assessment: read `asm/src/ls/*.asm` + runtime tests of `f00-* --core` vs `/usr/bin/*`.

| Status | Meaning |
|--------|--------|
| **full** | Accepted and matches coreutils for common cases |
| **partial** | Parsed and/or partially implemented; edges incomplete |
| **missing** | Not implemented |

Suite-wide modern flags (not GNU): `--core` (plain), `--json` (maximal), `--csv`, TTY color default where applicable.

Updated: 2026-07-23 — install -D/-m/-t; chmod -v/-c messages; timeout -v/--preserve-status + fractional duration; numfmt --to/--from (+stdin, 1.0M style); cp -a mode+mtime verified; suite --version → 0.15.0-beta.1.

---

## `arch`

| Flag | Status |
|------|--------|
| `(no options)` | full |

## `base64`

| Flag | Status |
|------|--------|
| `-d` | full |
| `--decode` | full |
| `-i` | partial |
| `--ignore-garbage` | partial |
| `-w` | partial |
| `--wrap` | partial |
| `--help` | full |
| `--version` | full |

## `basename`

| Flag | Status |
|------|--------|
| `-a` | full |
| `--multiple` | full |
| `-s` | full |
| `--suffix` | full |
| `-z` | full |
| `--zero` | full |
| `--help` | full |
| `--version` | full |

## `cat`

| Flag | Status |
|------|--------|
| `-A` | full |
| `--show-all` | full |
| `-v` | full |
| `-b` | full |
| `--number-nonblank` | full |
| `-n` | full |
| `-e` | full |
| `-E` | full |
| `--show-ends` | full |
| `--number` | full |
| `-s` | full |
| `--squeeze-blank` | full |
| `-t` | full |
| `-T` | full |
| `--show-tabs` | full |
| `-u` | full |
| `--show-nonprinting` | full |
| `--help` | full |
| `--version` | full |

## `chmod`

| Flag | Status |
|------|--------|
| `-c` | full |
| `--changes` | full |
| `-f` | partial |
| `--silent` | partial |
| `--quiet` | partial |
| `-v` | full |
| `--verbose` | full |
| `--dereference` | partial |
| `-h` | partial |
| `--no-dereference` | partial |
| `--no-preserve-root` | partial |
| `--preserve-root` | partial |
| `--reference` | full |
| `-R` | full |
| `--recursive` | full |
| `-H` | missing |
| `-L` | missing |
| `-P` | partial |
| `--help` | full |
| `--version` | full |
| `MODE-octal` | full |
| `MODE-symbolic` | full |

## `cp`

| Flag | Status |
|------|--------|
| `-a` | full |
| `--archive` | full |
| `--attributes-only` | partial |
| `--backup` | partial |
| `-b` | partial |
| `--copy-contents` | partial |
| `-d` | partial |
| `--debug` | partial |
| `-f` | partial |
| `--force` | partial |
| `-i` | partial |
| `--interactive` | partial |
| `-H` | partial |
| `-L` | partial |
| `--dereference` | partial |
| `-P` | partial |
| `--no-dereference` | partial |
| `--keep-directory-symlink` | partial |
| `-l` | partial |
| `--link` | partial |
| `-n` | partial |
| `--no-clobber` | partial |
| `-p` | full |
| `--preserve` | partial |
| `--no-preserve` | partial |
| `--parents` | partial |
| `-R` | partial |
| `-r` | partial |
| `--recursive` | partial |
| `--reflink` | partial |
| `--remove-destination` | partial |
| `--sparse` | partial |
| `--strip-trailing-slashes` | partial |
| `-s` | partial |
| `--symbolic-link` | partial |
| `-S` | partial |
| `--suffix` | partial |
| `-t` | full |
| `--target-directory` | full |
| `-T` | partial |
| `--no-target-directory` | partial |
| `--update` | partial |
| `-u` | partial |
| `-v` | partial |
| `--verbose` | partial |
| `-x` | partial |
| `--one-file-system` | partial |
| `-Z` | partial |
| `--context` | partial |
| `--help` | full |
| `--version` | full |

## `cut`

| Flag | Status |
|------|--------|
| `-b` | full |
| `--bytes` | full |
| `-c` | full |
| `--characters` | full |
| `--complement` | partial |
| `-d` | full |
| `--delimiter` | full |
| `-f` | full |
| `--fields` | full |
| `-F` | missing |
| `-n` | partial |
| `--no-partial` | missing |
| `-O` | partial |
| `--output-delimiter` | partial |
| `-s` | full |
| `--only-delimited` | full |
| `-w` | partial |
| `--whitespace-delimited` | missing |
| `-z` | full |
| `--zero-terminated` | full |
| `--help` | full |
| `--version` | full |
| `-M` | missing |

## `date`

| Flag | Status |
|------|--------|
| `-d` | partial |
| `--date` | partial |
| `--debug` | partial |
| `-f` | partial |
| `--file` | partial |
| `-I` | partial |
| `--iso-8601` | partial |
| `--resolution` | partial |
| `-R` | partial |
| `--rfc-email` | partial |
| `--rfc-3339` | partial |
| `-r` | partial |
| `--reference` | partial |
| `-s` | partial |
| `--set` | partial |
| `-u` | partial |
| `--utc` | partial |
| `--universal` | partial |
| `--help` | full |
| `--version` | full |

## `dirname`

| Flag | Status |
|------|--------|
| `-z` | full |
| `--zero` | full |
| `--help` | full |
| `--version` | full |

## `echo`

| Flag | Status |
|------|--------|
| `(no options)` | full |

## `env`

| Flag | Status |
|------|--------|
| `-a` | full |
| `--argv0` | full |
| `-i` | full |
| `--ignore-environment` | full |
| `-0` | full |
| `--null` | full |
| `-u` | full |
| `--unset` | full |
| `-C` | full |
| `--chdir` | full |
| `-S` | partial |
| `--split-string` | partial |
| `--block-signal` | partial |
| `--default-signal` | partial |
| `--ignore-signal` | partial |
| `--list-signal-handling` | partial |
| `-v` | partial |
| `--debug` | partial |
| `--help` | full |
| `--version` | full |

## `expr`

| Flag | Status |
|------|--------|
| `--help` | full |
| `--version` | full |

## `factor`

| Flag | Status |
|------|--------|
| `-h` | partial |
| `--exponents` | partial |
| `--help` | full |
| `--version` | full |

## `false`

| Flag | Status |
|------|--------|
| `(no options)` | full |

## `groups`

| Flag | Status |
|------|--------|
| `--help` | full |
| `--version` | full |

## `head`

| Flag | Status |
|------|--------|
| `-c` | full |
| `--bytes` | full |
| `-n` | full |
| `--lines` | full |
| `-q` | full |
| `--quiet` | full |
| `--silent` | full |
| `-v` | full |
| `--verbose` | full |
| `-z` | partial |
| `--zero-terminated` | partial |
| `--help` | full |
| `--version` | full |

## `hostid`

| Flag | Status |
|------|--------|
| `--help` | full |
| `--version` | full |

## `hostname`

| Flag | Status |
|------|--------|
| `(no options)` | full |

## `id`

| Flag | Status |
|------|--------|
| `-a` | partial |
| `-Z` | missing |
| `--context` | missing |
| `-g` | full |
| `--group` | full |
| `-G` | full |
| `--groups` | full |
| `-n` | full |
| `--name` | full |
| `-r` | full |
| `--real` | full |
| `-u` | full |
| `--user` | full |
| `-z` | partial |
| `--zero` | partial |
| `--help` | full |
| `--version` | full |

## `kill`

| Flag | Status |
|------|--------|
| `(no options)` | full |

## `ln`

| Flag | Status |
|------|--------|
| `--backup` | partial |
| `-b` | partial |
| `-d` | partial |
| `-F` | partial |
| `--directory` | partial |
| `-f` | partial |
| `--force` | partial |
| `-i` | partial |
| `--interactive` | partial |
| `-L` | partial |
| `--logical` | partial |
| `-n` | partial |
| `--no-dereference` | partial |
| `-P` | partial |
| `--physical` | partial |
| `-r` | partial |
| `--relative` | partial |
| `-s` | partial |
| `--symbolic` | partial |
| `-S` | partial |
| `--suffix` | partial |
| `-t` | partial |
| `--target-directory` | partial |
| `-T` | partial |
| `--no-target-directory` | partial |
| `-v` | partial |
| `--verbose` | partial |
| `--help` | full |
| `--version` | full |

## `logname`

| Flag | Status |
|------|--------|
| `--help` | full |
| `--version` | full |

## `md5sum`

| Flag | Status |
|------|--------|
| `-b` | partial |
| `--binary` | partial |
| `-c` | partial |
| `--check` | partial |
| `--tag` | partial |
| `-t` | partial |
| `--text` | partial |
| `-z` | partial |
| `--zero` | partial |
| `--ignore-missing` | missing |
| `--quiet` | partial |
| `--status` | partial |
| `--strict` | partial |
| `-w` | partial |
| `--warn` | partial |
| `--help` | full |
| `--version` | full |

## `mkdir`

| Flag | Status |
|------|--------|
| `-m` | partial |
| `--mode` | partial |
| `-p` | full |
| `--parents` | full |
| `-v` | partial |
| `--verbose` | partial |
| `-Z` | partial |
| `--context` | partial |
| `--help` | full |
| `--version` | full |

## `mktemp`

| Flag | Status |
|------|--------|
| `-d` | full |
| `--directory` | full |
| `-u` | full |
| `--dry-run` | full |
| `-q` | partial |
| `--quiet` | partial |
| `--suffix` | full |
| `-p` | full |
| `--tmpdir` | full |
| `-t` | full |
| `--help` | full |
| `--version` | full |

## `mv`

| Flag | Status |
|------|--------|
| `--backup` | partial |
| `-b` | partial |
| `--debug` | partial |
| `--exchange` | partial |
| `-f` | partial |
| `--force` | partial |
| `-i` | partial |
| `--interactive` | partial |
| `-n` | partial |
| `--no-clobber` | partial |
| `--no-copy` | partial |
| `--strip-trailing-slashes` | partial |
| `-S` | partial |
| `--suffix` | partial |
| `-t` | full |
| `--target-directory` | full |
| `-T` | partial |
| `--no-target-directory` | partial |
| `--update` | partial |
| `-u` | partial |
| `-v` | partial |
| `--verbose` | partial |
| `-Z` | partial |
| `--context` | partial |
| `--help` | full |
| `--version` | full |

## `nproc`

| Flag | Status |
|------|--------|
| `--all` | full |
| `--ignore` | full |
| `--help` | full |
| `--version` | full |

## `printenv`

| Flag | Status |
|------|--------|
| `-0` | full |
| `--null` | full |
| `--help` | full |
| `--version` | full |

## `printf`

| Flag | Status |
|------|--------|
| `(no options)` | full |

## `pwd`

| Flag | Status |
|------|--------|
| `(no options)` | full |

## `readlink`

| Flag | Status |
|------|--------|
| `-f` | full |
| `--canonicalize` | full |
| `-e` | full |
| `--canonicalize-existing` | full |
| `-m` | full |
| `--canonicalize-missing` | full |
| `-n` | full |
| `--no-newline` | full |
| `-q` | partial |
| `--quiet` | partial |
| `-s` | partial |
| `--silent` | partial |
| `-v` | partial |
| `--verbose` | partial |
| `-z` | full |
| `--zero` | full |
| `--help` | full |
| `--version` | full |

## `realpath`

| Flag | Status |
|------|--------|
| `-E` | partial |
| `--canonicalize` | partial |
| `-e` | full |
| `--canonicalize-existing` | full |
| `-m` | full |
| `--canonicalize-missing` | full |
| `-L` | partial |
| `--logical` | partial |
| `-P` | full |
| `--physical` | full |
| `-q` | partial |
| `--quiet` | partial |
| `--relative-to` | full |
| `--relative-base` | partial |
| `-s` | full |
| `--strip` | full |
| `--no-symlinks` | full |
| `-z` | full |
| `--zero` | full |
| `--help` | full |
| `--version` | full |

## `rm`

| Flag | Status |
|------|--------|
| `-f` | partial |
| `--force` | partial |
| `-i` | partial |
| `-I` | partial |
| `--interactive` | partial |
| `--one-file-system` | partial |
| `--no-preserve-root` | partial |
| `--preserve-root` | partial |
| `-r` | partial |
| `-R` | partial |
| `--recursive` | partial |
| `-d` | full |
| `--dir` | full |
| `-v` | partial |
| `--verbose` | partial |
| `--help` | full |
| `--version` | full |

## `rmdir`

| Flag | Status |
|------|--------|
| `--ignore-fail-on-non-empty` | full |
| `-p` | full |
| `--parents` | full |
| `-v` | full |
| `--verbose` | full |
| `--help` | full |
| `--version` | full |

## `seq`

| Flag | Status |
|------|--------|
| `-f` | partial |
| `--format` | partial |
| `-s` | full |
| `--separator` | full |
| `-w` | partial |
| `--equal-width` | partial |
| `--help` | full |
| `--version` | full |

## `sha256sum`

| Flag | Status |
|------|--------|
| `-b` | partial |
| `--binary` | partial |
| `-c` | partial |
| `--check` | partial |
| `--tag` | partial |
| `-t` | partial |
| `--text` | partial |
| `-z` | partial |
| `--zero` | partial |
| `--ignore-missing` | partial |
| `--quiet` | partial |
| `--status` | partial |
| `--strict` | partial |
| `-w` | partial |
| `--warn` | partial |
| `--help` | full |
| `--version` | full |

## `sleep`

| Flag | Status |
|------|--------|
| `--help` | full |
| `--version` | full |

## `sort`

| Flag | Status |
|------|--------|
| `-b` | partial |
| `--ignore-leading-blanks` | partial |
| `-d` | partial |
| `--dictionary-order` | partial |
| `-f` | partial |
| `--ignore-case` | partial |
| `-g` | partial |
| `--general-numeric-sort` | partial |
| `-i` | partial |
| `--ignore-nonprinting` | partial |
| `-M` | partial |
| `--month-sort` | partial |
| `-h` | partial |
| `--human-numeric-sort` | partial |
| `-n` | partial |
| `--numeric-sort` | partial |
| `-R` | partial |
| `--random-sort` | partial |
| `--random-source` | partial |
| `-r` | partial |
| `--reverse` | partial |
| `--sort` | partial |
| `-V` | partial |
| `--version-sort` | partial |
| `--batch-size` | partial |
| `-c` | partial |
| `--check` | partial |
| `-C` | partial |
| `--compress-program` | partial |
| `--debug` | partial |
| `--files0-from` | partial |
| `-k` | partial |
| `--key` | partial |
| `-m` | partial |
| `--merge` | partial |
| `-o` | partial |
| `--output` | partial |
| `-s` | partial |
| `--stable` | partial |
| `-S` | partial |
| `--buffer-size` | partial |
| `-t` | partial |
| `--field-separator` | partial |
| `-T` | partial |
| `--temporary-directory` | partial |
| `--parallel` | partial |
| `-u` | partial |
| `--unique` | partial |
| `-z` | partial |
| `--zero-terminated` | partial |
| `--help` | full |
| `--version` | full |

## `tail`

| Flag | Status |
|------|--------|
| `-c` | partial |
| `--bytes` | partial |
| `--debug` | partial |
| `-f` | partial |
| `--follow` | partial |
| `-F` | partial |
| `-n` | partial |
| `--lines` | partial |
| `--max-unchanged-stats` | partial |
| `--pid` | partial |
| `-q` | partial |
| `--quiet` | partial |
| `--silent` | partial |
| `--retry` | partial |
| `-s` | partial |
| `--sleep-interval` | partial |
| `-v` | partial |
| `--verbose` | partial |
| `-z` | partial |
| `--zero-terminated` | partial |
| `--help` | full |
| `--version` | full |

## `tee`

| Flag | Status |
|------|--------|
| `-a` | full |
| `--append` | full |
| `-i` | partial |
| `--ignore-interrupts` | partial |
| `-p` | partial |
| `--output-error` | missing |
| `--help` | full |
| `--version` | full |

## `test`

| Flag | Status |
|------|--------|
| `(no options)` | full |

## `timeout`

| Flag | Status |
|------|--------|
| `-f` | partial |
| `--foreground` | partial |
| `-k` | full |
| `--kill-after` | full |
| `-p` | full |
| `--preserve-status` | full |
| `-s` | full |
| `--signal` | full |
| `-v` | full |
| `--verbose` | full |
| `DURATION` (integer + fractional) | full |
| `--help` | full |
| `--version` | full |

## `touch`

| Flag | Status |
|------|--------|
| `-a` | full |
| `-c` | full |
| `--no-create` | full |
| `-d` | partial |
| `--date` | partial |
| `-f` | full |
| `-h` | full |
| `--no-dereference` | full |
| `-m` | full |
| `-r` | full |
| `--reference` | full |
| `-t` | full |
| `--time` | partial |
| `--help` | full |
| `--version` | full |

## `tr`

| Flag | Status |
|------|--------|
| `-c` | partial |
| `-C` | partial |
| `--complement` | partial |
| `-d` | full |
| `--delete` | full |
| `-s` | full |
| `--squeeze-repeats` | full |
| `-t` | partial |
| `--truncate-set1` | partial |
| `--help` | full |
| `--version` | full |

## `true`

| Flag | Status |
|------|--------|
| `(no options)` | full |

## `tty`

| Flag | Status |
|------|--------|
| `-s` | full |
| `--silent` | full |
| `--quiet` | full |
| `--help` | full |
| `--version` | full |

## `uname`

| Flag | Status |
|------|--------|
| `-a` | full |
| `--all` | full |
| `-s` | full |
| `--kernel-name` | full |
| `-n` | full |
| `--nodename` | full |
| `-r` | full |
| `--kernel-release` | full |
| `-v` | full |
| `--kernel-version` | full |
| `-m` | full |
| `--machine` | full |
| `-p` | partial |
| `--processor` | partial |
| `-i` | partial |
| `--hardware-platform` | partial |
| `-o` | full |
| `--operating-system` | full |
| `--help` | full |
| `--version` | full |

## `uniq`

| Flag | Status |
|------|--------|
| `-c` | partial |
| `--count` | partial |
| `-d` | partial |
| `--repeated` | partial |
| `-D` | partial |
| `--all-repeated` | partial |
| `-f` | partial |
| `--skip-fields` | partial |
| `--group` | partial |
| `-i` | partial |
| `--ignore-case` | partial |
| `-s` | partial |
| `--skip-chars` | partial |
| `-u` | partial |
| `--unique` | partial |
| `-z` | partial |
| `--zero-terminated` | partial |
| `-w` | partial |
| `--check-chars` | partial |
| `--help` | full |
| `--version` | full |

## `wc`

| Flag | Status |
|------|--------|
| `-c` | full |
| `--bytes` | full |
| `-m` | full |
| `--chars` | full |
| `-l` | full |
| `--lines` | full |
| `--debug` | partial |
| `--files0-from` | partial |
| `-L` | full |
| `--max-line-length` | full |
| `-w` | full |
| `--words` | full |
| `--total` | partial |
| `--help` | full |
| `--version` | full |

## `whoami`

| Flag | Status |
|------|--------|
| `--help` | full |
| `--version` | full |

## `yes`

| Flag | Status |
|------|--------|
| `--help` | full |
| `--version` | full |

---

## Summary counts (flags listed above)

- full: **294**
- partial: **320**
- missing: **11**

## `install`

| Flag | Status |
|------|--------|
| `-m` | full |
| `--mode` | full |
| `-D` | full |
| `-t` | full |
| `--target-directory` | full |
| `-T` | full |
| `--no-target-directory` | full |
| `-v` | full |
| `--verbose` | full |
| `--help` | full |
| `--version` | full |

## `numfmt`

| Flag | Status |
|------|--------|
| `--to=si` | full |
| `--to=iec` | full |
| `--to=iec-i` | full |
| `--from=si` | full |
| `--from=iec` | full |
| `--from=iec-i` | full |
| stdin numbers | full |
| `--suffix` | missing |
| `--help` | full |
| `--version` | full |

## Path suite focus

In `asm/src/ls/suite_path.asm`:

- **env**: `-i -0 -u -C -S -v -a`, `--chdir`, `--argv0`, `--split-string`, signal options (basic), lone `-` ⇒ `-i`; TTY-colored `KEY=VAL` unless `--core`; **`-u/--unset` full** (ordered against ambient + assignments)
- **printenv**: `-0/--null`
- **realpath**: `-e -m -E -L -P -q -s -z`, `--relative-to`, `--relative-base` (accepted); physical walk follows symlink chains
- **readlink**: `-e -f -m -n -q -s -v -z`; **`-f` full** (component walk, mid-path must exist, broken chains canonicalize)
- **mkdir**: `-m -p -v -Z`, `--mode`, `--context` (SELinux accept/no-op)
- **rmdir**: `-p -v`, `--ignore-fail-on-non-empty`
- **chmod**: octal full; symbolic `ugoa+-/=rwxXst` with **`X` dir/any-exec aware**; `--reference`; **`-v/--verbose` and `-c/--changes` full** (GNU `mode of '…' changed/retained` form); **`-R/--recursive` full** (post-order, no recurse into symlink dirs / `-P` default)
- **touch**: `-a -c -d -m -r -t -h -f`, `--reference`, `--date` (`@unix`/stamp), `--time`
- **mktemp**: `-d -p -q -t -u`, `--suffix`, `--tmpdir[=DIR]`
- Missing operands: `err_missing_operand` diagnostics
- **`--core`**: `apply_mod` sets `g_color=0` + `g_json_core=1`

Also verified full for common cases: **basename** `-a/-z` multi, **dirname** multi/`-z`, **rm** `-d`, **cp/mv** `-t`, **cp -a/-p** mode+mtime, **install** `-D/-m/-t`, **timeout** `-v/--preserve-status`, **numfmt** `--to/--from`, **head/tail/wc** core flags.

Suite **`--version`**: `0.15.0-beta.1` across multicall utils.

Parity: [`asm/benches/parity.sh`](../asm/benches/parity.sh).
