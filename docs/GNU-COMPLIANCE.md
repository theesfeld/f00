# GNU coreutils flag compliance (f00 pure ASM)

Source of truth for GNU flags: `/tmp/f00_gnu_help/*.txt` (coreutils 9.x `--help`), plus `mktemp --help`.

Assessment: read `asm/src/ls/*.asm` + runtime tests of `f00-* --core` vs `/usr/bin/*`.

| Status | Meaning |
|--------|--------|
| **full** | Accepted and matches coreutils for common cases |
| **partial** | Parsed and/or partially implemented; edges incomplete |
| **missing** | Not implemented |

Suite-wide modern flags (not GNU): `--core` (plain), `--json` (maximal), `--csv`, TTY color default where applicable.

Updated: 2026-07-23 — suite_text FULL + fs/path batch FULL: cp mv rm ln chown chgrp chmod mkdir mkfifo mknod install shred dd df du stat dir vdir link unlink sync truncate touch mktemp pathchk readlink realpath (common-case --core).

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
| `-i` | full |
| `--ignore-garbage` | full |
| `-w` | full |
| `--wrap` | full |
| `--help` | full |
| `--version` | full |

## `base32`

| Flag | Status |
|------|--------|
| `-d` | full |
| `--decode` | full |
| `-i` | full |
| `--ignore-garbage` | full |
| `-w` | full |
| `--wrap` | full |
| `--help` | full |
| `--version` | full |

## `basenc`

| Flag | Status |
|------|--------|
| `--base64` | full |
| `--base64url` | full |
| `--base32` | full |
| `--base32hex` | full |
| `--base16` | full |
| `--base2msbf` | full |
| `--base2lsbf` | full |
| `-d` | full |
| `--decode` | full |
| `-i` | full |
| `--ignore-garbage` | full |
| `-w` | full |
| `--wrap` | full |
| `--help` | full |
| `--version` | full |

## `dircolors`

| Flag | Status |
|------|--------|
| `-b` | full |
| `--sh` | full |
| `--bourne-shell` | full |
| `-c` | full |
| `--csh` | full |
| `--c-shell` | full |
| `-p` | full |
| `--print-database` | full |
| `--print-ls-colors` | full |
| `FILE` | full |
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
| `-f` | full |
| `--silent` | full |
| `--quiet` | full |
| `-v` | full |
| `--verbose` | full |
| `--dereference` | full |
| `-h` | full |
| `--no-dereference` | full |
| `--no-preserve-root` | full |
| `--preserve-root` | full |
| `--reference` | full |
| `-R` | full |
| `--recursive` | full |
| `-H` | full |
| `-L` | full |
| `-P` | full |
| `--help` | full |
| `--version` | full |
| `MODE-octal` | full |
| `MODE-symbolic` | full |

## `cp`

| Flag | Status |
|------|--------|
| `-a` | full |
| `--archive` | full |
| `--attributes-only` | full |
| `--backup` | full |
| `-b` | full |
| `--copy-contents` | full |
| `-d` | full |
| `--debug` | full |
| `-f` | full |
| `--force` | full |
| `-i` | full |
| `--interactive` | full |
| `-H` | full |
| `-L` | full |
| `--dereference` | full |
| `-P` | full |
| `--no-dereference` | full |
| `--keep-directory-symlink` | full |
| `-l` | full |
| `--link` | full |
| `-n` | full |
| `--no-clobber` | full |
| `-p` | full |
| `--preserve` | full |
| `--no-preserve` | full |
| `--parents` | full |
| `-R` | full |
| `-r` | full |
| `--recursive` | full |
| `--reflink` | full |
| `--remove-destination` | full |
| `--sparse` | full |
| `--strip-trailing-slashes` | full |
| `-s` | full |
| `--symbolic-link` | full |
| `-S` | full |
| `--suffix` | full |
| `-t` | full |
| `--target-directory` | full |
| `-T` | full |
| `--no-target-directory` | full |
| `--update` | full |
| `-u` | full |
| `-v` | full |
| `--verbose` | full |
| `-x` | full |
| `--one-file-system` | full |
| `-Z` | full |
| `--context` | full |
| `--help` | full |
| `--version` | full |

## `cut`

| Flag | Status |
|------|--------|
| `-b` | full |
| `--bytes` | full |
| `-c` | full |
| `--characters` | full |
| `--complement` | full |
| `-d` | full |
| `--delimiter` | full |
| `-f` | full |
| `--fields` | full |
| `-F` | full |
| `-n` | full |
| `--no-partial` | full |
| `-O` | full |
| `--output-delimiter` | full |
| `-s` | full |
| `--only-delimited` | full |
| `-w` | full |
| `--whitespace-delimited` | full |
| `-z` | full |
| `--zero-terminated` | full |
| `--help` | full |
| `--version` | full |


## `date`

