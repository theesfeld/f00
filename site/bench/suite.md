# Suite benchmarks (f00 vs GNU coreutils)

**Overall: 2.3× faster than GNU coreutils overall** (128% faster overall; geo mean of per-tool speedups)

Generated: `2026-07-24T13:58:10Z` · N=15 median · warm-cache spawn-inclusive median

Host: x86_64 · Linux 6.17.0-1020-azure

Tools timed: 91 · wins: 91 · median 2.18× · total-time 2.602×

| Tool | Command (f00) | GNU ms | f00 ms | Speedup | Sample output (f00) |
|------|---------------|-------:|-------:|--------:|---------------------|
| `true` | `f00-true --core` | 0.331 | **0.127** | **2.61×** | `` |
| `false` | `f00-false --core` | 0.335 | **0.129** | **2.59×** | `` |
| `basename` | `f00-basename --core /usr/bin/ls` | 0.483 | **0.128** | **3.77×** | `ls` |
| `dirname` | `f00-dirname --core /usr/bin/ls` | 0.472 | **0.131** | **3.61×** | `/usr/bin` |
| `echo` | `f00-echo --core hi` | 0.486 | **0.224** | **2.17×** | `hi` |
| `pwd` | `f00-pwd --core` | 0.548 | **0.265** | **2.07×** | `/home/runner/work/f00/f00` |
| `nproc` | `f00-nproc --core` | 0.591 | **0.164** | **3.61×** | `4` |
| `whoami` | `f00-whoami --core` | 0.551 | **0.141** | **3.91×** | `runner` |
| `uname` | `f00-uname --core -s` | 0.491 | **0.221** | **2.23×** | `Linux` |
| `id` | `f00-id --core -u` | 0.589 | **0.221** | **2.67×** | `1001` |
| `date` | `f00-date --core -u +%Y` | 0.485 | **0.236** | **2.06×** | `2026` |
| `printenv` | `f00-printenv --core PATH` | 0.467 | **0.221** | **2.11×** | `/snap/bin:/home/runner/.local/bin:/opt/pipx_bin:/home/runner/.cargo/bin:/home/ru` |
| `printf` | `f00-printf --core %s world` | 0.472 | **0.216** | **2.18×** | `world` |
| `factor` | `f00-factor --core 12` | 0.516 | **0.227** | **2.27×** | `12: 2 2 3` |
| `numfmt` | `f00-numfmt --core --to=si 1000` | 0.487 | **0.219** | **2.23×** | `1.0k` |
| `expr` | `f00-expr --core 1 + 1` | 0.543 | **0.227** | **2.40×** | `2` |
| `seq` | `f00-seq --core 1 5` | 0.477 | **0.220** | **2.17×** | `1 2 3 4 5` |
| `cat` | `f00-cat --core fixture.txt` | 0.488 | **0.152** | **3.22×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `wc` | `f00-wc --core -l fixture.txt` | 0.504 | **0.258** | **1.95×** | `400 /tmp/f00-suite-bench.00ayr4i3/fix.txt` |
| `head` | `f00-head --core -n 3 fixture.txt` | 0.487 | **0.217** | **2.24×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tail` | `f00-tail --core -n 3 fixture.txt` | 0.503 | **0.249** | **2.02×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `nl` | `f00-nl --core fixture.txt` | 0.539 | **0.298** | **1.81×** | `1 suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 2 suite-bench line abcd` |
| `od` | `f00-od --core -An -tx1 -N8 fixture.txt` | 0.493 | **0.224** | **2.20×** | `73 75 69 74 65 2d 62 65` |
| `cut` | `f00-cut --core -d: -f1 /etc/passwd` | 0.491 | **0.229** | **2.15×** | `root daemon bin sys sync games man lp mail news uucp proxy www-data backup list ` |
| `tr` | `f00-tr --core a-z A-Z` | 0.502 | **0.229** | **2.19×** | `HELLO` |
| `sort` | `f00-sort --core fixture.txt` | 0.951 | **0.627** | **1.52×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `uniq` | `f00-uniq --core a.txt` | 0.525 | **0.273** | **1.92×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789` |
| `paste` | `f00-paste --core a.txt b.txt` | 0.580 | **0.348** | **1.67×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `comm` | `f00-comm --core -12 a.txt b.txt` | 0.605 | **0.336** | **1.80×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `join` | `f00-join --core a.txt b.txt` | 11.174 | **0.363** | **30.79×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 line abcdefghijklmnopqrst` |
| `base64` | `f00-base64 --core fixture.txt` | 0.523 | **0.372** | **1.40×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `base32` | `f00-base32 --core fixture.txt` | 0.541 | **0.374** | **1.45×** | `ON2WS5DFFVRGK3TDNAQGY2LOMUQGCYTDMRSWMZ3INFVGW3DNNZXXA4LSON2HK5TXPB4XUIBQGEZD GNB` |
| `basenc` | `f00-basenc --core --base64 fixture.txt` | 0.526 | **0.366** | **1.44×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `md5sum` | `f00-md5sum --core fixture.txt` | 0.819 | **0.275** | **2.98×** | `a5e6b1aa8523bc01f561fcef58d16894 /tmp/f00-suite-bench.00ayr4i3/fix.txt` |
| `sha1sum` | `f00-sha1sum --core fixture.txt` | 0.786 | **0.304** | **2.59×** | `49f92a1f57c1a825b5ca5777c92d6e15ac26a8ea /tmp/f00-suite-bench.00ayr4i3/fix.txt` |
| `sha224sum` | `f00-sha224sum --core fixture.txt` | 0.785 | **0.348** | **2.25×** | `94c1dff65fd14336129da4555171327a3e1e3e684810af23fa13e7f3 /tmp/f00-suite-bench.00` |
| `sha256sum` | `f00-sha256sum --core fixture.txt` | 0.785 | **0.362** | **2.17×** | `7c28ea8726bc6923d5b38a6b6938ad5907c01dc6640e1645cf63cbf9df706132 /tmp/f00-suite-` |
| `sha384sum` | `f00-sha384sum --core fixture.txt` | 0.797 | **0.303** | **2.63×** | `f2578c293b7eeebf35402906e5e2fcd522b567687241b72950690c389f88baf83f9cd518c1fd67b3` |
| `sha512sum` | `f00-sha512sum --core fixture.txt` | 0.806 | **0.300** | **2.69×** | `a3282daa5cc665baa2b247ee17a0798f1d8028bbfb2107ea0df9493d0e57bfba10ba95d72d2550c2` |
| `b2sum` | `f00-b2sum --core fixture.txt` | 0.526 | **0.282** | **1.86×** | `915149393ea1091e4aa19ad9c68f980ebf83f5da2a576a20ca65fb001c685bf890523b442b840760` |
| `cksum` | `f00-cksum --core fixture.txt` | 0.795 | **0.274** | **2.91×** | `1448063438 22000 /tmp/f00-suite-bench.00ayr4i3/fix.txt` |
| `sum` | `f00-sum --core fixture.txt` | 0.531 | **0.254** | **2.09×** | `9481 22 /tmp/f00-suite-bench.00ayr4i3/fix.txt` |
| `ls` | `f00-ls --core -1 dir` | 0.638 | **0.337** | **1.89×** | `f01.txt f02.txt f03.txt f04.txt f05.txt f06.txt f07.txt f08.txt f09.txt f10.txt ` |
| `dir` | `f00-dir --core -1 dir` | 0.663 | **0.239** | **2.77×** | `f06.txt f02.txt f20.txt f14.txt f09.txt f13.txt f10.txt f17.txt f16.txt f08.txt ` |
| `vdir` | `f00-vdir --core -1 dir` | 0.637 | **0.224** | **2.84×** | `- f06.txt - f02.txt - f20.txt - f14.txt - f09.txt - f13.txt - f10.txt - f17.txt ` |
| `stat` | `f00-stat --core -c %s fixture.txt` | 0.601 | **0.215** | **2.79×** | `22000` |
| `realpath` | `f00-realpath --core .` | 0.469 | **0.333** | **1.41×** | `/home/runner/work/f00/f00/asm` |
| `readlink` | `f00-readlink --core /proc/self/exe` | 0.489 | **0.228** | **2.15×** | `/home/runner/work/f00/f00/asm/f00` |
| `df` | `f00-df --core -P /` | 0.588 | **0.245** | **2.40×** | `Filesystem 1K-blocks Used Available Use% Mounted on /dev/root 151263856 59919652` |
| `du` | `f00-du --core -s dir` | 0.541 | **0.253** | **2.14×** | `5 /tmp/f00-suite-bench.00ayr4i3/dir` |
| `dircolors` | `f00-dircolors --core -p` | 0.477 | **0.221** | **2.16×** | `# Configuration file for dircolors, a utility to help you set the # LS_COLORS en` |
| `env` | `f00-env --core -i true` | 0.767 | **0.248** | **3.10×** | `` |
| `timeout` | `f00-timeout --core 5 true` | 0.981 | **0.308** | **3.19×** | `` |
| `nice` | `f00-nice --core true` | 0.752 | **0.229** | **3.28×** | `` |
| `nohup` | `f00-nohup --core true` | 0.764 | **0.224** | **3.41×** | `` |
| `sleep` | `f00-sleep --core 0` | 0.541 | **0.279** | **1.93×** | `` |
| `test` | `f00-test --core -f fixture.txt` | 0.465 | **0.282** | **1.65×** | `` |
| `pathchk` | `f00-pathchk --core ok-name` | 0.460 | **0.220** | **2.08×** | `` |
| `mktemp` | `f00-mktemp --core -u` | 0.475 | **0.219** | **2.16×** | `/tmp/tmp.H5ezvq` |
| `sync` | `f00-sync --core` | 0.672 | **0.376** | **1.79×** | `` |
| `uptime` | `f00-uptime --core` | 0.951 | **0.236** | **4.03×** | `up 1 minute` |
| `hostid` | `f00-hostid --core` | 0.554 | **0.326** | **1.70×** | `db830370` |
| `logname` | `f00-logname --core` | 0.501 | **0.311** | **1.61×** | `runner` |
| `tty` | `f00-tty --core` | 0.485 | **0.129** | **3.75×** | `not a tty` |
| `groups` | `f00-groups --core` | 0.543 | **0.245** | **2.21×** | `adm users docker systemd-journal runner` |
| `arch` | `f00-arch --core` | 0.472 | **0.233** | **2.03×** | `x86_64` |
| `hostname` | `f00-hostname --core` | 0.352 | **0.221** | **1.59×** | `runnervmvrwv9` |
| `users` | `f00-users --core` | 0.498 | **0.222** | **2.24×** | `` |
| `who` | `f00-who --core` | 0.511 | **0.222** | **2.30×** | `` |
| `pinky` | `f00-pinky --core` | 0.522 | **0.413** | **1.26×** | `` |
| `fold` | `f00-fold --core -w 40 fixture.txt` | 0.565 | **0.299** | **1.89×** | `suite-bench line abcdefghijklmnopqrstuvw xyz 0123456789 suite-bench line abcdefg` |
| `fmt` | `f00-fmt --core -w 40 fixture.txt` | 0.599 | **0.274** | **2.18×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `expand` | `f00-expand --core fixture.txt` | 0.541 | **0.290** | **1.87×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `unexpand` | `f00-unexpand --core fixture.txt` | 0.527 | **0.320** | **1.64×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tac` | `f00-tac --core fixture.txt` | 0.526 | **0.275** | **1.92×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `rev` | `f00-rev --core fixture.txt` | 0.747 | **0.316** | **2.36×** | `9876543210 zyxwvutsrqponmlkjihgfedcba enil hcneb-etius 9876543210 zyxwvutsrqponm` |
| `ptx` | `f00-ptx --core -A fixture.txt` | 1.082 | **0.316** | **3.43×** | `suite bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite bench line abcdefgh` |
| `pr` | `f00-pr --core -t fixture.txt` | 0.631 | **0.288** | **2.19×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `shuf` | `f00-shuf --core fixture.txt` | 0.522 | **0.398** | **1.31×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tsort` | `f00-tsort --core` | 0.496 | **0.239** | **2.08×** | `a b c` |
| `tee` | `f00-tee --core tee.out` | 0.563 | **0.271** | **2.08×** | `tee data tee data tee data tee data tee data tee data tee data tee data tee data` |
| `split` | `f00-split --core -l 50 fixture.txt out` | 0.724 | **0.684** | **1.06×** | `` |
| `csplit` | `f00-csplit --core -f xx fixture 5` | 0.626 | **0.232** | **2.70×** | `` |
| `chmod` | `f00-chmod --core 644 fixture.txt` | 0.510 | **0.227** | **2.25×** | `` |
| `touch` | `f00-touch --core touched` | 0.504 | **0.326** | **1.55×** | `` |
| `truncate` | `f00-truncate --core -s 0 trunc` | 0.487 | **0.230** | **2.11×** | `` |
| `cp` | `f00-cp --core fixture.txt cp.out` | 0.794 | **0.294** | **2.70×** | `` |
| `dd` | `f00-dd --core if=fixture of=dd.out bs=4k count=1` | 0.587 | **0.297** | **1.98×** | `` |
| `install` | `f00-install --core -m 644 fixture inst.out` | 0.773 | **0.306** | **2.53×** | `` |
| `yes` | `f00-yes --core --version` | 0.481 | **0.136** | **3.54×** | `f00-yes (f00) 0.15.9 License: MIT · https://f00.sh` |
| `[` | `f00-[ --core -f fixture.txt` | 0.480 | **0.220** | **2.18×** | `` |

Full machine-readable data: [suite.json](suite.json)

