<!-- progress: total=106 shipped=106 core_full=22 core_partial=84 core_missing=0 -->
**Progress (goal = replace every coreutil):** **106/106** tools shipped · **`--core` depth:** 22 full · 84 partial · 0 missing

| Status | Count | Meaning |
|--------|------:|---------|
| shipped | 106/106 | Multicall name exists as `f00-*` |
| `--core` **full** | 22 | Tracked flags match for common cases |
| `--core` partial | 84 | Tool works; some GNU flags still deepening |
| `--core` **missing** | 0 | Not yet in multicall |

Legend — **speed:** `win` = measured faster under `--core`; `win*` = hash-family; `TBD` = not on formal speed-gate yet; `—` = not shipped.

| # | coreutils | f00 | shipped | `--core` depth | modern | speed vs GNU |
|--:|:----------|:----|:--------|:---------------|:-------|:-------------|
| 1 | `arch` | `f00-arch` | yes | **full** | yes | TBD |
| 2 | `b2sum` | `f00-b2sum` | yes | partial | yes | win |
| 3 | `base32` | `f00-base32` | yes | partial | yes | TBD |
| 4 | `base64` | `f00-base64` | yes | partial | yes | TBD |
| 5 | `basename` | `f00-basename` | yes | **full** | yes | win |
| 6 | `basenc` | `f00-basenc` | yes | partial | yes | TBD |
| 7 | `cat` | `f00-cat` | yes | **full** | deep | win |
| 8 | `chcon` | `f00-chcon` | yes | partial | yes | TBD |
| 9 | `chgrp` | `f00-chgrp` | yes | partial | yes | TBD |
| 10 | `chmod` | `f00-chmod` | yes | partial | yes | TBD |
| 11 | `chown` | `f00-chown` | yes | partial | yes | TBD |
| 12 | `chroot` | `f00-chroot` | yes | partial | yes | TBD |
| 13 | `cksum` | `f00-cksum` | yes | partial | yes | TBD |
| 14 | `comm` | `f00-comm` | yes | partial | yes | TBD |
| 15 | `cp` | `f00-cp` | yes | partial | yes | TBD |
| 16 | `csplit` | `f00-csplit` | yes | partial | yes | TBD |
| 17 | `cut` | `f00-cut` | yes | partial | yes | TBD |
| 18 | `date` | `f00-date` | yes | partial | yes | TBD |
| 19 | `dd` | `f00-dd` | yes | partial | yes | TBD |
| 20 | `df` | `f00-df` | yes | partial | yes | TBD |
| 21 | `dir` | `f00-dir` | yes | partial | yes | TBD |
| 22 | `dircolors` | `f00-dircolors` | yes | partial | yes | TBD |
| 23 | `dirname` | `f00-dirname` | yes | **full** | yes | TBD |
| 24 | `du` | `f00-du` | yes | partial | yes | TBD |
| 25 | `echo` | `f00-echo` | yes | **full** | yes | TBD |
| 26 | `env` | `f00-env` | yes | partial | yes | TBD |
| 27 | `expand` | `f00-expand` | yes | partial | yes | TBD |
| 28 | `expr` | `f00-expr` | yes | **full** | yes | TBD |
| 29 | `factor` | `f00-factor` | yes | partial | yes | TBD |
| 30 | `false` | `f00-false` | yes | **full** | yes | TBD |
| 31 | `fmt` | `f00-fmt` | yes | partial | yes | TBD |
| 32 | `fold` | `f00-fold` | yes | partial | yes | TBD |
| 33 | `groups` | `f00-groups` | yes | **full** | yes | TBD |
| 34 | `head` | `f00-head` | yes | partial | yes | win |
| 35 | `hostid` | `f00-hostid` | yes | **full** | yes | TBD |
| 36 | `id` | `f00-id` | yes | partial | yes | win |
| 37 | `install` | `f00-install` | yes | **full** | yes | TBD |
| 38 | `join` | `f00-join` | yes | partial | yes | TBD |
| 39 | `link` | `f00-link` | yes | partial | yes | TBD |
| 40 | `ln` | `f00-ln` | yes | partial | yes | TBD |
| 41 | `logname` | `f00-logname` | yes | **full** | yes | TBD |
| 42 | `ls` | `f00-ls` | yes | partial | deep | win |
| 43 | `md5sum` | `f00-md5sum` | yes | partial | yes | win |
| 44 | `mkdir` | `f00-mkdir` | yes | partial | yes | TBD |
| 45 | `mkfifo` | `f00-mkfifo` | yes | partial | yes | TBD |
| 46 | `mknod` | `f00-mknod` | yes | partial | yes | TBD |
| 47 | `mktemp` | `f00-mktemp` | yes | partial | yes | TBD |
| 48 | `mv` | `f00-mv` | yes | partial | yes | TBD |
| 49 | `nice` | `f00-nice` | yes | partial | yes | TBD |
| 50 | `nl` | `f00-nl` | yes | partial | yes | TBD |
| 51 | `nohup` | `f00-nohup` | yes | partial | yes | TBD |
| 52 | `nproc` | `f00-nproc` | yes | **full** | yes | win |
| 53 | `numfmt` | `f00-numfmt` | yes | partial | yes | TBD |
| 54 | `od` | `f00-od` | yes | partial | yes | TBD |
| 55 | `paste` | `f00-paste` | yes | partial | yes | TBD |
| 56 | `pathchk` | `f00-pathchk` | yes | partial | yes | TBD |
| 57 | `pinky` | `f00-pinky` | yes | partial | yes | TBD |
| 58 | `pr` | `f00-pr` | yes | partial | yes | TBD |
| 59 | `printenv` | `f00-printenv` | yes | **full** | yes | TBD |
| 60 | `printf` | `f00-printf` | yes | **full** | yes | TBD |
| 61 | `ptx` | `f00-ptx` | yes | partial | yes | TBD |
| 62 | `pwd` | `f00-pwd` | yes | **full** | yes | TBD |
| 63 | `readlink` | `f00-readlink` | yes | partial | yes | TBD |
| 64 | `realpath` | `f00-realpath` | yes | partial | yes | win |
| 65 | `rm` | `f00-rm` | yes | partial | yes | TBD |
| 66 | `rmdir` | `f00-rmdir` | yes | **full** | yes | TBD |
| 67 | `runcon` | `f00-runcon` | yes | partial | yes | TBD |
| 68 | `seq` | `f00-seq` | yes | partial | yes | win |
| 69 | `sha1sum` | `f00-sha1sum` | yes | partial | yes | win* |
| 70 | `sha224sum` | `f00-sha224sum` | yes | partial | yes | win* |
| 71 | `sha256sum` | `f00-sha256sum` | yes | partial | yes | win |
| 72 | `sha384sum` | `f00-sha384sum` | yes | partial | yes | win* |
| 73 | `sha512sum` | `f00-sha512sum` | yes | partial | yes | win* |
| 74 | `shred` | `f00-shred` | yes | partial | yes | TBD |
| 75 | `shuf` | `f00-shuf` | yes | partial | yes | TBD |
| 76 | `sleep` | `f00-sleep` | yes | **full** | yes | TBD |
| 77 | `sort` | `f00-sort` | yes | partial | yes | win |
| 78 | `split` | `f00-split` | yes | partial | yes | TBD |
| 79 | `stat` | `f00-stat` | yes | partial | yes | TBD |
| 80 | `stdbuf` | `f00-stdbuf` | yes | partial | yes | TBD |
| 81 | `stty` | `f00-stty` | yes | partial | yes | TBD |
| 82 | `sum` | `f00-sum` | yes | partial | yes | TBD |
| 83 | `sync` | `f00-sync` | yes | partial | yes | TBD |
| 84 | `tac` | `f00-tac` | yes | partial | yes | TBD |
| 85 | `tail` | `f00-tail` | yes | partial | yes | win |
| 86 | `tee` | `f00-tee` | yes | partial | yes | TBD |
| 87 | `test` | `f00-test` | yes | **full** | yes | TBD |
| 88 | `timeout` | `f00-timeout` | yes | partial | yes | TBD |
| 89 | `touch` | `f00-touch` | yes | partial | yes | TBD |
| 90 | `tr` | `f00-tr` | yes | partial | yes | TBD |
| 91 | `true` | `f00-true` | yes | **full** | yes | win |
| 92 | `truncate` | `f00-truncate` | yes | partial | yes | TBD |
| 93 | `tsort` | `f00-tsort` | yes | partial | yes | TBD |
| 94 | `tty` | `f00-tty` | yes | **full** | yes | TBD |
| 95 | `uname` | `f00-uname` | yes | partial | yes | win |
| 96 | `unexpand` | `f00-unexpand` | yes | partial | yes | TBD |
| 97 | `uniq` | `f00-uniq` | yes | partial | yes | TBD |
| 98 | `unlink` | `f00-unlink` | yes | partial | yes | TBD |
| 99 | `uptime` | `f00-uptime` | yes | partial | yes | TBD |
| 100 | `users` | `f00-users` | yes | partial | yes | TBD |
| 101 | `vdir` | `f00-vdir` | yes | partial | yes | TBD |
| 102 | `wc` | `f00-wc` | yes | partial | yes | win |
| 103 | `who` | `f00-who` | yes | partial | yes | TBD |
| 104 | `whoami` | `f00-whoami` | yes | **full** | yes | TBD |
| 105 | `yes` | `f00-yes` | yes | **full** | yes | TBD |
| 106 | `[` | `f00-[ / test` | yes | partial | yes | TBD |

Also shipped (useful multicall extras; not always in the coreutils package): `f00-hostname`, `f00-kill`, `f00-rev`.