| Flag | Status |
|------|--------|
| `-d` | full |
| `--date` | full |
| `--debug` | full |
| `-f` | full |
| `--file` | full |
| `-I` | full |
| `--iso-8601` | full |
| `--resolution` | full |
| `-R` | full |
| `--rfc-email` | full |
| `--rfc-3339` | full |
| `-r` | full |
| `--reference` | full |
| `-s` | full |
| `--set` | full |
| `-u` | full |
| `--utc` | full |
| `--universal` | full |
| `+FORMAT` | full |
| `--help` | full |
| `--version` | full |

Notes: freestanding always uses UTC (no TZ database). `-s/--set` accepted (requires CAP_SYS_TIME to apply). `-d` supports `@epoch`, `YYYY-MM-DD[THH:MM:SS]`, `YYYYMMDD`, `now`/`today`/`yesterday`/`tomorrow`, `N days ago`.


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
| `-S` | full |
| `--split-string` | full |
| `--block-signal` | full |
| `--default-signal` | full |
| `--ignore-signal` | full |
| `--list-signal-handling` | full |
| `-v` | full |
| `--debug` | full |
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
| `-h` | full |
| `--exponents` | full |
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
| `-z` | full |
| `--zero-terminated` | full |
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
| `--help` | full |
| `--version` | full |


## `id`

| Flag | Status |
|------|--------|
| `-a` | full |
| `-Z` | full |
| `--context` | full |
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
| `-z` | full |
| `--zero` | full |
| `--help` | full |
| `--version` | full |


## `kill`

| Flag | Status |
|------|--------|
| `-s` | full |
| `--signal` | full |
| `-SIGNAL` | full |
| `-l` | full |
| `--list` | full |
| `--help` | full |
| `--version` | full |


## `ln`

| Flag | Status |
|------|--------|
| `--backup` | full |
| `-b` | full |
| `-d` | full |
| `-F` | full |
| `--directory` | full |
| `-f` | full |
| `--force` | full |
| `-i` | full |
| `--interactive` | full |
| `-L` | full |
| `--logical` | full |
| `-n` | full |
| `--no-dereference` | full |
| `-P` | full |
| `--physical` | full |
| `-r` | full |
| `--relative` | full |
| `-s` | full |
| `--symbolic` | full |
| `-S` | full |
| `--suffix` | full |
| `-t` | full |
| `--target-directory` | full |
| `-T` | full |
| `--no-target-directory` | full |
| `-v` | full |
| `--verbose` | full |
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
| `-b` | full |
| `--binary` | full |
| `-c` | full |
| `--check` | full |
| `--tag` | full |
| `-t` | full |
| `--text` | full |
| `-z` | full |
| `--zero` | full |
| `--ignore-missing` | full |
| `--quiet` | full |
| `--status` | full |
| `--strict` | full |
| `-w` | full |
| `--warn` | full |
| `--help` | full |
| `--version` | full |

## `sha1sum`

| Flag | Status |
|------|--------|
| `-b` | full |
| `--binary` | full |
| `-c` | full |
| `--check` | full |
| `--tag` | full |
| `-t` | full |
| `--text` | full |
| `-z` | full |
| `--zero` | full |
| `--ignore-missing` | full |
| `--quiet` | full |
| `--status` | full |
| `--strict` | full |
| `-w` | full |
| `--warn` | full |
| `--help` | full |
| `--version` | full |

## `sha224sum`

| Flag | Status |
|------|--------|
| `-b` | full |
| `--binary` | full |
| `-c` | full |
| `--check` | full |
| `--tag` | full |
| `-t` | full |
| `--text` | full |
| `-z` | full |
| `--zero` | full |
| `--ignore-missing` | full |
| `--quiet` | full |
| `--status` | full |
| `--strict` | full |
| `-w` | full |
| `--warn` | full |
| `--help` | full |
| `--version` | full |

## `mkdir`

| Flag | Status |
|------|--------|
| `-m` | full |
| `--mode` | full |
| `-p` | full |
| `--parents` | full |
| `-v` | full |
| `--verbose` | full |
| `-Z` | full |
| `--context` | full |
| `--help` | full |
| `--version` | full |

## `mktemp`

| Flag | Status |
|------|--------|
| `-d` | full |
| `--directory` | full |
| `-u` | full |
| `--dry-run` | full |
| `-q` | full |
| `--quiet` | full |
| `--suffix` | full |
| `-p` | full |
| `--tmpdir` | full |
| `-t` | full |
| `--help` | full |
| `--version` | full |

## `mv`

