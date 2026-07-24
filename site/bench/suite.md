# Suite benchmarks (f00 vs GNU coreutils)

**Overall: 2.3× faster than GNU coreutils overall** (129% faster overall; geo mean of per-tool speedups)

Generated: `2026-07-24T13:49:42Z` · N=15 median · warm-cache spawn-inclusive median

Host: x86_64 · Linux 6.17.0-1020-azure

Tools timed: 91 · wins: 91 · median 2.17× · total-time 2.638×

| Tool | Command (f00) | GNU ms | f00 ms | Speedup | Sample output (f00) |
|------|---------------|-------:|-------:|--------:|---------------------|
| `true` | `f00-true --core` | 0.367 | **0.136** | **2.70×** | `` |
| `false` | `f00-false --core` | 0.345 | **0.124** | **2.78×** | `` |
| `basename` | `f00-basename --core /usr/bin/ls` | 0.546 | **0.167** | **3.27×** | `ls` |
| `dirname` | `f00-dirname --core /usr/bin/ls` | 0.548 | **0.127** | **4.32×** | `/usr/bin` |
| `echo` | `f00-echo --core hi` | 0.504 | **0.235** | **2.15×** | `hi` |
| `pwd` | `f00-pwd --core` | 0.513 | **0.237** | **2.17×** | `/home/runner/work/f00/f00` |
| `nproc` | `f00-nproc --core` | 0.543 | **0.132** | **4.11×** | `4` |
| `whoami` | `f00-whoami --core` | 0.562 | **0.132** | **4.24×** | `runner` |
| `uname` | `f00-uname --core -s` | 0.514 | **0.234** | **2.20×** | `Linux` |
| `id` | `f00-id --core -u` | 0.637 | **0.235** | **2.71×** | `1001` |
| `date` | `f00-date --core -u +%Y` | 0.544 | **0.232** | **2.34×** | `2026` |
| `printenv` | `f00-printenv --core PATH` | 0.521 | **0.236** | **2.21×** | `/snap/bin:/home/runner/.local/bin:/opt/pipx_bin:/home/runner/.cargo/bin:/home/ru` |
| `printf` | `f00-printf --core %s world` | 0.511 | **0.240** | **2.13×** | `world` |
| `factor` | `f00-factor --core 12` | 0.549 | **0.244** | **2.25×** | `12: 2 2 3` |
| `numfmt` | `f00-numfmt --core --to=si 1000` | 0.514 | **0.239** | **2.15×** | `1.0k` |
| `expr` | `f00-expr --core 1 + 1` | 0.557 | **0.232** | **2.40×** | `2` |
| `seq` | `f00-seq --core 1 5` | 0.515 | **0.306** | **1.68×** | `1 2 3 4 5` |
| `cat` | `f00-cat --core fixture.txt` | 0.639 | **0.171** | **3.72×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `wc` | `f00-wc --core -l fixture.txt` | 0.555 | **0.366** | **1.52×** | `400 /tmp/f00-suite-bench.6hkk9lgx/fix.txt` |
| `head` | `f00-head --core -n 3 fixture.txt` | 0.542 | **0.245** | **2.21×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tail` | `f00-tail --core -n 3 fixture.txt` | 0.553 | **0.349** | **1.58×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `nl` | `f00-nl --core fixture.txt` | 0.581 | **0.323** | **1.80×** | `1 suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 2 suite-bench line abcd` |
| `od` | `f00-od --core -An -tx1 -N8 fixture.txt` | 0.516 | **0.241** | **2.14×** | `73 75 69 74 65 2d 62 65` |
| `cut` | `f00-cut --core -d: -f1 /etc/passwd` | 0.525 | **0.258** | **2.03×** | `root daemon bin sys sync games man lp mail news uucp proxy www-data backup list ` |
| `tr` | `f00-tr --core a-z A-Z` | 0.509 | **0.246** | **2.07×** | `HELLO` |
| `sort` | `f00-sort --core fixture.txt` | 1.002 | **0.567** | **1.77×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `uniq` | `f00-uniq --core a.txt` | 0.614 | **0.318** | **1.93×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789` |
| `paste` | `f00-paste --core a.txt b.txt` | 0.666 | **0.381** | **1.75×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `comm` | `f00-comm --core -12 a.txt b.txt` | 0.647 | **0.359** | **1.80×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `join` | `f00-join --core a.txt b.txt` | 12.814 | **0.369** | **34.70×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 line abcdefghijklmnopqrst` |
| `base64` | `f00-base64 --core fixture.txt` | 0.522 | **0.378** | **1.38×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `base32` | `f00-base32 --core fixture.txt` | 0.521 | **0.391** | **1.33×** | `ON2WS5DFFVRGK3TDNAQGY2LOMUQGCYTDMRSWMZ3INFVGW3DNNZXXA4LSON2HK5TXPB4XUIBQGEZD GNB` |
| `basenc` | `f00-basenc --core --base64 fixture.txt` | 0.539 | **0.374** | **1.44×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `md5sum` | `f00-md5sum --core fixture.txt` | 0.890 | **0.292** | **3.05×** | `a5e6b1aa8523bc01f561fcef58d16894 /tmp/f00-suite-bench.6hkk9lgx/fix.txt` |
| `sha1sum` | `f00-sha1sum --core fixture.txt` | 0.797 | **0.322** | **2.47×** | `49f92a1f57c1a825b5ca5777c92d6e15ac26a8ea /tmp/f00-suite-bench.6hkk9lgx/fix.txt` |
| `sha224sum` | `f00-sha224sum --core fixture.txt` | 0.823 | **0.361** | **2.28×** | `94c1dff65fd14336129da4555171327a3e1e3e684810af23fa13e7f3 /tmp/f00-suite-bench.6h` |
| `sha256sum` | `f00-sha256sum --core fixture.txt` | 0.785 | **0.366** | **2.15×** | `7c28ea8726bc6923d5b38a6b6938ad5907c01dc6640e1645cf63cbf9df706132 /tmp/f00-suite-` |
| `sha384sum` | `f00-sha384sum --core fixture.txt` | 0.808 | **0.304** | **2.66×** | `f2578c293b7eeebf35402906e5e2fcd522b567687241b72950690c389f88baf83f9cd518c1fd67b3` |
| `sha512sum` | `f00-sha512sum --core fixture.txt` | 0.806 | **0.301** | **2.68×** | `a3282daa5cc665baa2b247ee17a0798f1d8028bbfb2107ea0df9493d0e57bfba10ba95d72d2550c2` |
| `b2sum` | `f00-b2sum --core fixture.txt` | 0.557 | **0.303** | **1.84×** | `915149393ea1091e4aa19ad9c68f980ebf83f5da2a576a20ca65fb001c685bf890523b442b840760` |
| `cksum` | `f00-cksum --core fixture.txt` | 0.789 | **0.292** | **2.70×** | `1448063438 22000 /tmp/f00-suite-bench.6hkk9lgx/fix.txt` |
| `sum` | `f00-sum --core fixture.txt` | 0.586 | **0.260** | **2.25×** | `9481 22 /tmp/f00-suite-bench.6hkk9lgx/fix.txt` |
| `ls` | `f00-ls --core -1 dir` | 0.635 | **0.347** | **1.83×** | `f01.txt f02.txt f03.txt f04.txt f05.txt f06.txt f07.txt f08.txt f09.txt f10.txt ` |
| `dir` | `f00-dir --core -1 dir` | 0.629 | **0.244** | **2.58×** | `f06.txt f02.txt f20.txt f14.txt f09.txt f13.txt f10.txt f17.txt f16.txt f08.txt ` |
| `vdir` | `f00-vdir --core -1 dir` | 0.711 | **0.256** | **2.77×** | `- f06.txt - f02.txt - f20.txt - f14.txt - f09.txt - f13.txt - f10.txt - f17.txt ` |
| `stat` | `f00-stat --core -c %s fixture.txt` | 0.646 | **0.243** | **2.66×** | `22000` |
| `realpath` | `f00-realpath --core .` | 0.471 | **0.335** | **1.40×** | `/home/runner/work/f00/f00/asm` |
| `readlink` | `f00-readlink --core /proc/self/exe` | 0.507 | **0.245** | **2.07×** | `/home/runner/work/f00/f00/asm/f00` |
| `df` | `f00-df --core -P /` | 0.634 | **0.269** | **2.36×** | `Filesystem 1K-blocks Used Available Use% Mounted on /dev/root 151263856 59012912` |
| `du` | `f00-du --core -s dir` | 0.545 | **0.262** | **2.08×** | `5 /tmp/f00-suite-bench.6hkk9lgx/dir` |
| `dircolors` | `f00-dircolors --core -p` | 0.526 | **0.247** | **2.13×** | `# Configuration file for dircolors, a utility to help you set the # LS_COLORS en` |
| `env` | `f00-env --core -i true` | 0.773 | **0.225** | **3.44×** | `` |
| `timeout` | `f00-timeout --core 5 true` | 0.999 | **0.320** | **3.12×** | `` |
| `nice` | `f00-nice --core true` | 0.741 | **0.238** | **3.12×** | `` |
| `nohup` | `f00-nohup --core true` | 0.755 | **0.235** | **3.22×** | `` |
| `sleep` | `f00-sleep --core 0` | 0.567 | **0.289** | **1.96×** | `` |
| `test` | `f00-test --core -f fixture.txt` | 0.465 | **0.224** | **2.08×** | `` |
| `pathchk` | `f00-pathchk --core ok-name` | 0.496 | **0.236** | **2.10×** | `` |
| `mktemp` | `f00-mktemp --core -u` | 0.534 | **0.233** | **2.29×** | `/tmp/tmp.mXC4uK` |
| `sync` | `f00-sync --core` | 0.697 | **0.431** | **1.62×** | `` |
| `uptime` | `f00-uptime --core` | 0.940 | **0.234** | **4.02×** | `up 3 minutes` |
| `hostid` | `f00-hostid --core` | 0.589 | **0.339** | **1.74×** | `db830370` |
| `logname` | `f00-logname --core` | 0.511 | **0.317** | **1.61×** | `runner` |
| `tty` | `f00-tty --core` | 0.513 | **0.128** | **4.01×** | `not a tty` |
| `groups` | `f00-groups --core` | 0.543 | **0.242** | **2.24×** | `adm users docker systemd-journal runner` |
| `arch` | `f00-arch --core` | 0.481 | **0.223** | **2.16×** | `x86_64` |
| `hostname` | `f00-hostname --core` | 0.355 | **0.243** | **1.46×** | `runnervmvrwv9` |
| `users` | `f00-users --core` | 0.499 | **0.226** | **2.21×** | `` |
| `who` | `f00-who --core` | 0.495 | **0.230** | **2.15×** | `` |
| `pinky` | `f00-pinky --core` | 0.540 | **0.242** | **2.23×** | `` |
| `fold` | `f00-fold --core -w 40 fixture.txt` | 0.566 | **0.299** | **1.89×** | `suite-bench line abcdefghijklmnopqrstuvw xyz 0123456789 suite-bench line abcdefg` |
| `fmt` | `f00-fmt --core -w 40 fixture.txt` | 0.654 | **0.307** | **2.13×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `expand` | `f00-expand --core fixture.txt` | 0.564 | **0.305** | **1.85×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `unexpand` | `f00-unexpand --core fixture.txt` | 0.558 | **0.330** | **1.69×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tac` | `f00-tac --core fixture.txt` | 0.568 | **0.272** | **2.09×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `rev` | `f00-rev --core fixture.txt` | 0.785 | **0.336** | **2.34×** | `9876543210 zyxwvutsrqponmlkjihgfedcba enil hcneb-etius 9876543210 zyxwvutsrqponm` |
| `ptx` | `f00-ptx --core -A fixture.txt` | 1.071 | **0.321** | **3.34×** | `suite bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite bench line abcdefgh` |
| `pr` | `f00-pr --core -t fixture.txt` | 0.683 | **0.288** | **2.37×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `shuf` | `f00-shuf --core fixture.txt` | 0.517 | **0.415** | **1.24×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tsort` | `f00-tsort --core` | 0.488 | **0.234** | **2.08×** | `a b c` |
| `tee` | `f00-tee --core tee.out` | 0.588 | **0.547** | **1.07×** | `tee data tee data tee data tee data tee data tee data tee data tee data tee data` |
| `split` | `f00-split --core -l 50 fixture.txt out` | 0.770 | **0.680** | **1.13×** | `` |
| `csplit` | `f00-csplit --core -f xx fixture 5` | 0.672 | **0.272** | **2.47×** | `` |
| `chmod` | `f00-chmod --core 644 fixture.txt` | 0.487 | **0.238** | **2.05×** | `` |
| `touch` | `f00-touch --core touched` | 0.511 | **0.343** | **1.49×** | `` |
| `truncate` | `f00-truncate --core -s 0 trunc` | 0.492 | **0.234** | **2.10×** | `` |
| `cp` | `f00-cp --core fixture.txt cp.out` | 0.883 | **0.305** | **2.90×** | `` |
| `dd` | `f00-dd --core if=fixture of=dd.out bs=4k count=1` | 0.632 | **0.275** | **2.29×** | `` |
| `install` | `f00-install --core -m 644 fixture inst.out` | 0.817 | **0.294** | **2.78×** | `` |
| `yes` | `f00-yes --core --version` | 0.544 | **0.137** | **3.96×** | `f00-yes (f00) 0.15.9 License: MIT · https://f00.sh` |
| `[` | `f00-[ --core -f fixture.txt` | 0.509 | **0.235** | **2.17×** | `` |

Full machine-readable data: [suite.json](suite.json)

