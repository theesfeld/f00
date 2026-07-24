# Suite benchmarks (f00 vs GNU coreutils)

Generated: `2026-07-24T12:00:10Z` · N=5 median · warm-cache spawn-inclusive median

Host: x86_64 · Linux 7.1.4-arch1-1

| Tool | Command (f00) | GNU ms | f00 ms | Speedup | Sample output (f00) |
|------|---------------|-------:|-------:|--------:|---------------------|
| `true` | `f00-true --core` | 0.375 | **0.081** | **4.63×** | `` |
| `false` | `f00-false --core` | 0.228 | **0.133** | **1.72×** | `` |
| `basename` | `f00-basename --core /usr/bin/ls` | 0.278 | **0.085** | **3.27×** | `ls` |
| `dirname` | `f00-dirname --core /usr/bin/ls` | 0.249 | **0.083** | **3.00×** | `/usr/bin` |
| `echo` | `f00-echo --core hi` | 0.258 | **0.114** | **2.26×** | `hi` |
| `pwd` | `f00-pwd --core` | 0.249 | **0.111** | **2.24×** | `/home/glenda/Projects/f00` |
| `nproc` | `f00-nproc --core` | 0.245 | **0.083** | **2.95×** | `24` |
| `whoami` | `f00-whoami --core` | 1.221 | **0.082** | **14.84×** | `glenda` |
| `uname` | `f00-uname --core -s` | 0.287 | **0.133** | **2.16×** | `Linux` |
| `id` | `f00-id --core -u` | 0.271 | **0.122** | **2.22×** | `1000` |
| `date` | `f00-date --core -u +%Y` | 0.261 | **0.148** | **1.77×** | `2026` |
| `printenv` | `f00-printenv --core PATH` | 0.226 | **0.263** | **0.86×** | `/usr/local/sbin:/usr/local/bin:/usr/bin:/usr/bin/site_perl:/usr/bin/vendor_perl:` |
| `printf` | `f00-printf --core %s world` | 0.312 | **0.137** | **2.28×** | `world` |
| `factor` | `f00-factor --core 12` | 0.290 | **0.133** | **2.18×** | `12: 2 2 3` |
| `numfmt` | `f00-numfmt --core --to=si 1000` | 0.245 | **0.122** | **2.02×** | `1.0k` |
| `expr` | `f00-expr --core 1 + 1` | 1.288 | **0.124** | **10.40×** | `2` |
| `seq` | `f00-seq --core 1 5` | 0.254 | **0.110** | **2.30×** | `1 2 3 4 5` |
| `cat` | `f00-cat --core fixture.txt` | 0.248 | **0.099** | **2.49×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `wc` | `f00-wc --core -l fixture.txt` | 0.336 | **0.146** | **2.31×** | `400 /tmp/f00-suite-bench.do2xe8ix/fix.txt` |
| `head` | `f00-head --core -n 3 fixture.txt` | 0.271 | **0.116** | **2.34×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tail` | `f00-tail --core -n 3 fixture.txt` | 0.273 | **0.150** | **1.82×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `nl` | `f00-nl --core fixture.txt` | 0.287 | **0.165** | **1.73×** | `1 suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 2 suite-bench line abcd` |
| `od` | `f00-od --core -An -tx1 -N8 fixture.txt` | 0.238 | **0.117** | **2.04×** | `73 75 69 74 65 2d 62 65` |
| `cut` | `f00-cut --core -d: -f1 /etc/passwd` | 0.240 | **0.126** | **1.91×** | `root bin daemon mail ftp http nobody dbus systemd-coredump systemd-imds systemd-` |
| `tr` | `f00-tr --core a-z A-Z` | 0.373 | **0.121** | **3.08×** | `HELLO` |
| `sort` | `f00-sort --core fixture.txt` | 0.429 | **0.333** | **1.29×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `uniq` | `f00-uniq --core a.txt` | 0.284 | **0.153** | **1.85×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789` |
| `paste` | `f00-paste --core a.txt b.txt` | 0.301 | **0.246** | **1.23×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `comm` | `f00-comm --core -12 a.txt b.txt` | 0.306 | **0.199** | **1.54×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `join` | `f00-join --core a.txt b.txt` | 10.808 | **0.249** | **43.49×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 line abcdefghijklmnopqrst` |
| `base64` | `f00-base64 --core fixture.txt` | 0.439 | **0.188** | **2.34×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `base32` | `f00-base32 --core fixture.txt` | 0.408 | **0.265** | **1.54×** | `ON2WS5DFFVRGK3TDNAQGY2LOMUQGCYTDMRSWMZ3INFVGW3DNNZXXA4LSON2HK5TXPB4XUIBQGEZD GNB` |
| `basenc` | `f00-basenc --core --base64 fixture.txt` | 0.478 | **0.369** | **1.29×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `md5sum` | `f00-md5sum --core fixture.txt` | 0.791 | **0.172** | **4.61×** | `a5e6b1aa8523bc01f561fcef58d16894 /tmp/f00-suite-bench.do2xe8ix/fix.txt` |
| `sha1sum` | `f00-sha1sum --core fixture.txt` | 0.638 | **0.199** | **3.21×** | `49f92a1f57c1a825b5ca5777c92d6e15ac26a8ea /tmp/f00-suite-bench.do2xe8ix/fix.txt` |
| `sha224sum` | `f00-sha224sum --core fixture.txt` | 0.650 | **0.211** | **3.08×** | `94c1dff65fd14336129da4555171327a3e1e3e684810af23fa13e7f3 /tmp/f00-suite-bench.do` |
| `sha256sum` | `f00-sha256sum --core fixture.txt` | 0.794 | **0.205** | **3.88×** | `7c28ea8726bc6923d5b38a6b6938ad5907c01dc6640e1645cf63cbf9df706132 /tmp/f00-suite-` |
| `sha384sum` | `f00-sha384sum --core fixture.txt` | 0.639 | **0.193** | **3.31×** | `f2578c293b7eeebf35402906e5e2fcd522b567687241b72950690c389f88baf83f9cd518c1fd67b3` |
| `sha512sum` | `f00-sha512sum --core fixture.txt` | 0.651 | **0.190** | **3.43×** | `a3282daa5cc665baa2b247ee17a0798f1d8028bbfb2107ea0df9493d0e57bfba10ba95d72d2550c2` |
| `b2sum` | `f00-b2sum --core fixture.txt` | 0.263 | **0.183** | **1.43×** | `915149393ea1091e4aa19ad9c68f980ebf83f5da2a576a20ca65fb001c685bf890523b442b840760` |
| `cksum` | `f00-cksum --core fixture.txt` | 0.849 | **0.161** | **5.29×** | `1448063438 22000 /tmp/f00-suite-bench.do2xe8ix/fix.txt` |
| `sum` | `f00-sum --core fixture.txt` | 0.384 | **0.165** | **2.32×** | `9481 22 /tmp/f00-suite-bench.do2xe8ix/fix.txt` |
| `ls` | `f00-ls --core -1 dir` | 0.279 | **0.188** | **1.49×** | `f01.txt f02.txt f03.txt f04.txt f05.txt f06.txt f07.txt f08.txt f09.txt f10.txt ` |
| `dir` | `f00-dir --core -1 dir` | 0.276 | **0.118** | **2.34×** | `f20.txt f19.txt f18.txt f17.txt f16.txt f15.txt f14.txt f13.txt f12.txt f11.txt ` |
| `vdir` | `f00-vdir --core -1 dir` | 0.358 | **0.115** | **3.12×** | `- f20.txt - f19.txt - f18.txt - f17.txt - f16.txt - f15.txt - f14.txt - f13.txt ` |
| `stat` | `f00-stat --core -c %s fixture.txt` | 0.232 | **0.111** | **2.09×** | `22000` |
| `realpath` | `f00-realpath --core .` | 0.338 | **0.492** | **0.69×** | `/home/glenda/Projects/f00/asm` |
| `readlink` | `f00-readlink --core /proc/self/exe` | 0.369 | **0.127** | **2.91×** | `/home/glenda/Projects/f00/asm/f00` |
| `df` | `f00-df --core -P /` | 0.297 | **0.127** | **2.33×** | `Filesystem 1K-blocks Used Available Use% Mounted on /dev/mapper/ArchinstallVg-ro` |
| `du` | `f00-du --core -s dir` | 0.287 | **0.123** | **2.33×** | `1 /tmp/f00-suite-bench.do2xe8ix/dir` |
| `dircolors` | `f00-dircolors --core -p` | 0.231 | **0.111** | **2.08×** | `# Configuration file for dircolors, a utility to help you set the # LS_COLORS en` |
| `env` | `f00-env --core -i true` | 0.446 | **0.115** | **3.86×** | `` |
| `timeout` | `f00-timeout --core 5 true` | 0.522 | **0.173** | **3.01×** | `` |
| `nice` | `f00-nice --core true` | 0.430 | **0.135** | **3.18×** | `` |
| `nohup` | `f00-nohup --core true` | 1.422 | **0.111** | **12.78×** | `` |
| `sleep` | `f00-sleep --core 0` | 1.205 | **0.336** | **3.58×** | `` |
| `test` | `f00-test --core -f fixture.txt` | 0.224 | **0.114** | **1.97×** | `` |
| `pathchk` | `f00-pathchk --core ok-name` | 0.252 | **0.122** | **2.07×** | `` |
| `mktemp` | `f00-mktemp --core -u` | 0.231 | **0.118** | **1.96×** | `/tmp/tmp.T1qWzr` |
| `sync` | `f00-sync --core` | 0.226 | **0.122** | **1.86×** | `` |
| `uptime` | `f00-uptime --core` | 0.431 | **0.128** | **3.36×** | `up 1 day, 11 hours, 52 minutes` |
| `hostid` | `f00-hostid --core` | 1.668 | **0.195** | **8.57×** | `0f5105ed` |
| `logname` | `f00-logname --core` | 1.179 | **0.170** | **6.92×** | `glenda` |
| `tty` | `f00-tty --core` | 0.260 | **0.089** | **2.93×** | `not a tty` |
| `groups` | `f00-groups --core` | 1.232 | **0.172** | **7.17×** | `seat wheel glenda` |
| `users` | `f00-users --core` | 1.445 | **0.153** | **9.43×** | `` |
| `who` | `f00-who --core` | 0.289 | **0.133** | **2.18×** | `` |
| `pinky` | `f00-pinky --core` | 0.291 | **0.127** | **2.29×** | `` |
| `fold` | `f00-fold --core -w 40 fixture.txt` | 0.484 | **0.183** | **2.65×** | `suite-bench line abcdefghijklmnopqrstuvw xyz 0123456789 suite-bench line abcdefg` |
| `fmt` | `f00-fmt --core -w 40 fixture.txt` | 0.345 | **0.154** | **2.25×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `expand` | `f00-expand --core fixture.txt` | 0.455 | **0.168** | **2.70×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `unexpand` | `f00-unexpand --core fixture.txt` | 0.446 | **0.216** | **2.06×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tac` | `f00-tac --core fixture.txt` | 0.300 | **0.180** | **1.67×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `rev` | `f00-rev --core fixture.txt` | 1.529 | **0.169** | **9.03×** | `9876543210 zyxwvutsrqponmlkjihgfedcba enil hcneb-etius 9876543210 zyxwvutsrqponm` |
| `ptx` | `f00-ptx --core -A fixture.txt` | 0.614 | **0.170** | **3.62×** | `suite bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite bench line abcdefgh` |
| `pr` | `f00-pr --core -t fixture.txt` | 0.364 | **0.170** | **2.15×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `shuf` | `f00-shuf --core fixture.txt` | 0.307 | **0.241** | **1.28×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tsort` | `f00-tsort --core` | 0.237 | **0.121** | **1.96×** | `a b c` |
| `tee` | `f00-tee --core tee.out` | 0.253 | **0.120** | **2.11×** | `tee data tee data tee data tee data tee data tee data tee data tee data tee data` |
| `split` | `f00-split --core -l 50 fixture.txt out` | 0.255 | **0.259** | **0.99×** | `` |
| `csplit` | `f00-csplit --core -f xx fixture 5` | 0.266 | **0.120** | **2.21×** | `` |
| `chmod` | `f00-chmod --core 644 fixture.txt` | 0.225 | **0.114** | **1.98×** | `` |
| `touch` | `f00-touch --core touched` | 0.246 | **0.176** | **1.40×** | `` |
| `truncate` | `f00-truncate --core -s 0 trunc` | 0.245 | **0.114** | **2.16×** | `` |
| `cp` | `f00-cp --core fixture.txt cp.out` | 0.313 | **0.115** | **2.72×** | `` |
| `dd` | `f00-dd --core if=fixture of=dd.out bs=4k count=1` | 1.295 | **0.135** | **9.63×** | `` |
| `install` | `f00-install --core -m 644 fixture inst.out` | 0.325 | **0.118** | **2.75×** | `` |
| `yes` | `f00-yes --core --version` | 1.259 | **0.080** | **15.82×** | `f00-yes (f00) 0.15.2 License: MIT · https://f00.sh` |
| `[` | `f00-[ --core -f fixture.txt` | 0.227 | **0.108** | **2.10×** | `` |

Full machine-readable data: [suite.json](suite.json)