| Flag | Status |
|------|--------|
| `--backup` | full |
| `-b` | full |
| `--debug` | full |
| `--exchange` | full |
| `-f` | full |
| `--force` | full |
| `-i` | full |
| `--interactive` | full |
| `-n` | full |
| `--no-clobber` | full |
| `--no-copy` | full |
| `--strip-trailing-slashes` | full |
| `-S` | full |
| `--suffix` | full |
| `-t` | full |
| `--target-directory` | full |
| `-T` | full |
| `--no-target-directory` | full |
| `--update` | full |
| `-u` | full |
| `-v` | full |
| `--verbose` | full |
| `-Z` | full |
| `--context` | full |
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
| `-q` | full |
| `--quiet` | full |
| `-s` | full |
| `--silent` | full |
| `-v` | full |
| `--verbose` | full |
| `-z` | full |
| `--zero` | full |
| `--help` | full |
| `--version` | full |

## `realpath`

| Flag | Status |
|------|--------|
| `-E` | full |
| `--canonicalize` | full |
| `-e` | full |
| `--canonicalize-existing` | full |
| `-m` | full |
| `--canonicalize-missing` | full |
| `-L` | full |
| `--logical` | full |
| `-P` | full |
| `--physical` | full |
| `-q` | full |
| `--quiet` | full |
| `--relative-to` | full |
| `--relative-base` | full |
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
| `-f` | full |
| `--force` | full |
| `-i` | full |
| `-I` | full |
| `--interactive` | full |
| `--one-file-system` | full |
| `--no-preserve-root` | full |
| `--preserve-root` | full |
| `-r` | full |
| `-R` | full |
| `--recursive` | full |
| `-d` | full |
| `--dir` | full |
| `-v` | full |
| `--verbose` | full |
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
| `-f` | full |
| `--format` | full |
| `-s` | full |
| `--separator` | full |
| `-w` | full |
| `--equal-width` | full |
| `--help` | full |
| `--version` | full |

## `sha256sum`

| Flag | Status |
|------|--------|
| `-b` | full |
| `--binary` | full |
| `-c` | full |
| `--check` | full |
| `--tag` | full |
| `-t` | full |
| `--text` | full |
| `-z` | full |
| `--zero` | full |
| `--ignore-missing` | full |
| `--quiet` | full |
| `--status` | full |
| `--strict` | full |
| `-w` | full |
| `--warn` | full |
| `--help` | full |
| `--version` | full |

## `sha384sum`

| Flag | Status |
|------|--------|
| `-b` | full |
| `--binary` | full |
| `-c` | full |
| `--check` | full |
| `--tag` | full |
| `-t` | full |
| `--text` | full |
| `-z` | full |
| `--zero` | full |
| `--ignore-missing` | full |
| `--quiet` | full |
| `--status` | full |
| `--strict` | full |
| `-w` | full |
| `--warn` | full |
| `--help` | full |
| `--version` | full |

## `sha512sum`

| Flag | Status |
|------|--------|
| `-b` | full |
| `--binary` | full |
| `-c` | full |
| `--check` | full |
| `--tag` | full |
| `-t` | full |
| `--text` | full |
| `-z` | full |
| `--zero` | full |
| `--ignore-missing` | full |
| `--quiet` | full |
| `--status` | full |
| `--strict` | full |
| `-w` | full |
| `--warn` | full |
| `--help` | full |
| `--version` | full |

## `b2sum`

| Flag | Status |
|------|--------|
| `-b` | full |
| `--binary` | full |
| `-c` | full |
| `--check` | full |
| `--tag` | full |
| `-t` | full |
| `--text` | full |
| `-z` | full |
| `--zero` | full |
| `--ignore-missing` | full |
| `--quiet` | full |
| `--status` | full |
| `--strict` | full |
| `-w` | full |
| `--warn` | full |
| `--help` | full |
| `--version` | full |

## `cksum`

| Flag | Status |
|------|--------|
| `(default CRC)` | full |
| `-c` | full |
| `--check` | full |
| `--tag` | full |
| `-z` | full |
| `--zero` | full |
| `--ignore-missing` | full |
| `--quiet` | full |
| `--status` | full |
| `--strict` | full |
| `-w` | full |
| `--warn` | full |
| `--help` | full |
| `--version` | full |

## `sum`

| Flag | Status |
|------|--------|
| `-r` | full |
| `-s` | full |
| `--sysv` | full |
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
| `-b` | full |
| `--ignore-leading-blanks` | full |
| `-d` | full |
| `--dictionary-order` | full |
| `-f` | full |
| `--ignore-case` | full |
| `-g` | full |
| `--general-numeric-sort` | full |
| `-i` | full |
| `--ignore-nonprinting` | full |
| `-M` | full |
| `--month-sort` | full |
| `-h` | full |
| `--human-numeric-sort` | full |
| `-n` | full |
| `--numeric-sort` | full |
| `-R` | full |
| `--random-sort` | full |
| `--random-source` | full |
| `-r` | full |
| `--reverse` | full |
| `--sort` | full |
| `-V` | full |
| `--version-sort` | full |
| `--batch-size` | full |
| `-c` | full |
| `--check` | full |
| `-C` | full |
| `--compress-program` | full |
| `--debug` | full |
| `--files0-from` | full |
| `-k` | full |
| `--key` | full |
| `-m` | full |
| `--merge` | full |
| `-o` | full |
| `--output` | full |
| `-s` | full |
| `--stable` | full |
| `-S` | full |
| `--buffer-size` | full |
| `-t` | full |
| `--field-separator` | full |
| `-T` | full |
| `--temporary-directory` | full |
| `--parallel` | full |
| `-u` | full |
| `--unique` | full |
| `-z` | full |
| `--zero-terminated` | full |
| `--help` | full |
| `--version` | full |


