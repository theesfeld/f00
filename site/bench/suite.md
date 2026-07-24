# Suite benchmarks (f00 vs GNU coreutils)

Generated: `2026-07-24T10:47:18Z` · N=20 median · warm-cache spawn-inclusive median

Host: x86_64 · Linux 7.1.4-arch1-1

| Tool | Command (f00) | GNU ms | f00 ms | Speedup | Sample output (f00) |
|------|---------------|-------:|-------:|--------:|---------------------|
| `true` | `f00-true --core` | 0.258 | **0.075** | **3.45×** | `` |
| `false` | `f00-false --core` | 0.219 | **0.079** | **2.77×** | `` |
| `basename` | `f00-basename --core /usr/bin/ls` | 0.277 | **0.086** | **3.23×** | `ls` |
| `dirname` | `f00-dirname --core /usr/bin/ls` | 0.250 | **0.075** | **3.35×** | `/usr/bin` |
| `echo` | `f00-echo --core hi` | 0.241 | **0.123** | **1.95×** | `hi` |
| `pwd` | `f00-pwd --core` | 0.267 | **0.142** | **1.87×** | `/home/glenda/Projects/f00/asm` |
| `nproc` | `f00-nproc --core` | 0.316 | **0.084** | **3.76×** | `24` |
| `whoami` | `f00-whoami --core` | 1.425 | **0.084** | **16.90×** | `glenda` |
| `uname` | `f00-uname --core -s` | 0.264 | **0.123** | **2.14×** | `Linux` |
| `id` | `f00-id --core -u` | 0.292 | **0.131** | **2.24×** | `1000` |
| `date` | `f00-date --core -u +%Y` | 0.308 | **0.195** | **1.58×** | `2026` |
| `printenv` | `f00-printenv --core PATH` | 0.257 | **0.193** | **1.33×** | `/usr/local/sbin:/usr/local/bin:/usr/bin:/usr/bin/site_perl:/usr/bin/vendor_perl:` |
| `printf` | `f00-printf --core %s world` | 0.312 | **0.131** | **2.38×** | `world` |
| `factor` | `f00-factor --core 12` | 0.343 | **0.180** | **1.91×** | `12: 2 2 3` |
| `numfmt` | `f00-numfmt --core --to=si 1000` | 0.281 | **0.127** | **2.22×** | `1.0k` |
| `expr` | `f00-expr --core 1 + 1` | 1.469 | **0.144** | **10.18×** | `2` |
| `seq` | `f00-seq --core 1 5` | 0.309 | **0.133** | **2.33×** | `1 2 3 4 5` |
| `cat` | `f00-cat --core fixture.txt` | 0.268 | **0.104** | **2.58×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `wc` | `f00-wc --core -l fixture.txt` | 0.409 | **0.290** | **1.41×** | `400 /tmp/f00-suite-bench.2_9yw6r3/fix.txt` |
| `head` | `f00-head --core -n 3 fixture.txt` | 0.284 | **0.130** | **2.19×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tail` | `f00-tail --core -n 3 fixture.txt` | 0.297 | **0.159** | **1.87×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `nl` | `f00-nl --core fixture.txt` | 0.332 | **0.189** | **1.76×** | `1 suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 2 suite-bench line abcd` |
| `od` | `f00-od --core -An -tx1 -N8 fixture.txt` | 0.267 | **0.127** | **2.11×** | `73 75 69 74 65 2d 62 65` |
| `cut` | `f00-cut --core -d: -f1 /etc/passwd` | 0.264 | **0.134** | **1.97×** | `root bin daemon mail ftp http nobody dbus systemd-coredump systemd-imds systemd-` |
| `tr` | `f00-tr --core a-z A-Z` | 0.328 | **0.148** | **2.21×** | `HELLO` |
| `sort` | `f00-sort --core fixture.txt` | 0.409 | **0.384** | **1.07×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `uniq` | `f00-uniq --core a.txt` | 0.298 | **0.162** | **1.84×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789` |
| `paste` | `f00-paste --core a.txt b.txt` | 0.313 | **0.259** | **1.21×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `comm` | `f00-comm --core -12 a.txt b.txt` | 0.366 | **0.257** | **1.42×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `join` | `f00-join --core a.txt b.txt` | 12.718 | **0.479** | **26.56×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 line abcdefghijklmnopqrst` |
| `base64` | `f00-base64 --core fixture.txt` | 0.443 | **0.220** | **2.01×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `base32` | `f00-base32 --core fixture.txt` | 0.308 | **0.229** | **1.34×** | `ON2WS5DFFVRGK3TDNAQGY2LOMUQGCYTDMRSWMZ3INFVGW3DNNZXXA4LSON2HK5TXPB4XUIBQGEZD GNB` |
| `md5sum` | `f00-md5sum --core fixture.txt` | 0.994 | **0.207** | **4.80×** | `a5e6b1aa8523bc01f561fcef58d16894 /tmp/f00-suite-bench.2_9yw6r3/fix.txt` |
| `sha1sum` | `f00-sha1sum --core fixture.txt` | 1.000 | **0.222** | **4.51×** | `49f92a1f57c1a825b5ca5777c92d6e15ac26a8ea /tmp/f00-suite-bench.2_9yw6r3/fix.txt` |
| `sha256sum` | `f00-sha256sum --core fixture.txt` | 0.947 | **0.235** | **4.02×** | `7c28ea8726bc6923d5b38a6b6938ad5907c01dc6640e1645cf63cbf9df706132 /tmp/f00-suite-` |
| `sha512sum` | `f00-sha512sum --core fixture.txt` | 0.844 | **0.216** | **3.90×** | `a3282daa5cc665baa2b247ee17a0798f1d8028bbfb2107ea0df9493d0e57bfba10ba95d72d2550c2` |
| `cksum` | `f00-cksum --core fixture.txt` | 0.843 | **0.175** | **4.82×** | `1448063438 22000 /tmp/f00-suite-bench.2_9yw6r3/fix.txt` |
| `sum` | `f00-sum --core fixture.txt` | 0.459 | **0.169** | **2.71×** | `9481 22 /tmp/f00-suite-bench.2_9yw6r3/fix.txt` |
| `ls` | `f00-ls --core -1 dir` | 0.355 | **0.219** | **1.62×** | `f01.txt f02.txt f03.txt f04.txt f05.txt f06.txt f07.txt f08.txt f09.txt f10.txt ` |
| `dir` | `f00-dir --core -1 dir` | 0.323 | **0.132** | **2.44×** | `f20.txt f19.txt f18.txt f17.txt f16.txt f15.txt f14.txt f13.txt f12.txt f11.txt ` |
| `stat` | `f00-stat --core -c %s fixture.txt` | 0.299 | **0.137** | **2.19×** | `22000` |
| `realpath` | `f00-realpath --core .` | 0.269 | **0.205** | **1.31×** | `/home/glenda/Projects/f00/asm` |
| `df` | `f00-df --core -P /` | 0.447 | **0.176** | **2.55×** | `Filesystem 1K-blocks Used Available Use% Mounted on /dev/mapper/ArchinstallVg-ro` |
| `du` | `f00-du --core -s dir` | 0.354 | **0.180** | **1.97×** | `1 /tmp/f00-suite-bench.2_9yw6r3/dir` |
| `dircolors` | `f00-dircolors --core -p` | 0.315 | **0.149** | **2.12×** | `# Configuration file for dircolors, a utility to help you set the # LS_COLORS en` |
| `env` | `f00-env --core -i true` | 0.560 | **0.180** | **3.12×** | `` |
| `timeout` | `f00-timeout --core 5 true` | 0.641 | **0.243** | **2.64×** | `` |
| `sleep` | `f00-sleep --core 0` | 1.550 | **0.194** | **7.98×** | `` |
| `test` | `f00-test --core -f fixture.txt` | 0.281 | **0.130** | **2.15×** | `` |
| `pathchk` | `f00-pathchk --core ok-name` | 0.262 | **0.137** | **1.92×** | `` |
| `mktemp` | `f00-mktemp --core -u` | 0.308 | **0.124** | **2.48×** | `/tmp/tmp.LHSjGy` |
| `sync` | `f00-sync --core` | 0.300 | **0.150** | **1.99×** | `` |
| `uptime` | `f00-uptime --core` | 0.495 | **0.136** | **3.63×** | `up 1 day, 10 hours, 39 minutes` |
| `hostid` | `f00-hostid --core` | 1.783 | **0.201** | **8.86×** | `0f5105ed` |
| `logname` | `f00-logname --core` | 1.508 | **0.185** | **8.14×** | `glenda` |
| `tty` | `f00-tty --core` | 0.277 | **0.081** | **3.42×** | `not a tty` |
| `groups` | `f00-groups --core` | 1.421 | **0.127** | **11.22×** | `seat wheel glenda` |
| `users` | `f00-users --core` | 1.464 | **0.141** | **10.35×** | `` |
| `fold` | `f00-fold --core -w 40 fixture.txt` | 0.481 | **0.170** | **2.83×** | `suite-bench line abcdefghijklmnopqrstuvw xyz 0123456789 suite-bench line abcdefg` |
| `fmt` | `f00-fmt --core -w 40 fixture.txt` | 0.357 | **0.173** | **2.07×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `expand` | `f00-expand --core fixture.txt` | 0.584 | **0.173** | **3.38×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tac` | `f00-tac --core fixture.txt` | 0.320 | **0.182** | **1.76×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `rev` | `f00-rev --core fixture.txt` | 1.726 | **0.228** | **7.58×** | `9876543210 zyxwvutsrqponmlkjihgfedcba enil hcneb-etius 9876543210 zyxwvutsrqponm` |
| `ptx` | `f00-ptx --core -A fixture.txt` | 0.883 | **0.203** | **4.35×** | `suite bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite bench line abcdefgh` |
| `pr` | `f00-pr --core -t fixture.txt` | 0.353 | **0.245** | **1.44×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `shuf` | `f00-shuf --core fixture.txt` | 0.322 | **0.260** | **1.24×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tsort` | `f00-tsort --core` | 0.312 | **0.142** | **2.20×** | `a b c` |
| `yes` | `f00-yes --core --version` | 1.376 | **0.070** | **19.53×** | `f00-yes (f00) 0.15.0 License: MIT · https://f00.sh` |

Full machine-readable data: [suite.json](suite.json)

