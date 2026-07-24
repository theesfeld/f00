# Suite benchmarks (f00 vs GNU coreutils)

Generated: `2026-07-24T11:44:00Z` · N=15 median · warm-cache spawn-inclusive median

Host: x86_64 · Linux 7.1.4-arch1-1

| Tool | Command (f00) | GNU ms | f00 ms | Speedup | Sample output (f00) |
|------|---------------|-------:|-------:|--------:|---------------------|
| `true` | `f00-true --core` | 0.201 | **0.072** | **2.79×** | `` |
| `false` | `f00-false --core` | 0.197 | **0.073** | **2.72×** | `` |
| `basename` | `f00-basename --core /usr/bin/ls` | 0.218 | **0.072** | **3.01×** | `ls` |
| `dirname` | `f00-dirname --core /usr/bin/ls` | 0.236 | **0.072** | **3.29×** | `/usr/bin` |
| `echo` | `f00-echo --core hi` | 0.243 | **0.108** | **2.26×** | `hi` |
| `pwd` | `f00-pwd --core` | 0.240 | **0.111** | **2.17×** | `/home/glenda/Projects/f00/asm` |
| `nproc` | `f00-nproc --core` | 0.264 | **0.069** | **3.84×** | `24` |
| `whoami` | `f00-whoami --core` | 1.236 | **0.079** | **15.72×** | `glenda` |
| `uname` | `f00-uname --core -s` | 0.228 | **0.113** | **2.01×** | `Linux` |
| `id` | `f00-id --core -u` | 0.226 | **0.115** | **1.98×** | `1000` |
| `date` | `f00-date --core -u +%Y` | 0.261 | **0.111** | **2.36×** | `2026` |
| `printenv` | `f00-printenv --core PATH` | 0.220 | **0.112** | **1.97×** | `/usr/local/sbin:/usr/local/bin:/usr/bin:/usr/bin/site_perl:/usr/bin/vendor_perl:` |
| `printf` | `f00-printf --core %s world` | 0.224 | **0.117** | **1.90×** | `world` |
| `factor` | `f00-factor --core 12` | 0.277 | **0.118** | **2.34×** | `12: 2 2 3` |
| `numfmt` | `f00-numfmt --core --to=si 1000` | 0.229 | **0.112** | **2.04×** | `1.0k` |
| `expr` | `f00-expr --core 1 + 1` | 1.271 | **0.130** | **9.78×** | `2` |
| `seq` | `f00-seq --core 1 5` | 0.266 | **0.123** | **2.17×** | `1 2 3 4 5` |
| `cat` | `f00-cat --core fixture.txt` | 0.237 | **0.097** | **2.45×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `wc` | `f00-wc --core -l fixture.txt` | 0.367 | **0.162** | **2.26×** | `400 /tmp/f00-suite-bench.vghk3tco/fix.txt` |
| `head` | `f00-head --core -n 3 fixture.txt` | 0.277 | **0.129** | **2.15×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tail` | `f00-tail --core -n 3 fixture.txt` | 0.261 | **0.129** | **2.02×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `nl` | `f00-nl --core fixture.txt` | 0.324 | **0.159** | **2.04×** | `1 suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 2 suite-bench line abcd` |
| `od` | `f00-od --core -An -tx1 -N8 fixture.txt` | 0.230 | **0.120** | **1.92×** | `73 75 69 74 65 2d 62 65` |
| `cut` | `f00-cut --core -d: -f1 /etc/passwd` | 0.250 | **0.125** | **2.00×** | `root bin daemon mail ftp http nobody dbus systemd-coredump systemd-imds systemd-` |
| `tr` | `f00-tr --core a-z A-Z` | 0.259 | **0.122** | **2.12×** | `HELLO` |
| `sort` | `f00-sort --core fixture.txt` | 0.418 | **0.333** | **1.26×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `uniq` | `f00-uniq --core a.txt` | 0.267 | **0.136** | **1.96×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789` |
| `paste` | `f00-paste --core a.txt b.txt` | 0.271 | **0.215** | **1.26×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `comm` | `f00-comm --core -12 a.txt b.txt` | 0.287 | **0.197** | **1.46×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `join` | `f00-join --core a.txt b.txt` | 10.691 | **0.230** | **46.40×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 line abcdefghijklmnopqrst` |
| `base64` | `f00-base64 --core fixture.txt` | 0.263 | **0.193** | **1.36×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `base32` | `f00-base32 --core fixture.txt` | 0.466 | **0.208** | **2.25×** | `ON2WS5DFFVRGK3TDNAQGY2LOMUQGCYTDMRSWMZ3INFVGW3DNNZXXA4LSON2HK5TXPB4XUIBQGEZD GNB` |
| `basenc` | `f00-basenc --core --base64 fixture.txt` | 0.488 | **0.199** | **2.45×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `md5sum` | `f00-md5sum --core fixture.txt` | 0.930 | **0.158** | **5.88×** | `a5e6b1aa8523bc01f561fcef58d16894 /tmp/f00-suite-bench.vghk3tco/fix.txt` |
| `sha1sum` | `f00-sha1sum --core fixture.txt` | 0.769 | **0.201** | **3.83×** | `49f92a1f57c1a825b5ca5777c92d6e15ac26a8ea /tmp/f00-suite-bench.vghk3tco/fix.txt` |
| `sha224sum` | `f00-sha224sum --core fixture.txt` | 0.814 | **0.213** | **3.81×** | `94c1dff65fd14336129da4555171327a3e1e3e684810af23fa13e7f3 /tmp/f00-suite-bench.vg` |
| `sha256sum` | `f00-sha256sum --core fixture.txt` | 0.680 | **0.222** | **3.06×** | `7c28ea8726bc6923d5b38a6b6938ad5907c01dc6640e1645cf63cbf9df706132 /tmp/f00-suite-` |
| `sha384sum` | `f00-sha384sum --core fixture.txt` | 0.639 | **0.240** | **2.66×** | `f2578c293b7eeebf35402906e5e2fcd522b567687241b72950690c389f88baf83f9cd518c1fd67b3` |
| `sha512sum` | `f00-sha512sum --core fixture.txt` | 0.739 | **0.191** | **3.87×** | `a3282daa5cc665baa2b247ee17a0798f1d8028bbfb2107ea0df9493d0e57bfba10ba95d72d2550c2` |
| `b2sum` | `f00-b2sum --core fixture.txt` | 0.270 | **0.184** | **1.46×** | `915149393ea1091e4aa19ad9c68f980ebf83f5da2a576a20ca65fb001c685bf890523b442b840760` |
| `cksum` | `f00-cksum --core fixture.txt` | 0.792 | **0.161** | **4.91×** | `1448063438 22000 /tmp/f00-suite-bench.vghk3tco/fix.txt` |
| `sum` | `f00-sum --core fixture.txt` | 0.259 | **0.139** | **1.87×** | `9481 22 /tmp/f00-suite-bench.vghk3tco/fix.txt` |
| `ls` | `f00-ls --core -1 dir` | 0.283 | **0.189** | **1.50×** | `f01.txt f02.txt f03.txt f04.txt f05.txt f06.txt f07.txt f08.txt f09.txt f10.txt ` |
| `dir` | `f00-dir --core -1 dir` | 0.271 | **0.112** | **2.42×** | `f20.txt f19.txt f18.txt f17.txt f16.txt f15.txt f14.txt f13.txt f12.txt f11.txt ` |
| `vdir` | `f00-vdir --core -1 dir` | 0.276 | **0.114** | **2.42×** | `- f20.txt - f19.txt - f18.txt - f17.txt - f16.txt - f15.txt - f14.txt - f13.txt ` |
| `stat` | `f00-stat --core -c %s fixture.txt` | 0.231 | **0.114** | **2.03×** | `22000` |
| `realpath` | `f00-realpath --core .` | 0.220 | **0.202** | **1.09×** | `/home/glenda/Projects/f00/asm` |
| `readlink` | `f00-readlink --core /proc/self/exe` | 0.236 | **0.119** | **1.98×** | `/home/glenda/Projects/f00/asm/f00` |
| `df` | `f00-df --core -P /` | 0.423 | **0.131** | **3.23×** | `Filesystem 1K-blocks Used Available Use% Mounted on /dev/mapper/ArchinstallVg-ro` |
| `du` | `f00-du --core -s dir` | 0.249 | **0.119** | **2.08×** | `1 /tmp/f00-suite-bench.vghk3tco/dir` |
| `dircolors` | `f00-dircolors --core -p` | 0.230 | **0.109** | **2.11×** | `# Configuration file for dircolors, a utility to help you set the # LS_COLORS en` |
| `env` | `f00-env --core -i true` | 0.373 | **0.142** | **2.62×** | `` |
| `timeout` | `f00-timeout --core 5 true` | 0.515 | **0.211** | **2.43×** | `` |
| `nice` | `f00-nice --core true` | 0.424 | **0.149** | **2.86×** | `` |
| `nohup` | `f00-nohup --core true` | 1.274 | **0.147** | **8.68×** | `` |
| `sleep` | `f00-sleep --core 0` | 1.213 | **0.162** | **7.50×** | `` |
| `test` | `f00-test --core -f fixture.txt` | 0.223 | **0.108** | **2.05×** | `` |
| `pathchk` | `f00-pathchk --core ok-name` | 0.215 | **0.109** | **1.97×** | `` |
| `mktemp` | `f00-mktemp --core -u` | 0.234 | **0.110** | **2.13×** | `/tmp/tmp.q88THu` |
| `sync` | `f00-sync --core` | 0.268 | **0.126** | **2.13×** | `` |
| `uptime` | `f00-uptime --core` | 0.527 | **0.113** | **4.68×** | `up 1 day, 11 hours, 36 minutes` |
| `hostid` | `f00-hostid --core` | 1.574 | **0.167** | **9.42×** | `0f5105ed` |
| `logname` | `f00-logname --core` | 1.237 | **0.171** | **7.22×** | `glenda` |
| `tty` | `f00-tty --core` | 0.238 | **0.074** | **3.23×** | `not a tty` |
| `groups` | `f00-groups --core` | 0.982 | **0.114** | **8.65×** | `seat wheel glenda` |
| `users` | `f00-users --core` | 1.269 | **0.113** | **11.20×** | `` |
| `who` | `f00-who --core` | 0.234 | **0.112** | **2.09×** | `` |
| `pinky` | `f00-pinky --core` | 0.260 | **0.111** | **2.33×** | `` |
| `fold` | `f00-fold --core -w 40 fixture.txt` | 0.366 | **0.153** | **2.39×** | `suite-bench line abcdefghijklmnopqrstuvw xyz 0123456789 suite-bench line abcdefg` |
| `fmt` | `f00-fmt --core -w 40 fixture.txt` | 0.299 | **0.142** | **2.11×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `expand` | `f00-expand --core fixture.txt` | 0.520 | **0.168** | **3.09×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `unexpand` | `f00-unexpand --core fixture.txt` | 0.377 | **0.169** | **2.24×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tac` | `f00-tac --core fixture.txt` | 0.235 | **0.145** | **1.62×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `rev` | `f00-rev --core fixture.txt` | 1.315 | **0.161** | **8.15×** | `9876543210 zyxwvutsrqponmlkjihgfedcba enil hcneb-etius 9876543210 zyxwvutsrqponm` |
| `ptx` | `f00-ptx --core -A fixture.txt` | 0.580 | **0.157** | **3.70×** | `suite bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite bench line abcdefgh` |
| `pr` | `f00-pr --core -t fixture.txt` | 0.331 | **0.147** | **2.24×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `shuf` | `f00-shuf --core fixture.txt` | 0.277 | **0.239** | **1.16×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tsort` | `f00-tsort --core` | 0.263 | **0.154** | **1.71×** | `a b c` |
| `tee` | `f00-tee --core tee.out` | 0.253 | **0.137** | **1.85×** | `tee data tee data tee data tee data tee data tee data tee data tee data tee data` |
| `split` | `f00-split --core -l 50 fixture.txt out` | 0.286 | **0.265** | **1.08×** | `` |
| `csplit` | `f00-csplit --core -f xx fixture 5` | 0.308 | **0.137** | **2.25×** | `` |
| `chmod` | `f00-chmod --core 644 fixture.txt` | 0.222 | **0.114** | **1.95×** | `` |
| `touch` | `f00-touch --core touched` | 0.256 | **0.215** | **1.19×** | `` |
| `truncate` | `f00-truncate --core -s 0 trunc` | 0.238 | **0.115** | **2.07×** | `` |
| `cp` | `f00-cp --core fixture.txt cp.out` | 0.327 | **0.134** | **2.44×** | `` |
| `dd` | `f00-dd --core if=fixture of=dd.out bs=4k count=1` | 1.181 | **0.132** | **8.96×** | `` |
| `install` | `f00-install --core -m 644 fixture inst.out` | 0.436 | **0.132** | **3.29×** | `` |
| `yes` | `f00-yes --core --version` | 1.165 | **0.074** | **15.68×** | `f00-yes (f00) 0.15.1 License: MIT · https://f00.sh` |
| `[` | `f00-[ --core -f fixture.txt` | 0.242 | **0.117** | **2.07×** | `` |

Full machine-readable data: [suite.json](suite.json)