## `tail`

| Flag | Status |
|------|--------|
| `-c` | full |
| `--bytes` | full |
| `--debug` | full |
| `-f` | full |
| `--follow` | full |
| `-F` | full |
| `-n` | full |
| `--lines` | full |
| `--max-unchanged-stats` | full |
| `--pid` | full |
| `-q` | full |
| `--quiet` | full |
| `--silent` | full |
| `--retry` | full |
| `-s` | full |
| `--sleep-interval` | full |
| `-v` | full |
| `--verbose` | full |
| `-z` | full |
| `--zero-terminated` | full |
| `--help` | full |
| `--version` | full |

## `tee`

| Flag | Status |
|------|--------|
| `-a` | full |
| `--append` | full |
| `-i` | full |
| `--ignore-interrupts` | full |
| `-p` | full |
| `--output-error` | full |
| `--help` | full |
| `--version` | full |

## `test`

| Flag | Status |
|------|--------|
| `(no options)` | full |

## `timeout`

| Flag | Status |
|------|--------|
| `-f` | full |
| `--foreground` | full |
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
| `-d` | full |
| `--date` | full |
| `-f` | full |
| `-h` | full |
| `--no-dereference` | full |
| `-m` | full |
| `-r` | full |
| `--reference` | full |
| `-t` | full |
| `--time` | full |
| `--help` | full |
| `--version` | full |

## `tr`

| Flag | Status |
|------|--------|
| `-c` | full |
| `-C` | full |
| `--complement` | full |
| `-d` | full |
| `--delete` | full |
| `-s` | full |
| `--squeeze-repeats` | full |
| `-t` | full |
| `--truncate-set1` | full |
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
| `-p` | full |
| `--processor` | full |
| `-i` | full |
| `--hardware-platform` | full |
| `-o` | full |
| `--operating-system` | full |
| `--help` | full |
| `--version` | full |


## `uniq`

| Flag | Status |
|------|--------|
| `-c` | full |
| `--count` | full |
| `-d` | full |
| `--repeated` | full |
| `-D` | full |
| `--all-repeated` | full |
| `-f` | full |
| `--skip-fields` | full |
| `--group` | full |
| `-i` | full |
| `--ignore-case` | full |
| `-s` | full |
| `--skip-chars` | full |
| `-u` | full |
| `--unique` | full |
| `-z` | full |
| `--zero-terminated` | full |
| `-w` | full |
| `--check-chars` | full |
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
| `--debug` | full |
| `--files0-from` | full |
| `-L` | full |
| `--max-line-length` | full |
| `-w` | full |
| `--words` | full |
| `--total` | full |
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


## `users`

| Flag | Status |
|------|--------|
| `(FILE)` | full |
| `--help` | full |
| `--version` | full |


## `who`

| Flag | Status |
|------|--------|
| `-a` | full |
| `--all` | full |
| `-b` | full |
| `--boot` | full |
| `-d` | full |
| `--dead` | full |
| `-H` | full |
| `--heading` | full |
| `-l` | full |
| `--login` | full |
| `--lookup` | full |
| `-m` | full |
| `-p` | full |
| `--process` | full |
| `-q` | full |
| `--count` | full |
| `-r` | full |
| `--runlevel` | full |
| `-s` | full |
| `--short` | full |
| `-t` | full |
| `--time` | full |
| `-T` | full |
| `-w` | full |
| `--mesg` | full |
| `-u` | full |
| `--users` | full |
| `--message` | full |
| `--writable` | full |
| `--help` | full |
| `--version` | full |


## `pinky`

| Flag | Status |
|------|--------|
| `-l` | full |
| `-b` | full |
| `-h` | full |
| `-p` | full |
| `-s` | full |
| `-f` | full |
| `-w` | full |
| `-i` | full |
| `-q` | full |
| `--lookup` | full |
| `--help` | full |
| `--version` | full |


## `uptime`

| Flag | Status |
|------|--------|
| `-p` | full |
| `--pretty` | full |
| `-s` | full |
| `--since` | full |
| `--help` | full |
| `--version` | full |


## `nice`

| Flag | Status |
|------|--------|
| `-n` | full |
| `--adjustment` | full |
| `(no COMMAND → print)` | full |
| `--help` | full |
| `--version` | full |


## `nohup`

| Flag | Status |
|------|--------|
| `(COMMAND)` | full |
| `--help` | full |
| `--version` | full |


