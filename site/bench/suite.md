# Suite benchmarks (f00 vs GNU coreutils)

**Overall: 2.5× faster than GNU coreutils overall** (147% faster overall; geo mean of per-tool speedups)

Generated: `2026-07-24T14:09:53Z` · N=15 median · warm-cache spawn-inclusive median

Host: x86_64 · Linux 6.17.0-1020-azure

Tools timed: 91 · wins: 90 · median 2.42× · total-time 2.771×

| Tool | Command (f00) | GNU ms | f00 ms | Speedup | Sample output (f00) |
|------|---------------|-------:|-------:|--------:|---------------------|
| `true` | `f00-true --core` | 0.527 | **0.254** | **2.07×** | `` |
| `false` | `f00-false --core` | 0.549 | **0.256** | **2.14×** | `` |
| `basename` | `f00-basename --core /usr/bin/ls` | 0.773 | **0.262** | **2.95×** | `ls` |
| `dirname` | `f00-dirname --core /usr/bin/ls` | 0.773 | **0.256** | **3.02×** | `/usr/bin` |
| `echo` | `f00-echo --core hi` | 0.773 | **0.321** | **2.41×** | `hi` |
| `pwd` | `f00-pwd --core` | 0.800 | **0.331** | **2.41×** | `/home/runner/work/f00/f00` |
| `nproc` | `f00-nproc --core` | 0.779 | **0.265** | **2.94×** | `4` |
| `whoami` | `f00-whoami --core` | 0.919 | **0.300** | **3.06×** | `runner` |
| `uname` | `f00-uname --core -s` | 0.826 | **0.340** | **2.43×** | `Linux` |
| `id` | `f00-id --core -u` | 0.970 | **0.358** | **2.71×** | `1001` |
| `date` | `f00-date --core -u +%Y` | 0.806 | **0.346** | **2.33×** | `2026` |
| `printenv` | `f00-printenv --core PATH` | 0.786 | **0.333** | **2.36×** | `/snap/bin:/home/runner/.local/bin:/opt/pipx_bin:/home/runner/.cargo/bin:/home/ru` |
| `printf` | `f00-printf --core %s world` | 0.789 | **0.317** | **2.49×** | `world` |
| `factor` | `f00-factor --core 12` | 0.827 | **0.325** | **2.54×** | `12: 2 2 3` |
| `numfmt` | `f00-numfmt --core --to=si 1000` | 0.788 | **0.331** | **2.38×** | `1.0k` |
| `expr` | `f00-expr --core 1 + 1` | 0.854 | **0.330** | **2.58×** | `2` |
| `seq` | `f00-seq --core 1 5` | 0.794 | **0.321** | **2.48×** | `1 2 3 4 5` |
| `cat` | `f00-cat --core fixture.txt` | 0.804 | **0.296** | **2.71×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `wc` | `f00-wc --core -l fixture.txt` | 0.839 | **0.368** | **2.28×** | `400 /tmp/f00-suite-bench.fhmhmsj3/fix.txt` |
| `head` | `f00-head --core -n 3 fixture.txt` | 0.806 | **0.325** | **2.48×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tail` | `f00-tail --core -n 3 fixture.txt` | 0.808 | **0.376** | **2.15×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `nl` | `f00-nl --core fixture.txt` | 0.873 | **0.395** | **2.21×** | `1 suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 2 suite-bench line abcd` |
| `od` | `f00-od --core -An -tx1 -N8 fixture.txt` | 0.800 | **0.334** | **2.40×** | `73 75 69 74 65 2d 62 65` |
| `cut` | `f00-cut --core -d: -f1 /etc/passwd` | 0.801 | **0.364** | **2.20×** | `root daemon bin sys sync games man lp mail news uucp proxy www-data backup list ` |
| `tr` | `f00-tr --core a-z A-Z` | 0.821 | **0.344** | **2.39×** | `HELLO` |
| `sort` | `f00-sort --core fixture.txt` | 1.322 | **0.773** | **1.71×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `uniq` | `f00-uniq --core a.txt` | 0.891 | **0.389** | **2.29×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789` |
| `paste` | `f00-paste --core a.txt b.txt` | 0.909 | **0.491** | **1.85×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `comm` | `f00-comm --core -12 a.txt b.txt` | 0.879 | **0.477** | **1.84×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `join` | `f00-join --core a.txt b.txt` | 15.074 | **0.499** | **30.21×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 line abcdefghijklmnopqrst` |
| `base64` | `f00-base64 --core fixture.txt` | 0.840 | **0.503** | **1.67×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `base32` | `f00-base32 --core fixture.txt` | 0.842 | **0.557** | **1.51×** | `ON2WS5DFFVRGK3TDNAQGY2LOMUQGCYTDMRSWMZ3INFVGW3DNNZXXA4LSON2HK5TXPB4XUIBQGEZD GNB` |
| `basenc` | `f00-basenc --core --base64 fixture.txt` | 0.839 | **0.511** | **1.64×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `md5sum` | `f00-md5sum --core fixture.txt` | 1.196 | **0.395** | **3.02×** | `a5e6b1aa8523bc01f561fcef58d16894 /tmp/f00-suite-bench.fhmhmsj3/fix.txt` |
| `sha1sum` | `f00-sha1sum --core fixture.txt` | 1.181 | **0.430** | **2.75×** | `49f92a1f57c1a825b5ca5777c92d6e15ac26a8ea /tmp/f00-suite-bench.fhmhmsj3/fix.txt` |
| `sha224sum` | `f00-sha224sum --core fixture.txt` | 1.171 | **0.456** | **2.57×** | `94c1dff65fd14336129da4555171327a3e1e3e684810af23fa13e7f3 /tmp/f00-suite-bench.fh` |
| `sha256sum` | `f00-sha256sum --core fixture.txt` | 1.174 | **0.447** | **2.62×** | `7c28ea8726bc6923d5b38a6b6938ad5907c01dc6640e1645cf63cbf9df706132 /tmp/f00-suite-` |
| `sha384sum` | `f00-sha384sum --core fixture.txt` | 1.195 | **0.444** | **2.69×** | `f2578c293b7eeebf35402906e5e2fcd522b567687241b72950690c389f88baf83f9cd518c1fd67b3` |
| `sha512sum` | `f00-sha512sum --core fixture.txt` | 1.199 | **0.442** | **2.71×** | `a3282daa5cc665baa2b247ee17a0798f1d8028bbfb2107ea0df9493d0e57bfba10ba95d72d2550c2` |
| `b2sum` | `f00-b2sum --core fixture.txt` | 0.849 | **0.408** | **2.08×** | `915149393ea1091e4aa19ad9c68f980ebf83f5da2a576a20ca65fb001c685bf890523b442b840760` |
| `cksum` | `f00-cksum --core fixture.txt` | 1.206 | **0.416** | **2.90×** | `1448063438 22000 /tmp/f00-suite-bench.fhmhmsj3/fix.txt` |
| `sum` | `f00-sum --core fixture.txt` | 0.852 | **0.369** | **2.31×** | `9481 22 /tmp/f00-suite-bench.fhmhmsj3/fix.txt` |
| `ls` | `f00-ls --core -1 dir` | 1.014 | **0.460** | **2.20×** | `f01.txt f02.txt f03.txt f04.txt f05.txt f06.txt f07.txt f08.txt f09.txt f10.txt ` |
| `dir` | `f00-dir --core -1 dir` | 1.009 | **0.339** | **2.98×** | `f06.txt f02.txt f20.txt f14.txt f09.txt f13.txt f10.txt f17.txt f16.txt f08.txt ` |
| `vdir` | `f00-vdir --core -1 dir` | 1.008 | **0.346** | **2.92×** | `- f06.txt - f02.txt - f20.txt - f14.txt - f09.txt - f13.txt - f10.txt - f17.txt ` |
| `stat` | `f00-stat --core -c %s fixture.txt` | 0.968 | **0.327** | **2.96×** | `22000` |
| `realpath` | `f00-realpath --core .` | 0.792 | **0.426** | **1.86×** | `/home/runner/work/f00/f00/asm` |
| `readlink` | `f00-readlink --core /proc/self/exe` | 0.802 | **0.350** | **2.29×** | `/home/runner/work/f00/f00/asm/f00` |
| `df` | `f00-df --core -P /` | 0.925 | **0.383** | **2.42×** | `Filesystem 1K-blocks Used Available Use% Mounted on /dev/root 151263856 59919808` |
| `du` | `f00-du --core -s dir` | 0.878 | **0.392** | **2.24×** | `5 /tmp/f00-suite-bench.fhmhmsj3/dir` |
| `dircolors` | `f00-dircolors --core -p` | 0.795 | **0.328** | **2.42×** | `# Configuration file for dircolors, a utility to help you set the # LS_COLORS en` |
| `env` | `f00-env --core -i true` | 1.135 | **0.338** | **3.35×** | `` |
| `timeout` | `f00-timeout --core 5 true` | 1.410 | **0.448** | **3.15×** | `` |
| `nice` | `f00-nice --core true` | 1.202 | **0.332** | **3.62×** | `` |
| `nohup` | `f00-nohup --core true` | 1.214 | **0.344** | **3.53×** | `` |
| `sleep` | `f00-sleep --core 0` | 0.906 | **0.442** | **2.05×** | `` |
| `test` | `f00-test --core -f fixture.txt` | 0.778 | **0.332** | **2.35×** | `` |
| `pathchk` | `f00-pathchk --core ok-name` | 0.773 | **0.326** | **2.37×** | `` |
| `mktemp` | `f00-mktemp --core -u` | 0.795 | **0.329** | **2.42×** | `/tmp/tmp.TuzumS` |
| `sync` | `f00-sync --core` | 0.799 | **0.362** | **2.21×** | `` |
| `uptime` | `f00-uptime --core` | 1.427 | **0.334** | **4.28×** | `up 0 minutes` |
| `hostid` | `f00-hostid --core` | 0.891 | **0.386** | **2.31×** | `db830370` |
| `logname` | `f00-logname --core` | 0.787 | **0.379** | **2.08×** | `runner` |
| `tty` | `f00-tty --core` | 0.772 | **0.259** | **2.98×** | `not a tty` |
| `groups` | `f00-groups --core` | 0.891 | **0.344** | **2.59×** | `adm users docker systemd-journal runner` |
| `arch` | `f00-arch --core` | 0.766 | **0.317** | **2.42×** | `x86_64` |
| `hostname` | `f00-hostname --core` | 0.559 | **0.332** | **1.68×** | `runnervmvrwv9` |
| `users` | `f00-users --core` | 0.805 | **0.328** | **2.45×** | `` |
| `who` | `f00-who --core` | 0.813 | **0.328** | **2.48×** | `` |
| `pinky` | `f00-pinky --core` | 0.811 | **0.329** | **2.47×** | `` |
| `fold` | `f00-fold --core -w 40 fixture.txt` | 0.865 | **0.412** | **2.10×** | `suite-bench line abcdefghijklmnopqrstuvw xyz 0123456789 suite-bench line abcdefg` |
| `fmt` | `f00-fmt --core -w 40 fixture.txt` | 0.931 | **0.370** | **2.52×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `expand` | `f00-expand --core fixture.txt` | 0.840 | **0.397** | **2.12×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `unexpand` | `f00-unexpand --core fixture.txt` | 0.815 | **0.432** | **1.88×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tac` | `f00-tac --core fixture.txt` | 0.809 | **0.374** | **2.17×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `rev` | `f00-rev --core fixture.txt` | 1.084 | **0.397** | **2.73×** | `9876543210 zyxwvutsrqponmlkjihgfedcba enil hcneb-etius 9876543210 zyxwvutsrqponm` |
| `ptx` | `f00-ptx --core -A fixture.txt` | 1.549 | **0.409** | **3.79×** | `suite bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite bench line abcdefgh` |
| `pr` | `f00-pr --core -t fixture.txt` | 0.999 | **0.366** | **2.73×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `shuf` | `f00-shuf --core fixture.txt` | 0.815 | **0.543** | **1.50×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tsort` | `f00-tsort --core` | 0.792 | **0.330** | **2.40×** | `a b c` |
| `tee` | `f00-tee --core tee.out` | 0.886 | **0.411** | **2.15×** | `tee data tee data tee data tee data tee data tee data tee data tee data tee data` |
| `split` | `f00-split --core -l 50 fixture.txt out` | 1.207 | **1.354** | **0.89×** | `` |
| `csplit` | `f00-csplit --core -f xx fixture 5` | 0.967 | **0.342** | **2.83×** | `` |
| `chmod` | `f00-chmod --core 644 fixture.txt` | 0.782 | **0.329** | **2.38×** | `` |
| `touch` | `f00-touch --core touched` | 0.787 | **0.382** | **2.06×** | `` |
| `truncate` | `f00-truncate --core -s 0 trunc` | 0.789 | **0.334** | **2.36×** | `` |
| `cp` | `f00-cp --core fixture.txt cp.out` | 1.149 | **0.407** | **2.83×** | `` |
| `dd` | `f00-dd --core if=fixture of=dd.out bs=4k count=1` | 0.871 | **0.396** | **2.20×** | `` |
| `install` | `f00-install --core -m 644 fixture inst.out` | 1.143 | **0.407** | **2.81×** | `` |
| `yes` | `f00-yes --core --version` | 0.788 | **0.252** | **3.13×** | `f00-yes (f00) 0.15.9 License: MIT · https://f00.sh` |
| `[` | `f00-[ --core -f fixture.txt` | 0.760 | **0.315** | **2.42×** | `` |

Full machine-readable data: [suite.json](suite.json)

