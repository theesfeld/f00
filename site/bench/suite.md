# Suite benchmarks (f00 vs GNU coreutils)

Generated: `2026-07-24T12:02:39Z` · N=15 median · warm-cache spawn-inclusive median

Host: x86_64 · Linux 6.17.0-1020-azure

| Tool | Command (f00) | GNU ms | f00 ms | Speedup | Sample output (f00) |
|------|---------------|-------:|-------:|--------:|---------------------|
| `true` | `f00-true --core` | 0.552 | **0.254** | **2.17×** | `` |
| `false` | `f00-false --core` | 0.553 | **0.242** | **2.29×** | `` |
| `basename` | `f00-basename --core /usr/bin/ls` | 0.810 | **0.247** | **3.28×** | `ls` |
| `dirname` | `f00-dirname --core /usr/bin/ls` | 0.796 | **0.249** | **3.19×** | `/usr/bin` |
| `echo` | `f00-echo --core hi` | 0.824 | **0.302** | **2.73×** | `hi` |
| `pwd` | `f00-pwd --core` | 0.821 | **0.309** | **2.66×** | `/home/runner/work/f00/f00` |
| `nproc` | `f00-nproc --core` | 0.884 | **0.251** | **3.52×** | `4` |
| `whoami` | `f00-whoami --core` | 0.898 | **0.257** | **3.49×** | `runner` |
| `uname` | `f00-uname --core -s` | 0.821 | **0.306** | **2.69×** | `Linux` |
| `id` | `f00-id --core -u` | 1.009 | **0.313** | **3.22×** | `1001` |
| `date` | `f00-date --core -u +%Y` | 0.852 | **0.301** | **2.83×** | `2026` |
| `printenv` | `f00-printenv --core PATH` | 0.808 | **0.305** | **2.65×** | `/snap/bin:/home/runner/.local/bin:/opt/pipx_bin:/home/runner/.cargo/bin:/home/ru` |
| `printf` | `f00-printf --core %s world` | 0.813 | **0.300** | **2.71×** | `world` |
| `factor` | `f00-factor --core 12` | 0.860 | **0.317** | **2.72×** | `12: 2 2 3` |
| `numfmt` | `f00-numfmt --core --to=si 1000` | 0.834 | **0.319** | **2.62×** | `1.0k` |
| `expr` | `f00-expr --core 1 + 1` | 0.885 | **0.310** | **2.86×** | `2` |
| `seq` | `f00-seq --core 1 5` | 0.812 | **0.297** | **2.73×** | `1 2 3 4 5` |
| `cat` | `f00-cat --core fixture.txt` | 0.878 | **0.344** | **2.55×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `wc` | `f00-wc --core -l fixture.txt` | 0.914 | **0.399** | **2.29×** | `400 /tmp/f00-suite-bench.u36ghr6l/fix.txt` |
| `head` | `f00-head --core -n 3 fixture.txt` | 0.886 | **0.370** | **2.40×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tail` | `f00-tail --core -n 3 fixture.txt` | 0.887 | **0.403** | **2.20×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `nl` | `f00-nl --core fixture.txt` | 0.887 | **0.416** | **2.13×** | `1 suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 2 suite-bench line abcd` |
| `od` | `f00-od --core -An -tx1 -N8 fixture.txt` | 0.831 | **0.309** | **2.69×** | `73 75 69 74 65 2d 62 65` |
| `cut` | `f00-cut --core -d: -f1 /etc/passwd` | 0.847 | **0.320** | **2.65×** | `root daemon bin sys sync games man lp mail news uucp proxy www-data backup list ` |
| `tr` | `f00-tr --core a-z A-Z` | 0.856 | **0.337** | **2.54×** | `HELLO` |
| `sort` | `f00-sort --core fixture.txt` | 1.354 | **0.845** | **1.60×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `uniq` | `f00-uniq --core a.txt` | 0.857 | **0.358** | **2.39×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789` |
| `paste` | `f00-paste --core a.txt b.txt` | 0.924 | **0.459** | **2.01×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `comm` | `f00-comm --core -12 a.txt b.txt` | 0.903 | **0.430** | **2.10×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `join` | `f00-join --core a.txt b.txt` | 15.102 | **0.451** | **33.47×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 line abcdefghijklmnopqrst` |
| `base64` | `f00-base64 --core fixture.txt` | 0.900 | **0.464** | **1.94×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `base32` | `f00-base32 --core fixture.txt` | 0.887 | **0.488** | **1.82×** | `ON2WS5DFFVRGK3TDNAQGY2LOMUQGCYTDMRSWMZ3INFVGW3DNNZXXA4LSON2HK5TXPB4XUIBQGEZD GNB` |
| `basenc` | `f00-basenc --core --base64 fixture.txt` | 0.877 | **0.453** | **1.94×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `md5sum` | `f00-md5sum --core fixture.txt` | 1.259 | **0.383** | **3.28×** | `a5e6b1aa8523bc01f561fcef58d16894 /tmp/f00-suite-bench.u36ghr6l/fix.txt` |
| `sha1sum` | `f00-sha1sum --core fixture.txt` | 1.235 | **0.403** | **3.07×** | `49f92a1f57c1a825b5ca5777c92d6e15ac26a8ea /tmp/f00-suite-bench.u36ghr6l/fix.txt` |
| `sha224sum` | `f00-sha224sum --core fixture.txt` | 1.234 | **0.434** | **2.84×** | `94c1dff65fd14336129da4555171327a3e1e3e684810af23fa13e7f3 /tmp/f00-suite-bench.u3` |
| `sha256sum` | `f00-sha256sum --core fixture.txt` | 1.237 | **0.439** | **2.82×** | `7c28ea8726bc6923d5b38a6b6938ad5907c01dc6640e1645cf63cbf9df706132 /tmp/f00-suite-` |
| `sha384sum` | `f00-sha384sum --core fixture.txt` | 1.269 | **0.415** | **3.06×** | `f2578c293b7eeebf35402906e5e2fcd522b567687241b72950690c389f88baf83f9cd518c1fd67b3` |
| `sha512sum` | `f00-sha512sum --core fixture.txt` | 1.242 | **0.404** | **3.07×** | `a3282daa5cc665baa2b247ee17a0798f1d8028bbfb2107ea0df9493d0e57bfba10ba95d72d2550c2` |
| `b2sum` | `f00-b2sum --core fixture.txt` | 0.883 | **0.397** | **2.23×** | `915149393ea1091e4aa19ad9c68f980ebf83f5da2a576a20ca65fb001c685bf890523b442b840760` |
| `cksum` | `f00-cksum --core fixture.txt` | 1.249 | **0.380** | **3.29×** | `1448063438 22000 /tmp/f00-suite-bench.u36ghr6l/fix.txt` |
| `sum` | `f00-sum --core fixture.txt` | 0.897 | **0.365** | **2.46×** | `9481 22 /tmp/f00-suite-bench.u36ghr6l/fix.txt` |
| `ls` | `f00-ls --core -1 dir` | 1.048 | **0.419** | **2.50×** | `f01.txt f02.txt f03.txt f04.txt f05.txt f06.txt f07.txt f08.txt f09.txt f10.txt ` |
| `dir` | `f00-dir --core -1 dir` | 1.048 | **0.320** | **3.27×** | `f06.txt f02.txt f20.txt f14.txt f09.txt f13.txt f10.txt f17.txt f16.txt f08.txt ` |
| `vdir` | `f00-vdir --core -1 dir` | 1.092 | **0.317** | **3.45×** | `- f06.txt - f02.txt - f20.txt - f14.txt - f09.txt - f13.txt - f10.txt - f17.txt ` |
| `stat` | `f00-stat --core -c %s fixture.txt` | 0.993 | **0.305** | **3.25×** | `22000` |
| `realpath` | `f00-realpath --core .` | 0.830 | **0.383** | **2.16×** | `/home/runner/work/f00/f00/asm` |
| `readlink` | `f00-readlink --core /proc/self/exe` | 0.832 | **0.323** | **2.58×** | `/home/runner/work/f00/f00/asm/f00` |
| `df` | `f00-df --core -P /` | 0.948 | **0.358** | **2.65×** | `Filesystem 1K-blocks Used Available Use% Mounted on /dev/root 75085112 56775080 ` |
| `du` | `f00-du --core -s dir` | 0.907 | **0.371** | **2.45×** | `5 /tmp/f00-suite-bench.u36ghr6l/dir` |
| `dircolors` | `f00-dircolors --core -p` | 0.813 | **0.305** | **2.67×** | `# Configuration file for dircolors, a utility to help you set the # LS_COLORS en` |
| `env` | `f00-env --core -i true` | 1.179 | **0.305** | **3.87×** | `` |
| `timeout` | `f00-timeout --core 5 true` | 1.507 | **0.432** | **3.49×** | `` |
| `nice` | `f00-nice --core true` | 1.244 | **0.307** | **4.05×** | `` |
| `nohup` | `f00-nohup --core true` | 1.272 | **0.313** | **4.07×** | `` |
| `sleep` | `f00-sleep --core 0` | 0.885 | **0.366** | **2.42×** | `` |
| `test` | `f00-test --core -f fixture.txt` | 0.820 | **0.307** | **2.67×** | `` |
| `pathchk` | `f00-pathchk --core ok-name` | 0.805 | **0.301** | **2.68×** | `` |
| `mktemp` | `f00-mktemp --core -u` | 0.828 | **0.304** | **2.73×** | `/tmp/tmp.WjzSCS` |
| `sync` | `f00-sync --core` | 0.855 | **0.337** | **2.53×** | `` |
| `uptime` | `f00-uptime --core` | 1.497 | **0.317** | **4.72×** | `up 0 minutes` |
| `hostid` | `f00-hostid --core` | 0.949 | **0.357** | **2.66×** | `db830370` |
| `logname` | `f00-logname --core` | 0.835 | **0.361** | **2.31×** | `runner` |
| `tty` | `f00-tty --core` | 0.819 | **0.247** | **3.32×** | `not a tty` |
| `groups` | `f00-groups --core` | 0.932 | **0.328** | **2.84×** | `adm users docker systemd-journal runner` |
| `arch` | `f00-arch --core` | 0.807 | **0.360** | **2.24×** | `x86_64` |
| `hostname` | `f00-hostname --core` | 0.587 | **0.315** | **1.86×** | `runnervmvrwv9` |
| `users` | `f00-users --core` | 0.862 | **0.310** | **2.78×** | `` |
| `who` | `f00-who --core` | 0.869 | **0.309** | **2.82×** | `` |
| `pinky` | `f00-pinky --core` | 0.866 | **0.311** | **2.78×** | `` |
| `fold` | `f00-fold --core -w 40 fixture.txt` | 0.904 | **0.397** | **2.28×** | `suite-bench line abcdefghijklmnopqrstuvw xyz 0123456789 suite-bench line abcdefg` |
| `fmt` | `f00-fmt --core -w 40 fixture.txt` | 0.986 | **0.369** | **2.67×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `expand` | `f00-expand --core fixture.txt` | 0.873 | **0.379** | **2.31×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `unexpand` | `f00-unexpand --core fixture.txt` | 0.861 | **0.435** | **1.98×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tac` | `f00-tac --core fixture.txt` | 0.867 | **0.386** | **2.25×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `rev` | `f00-rev --core fixture.txt` | 1.162 | **0.408** | **2.85×** | `9876543210 zyxwvutsrqponmlkjihgfedcba enil hcneb-etius 9876543210 zyxwvutsrqponm` |
| `ptx` | `f00-ptx --core -A fixture.txt` | 1.570 | **0.419** | **3.75×** | `suite bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite bench line abcdefgh` |
| `pr` | `f00-pr --core -t fixture.txt` | 1.045 | **0.372** | **2.81×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `shuf` | `f00-shuf --core fixture.txt` | 0.874 | **0.593** | **1.47×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tsort` | `f00-tsort --core` | 0.848 | **0.328** | **2.59×** | `a b c` |
| `tee` | `f00-tee --core tee.out` | 0.916 | **0.405** | **2.26×** | `tee data tee data tee data tee data tee data tee data tee data tee data tee data` |
| `split` | `f00-split --core -l 50 fixture.txt out` | 1.268 | **1.332** | **0.95×** | `` |
| `csplit` | `f00-csplit --core -f xx fixture 5` | 1.051 | **0.328** | **3.21×** | `` |
| `chmod` | `f00-chmod --core 644 fixture.txt` | 0.831 | **0.314** | **2.65×** | `` |
| `touch` | `f00-touch --core touched` | 0.832 | **0.363** | **2.29×** | `` |
| `truncate` | `f00-truncate --core -s 0 trunc` | 0.825 | **0.326** | **2.53×** | `` |
| `cp` | `f00-cp --core fixture.txt cp.out` | 1.209 | **0.401** | **3.02×** | `` |
| `dd` | `f00-dd --core if=fixture of=dd.out bs=4k count=1` | 0.917 | **0.381** | **2.40×** | `` |
| `install` | `f00-install --core -m 644 fixture inst.out` | 1.202 | **0.406** | **2.96×** | `` |
| `yes` | `f00-yes --core --version` | 0.806 | **0.246** | **3.28×** | `f00-yes (f00) 0.15.2 License: MIT · https://f00.sh` |
| `[` | `f00-[ --core -f fixture.txt` | 0.794 | **0.300** | **2.64×** | `` |

Full machine-readable data: [suite.json](suite.json)