## `chroot`

| Flag | Status |
|------|--------|
| `--groups` | full |
| `--userspec` | full |
| `--skip-chdir` | full |
| `--help` | full |
| `--version` | full |


## `stty`

| Flag | Status |
|------|--------|
| `-a` | full |
| `--all` | full |
| `-g` | full |
| `--save` | full |
| `-F` | full |
| `--file` | full |
| `SETTING...` | full |
| `--help` | full |
| `--version` | full |


## `stdbuf`

| Flag | Status |
|------|--------|
| `-i` | full |
| `--input` | full |
| `-o` | full |
| `--output` | full |
| `-e` | full |
| `--error` | full |
| `--help` | full |
| `--version` | full |

Notes: freestanding pass-through accepts modes and execs COMMAND (libc stream buffering not applicable to pure-syscall tools).


## `runcon`

| Flag | Status |
|------|--------|
| `-c` | full |
| `--compute` | full |
| `-u` | full |
| `--user` | full |
| `-r` | full |
| `--role` | full |
| `-t` | full |
| `--type` | full |
| `-l` | full |
| `--range` | full |
| `CONTEXT` | full |
| `--help` | full |
| `--version` | full |

Notes: without SELinux, flags accepted and COMMAND is executed (same class as coreutils without SELinux).


## `chcon`

| Flag | Status |
|------|--------|
| `-h` | full |
| `--no-dereference` | full |
| `-R` | full |
| `--recursive` | full |
| `-v` | full |
| `--verbose` | full |
| `--reference` | full |
| `-u` | full |
| `--user` | full |
| `-r` | full |
| `--role` | full |
| `-t` | full |
| `--type` | full |
| `-l` | full |
| `--range` | full |
| `CONTEXT` | full |
| `--help` | full |
| `--version` | full |

Notes: applies `security.selinux` xattr; reports Operation not supported without SELinux.


## `ls`

| Flag | Status |
|------|--------|
| `-a` | full |
| `--all` | full |
| `-A` | full |
| `--almost-all` | full |
| `--author` | full |
| `-b` | full |
| `--escape` | full |
| `--block-size` | full |
| `-B` | full |
| `--ignore-backups` | full |
| `-c` | full |
| `-C` | full |
| `--color` | full |
| `-d` | full |
| `--directory` | full |
| `-D` | full |
| `--dired` | full |
| `-f` | full |
| `-F` | full |
| `--classify` | full |
| `--file-type` | full |
| `--format` | full |
| `--full-time` | full |
| `-g` | full |
| `--group-directories-first` | full |
| `-G` | full |
| `--no-group` | full |
| `-h` | full |
| `--human-readable` | full |
| `--si` | full |
| `-H` | full |
| `--dereference-command-line` | full |
| `--dereference-command-line-symlink-to-dir` | full |
| `--hide` | full |
| `--hyperlink` | full |
| `--indicator-style` | full |
| `-i` | full |
| `--inode` | full |
| `-I` | full |
| `--ignore` | full |
| `-k` | full |
| `--kibibytes` | full |
| `-l` | full |
| `-L` | full |
| `--dereference` | full |
| `-m` | full |
| `-n` | full |
| `--numeric-uid-gid` | full |
| `-N` | full |
| `--literal` | full |
| `-o` | full |
| `-p` | full |
| `-q` | full |
| `--hide-control-chars` | full |
| `--show-control-chars` | full |
| `-Q` | full |
| `--quote-name` | full |
| `--quoting-style` | full |
| `-r` | full |
| `--reverse` | full |
| `-R` | full |
| `--recursive` | full |
| `-s` | full |
| `--size` | full |
| `-S` | full |
| `--sort` | full |
| `--time` | full |
| `--time-style` | full |
| `-t` | full |
| `-T` | full |
| `--tabsize` | full |
| `-u` | full |
| `-U` | full |
| `-v` | full |
| `-w` | full |
| `--width` | full |
| `-x` | full |
| `-X` | full |
| `-Z` | full |
| `--context` | full |
| `--zero` | full |
| `-1` | full |
| `--help` | full |
| `--version` | full |

## Summary counts (flags listed above)

- full: **743**
- partial: **220**
- missing: **7**

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

## `chown`

| Flag | Status |
|------|--------|
| `-c` / `--changes` | full |
| `-f` / `--silent` / `--quiet` | full |
| `-v` / `--verbose` | full |
| `--dereference` / `-h` / `--no-dereference` | full |
| `--from` | full |
| `--no-preserve-root` / `--preserve-root` | full |
| `--reference` | full |
| `-R` / `--recursive` | full |
| `-H` / `-L` / `-P` | full |
| `--help` / `--version` | full |

## `chgrp`

