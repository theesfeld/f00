# Suite benchmarks (f00 vs GNU coreutils)

**Overall: 2.5× faster than GNU coreutils overall** (148% faster overall; geo mean of per-tool speedups)

Generated: `2026-07-24T13:40:17Z` · N=15 median · warm-cache spawn-inclusive median

Host: x86_64 · Linux 6.17.0-1020-azure

Tools timed: 91 · wins: 90 · median 2.44× · total-time 2.776×

| Tool | Command (f00) | GNU ms | f00 ms | Speedup | Sample output (f00) |
|------|---------------|-------:|-------:|--------:|---------------------|
| `true` | `f00-true --core` | 0.582 | **0.298** | **1.95×** | `` |
| `false` | `f00-false --core` | 0.589 | **0.288** | **2.05×** | `` |
| `basename` | `f00-basename --core /usr/bin/ls` | 0.800 | **0.260** | **3.08×** | `ls` |
| `dirname` | `f00-dirname --core /usr/bin/ls` | 0.764 | **0.261** | **2.93×** | `/usr/bin` |
| `echo` | `f00-echo --core hi` | 0.790 | **0.317** | **2.49×** | `hi` |
| `pwd` | `f00-pwd --core` | 0.787 | **0.322** | **2.44×** | `/home/runner/work/f00/f00` |
| `nproc` | `f00-nproc --core` | 0.792 | **0.261** | **3.03×** | `4` |
| `whoami` | `f00-whoami --core` | 0.856 | **0.262** | **3.27×** | `runner` |
| `uname` | `f00-uname --core -s` | 0.786 | **0.327** | **2.40×** | `Linux` |
| `id` | `f00-id --core -u` | 0.944 | **0.317** | **2.98×** | `1001` |
| `date` | `f00-date --core -u +%Y` | 0.810 | **0.332** | **2.44×** | `2026` |
| `printenv` | `f00-printenv --core PATH` | 0.801 | **0.326** | **2.46×** | `/snap/bin:/home/runner/.local/bin:/opt/pipx_bin:/home/runner/.cargo/bin:/home/ru` |
| `printf` | `f00-printf --core %s world` | 0.803 | **0.336** | **2.39×** | `world` |
| `factor` | `f00-factor --core 12` | 0.839 | **0.317** | **2.65×** | `12: 2 2 3` |
| `numfmt` | `f00-numfmt --core --to=si 1000` | 0.802 | **0.335** | **2.39×** | `1.0k` |
| `expr` | `f00-expr --core 1 + 1` | 0.851 | **0.328** | **2.59×** | `2` |
| `seq` | `f00-seq --core 1 5` | 0.786 | **0.331** | **2.38×** | `1 2 3 4 5` |
| `cat` | `f00-cat --core fixture.txt` | 0.802 | **0.301** | **2.67×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `wc` | `f00-wc --core -l fixture.txt` | 0.820 | **0.372** | **2.20×** | `400 /tmp/f00-suite-bench.6ngr9bxb/fix.txt` |
| `head` | `f00-head --core -n 3 fixture.txt` | 0.787 | **0.324** | **2.43×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tail` | `f00-tail --core -n 3 fixture.txt` | 0.800 | **0.371** | **2.16×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `nl` | `f00-nl --core fixture.txt` | 0.867 | **0.386** | **2.25×** | `1 suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 2 suite-bench line abcd` |
| `od` | `f00-od --core -An -tx1 -N8 fixture.txt` | 0.797 | **0.322** | **2.47×** | `73 75 69 74 65 2d 62 65` |
| `cut` | `f00-cut --core -d: -f1 /etc/passwd` | 0.783 | **0.337** | **2.32×** | `root daemon bin sys sync games man lp mail news uucp proxy www-data backup list ` |
| `tr` | `f00-tr --core a-z A-Z` | 0.809 | **0.333** | **2.43×** | `HELLO` |
| `sort` | `f00-sort --core fixture.txt` | 1.298 | **0.730** | **1.78×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `uniq` | `f00-uniq --core a.txt` | 0.839 | **0.364** | **2.30×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789` |
| `paste` | `f00-paste --core a.txt b.txt` | 0.884 | **0.482** | **1.83×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `comm` | `f00-comm --core -12 a.txt b.txt` | 0.875 | **0.462** | **1.89×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `join` | `f00-join --core a.txt b.txt` | 15.099 | **0.507** | **29.80×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 line abcdefghijklmnopqrst` |
| `base64` | `f00-base64 --core fixture.txt` | 0.840 | **0.510** | **1.65×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `base32` | `f00-base32 --core fixture.txt` | 0.852 | **0.539** | **1.58×** | `ON2WS5DFFVRGK3TDNAQGY2LOMUQGCYTDMRSWMZ3INFVGW3DNNZXXA4LSON2HK5TXPB4XUIBQGEZD GNB` |
| `basenc` | `f00-basenc --core --base64 fixture.txt` | 0.851 | **0.498** | **1.71×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `md5sum` | `f00-md5sum --core fixture.txt` | 1.166 | **0.391** | **2.98×** | `a5e6b1aa8523bc01f561fcef58d16894 /tmp/f00-suite-bench.6ngr9bxb/fix.txt` |
| `sha1sum` | `f00-sha1sum --core fixture.txt` | 1.156 | **0.418** | **2.77×** | `49f92a1f57c1a825b5ca5777c92d6e15ac26a8ea /tmp/f00-suite-bench.6ngr9bxb/fix.txt` |
| `sha224sum` | `f00-sha224sum --core fixture.txt` | 1.156 | **0.471** | **2.45×** | `94c1dff65fd14336129da4555171327a3e1e3e684810af23fa13e7f3 /tmp/f00-suite-bench.6n` |
| `sha256sum` | `f00-sha256sum --core fixture.txt` | 1.165 | **0.455** | **2.56×** | `7c28ea8726bc6923d5b38a6b6938ad5907c01dc6640e1645cf63cbf9df706132 /tmp/f00-suite-` |
| `sha384sum` | `f00-sha384sum --core fixture.txt` | 1.177 | **0.425** | **2.77×** | `f2578c293b7eeebf35402906e5e2fcd522b567687241b72950690c389f88baf83f9cd518c1fd67b3` |
| `sha512sum` | `f00-sha512sum --core fixture.txt` | 1.176 | **0.424** | **2.77×** | `a3282daa5cc665baa2b247ee17a0798f1d8028bbfb2107ea0df9493d0e57bfba10ba95d72d2550c2` |
| `b2sum` | `f00-b2sum --core fixture.txt` | 0.846 | **0.412** | **2.05×** | `915149393ea1091e4aa19ad9c68f980ebf83f5da2a576a20ca65fb001c685bf890523b442b840760` |
| `cksum` | `f00-cksum --core fixture.txt` | 1.167 | **0.379** | **3.08×** | `1448063438 22000 /tmp/f00-suite-bench.6ngr9bxb/fix.txt` |
| `sum` | `f00-sum --core fixture.txt` | 0.836 | **0.362** | **2.31×** | `9481 22 /tmp/f00-suite-bench.6ngr9bxb/fix.txt` |
| `ls` | `f00-ls --core -1 dir` | 0.988 | **0.439** | **2.25×** | `f01.txt f02.txt f03.txt f04.txt f05.txt f06.txt f07.txt f08.txt f09.txt f10.txt ` |
| `dir` | `f00-dir --core -1 dir` | 0.975 | **0.330** | **2.95×** | `f06.txt f02.txt f20.txt f14.txt f09.txt f13.txt f10.txt f17.txt f16.txt f08.txt ` |
| `vdir` | `f00-vdir --core -1 dir` | 0.983 | **0.326** | **3.01×** | `- f06.txt - f02.txt - f20.txt - f14.txt - f09.txt - f13.txt - f10.txt - f17.txt ` |
| `stat` | `f00-stat --core -c %s fixture.txt` | 0.954 | **0.316** | **3.02×** | `22000` |
| `realpath` | `f00-realpath --core .` | 0.773 | **0.399** | **1.94×** | `/home/runner/work/f00/f00/asm` |
| `readlink` | `f00-readlink --core /proc/self/exe` | 0.782 | **0.331** | **2.37×** | `/home/runner/work/f00/f00/asm/f00` |
| `df` | `f00-df --core -P /` | 0.907 | **0.375** | **2.42×** | `Filesystem 1K-blocks Used Available Use% Mounted on /dev/root 151263856 59919660` |
| `du` | `f00-du --core -s dir` | 0.845 | **0.366** | **2.31×** | `5 /tmp/f00-suite-bench.6ngr9bxb/dir` |
| `dircolors` | `f00-dircolors --core -p` | 0.764 | **0.311** | **2.46×** | `# Configuration file for dircolors, a utility to help you set the # LS_COLORS en` |
| `env` | `f00-env --core -i true` | 1.115 | **0.317** | **3.51×** | `` |
| `timeout` | `f00-timeout --core 5 true` | 1.396 | **0.435** | **3.21×** | `` |
| `nice` | `f00-nice --core true` | 1.177 | **0.323** | **3.65×** | `` |
| `nohup` | `f00-nohup --core true` | 1.209 | **0.320** | **3.78×** | `` |
| `sleep` | `f00-sleep --core 0` | 0.896 | **0.446** | **2.01×** | `` |
| `test` | `f00-test --core -f fixture.txt` | 0.775 | **0.316** | **2.45×** | `` |
| `pathchk` | `f00-pathchk --core ok-name` | 0.758 | **0.318** | **2.38×** | `` |
| `mktemp` | `f00-mktemp --core -u` | 0.798 | **0.321** | **2.49×** | `/tmp/tmp.CPPumL` |
| `sync` | `f00-sync --core` | 0.803 | **0.342** | **2.35×** | `` |
| `uptime` | `f00-uptime --core` | 1.394 | **0.334** | **4.17×** | `up 0 minutes` |
| `hostid` | `f00-hostid --core` | 0.886 | **0.379** | **2.33×** | `db830370` |
| `logname` | `f00-logname --core` | 0.781 | **0.390** | **2.00×** | `runner` |
| `tty` | `f00-tty --core` | 0.774 | **0.250** | **3.10×** | `not a tty` |
| `groups` | `f00-groups --core` | 0.878 | **0.345** | **2.55×** | `adm users docker systemd-journal runner` |
| `arch` | `f00-arch --core` | 0.773 | **0.317** | **2.44×** | `x86_64` |
| `hostname` | `f00-hostname --core` | 0.559 | **0.345** | **1.62×** | `runnervmvrwv9` |
| `users` | `f00-users --core` | 0.812 | **0.332** | **2.44×** | `` |
| `who` | `f00-who --core` | 0.820 | **0.330** | **2.48×** | `` |
| `pinky` | `f00-pinky --core` | 0.823 | **0.329** | **2.50×** | `` |
| `fold` | `f00-fold --core -w 40 fixture.txt` | 0.875 | **0.420** | **2.08×** | `suite-bench line abcdefghijklmnopqrstuvw xyz 0123456789 suite-bench line abcdefg` |
| `fmt` | `f00-fmt --core -w 40 fixture.txt` | 0.929 | **0.379** | **2.45×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `expand` | `f00-expand --core fixture.txt` | 0.845 | **0.410** | **2.06×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `unexpand` | `f00-unexpand --core fixture.txt` | 0.815 | **0.454** | **1.80×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tac` | `f00-tac --core fixture.txt` | 0.809 | **0.382** | **2.12×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `rev` | `f00-rev --core fixture.txt` | 1.092 | **0.437** | **2.50×** | `9876543210 zyxwvutsrqponmlkjihgfedcba enil hcneb-etius 9876543210 zyxwvutsrqponm` |
| `ptx` | `f00-ptx --core -A fixture.txt` | 1.573 | **0.427** | **3.68×** | `suite bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite bench line abcdefgh` |
| `pr` | `f00-pr --core -t fixture.txt` | 1.025 | **0.395** | **2.59×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `shuf` | `f00-shuf --core fixture.txt` | 0.835 | **0.561** | **1.49×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tsort` | `f00-tsort --core` | 0.801 | **0.335** | **2.39×** | `a b c` |
| `tee` | `f00-tee --core tee.out` | 0.858 | **0.421** | **2.04×** | `tee data tee data tee data tee data tee data tee data tee data tee data tee data` |
| `split` | `f00-split --core -l 50 fixture.txt out` | 1.271 | **1.457** | **0.87×** | `` |
| `csplit` | `f00-csplit --core -f xx fixture 5` | 1.012 | **0.340** | **2.98×** | `` |
| `chmod` | `f00-chmod --core 644 fixture.txt` | 0.792 | **0.330** | **2.40×** | `` |
| `touch` | `f00-touch --core touched` | 0.790 | **0.396** | **2.00×** | `` |
| `truncate` | `f00-truncate --core -s 0 trunc` | 0.782 | **0.340** | **2.30×** | `` |
| `cp` | `f00-cp --core fixture.txt cp.out` | 1.147 | **0.409** | **2.80×** | `` |
| `dd` | `f00-dd --core if=fixture of=dd.out bs=4k count=1` | 0.894 | **0.436** | **2.05×** | `` |
| `install` | `f00-install --core -m 644 fixture inst.out` | 1.154 | **0.418** | **2.76×** | `` |
| `yes` | `f00-yes --core --version` | 0.798 | **0.256** | **3.12×** | `f00-yes (f00) 0.15.9 License: MIT · https://f00.sh` |
| `[` | `f00-[ --core -f fixture.txt` | 0.883 | **0.319** | **2.77×** | `` |

Full machine-readable data: [suite.json](suite.json)

