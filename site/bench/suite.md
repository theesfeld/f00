# Suite benchmarks (f00 vs GNU coreutils)

Generated: `2026-07-24T12:06:46Z` · N=15 median · warm-cache spawn-inclusive median

Host: x86_64 · Linux 6.17.0-1020-azure

| Tool | Command (f00) | GNU ms | f00 ms | Speedup | Sample output (f00) |
|------|---------------|-------:|-------:|--------:|---------------------|
| `true` | `f00-true --core` | 0.540 | **0.251** | **2.15×** | `` |
| `false` | `f00-false --core` | 0.530 | **0.255** | **2.08×** | `` |
| `basename` | `f00-basename --core /usr/bin/ls` | 0.766 | **0.250** | **3.07×** | `ls` |
| `dirname` | `f00-dirname --core /usr/bin/ls` | 0.768 | **0.248** | **3.10×** | `/usr/bin` |
| `echo` | `f00-echo --core hi` | 0.773 | **0.314** | **2.46×** | `hi` |
| `pwd` | `f00-pwd --core` | 0.814 | **0.303** | **2.69×** | `/home/runner/work/f00/f00` |
| `nproc` | `f00-nproc --core` | 0.780 | **0.254** | **3.08×** | `4` |
| `whoami` | `f00-whoami --core` | 0.849 | **0.256** | **3.31×** | `runner` |
| `uname` | `f00-uname --core -s` | 0.763 | **0.321** | **2.37×** | `Linux` |
| `id` | `f00-id --core -u` | 0.950 | **0.303** | **3.14×** | `1001` |
| `date` | `f00-date --core -u +%Y` | 0.794 | **0.302** | **2.63×** | `2026` |
| `printenv` | `f00-printenv --core PATH` | 0.758 | **0.313** | **2.43×** | `/snap/bin:/home/runner/.local/bin:/opt/pipx_bin:/home/runner/.cargo/bin:/home/ru` |
| `printf` | `f00-printf --core %s world` | 0.772 | **0.319** | **2.42×** | `world` |
| `factor` | `f00-factor --core 12` | 0.823 | **0.349** | **2.35×** | `12: 2 2 3` |
| `numfmt` | `f00-numfmt --core --to=si 1000` | 0.827 | **0.350** | **2.36×** | `1.0k` |
| `expr` | `f00-expr --core 1 + 1` | 0.839 | **0.320** | **2.62×** | `2` |
| `seq` | `f00-seq --core 1 5` | 0.780 | **0.305** | **2.56×** | `1 2 3 4 5` |
| `cat` | `f00-cat --core fixture.txt` | 0.783 | **0.283** | **2.76×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `wc` | `f00-wc --core -l fixture.txt` | 0.828 | **0.353** | **2.35×** | `400 /tmp/f00-suite-bench.v3htcw45/fix.txt` |
| `head` | `f00-head --core -n 3 fixture.txt` | 0.788 | **0.325** | **2.43×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tail` | `f00-tail --core -n 3 fixture.txt` | 0.779 | **0.343** | **2.27×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `nl` | `f00-nl --core fixture.txt` | 0.856 | **0.378** | **2.26×** | `1 suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 2 suite-bench line abcd` |
| `od` | `f00-od --core -An -tx1 -N8 fixture.txt` | 0.810 | **0.323** | **2.51×** | `73 75 69 74 65 2d 62 65` |
| `cut` | `f00-cut --core -d: -f1 /etc/passwd` | 0.785 | **0.326** | **2.41×** | `root daemon bin sys sync games man lp mail news uucp proxy www-data backup list ` |
| `tr` | `f00-tr --core a-z A-Z` | 0.800 | **0.338** | **2.36×** | `HELLO` |
| `sort` | `f00-sort --core fixture.txt` | 1.279 | **0.719** | **1.78×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `uniq` | `f00-uniq --core a.txt` | 0.808 | **0.355** | **2.28×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789` |
| `paste` | `f00-paste --core a.txt b.txt` | 0.887 | **0.466** | **1.90×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `comm` | `f00-comm --core -12 a.txt b.txt` | 0.849 | **0.441** | **1.92×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `join` | `f00-join --core a.txt b.txt` | 14.962 | **0.457** | **32.76×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 line abcdefghijklmnopqrst` |
| `base64` | `f00-base64 --core fixture.txt` | 0.808 | **0.456** | **1.77×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `base32` | `f00-base32 --core fixture.txt` | 0.812 | **0.508** | **1.60×** | `ON2WS5DFFVRGK3TDNAQGY2LOMUQGCYTDMRSWMZ3INFVGW3DNNZXXA4LSON2HK5TXPB4XUIBQGEZD GNB` |
| `basenc` | `f00-basenc --core --base64 fixture.txt` | 0.808 | **0.463** | **1.74×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `md5sum` | `f00-md5sum --core fixture.txt` | 1.162 | **0.384** | **3.03×** | `a5e6b1aa8523bc01f561fcef58d16894 /tmp/f00-suite-bench.v3htcw45/fix.txt` |
| `sha1sum` | `f00-sha1sum --core fixture.txt` | 1.141 | **0.396** | **2.88×** | `49f92a1f57c1a825b5ca5777c92d6e15ac26a8ea /tmp/f00-suite-bench.v3htcw45/fix.txt` |
| `sha224sum` | `f00-sha224sum --core fixture.txt` | 1.137 | **0.424** | **2.68×** | `94c1dff65fd14336129da4555171327a3e1e3e684810af23fa13e7f3 /tmp/f00-suite-bench.v3` |
| `sha256sum` | `f00-sha256sum --core fixture.txt` | 1.139 | **0.424** | **2.69×** | `7c28ea8726bc6923d5b38a6b6938ad5907c01dc6640e1645cf63cbf9df706132 /tmp/f00-suite-` |
| `sha384sum` | `f00-sha384sum --core fixture.txt` | 1.187 | **0.411** | **2.89×** | `f2578c293b7eeebf35402906e5e2fcd522b567687241b72950690c389f88baf83f9cd518c1fd67b3` |
| `sha512sum` | `f00-sha512sum --core fixture.txt` | 1.156 | **0.404** | **2.86×** | `a3282daa5cc665baa2b247ee17a0798f1d8028bbfb2107ea0df9493d0e57bfba10ba95d72d2550c2` |
| `b2sum` | `f00-b2sum --core fixture.txt` | 0.822 | **0.388** | **2.12×** | `915149393ea1091e4aa19ad9c68f980ebf83f5da2a576a20ca65fb001c685bf890523b442b840760` |
| `cksum` | `f00-cksum --core fixture.txt` | 1.161 | **0.366** | **3.18×** | `1448063438 22000 /tmp/f00-suite-bench.v3htcw45/fix.txt` |
| `sum` | `f00-sum --core fixture.txt` | 0.826 | **0.347** | **2.38×** | `9481 22 /tmp/f00-suite-bench.v3htcw45/fix.txt` |
| `ls` | `f00-ls --core -1 dir` | 0.981 | **0.407** | **2.41×** | `f01.txt f02.txt f03.txt f04.txt f05.txt f06.txt f07.txt f08.txt f09.txt f10.txt ` |
| `dir` | `f00-dir --core -1 dir` | 0.970 | **0.327** | **2.97×** | `f06.txt f02.txt f20.txt f14.txt f09.txt f13.txt f10.txt f17.txt f16.txt f08.txt ` |
| `vdir` | `f00-vdir --core -1 dir` | 0.980 | **0.320** | **3.06×** | `- f06.txt - f02.txt - f20.txt - f14.txt - f09.txt - f13.txt - f10.txt - f17.txt ` |
| `stat` | `f00-stat --core -c %s fixture.txt` | 0.946 | **0.302** | **3.13×** | `22000` |
| `realpath` | `f00-realpath --core .` | 0.768 | **0.389** | **1.98×** | `/home/runner/work/f00/f00/asm` |
| `readlink` | `f00-readlink --core /proc/self/exe` | 0.797 | **0.318** | **2.51×** | `/home/runner/work/f00/f00/asm/f00` |
| `df` | `f00-df --core -P /` | 0.897 | **0.361** | **2.48×** | `Filesystem 1K-blocks Used Available Use% Mounted on /dev/root 75085112 55868392 ` |
| `du` | `f00-du --core -s dir` | 0.860 | **0.362** | **2.38×** | `5 /tmp/f00-suite-bench.v3htcw45/dir` |
| `dircolors` | `f00-dircolors --core -p` | 0.759 | **0.298** | **2.55×** | `# Configuration file for dircolors, a utility to help you set the # LS_COLORS en` |
| `env` | `f00-env --core -i true` | 1.128 | **0.311** | **3.63×** | `` |
| `timeout` | `f00-timeout --core 5 true` | 1.393 | **0.415** | **3.36×** | `` |
| `nice` | `f00-nice --core true` | 1.181 | **0.309** | **3.82×** | `` |
| `nohup` | `f00-nohup --core true` | 1.194 | **0.305** | **3.91×** | `` |
| `sleep` | `f00-sleep --core 0` | 0.887 | **0.431** | **2.06×** | `` |
| `test` | `f00-test --core -f fixture.txt` | 0.756 | **0.305** | **2.48×** | `` |
| `pathchk` | `f00-pathchk --core ok-name` | 0.741 | **0.306** | **2.43×** | `` |
| `mktemp` | `f00-mktemp --core -u` | 0.789 | **0.307** | **2.58×** | `/tmp/tmp.yDqS4D` |
| `sync` | `f00-sync --core` | 0.788 | **0.336** | **2.35×** | `` |
| `uptime` | `f00-uptime --core` | 1.383 | **0.306** | **4.52×** | `up 0 minutes` |
| `hostid` | `f00-hostid --core` | 0.875 | **0.362** | **2.42×** | `db830370` |
| `logname` | `f00-logname --core` | 0.783 | **0.361** | **2.17×** | `runner` |
| `tty` | `f00-tty --core` | 0.757 | **0.246** | **3.08×** | `not a tty` |
| `groups` | `f00-groups --core` | 0.882 | **0.332** | **2.66×** | `adm users docker systemd-journal runner` |
| `arch` | `f00-arch --core` | 0.773 | **0.310** | **2.50×** | `x86_64` |
| `hostname` | `f00-hostname --core` | 0.543 | **0.306** | **1.77×** | `runnervmvrwv9` |
| `users` | `f00-users --core` | 0.783 | **0.321** | **2.44×** | `` |
| `who` | `f00-who --core` | 0.810 | **0.309** | **2.62×** | `` |
| `pinky` | `f00-pinky --core` | 0.809 | **0.313** | **2.59×** | `` |
| `fold` | `f00-fold --core -w 40 fixture.txt` | 0.853 | **0.404** | **2.11×** | `suite-bench line abcdefghijklmnopqrstuvw xyz 0123456789 suite-bench line abcdefg` |
| `fmt` | `f00-fmt --core -w 40 fixture.txt` | 0.924 | **0.362** | **2.55×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `expand` | `f00-expand --core fixture.txt` | 0.822 | **0.388** | **2.12×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `unexpand` | `f00-unexpand --core fixture.txt` | 0.813 | **0.439** | **1.85×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tac` | `f00-tac --core fixture.txt` | 0.812 | **0.357** | **2.27×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `rev` | `f00-rev --core fixture.txt` | 1.088 | **0.410** | **2.66×** | `9876543210 zyxwvutsrqponmlkjihgfedcba enil hcneb-etius 9876543210 zyxwvutsrqponm` |
| `ptx` | `f00-ptx --core -A fixture.txt` | 1.549 | **0.420** | **3.68×** | `suite bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite bench line abcdefgh` |
| `pr` | `f00-pr --core -t fixture.txt` | 0.993 | **0.359** | **2.77×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `shuf` | `f00-shuf --core fixture.txt` | 0.822 | **0.537** | **1.53×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tsort` | `f00-tsort --core` | 0.787 | **0.340** | **2.31×** | `a b c` |
| `tee` | `f00-tee --core tee.out` | 0.853 | **0.397** | **2.15×** | `tee data tee data tee data tee data tee data tee data tee data tee data tee data` |
| `split` | `f00-split --core -l 50 fixture.txt out` | 1.189 | **1.378** | **0.86×** | `` |
| `csplit` | `f00-csplit --core -f xx fixture 5` | 0.959 | **0.325** | **2.95×** | `` |
| `chmod` | `f00-chmod --core 644 fixture.txt` | 0.781 | **0.326** | **2.39×** | `` |
| `touch` | `f00-touch --core touched` | 0.768 | **0.393** | **1.95×** | `` |
| `truncate` | `f00-truncate --core -s 0 trunc` | 0.783 | **0.332** | **2.36×** | `` |
| `cp` | `f00-cp --core fixture.txt cp.out` | 1.157 | **0.392** | **2.95×** | `` |
| `dd` | `f00-dd --core if=fixture of=dd.out bs=4k count=1` | 0.863 | **0.387** | **2.23×** | `` |
| `install` | `f00-install --core -m 644 fixture inst.out` | 1.139 | **0.409** | **2.79×** | `` |
| `yes` | `f00-yes --core --version` | 0.765 | **0.247** | **3.10×** | `f00-yes (f00) 0.15.3 License: MIT · https://f00.sh` |
| `[` | `f00-[ --core -f fixture.txt` | 0.763 | **0.304** | **2.51×** | `` |

Full machine-readable data: [suite.json](suite.json)