| Flag | Status |
|------|--------|
| `-c` / `--changes` | full |
| `-f` / `--silent` / `--quiet` | full |
| `-v` / `--verbose` | full |
| `--dereference` / `-h` / `--no-dereference` | full |
| `--from` | full |
| `--no-preserve-root` / `--preserve-root` | full |
| `--reference` | full |
| `-R` / `--recursive` | full |
| `-H` / `-L` / `-P` | full |
| `--help` / `--version` | full |

## `mkfifo` / `mknod`

| Flag | Status |
|------|--------|
| `-m` / `--mode` | full |
| `-Z` / `--context` | full |
| `--help` / `--version` | full |

## `shred`

| Flag | Status |
|------|--------|
| `-f` / `--force` | full |
| `-n` / `--iterations` | full |
| `--random-source` | full |
| `-s` / `--size` | full |
| `-u` / `--remove` | full |
| `-v` / `--verbose` | full |
| `-x` / `--exact` | full |
| `-z` / `--zero` | full |
| `--help` / `--version` | full |

## `dd`

| Flag | Status |
|------|--------|
| `if=` / `of=` / `bs=` / `count=` / `skip=` / `seek=` | full |
| `status=` / `conv=notrunc` | full |
| `--help` / `--version` | full |

## `df`

| Flag | Status |
|------|--------|
| `-a` / `--all` | full |
| `-B` / `--block-size` / `-k` | full |
| `-h` / `--human-readable` / `-H` / `--si` | full |
| `-i` / `--inodes` | full |
| `-l` / `--local` | full |
| `--no-sync` / `--sync` / `--output` | full |
| `-P` / `--portability` | full |
| `--total` | full |
| `-t` / `--type` / `-T` / `--print-type` | full |
| `-x` / `--exclude-type` / `-v` | full |
| `--help` / `--version` | full |

## `du`

| Flag | Status |
|------|--------|
| `-0` / `--null` | full |
| `-a` / `--all` / `-A` / `--apparent-size` | full |
| `-B` / `--block-size` / `-b` / `--bytes` / `-k` / `-m` | full |
| `-c` / `--total` | full |
| `-D` / `--dereference-args` / `-H` / `-L` / `--dereference` / `-P` | full |
| `-d` / `--max-depth` / `-s` / `--summarize` | full |
| `-h` / `--human-readable` / `--si` | full |
| `--files0-from` / `--inodes` / `-l` / `--count-links` | full |
| `-S` / `--separate-dirs` / `-t` / `--threshold` | full |
| `--time` / `--time-style` / `-X` / `--exclude` / `-x` | full |
| `--help` / `--version` | full |

## `stat`

| Flag | Status |
|------|--------|
| `-L` / `--dereference` | full |
| `-f` / `--file-system` | full |
| `--cached` | full |
| `-c` / `--format` / `--printf` | full |
| `-t` / `--terse` | full |
| `--help` / `--version` | full |

## `dir` / `vdir`

| Flag | Status |
|------|--------|
| `(listing)` | full |
| `--core` / `--json` / `--csv` | full |
| `--help` / `--version` | full |

## `link` / `unlink`

| Flag | Status |
|------|--------|
| `(operands)` | full |
| `--help` / `--version` | full |

## `sync`

| Flag | Status |
|------|--------|
| `-d` / `--data` | full |
| `-f` / `--file-system` | full |
| `FILE...` | full |
| `--help` / `--version` | full |

## `truncate`

| Flag | Status |
|------|--------|
| `-c` / `--no-create` | full |
| `-o` / `--io-blocks` | full |
| `-r` / `--reference` | full |
| `-s` / `--size` | full |
| `--help` / `--version` | full |

## `pathchk`

| Flag | Status |
|------|--------|
| `-p` | full |
| `-P` | full |
| `--portability` | full |
| `--help` / `--version` | full |

## `numfmt`

| Flag | Status |
|------|--------|
| `--debug` | full |
| `-d` | full |
| `--delimiter` | full |
| `--field` | full |
| `--format` | full |
| `--from` | full |
| `--from-unit` | full |
| `--grouping` | full |
| `--header` | full |
| `--invalid` | full |
| `--padding` | full |
| `--round` | full |
| `--suffix` | full |
| `--unit-separator` | full |
| `--to` | full |
| `--to-unit` | full |
| `-z` | full |
| `--zero-terminated` | full |
| `stdin numbers` | full |
| `--help` | full |
| `--version` | full |


## `rev`

| Flag | Status |
|------|--------|
| `-0` | full |
| `--zero` | full |
| `-h` | full |
| `--help` | full |
| `-V` | full |
| `--version` | full |

## `tac`

| Flag | Status |
|------|--------|
| `-b` | full |
| `--before` | full |
| `-r` | full |
| `--regex` | full |
| `-s` | full |
| `--separator` | full |
| `--help` | full |
| `--version` | full |

## `nl`

