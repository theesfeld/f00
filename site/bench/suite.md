# Suite benchmarks (f00 vs GNU coreutils)

Generated: `2026-07-24T13:23:37Z` · N=15 median · warm-cache spawn-inclusive median

Host: x86_64 · Linux 6.17.0-1020-azure

| Tool | Command (f00) | GNU ms | f00 ms | Speedup | Sample output (f00) |
|------|---------------|-------:|-------:|--------:|---------------------|
| `true` | `f00-true --core` | 0.579 | **0.262** | **2.21×** | `` |
| `false` | `f00-false --core` | 0.561 | **0.262** | **2.14×** | `` |
| `basename` | `f00-basename --core /usr/bin/ls` | 0.791 | **0.261** | **3.03×** | `ls` |
| `dirname` | `f00-dirname --core /usr/bin/ls` | 0.867 | **0.262** | **3.31×** | `/usr/bin` |
| `echo` | `f00-echo --core hi` | 0.799 | **0.333** | **2.40×** | `hi` |
| `pwd` | `f00-pwd --core` | 0.797 | **0.335** | **2.38×** | `/home/runner/work/f00/f00` |
| `nproc` | `f00-nproc --core` | 0.819 | **0.267** | **3.06×** | `4` |
| `whoami` | `f00-whoami --core` | 0.943 | **0.278** | **3.39×** | `runner` |
| `uname` | `f00-uname --core -s` | 0.788 | **0.342** | **2.30×** | `Linux` |
| `id` | `f00-id --core -u` | 1.008 | **0.332** | **3.03×** | `1001` |
| `date` | `f00-date --core -u +%Y` | 0.821 | **0.339** | **2.42×** | `2026` |
| `printenv` | `f00-printenv --core PATH` | 0.856 | **0.341** | **2.51×** | `/snap/bin:/home/runner/.local/bin:/opt/pipx_bin:/home/runner/.cargo/bin:/home/ru` |
| `printf` | `f00-printf --core %s world` | 0.836 | **0.337** | **2.48×** | `world` |
| `factor` | `f00-factor --core 12` | 0.902 | **0.352** | **2.57×** | `12: 2 2 3` |
| `numfmt` | `f00-numfmt --core --to=si 1000` | 0.816 | **0.340** | **2.40×** | `1.0k` |
| `expr` | `f00-expr --core 1 + 1` | 0.859 | **0.341** | **2.52×** | `2` |
| `seq` | `f00-seq --core 1 5` | 0.839 | **0.392** | **2.14×** | `1 2 3 4 5` |
| `cat` | `f00-cat --core fixture.txt` | 0.879 | **0.343** | **2.56×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `wc` | `f00-wc --core -l fixture.txt` | 0.847 | **0.378** | **2.24×** | `400 /tmp/f00-suite-bench.y20v28sm/fix.txt` |
| `head` | `f00-head --core -n 3 fixture.txt` | 0.836 | **0.350** | **2.39×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tail` | `f00-tail --core -n 3 fixture.txt` | 0.869 | **0.389** | **2.23×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `nl` | `f00-nl --core fixture.txt` | 0.911 | **0.397** | **2.29×** | `1 suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 2 suite-bench line abcd` |
| `od` | `f00-od --core -An -tx1 -N8 fixture.txt` | 0.841 | **0.371** | **2.26×** | `73 75 69 74 65 2d 62 65` |
| `cut` | `f00-cut --core -d: -f1 /etc/passwd` | 0.842 | **0.354** | **2.38×** | `root daemon bin sys sync games man lp mail news uucp proxy www-data backup list ` |
| `tr` | `f00-tr --core a-z A-Z` | 0.844 | **0.371** | **2.28×** | `HELLO` |
| `sort` | `f00-sort --core fixture.txt` | 1.475 | **0.764** | **1.93×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `uniq` | `f00-uniq --core a.txt` | 0.850 | **0.377** | **2.25×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789` |
| `paste` | `f00-paste --core a.txt b.txt` | 0.957 | **0.560** | **1.71×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `comm` | `f00-comm --core -12 a.txt b.txt` | 0.918 | **0.484** | **1.89×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `join` | `f00-join --core a.txt b.txt` | 15.504 | **0.529** | **29.31×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 line abcdefghijklmnopqrst` |
| `base64` | `f00-base64 --core fixture.txt` | 0.899 | **0.514** | **1.75×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `base32` | `f00-base32 --core fixture.txt` | 0.885 | **0.539** | **1.64×** | `ON2WS5DFFVRGK3TDNAQGY2LOMUQGCYTDMRSWMZ3INFVGW3DNNZXXA4LSON2HK5TXPB4XUIBQGEZD GNB` |
| `basenc` | `f00-basenc --core --base64 fixture.txt` | 0.859 | **0.511** | **1.68×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `md5sum` | `f00-md5sum --core fixture.txt` | 1.256 | **0.424** | **2.96×** | `a5e6b1aa8523bc01f561fcef58d16894 /tmp/f00-suite-bench.y20v28sm/fix.txt` |
| `sha1sum` | `f00-sha1sum --core fixture.txt` | 1.272 | **0.429** | **2.96×** | `49f92a1f57c1a825b5ca5777c92d6e15ac26a8ea /tmp/f00-suite-bench.y20v28sm/fix.txt` |
| `sha224sum` | `f00-sha224sum --core fixture.txt` | 1.235 | **0.458** | **2.70×** | `94c1dff65fd14336129da4555171327a3e1e3e684810af23fa13e7f3 /tmp/f00-suite-bench.y2` |
| `sha256sum` | `f00-sha256sum --core fixture.txt` | 1.242 | **0.465** | **2.67×** | `7c28ea8726bc6923d5b38a6b6938ad5907c01dc6640e1645cf63cbf9df706132 /tmp/f00-suite-` |
| `sha384sum` | `f00-sha384sum --core fixture.txt` | 1.258 | **0.422** | **2.98×** | `f2578c293b7eeebf35402906e5e2fcd522b567687241b72950690c389f88baf83f9cd518c1fd67b3` |
| `sha512sum` | `f00-sha512sum --core fixture.txt` | 1.249 | **0.446** | **2.80×** | `a3282daa5cc665baa2b247ee17a0798f1d8028bbfb2107ea0df9493d0e57bfba10ba95d72d2550c2` |
| `b2sum` | `f00-b2sum --core fixture.txt` | 0.894 | **0.440** | **2.03×** | `915149393ea1091e4aa19ad9c68f980ebf83f5da2a576a20ca65fb001c685bf890523b442b840760` |
| `cksum` | `f00-cksum --core fixture.txt` | 1.236 | **0.398** | **3.10×** | `1448063438 22000 /tmp/f00-suite-bench.y20v28sm/fix.txt` |
| `sum` | `f00-sum --core fixture.txt` | 0.870 | **0.371** | **2.34×** | `9481 22 /tmp/f00-suite-bench.y20v28sm/fix.txt` |
| `ls` | `f00-ls --core -1 dir` | 1.024 | **0.475** | **2.16×** | `f01.txt f02.txt f03.txt f04.txt f05.txt f06.txt f07.txt f08.txt f09.txt f10.txt ` |
| `dir` | `f00-dir --core -1 dir` | 1.028 | **0.403** | **2.55×** | `f06.txt f02.txt f20.txt f14.txt f09.txt f13.txt f10.txt f17.txt f16.txt f08.txt ` |
| `vdir` | `f00-vdir --core -1 dir` | 1.045 | **0.353** | **2.96×** | `- f06.txt - f02.txt - f20.txt - f14.txt - f09.txt - f13.txt - f10.txt - f17.txt ` |
| `stat` | `f00-stat --core -c %s fixture.txt` | 1.016 | **0.334** | **3.04×** | `22000` |
| `realpath` | `f00-realpath --core .` | 0.785 | **0.406** | **1.93×** | `/home/runner/work/f00/f00/asm` |
| `readlink` | `f00-readlink --core /proc/self/exe` | 0.800 | **0.343** | **2.33×** | `/home/runner/work/f00/f00/asm/f00` |
| `df` | `f00-df --core -P /` | 0.965 | **0.387** | **2.50×** | `Filesystem 1K-blocks Used Available Use% Mounted on /dev/root 75085112 55862880 ` |
| `du` | `f00-du --core -s dir` | 0.890 | **0.409** | **2.18×** | `5 /tmp/f00-suite-bench.y20v28sm/dir` |
| `dircolors` | `f00-dircolors --core -p` | 0.806 | **0.330** | **2.44×** | `# Configuration file for dircolors, a utility to help you set the # LS_COLORS en` |
| `env` | `f00-env --core -i true` | 1.179 | **0.340** | **3.47×** | `` |
| `timeout` | `f00-timeout --core 5 true` | 1.532 | **0.476** | **3.22×** | `` |
| `nice` | `f00-nice --core true` | 1.223 | **0.330** | **3.70×** | `` |
| `nohup` | `f00-nohup --core true` | 1.255 | **0.328** | **3.82×** | `` |
| `sleep` | `f00-sleep --core 0` | 0.909 | **0.451** | **2.02×** | `` |
| `test` | `f00-test --core -f fixture.txt` | 0.782 | **0.334** | **2.34×** | `` |
| `pathchk` | `f00-pathchk --core ok-name` | 0.788 | **0.330** | **2.39×** | `` |
| `mktemp` | `f00-mktemp --core -u` | 0.821 | **0.332** | **2.48×** | `/tmp/tmp.CLi5m8` |
| `sync` | `f00-sync --core` | 0.854 | **0.376** | **2.27×** | `` |
| `uptime` | `f00-uptime --core` | 1.499 | **0.345** | **4.35×** | `up 2 minutes` |
| `hostid` | `f00-hostid --core` | 0.900 | **0.403** | **2.23×** | `db830370` |
| `logname` | `f00-logname --core` | 0.811 | **0.381** | **2.13×** | `runner` |
| `tty` | `f00-tty --core` | 0.785 | **0.256** | **3.06×** | `not a tty` |
| `groups` | `f00-groups --core` | 0.902 | **0.357** | **2.53×** | `adm users docker systemd-journal runner` |
| `arch` | `f00-arch --core` | 0.788 | **0.327** | **2.41×** | `x86_64` |
| `hostname` | `f00-hostname --core` | 0.573 | **0.327** | **1.75×** | `runnervmvrwv9` |
| `users` | `f00-users --core` | 0.845 | **0.341** | **2.47×** | `` |
| `who` | `f00-who --core` | 0.831 | **0.342** | **2.43×** | `` |
| `pinky` | `f00-pinky --core` | 0.852 | **0.340** | **2.50×** | `` |
| `fold` | `f00-fold --core -w 40 fixture.txt` | 0.907 | **0.420** | **2.16×** | `suite-bench line abcdefghijklmnopqrstuvw xyz 0123456789 suite-bench line abcdefg` |
| `fmt` | `f00-fmt --core -w 40 fixture.txt` | 0.972 | **0.418** | **2.32×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `expand` | `f00-expand --core fixture.txt` | 0.881 | **0.415** | **2.12×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `unexpand` | `f00-unexpand --core fixture.txt` | 0.861 | **0.485** | **1.77×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tac` | `f00-tac --core fixture.txt` | 0.845 | **0.390** | **2.17×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `rev` | `f00-rev --core fixture.txt` | 1.119 | **0.420** | **2.66×** | `9876543210 zyxwvutsrqponmlkjihgfedcba enil hcneb-etius 9876543210 zyxwvutsrqponm` |
| `ptx` | `f00-ptx --core -A fixture.txt` | 1.626 | **0.429** | **3.79×** | `suite bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite bench line abcdefgh` |
| `pr` | `f00-pr --core -t fixture.txt` | 1.031 | **0.400** | **2.58×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `shuf` | `f00-shuf --core fixture.txt` | 0.840 | **0.572** | **1.47×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tsort` | `f00-tsort --core` | 0.826 | **0.355** | **2.33×** | `a b c` |
| `tee` | `f00-tee --core tee.out` | 0.890 | **0.431** | **2.07×** | `tee data tee data tee data tee data tee data tee data tee data tee data tee data` |
| `split` | `f00-split --core -l 50 fixture.txt out` | 1.315 | **1.559** | **0.84×** | `` |
| `csplit` | `f00-csplit --core -f xx fixture 5` | 1.022 | **0.355** | **2.87×** | `` |
| `chmod` | `f00-chmod --core 644 fixture.txt` | 0.828 | **0.342** | **2.42×** | `` |
| `touch` | `f00-touch --core touched` | 0.811 | **0.392** | **2.07×** | `` |
| `truncate` | `f00-truncate --core -s 0 trunc` | 0.801 | **0.366** | **2.19×** | `` |
| `cp` | `f00-cp --core fixture.txt cp.out` | 1.233 | **0.439** | **2.81×** | `` |
| `dd` | `f00-dd --core if=fixture of=dd.out bs=4k count=1` | 0.903 | **0.446** | **2.02×** | `` |
| `install` | `f00-install --core -m 644 fixture inst.out` | 1.225 | **0.465** | **2.64×** | `` |
| `yes` | `f00-yes --core --version` | 0.815 | **0.261** | **3.13×** | `f00-yes (f00) 0.15.8 License: MIT · https://f00.sh` |
| `[` | `f00-[ --core -f fixture.txt` | 0.793 | **0.336** | **2.36×** | `` |

Full machine-readable data: [suite.json](suite.json)

