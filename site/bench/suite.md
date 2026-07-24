# Suite benchmarks (f00 vs GNU coreutils)

**Overall: 2.5× faster than GNU coreutils overall** (151% faster overall; geo mean of per-tool speedups)

Generated: `2026-07-24T14:39:18Z` · N=15 median · warm-cache spawn-inclusive median

Host: x86_64 · Linux 6.17.0-1020-azure

Tools timed: 91 · wins: 90 · median 2.46× · total-time 2.812×

| Tool | Command (f00) | GNU ms | f00 ms | Speedup | Sample output (f00) |
|------|---------------|-------:|-------:|--------:|---------------------|
| `true` | `f00-true --core` | 0.533 | **0.257** | **2.07×** | `` |
| `false` | `f00-false --core` | 0.546 | **0.250** | **2.18×** | `` |
| `basename` | `f00-basename --core /usr/bin/ls` | 0.758 | **0.251** | **3.02×** | `ls` |
| `dirname` | `f00-dirname --core /usr/bin/ls` | 0.762 | **0.249** | **3.06×** | `/usr/bin` |
| `echo` | `f00-echo --core hi` | 0.776 | **0.318** | **2.44×** | `hi` |
| `pwd` | `f00-pwd --core` | 0.771 | **0.306** | **2.52×** | `/home/runner/work/f00/f00` |
| `nproc` | `f00-nproc --core` | 0.774 | **0.252** | **3.07×** | `4` |
| `whoami` | `f00-whoami --core` | 0.860 | **0.265** | **3.25×** | `runner` |
| `uname` | `f00-uname --core -s` | 0.763 | **0.314** | **2.43×** | `Linux` |
| `id` | `f00-id --core -u` | 0.915 | **0.329** | **2.78×** | `1001` |
| `date` | `f00-date --core -u +%Y` | 0.816 | **0.316** | **2.59×** | `2026` |
| `printenv` | `f00-printenv --core PATH` | 0.759 | **0.314** | **2.42×** | `/snap/bin:/home/runner/.local/bin:/opt/pipx_bin:/home/runner/.cargo/bin:/home/ru` |
| `printf` | `f00-printf --core %s world` | 0.765 | **0.327** | **2.34×** | `world` |
| `factor` | `f00-factor --core 12` | 0.812 | **0.305** | **2.66×** | `12: 2 2 3` |
| `numfmt` | `f00-numfmt --core --to=si 1000` | 0.778 | **0.304** | **2.56×** | `1.0k` |
| `expr` | `f00-expr --core 1 + 1` | 0.827 | **0.303** | **2.73×** | `2` |
| `seq` | `f00-seq --core 1 5` | 0.780 | **0.307** | **2.54×** | `1 2 3 4 5` |
| `cat` | `f00-cat --core fixture.txt` | 0.792 | **0.298** | **2.65×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `wc` | `f00-wc --core -l fixture.txt` | 0.799 | **0.345** | **2.32×** | `400 /tmp/f00-suite-bench.rlqsoy4z/fix.txt` |
| `head` | `f00-head --core -n 3 fixture.txt` | 0.827 | **0.360** | **2.30×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tail` | `f00-tail --core -n 3 fixture.txt` | 0.831 | **0.378** | **2.20×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `nl` | `f00-nl --core fixture.txt` | 0.871 | **0.388** | **2.25×** | `1 suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 2 suite-bench line abcd` |
| `od` | `f00-od --core -An -tx1 -N8 fixture.txt` | 0.789 | **0.317** | **2.49×** | `73 75 69 74 65 2d 62 65` |
| `cut` | `f00-cut --core -d: -f1 /etc/passwd` | 0.768 | **0.318** | **2.42×** | `root daemon bin sys sync games man lp mail news uucp proxy www-data backup list ` |
| `tr` | `f00-tr --core a-z A-Z` | 0.799 | **0.325** | **2.46×** | `HELLO` |
| `sort` | `f00-sort --core fixture.txt` | 1.258 | **0.838** | **1.50×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `uniq` | `f00-uniq --core a.txt` | 0.819 | **0.361** | **2.27×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789` |
| `paste` | `f00-paste --core a.txt b.txt` | 0.885 | **0.451** | **1.96×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `comm` | `f00-comm --core -12 a.txt b.txt` | 0.866 | **0.442** | **1.96×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `join` | `f00-join --core a.txt b.txt` | 15.144 | **0.475** | **31.86×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 line abcdefghijklmnopqrst` |
| `base64` | `f00-base64 --core fixture.txt` | 0.834 | **0.477** | **1.75×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `base32` | `f00-base32 --core fixture.txt` | 0.832 | **0.533** | **1.56×** | `ON2WS5DFFVRGK3TDNAQGY2LOMUQGCYTDMRSWMZ3INFVGW3DNNZXXA4LSON2HK5TXPB4XUIBQGEZD GNB` |
| `basenc` | `f00-basenc --core --base64 fixture.txt` | 0.819 | **0.475** | **1.72×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `md5sum` | `f00-md5sum --core fixture.txt` | 1.173 | **0.381** | **3.08×** | `a5e6b1aa8523bc01f561fcef58d16894 /tmp/f00-suite-bench.rlqsoy4z/fix.txt` |
| `sha1sum` | `f00-sha1sum --core fixture.txt` | 1.137 | **0.404** | **2.81×** | `49f92a1f57c1a825b5ca5777c92d6e15ac26a8ea /tmp/f00-suite-bench.rlqsoy4z/fix.txt` |
| `sha224sum` | `f00-sha224sum --core fixture.txt` | 1.131 | **0.438** | **2.58×** | `94c1dff65fd14336129da4555171327a3e1e3e684810af23fa13e7f3 /tmp/f00-suite-bench.rl` |
| `sha256sum` | `f00-sha256sum --core fixture.txt` | 1.148 | **0.439** | **2.61×** | `7c28ea8726bc6923d5b38a6b6938ad5907c01dc6640e1645cf63cbf9df706132 /tmp/f00-suite-` |
| `sha384sum` | `f00-sha384sum --core fixture.txt` | 1.156 | **0.405** | **2.85×** | `f2578c293b7eeebf35402906e5e2fcd522b567687241b72950690c389f88baf83f9cd518c1fd67b3` |
| `sha512sum` | `f00-sha512sum --core fixture.txt` | 1.157 | **0.402** | **2.88×** | `a3282daa5cc665baa2b247ee17a0798f1d8028bbfb2107ea0df9493d0e57bfba10ba95d72d2550c2` |
| `b2sum` | `f00-b2sum --core fixture.txt` | 0.829 | **0.397** | **2.09×** | `915149393ea1091e4aa19ad9c68f980ebf83f5da2a576a20ca65fb001c685bf890523b442b840760` |
| `cksum` | `f00-cksum --core fixture.txt` | 1.149 | **0.368** | **3.12×** | `1448063438 22000 /tmp/f00-suite-bench.rlqsoy4z/fix.txt` |
| `sum` | `f00-sum --core fixture.txt` | 0.834 | **0.368** | **2.27×** | `9481 22 /tmp/f00-suite-bench.rlqsoy4z/fix.txt` |
| `ls` | `f00-ls --core -1 dir` | 0.968 | **0.424** | **2.28×** | `f01.txt f02.txt f03.txt f04.txt f05.txt f06.txt f07.txt f08.txt f09.txt f10.txt ` |
| `dir` | `f00-dir --core -1 dir` | 0.962 | **0.323** | **2.98×** | `f06.txt f02.txt f20.txt f14.txt f09.txt f13.txt f10.txt f17.txt f16.txt f08.txt ` |
| `vdir` | `f00-vdir --core -1 dir` | 0.968 | **0.322** | **3.00×** | `- f06.txt - f02.txt - f20.txt - f14.txt - f09.txt - f13.txt - f10.txt - f17.txt ` |
| `stat` | `f00-stat --core -c %s fixture.txt` | 0.939 | **0.309** | **3.04×** | `22000` |
| `realpath` | `f00-realpath --core .` | 0.757 | **0.386** | **1.96×** | `/home/runner/work/f00/f00/asm` |
| `readlink` | `f00-readlink --core /proc/self/exe` | 0.764 | **0.379** | **2.02×** | `/home/runner/work/f00/f00/asm/f00` |
| `df` | `f00-df --core -P /` | 0.895 | **0.363** | **2.47×** | `Filesystem 1K-blocks Used Available Use% Mounted on /dev/root 151263856 59919920` |
| `du` | `f00-du --core -s dir` | 0.849 | **0.364** | **2.34×** | `5 /tmp/f00-suite-bench.rlqsoy4z/dir` |
| `dircolors` | `f00-dircolors --core -p` | 0.756 | **0.305** | **2.48×** | `# Configuration file for dircolors, a utility to help you set the # LS_COLORS en` |
| `env` | `f00-env --core -i true` | 1.114 | **0.314** | **3.54×** | `` |
| `timeout` | `f00-timeout --core 5 true` | 1.373 | **0.422** | **3.25×** | `` |
| `nice` | `f00-nice --core true` | 1.164 | **0.309** | **3.76×** | `` |
| `nohup` | `f00-nohup --core true` | 1.197 | **0.311** | **3.85×** | `` |
| `sleep` | `f00-sleep --core 0` | 0.877 | **0.425** | **2.07×** | `` |
| `test` | `f00-test --core -f fixture.txt` | 0.760 | **0.307** | **2.48×** | `` |
| `pathchk` | `f00-pathchk --core ok-name` | 0.771 | **0.314** | **2.46×** | `` |
| `mktemp` | `f00-mktemp --core -u` | 0.781 | **0.375** | **2.08×** | `/tmp/tmp.r5vmzu` |
| `sync` | `f00-sync --core` | 0.791 | **0.339** | **2.34×** | `` |
| `uptime` | `f00-uptime --core` | 1.386 | **0.318** | **4.36×** | `up 2 minutes` |
| `hostid` | `f00-hostid --core` | 0.855 | **0.381** | **2.24×** | `db830370` |
| `logname` | `f00-logname --core` | 0.806 | **0.364** | **2.21×** | `runner` |
| `tty` | `f00-tty --core` | 0.763 | **0.251** | **3.05×** | `not a tty` |
| `groups` | `f00-groups --core` | 0.864 | **0.329** | **2.63×** | `adm users docker systemd-journal runner` |
| `arch` | `f00-arch --core` | 0.753 | **0.320** | **2.35×** | `x86_64` |
| `hostname` | `f00-hostname --core` | 0.540 | **0.324** | **1.67×** | `runnervmvrwv9` |
| `users` | `f00-users --core` | 0.813 | **0.330** | **2.46×** | `` |
| `who` | `f00-who --core` | 0.785 | **0.326** | **2.41×** | `` |
| `pinky` | `f00-pinky --core` | 0.792 | **0.319** | **2.49×** | `` |
| `fold` | `f00-fold --core -w 40 fixture.txt` | 0.848 | **0.383** | **2.22×** | `suite-bench line abcdefghijklmnopqrstuvw xyz 0123456789 suite-bench line abcdefg` |
| `fmt` | `f00-fmt --core -w 40 fixture.txt` | 0.919 | **0.359** | **2.56×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `expand` | `f00-expand --core fixture.txt` | 0.821 | **0.393** | **2.09×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `unexpand` | `f00-unexpand --core fixture.txt` | 0.847 | **0.435** | **1.95×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tac` | `f00-tac --core fixture.txt` | 0.815 | **0.368** | **2.21×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `rev` | `f00-rev --core fixture.txt` | 1.075 | **0.418** | **2.57×** | `9876543210 zyxwvutsrqponmlkjihgfedcba enil hcneb-etius 9876543210 zyxwvutsrqponm` |
| `ptx` | `f00-ptx --core -A fixture.txt` | 1.563 | **0.402** | **3.89×** | `suite bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite bench line abcdefgh` |
| `pr` | `f00-pr --core -t fixture.txt` | 0.991 | **0.372** | **2.67×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `shuf` | `f00-shuf --core fixture.txt` | 0.804 | **0.547** | **1.47×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tsort` | `f00-tsort --core` | 0.802 | **0.324** | **2.47×** | `a b c` |
| `tee` | `f00-tee --core tee.out` | 0.858 | **0.390** | **2.20×** | `tee data tee data tee data tee data tee data tee data tee data tee data tee data` |
| `split` | `f00-split --core -l 50 fixture.txt out` | 1.185 | **1.400** | **0.85×** | `` |
| `csplit` | `f00-csplit --core -f xx fixture 5` | 0.946 | **0.330** | **2.87×** | `` |
| `chmod` | `f00-chmod --core 644 fixture.txt` | 0.772 | **0.332** | **2.32×** | `` |
| `touch` | `f00-touch --core touched` | 0.781 | **0.397** | **1.97×** | `` |
| `truncate` | `f00-truncate --core -s 0 trunc` | 0.769 | **0.323** | **2.38×** | `` |
| `cp` | `f00-cp --core fixture.txt cp.out` | 1.146 | **0.422** | **2.72×** | `` |
| `dd` | `f00-dd --core if=fixture of=dd.out bs=4k count=1` | 0.847 | **0.387** | **2.19×** | `` |
| `install` | `f00-install --core -m 644 fixture inst.out` | 1.136 | **0.394** | **2.88×** | `` |
| `yes` | `f00-yes --core --version` | 0.758 | **0.248** | **3.06×** | `f00-yes (f00) 0.15.11 License: MIT · https://f00.sh` |
| `[` | `f00-[ --core -f fixture.txt` | 0.759 | **0.306** | **2.48×** | `` |

Full machine-readable data: [suite.json](suite.json)