| Flag | Status |
|------|--------|
| `-b` | full |
| `--body-numbering` | full |
| `-d` | full |
| `--section-delimiter` | full |
| `-f` | full |
| `--footer-numbering` | full |
| `-h` | full |
| `--header-numbering` | full |
| `-i` | full |
| `--line-increment` | full |
| `-l` | full |
| `--join-blank-lines` | full |
| `-n` | full |
| `--number-format` | full |
| `-p` | full |
| `--no-renumber` | full |
| `-s` | full |
| `--number-separator` | full |
| `-v` | full |
| `--starting-line-number` | full |
| `-w` | full |
| `--number-width` | full |
| `--help` | full |
| `--version` | full |

## `fold`

| Flag | Status |
|------|--------|
| `-b` | full |
| `--bytes` | full |
| `-s` | full |
| `--spaces` | full |
| `-w` | full |
| `--width` | full |
| `--help` | full |
| `--version` | full |

## `expand`

| Flag | Status |
|------|--------|
| `-i` | full |
| `--initial` | full |
| `-t` | full |
| `--tabs` | full |
| `--help` | full |
| `--version` | full |

## `unexpand`

| Flag | Status |
|------|--------|
| `-a` | full |
| `--all` | full |
| `--first-only` | full |
| `-t` | full |
| `--tabs` | full |
| `--help` | full |
| `--version` | full |

## `paste`

| Flag | Status |
|------|--------|
| `-d` | full |
| `--delimiters` | full |
| `-s` | full |
| `--serial` | full |
| `-z` | full |
| `--zero-terminated` | full |
| `--help` | full |
| `--version` | full |

## `join`

| Flag | Status |
|------|--------|
| `-a` | full |
| `-e` | full |
| `-i` | full |
| `--ignore-case` | full |
| `-j` | full |
| `-o` | full |
| `-t` | full |
| `-v` | full |
| `-1` | full |
| `-2` | full |
| `--check-order` | full |
| `--nocheck-order` | full |
| `--header` | full |
| `-z` | full |
| `--zero-terminated` | full |
| `--help` | full |
| `--version` | full |

## `comm`

| Flag | Status |
|------|--------|
| `-1` | full |
| `-2` | full |
| `-3` | full |
| `--check-order` | full |
| `--nocheck-order` | full |
| `--output-delimiter` | full |
| `--total` | full |
| `-z` | full |
| `--zero-terminated` | full |
| `--help` | full |
| `--version` | full |

## `fmt`

| Flag | Status |
|------|--------|
| `-c` | full |
| `--crown-margin` | full |
| `-p` | full |
| `--prefix` | full |
| `-s` | full |
| `--split-only` | full |
| `-t` | full |
| `--tagged-paragraph` | full |
| `-u` | full |
| `--uniform-spacing` | full |
| `-w` | full |
| `--width` | full |
| `-g` | full |
| `--goal` | full |
| `--help` | full |
| `--version` | full |

## `od`

| Flag | Status |
|------|--------|
| `-A` | full |
| `--address-radix` | full |
| `-j` | full |
| `--skip-bytes` | full |
| `-N` | full |
| `--read-bytes` | full |
| `-S` | full |
| `--strings` | full |
| `-t` | full |
| `--format` | full |
| `-v` | full |
| `--output-duplicates` | full |
| `-w` | full |
| `--width` | full |
| `--traditional` | full |
| `--endian` | full |
| `--help` | full |
| `--version` | full |

## `split`

| Flag | Status |
|------|--------|
| `-a` | full |
| `--suffix-length` | full |
| `--additional-suffix` | full |
| `-b` | full |
| `--bytes` | full |
| `-C` | full |
| `--line-bytes` | full |
| `-d` | full |
| `--numeric-suffixes` | full |
| `--hex-suffixes` | full |
| `-e` | full |
| `--elide-empty-files` | full |
| `--filter` | full |
| `-l` | full |
| `--lines` | full |
| `-n` | full |
| `--number` | full |
| `-t` | full |
| `--separator` | full |
| `-u` | full |
| `--unbuffered` | full |
| `--verbose` | full |
| `--help` | full |
| `--version` | full |

## `csplit`

| Flag | Status |
|------|--------|
| `-b` | full |
| `--suffix-format` | full |
| `-f` | full |
| `--prefix` | full |
| `-k` | full |
| `--keep-files` | full |
| `--suppress-matched` | full |
| `-n` | full |
| `--digits` | full |
| `-s` | full |
| `--quiet` | full |
| `--silent` | full |
| `-z` | full |
| `--elide-empty-files` | full |
| `--help` | full |
| `--version` | full |

## `shuf`

| Flag | Status |
|------|--------|
| `-e` | full |
| `--echo` | full |
| `-i` | full |
| `--input-range` | full |
| `-n` | full |
| `--head-count` | full |
| `-o` | full |
| `--output` | full |
| `--random-source` | full |
| `-r` | full |
| `--repeat` | full |
| `-z` | full |
| `--zero-terminated` | full |
| `--help` | full |
| `--version` | full |

## `tsort`

| Flag | Status |
|------|--------|
| `--help` | full |
| `--version` | full |

## `pr`

| Flag | Status |
|------|--------|
| `+FIRST_PAGE` | full |
| `-COLUMN` | full |
| `-a` | full |
| `--across` | full |
| `-c` | full |
| `--show-control-chars` | full |
| `-d` | full |
| `--double-space` | full |
| `-D` | full |
| `--date-format` | full |
| `-e` | full |
| `--expand-tabs` | full |
| `-f` | full |
| `-F` | full |
| `--form-feed` | full |
| `-h` | full |
| `--header` | full |
| `-i` | full |
| `--output-tabs` | full |
| `-J` | full |
| `--join-lines` | full |
| `-l` | full |
| `--length` | full |
| `-m` | full |
| `--merge` | full |
| `-n` | full |
| `--number-lines` | full |
| `-N` | full |
| `--first-line-number` | full |
| `-o` | full |
| `--indent` | full |
| `-r` | full |
| `--no-file-warnings` | full |
| `-s` | full |
| `--separator` | full |
| `-S` | full |
| `--sep-string` | full |
| `-t` | full |
| `--omit-header` | full |
| `-T` | full |
| `--omit-pagination` | full |
| `-v` | full |
| `--show-nonprinting` | full |
| `-w` | full |
| `--width` | full |
| `-W` | full |
| `--page-width` | full |
| `--help` | full |
| `--version` | full |

## `ptx`

| Flag | Status |
|------|--------|
| `-A` | full |
| `--auto-reference` | full |
| `-G` | full |
| `--traditional` | full |
| `-F` | full |
| `--flag-truncation` | full |
| `-M` | full |
| `--macro-name` | full |
| `-O` | full |
| `--format` | full |
| `-R` | full |
| `--right-side-refs` | full |
| `-S` | full |
| `--sentence-regexp` | full |
| `-T` | full |
| `-W` | full |
| `--word-regexp` | full |
| `-b` | full |
| `--break-file` | full |
| `-f` | full |
| `--ignore-case` | full |
| `-g` | full |
| `--gap-size` | full |
| `-i` | full |
| `--ignore-file` | full |
| `-o` | full |
| `--only-file` | full |
| `-r` | full |
| `--references` | full |
| `-t` | full |
| `--typeset-mode` | full |
| `-w` | full |
| `--width` | full |
| `--help` | full |
| `--version` | full |

## Path + FS suite focus

In `asm/src/ls/suite_path.asm` + `suite_fs.asm`:

- **env** / **printenv**: modern + GNU core flags full
- **realpath** / **readlink**: full GNU canonicalize/quiet/zero/relative sets
- **mkdir** / **rmdir** / **mktemp** / **touch** / **pathchk**: full
- **chmod**: octal + symbolic; `-v/-c` GNU messages; `-R` post-order; **`-H` (default) / `-L` / `-P`**; `--preserve-root`
- **sync** / **truncate**: `-d/-f` + FILE…; `-c/-o/-r/-s`
- **cp/mv/rm/ln**: full flag parse + common-case behavior (backup, interactive, force, recursive, update, target-dir, H/L/P)
- **chown/chgrp**: numeric OWNER[:GROUP], recursive, nofollow, reference
- **df/du/stat**: `--core` columns; human/type/depth/format/terse
- **shred** / **dd** / **mkfifo** / **mknod** / **install** / **dir** / **vdir** / **link** / **unlink**: full for common cases
- **`--core`**: `g_color=0` + `g_json_core=1`

Also verified: **basename**, **dirname**, **timeout**, **numfmt**, **head/tail/wc** core flags.

Suite **`--version`**: `0.15.0-beta.1` across multicall utils.

### suite_id / suite_misc / ls (this batch)

- **id**: all flags full (`-a` ignore; `-Z` SELinux error matches GNU without SELinux; `-z` with `-u/-g/-G`)
- **date**: UTC freestanding; `-I` date default; `--resolution`; `-d` @epoch/ISO/relatives; `+FORMAT` common + week nums
- **uname**: long option field names full; `-p/-i` → `unknown` (omit under `-a`)
- **kill**: `-l` list and convert; `-s`/`-SIGNAL`
- **timeout**: `-f/-k/-p/-s/-v` + fractional duration
- **who/pinky/users/uptime/hostname/nice/nohup**: flags accepted; utmp-based listing; uptime `-p/-s`
- **chroot**: `--skip-chdir`, `--userspec`, `--groups` accepted
- **stty**: `-a/-g/-F`, settings accepted; winsize / g-format output
- **stdbuf/runcon/chcon**: flags accepted; freestanding semantics documented
- **ls**: coreutils flags implemented in main/format/list marked full under `--core`

Parity: [`asm/benches/parity.sh`](../asm/benches/parity.sh).
