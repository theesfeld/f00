# Suite benchmarks (f00 vs GNU coreutils)

Generated: `2026-07-24T13:08:21Z` · N=15 median · warm-cache spawn-inclusive median

Host: x86_64 · Linux 6.17.0-1020-azure

| Tool | Command (f00) | GNU ms | f00 ms | Speedup | Sample output (f00) |
|------|---------------|-------:|-------:|--------:|---------------------|
| `true` | `f00-true --core` | 0.565 | **0.250** | **2.26×** | `` |
| `false` | `f00-false --core` | 0.558 | **0.277** | **2.02×** | `` |
| `basename` | `f00-basename --core /usr/bin/ls` | 0.812 | **0.252** | **3.23×** | `ls` |
| `dirname` | `f00-dirname --core /usr/bin/ls` | 0.812 | **0.247** | **3.29×** | `/usr/bin` |
| `echo` | `f00-echo --core hi` | 0.838 | **0.304** | **2.76×** | `hi` |
| `pwd` | `f00-pwd --core` | 0.840 | **0.309** | **2.72×** | `/home/runner/work/f00/f00` |
| `nproc` | `f00-nproc --core` | 0.873 | **0.303** | **2.88×** | `4` |
| `whoami` | `f00-whoami --core` | 0.947 | **0.313** | **3.03×** | `runner` |
| `uname` | `f00-uname --core -s` | 0.843 | **0.306** | **2.75×** | `Linux` |
| `id` | `f00-id --core -u` | 0.982 | **0.309** | **3.18×** | `1001` |
| `date` | `f00-date --core -u +%Y` | 0.918 | **0.314** | **2.93×** | `2026` |
| `printenv` | `f00-printenv --core PATH` | 0.822 | **0.314** | **2.62×** | `/snap/bin:/home/runner/.local/bin:/opt/pipx_bin:/home/runner/.cargo/bin:/home/ru` |
| `printf` | `f00-printf --core %s world` | 0.841 | **0.301** | **2.80×** | `world` |
| `factor` | `f00-factor --core 12` | 0.876 | **0.307** | **2.85×** | `12: 2 2 3` |
| `numfmt` | `f00-numfmt --core --to=si 1000` | 0.851 | **0.305** | **2.79×** | `1.0k` |
| `expr` | `f00-expr --core 1 + 1` | 0.888 | **0.321** | **2.77×** | `2` |
| `seq` | `f00-seq --core 1 5` | 0.823 | **0.298** | **2.76×** | `1 2 3 4 5` |
| `cat` | `f00-cat --core fixture.txt` | 0.831 | **0.292** | **2.84×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `wc` | `f00-wc --core -l fixture.txt` | 0.882 | **0.344** | **2.57×** | `400 /tmp/f00-suite-bench.9b8wtaf8/fix.txt` |
| `head` | `f00-head --core -n 3 fixture.txt` | 0.826 | **0.320** | **2.58×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tail` | `f00-tail --core -n 3 fixture.txt` | 0.841 | **0.343** | **2.45×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `nl` | `f00-nl --core fixture.txt` | 0.892 | **0.392** | **2.28×** | `1 suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 2 suite-bench line abcd` |
| `od` | `f00-od --core -An -tx1 -N8 fixture.txt` | 0.855 | **0.315** | **2.71×** | `73 75 69 74 65 2d 62 65` |
| `cut` | `f00-cut --core -d: -f1 /etc/passwd` | 0.827 | **0.317** | **2.61×** | `root daemon bin sys sync games man lp mail news uucp proxy www-data backup list ` |
| `tr` | `f00-tr --core a-z A-Z` | 0.859 | **0.332** | **2.59×** | `HELLO` |
| `sort` | `f00-sort --core fixture.txt` | 1.364 | **0.883** | **1.54×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `uniq` | `f00-uniq --core a.txt` | 0.851 | **0.371** | **2.29×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789` |
| `paste` | `f00-paste --core a.txt b.txt` | 0.934 | **0.466** | **2.00×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `comm` | `f00-comm --core -12 a.txt b.txt` | 0.904 | **0.440** | **2.05×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `join` | `f00-join --core a.txt b.txt` | 15.142 | **0.464** | **32.66×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 line abcdefghijklmnopqrst` |
| `base64` | `f00-base64 --core fixture.txt` | 0.873 | **0.462** | **1.89×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `base32` | `f00-base32 --core fixture.txt` | 0.893 | **0.505** | **1.77×** | `ON2WS5DFFVRGK3TDNAQGY2LOMUQGCYTDMRSWMZ3INFVGW3DNNZXXA4LSON2HK5TXPB4XUIBQGEZD GNB` |
| `basenc` | `f00-basenc --core --base64 fixture.txt` | 0.884 | **0.465** | **1.90×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `md5sum` | `f00-md5sum --core fixture.txt` | 1.270 | **0.386** | **3.29×** | `a5e6b1aa8523bc01f561fcef58d16894 /tmp/f00-suite-bench.9b8wtaf8/fix.txt` |
| `sha1sum` | `f00-sha1sum --core fixture.txt` | 1.233 | **0.420** | **2.93×** | `49f92a1f57c1a825b5ca5777c92d6e15ac26a8ea /tmp/f00-suite-bench.9b8wtaf8/fix.txt` |
| `sha224sum` | `f00-sha224sum --core fixture.txt` | 1.234 | **0.434** | **2.84×** | `94c1dff65fd14336129da4555171327a3e1e3e684810af23fa13e7f3 /tmp/f00-suite-bench.9b` |
| `sha256sum` | `f00-sha256sum --core fixture.txt` | 1.236 | **0.448** | **2.76×** | `7c28ea8726bc6923d5b38a6b6938ad5907c01dc6640e1645cf63cbf9df706132 /tmp/f00-suite-` |
| `sha384sum` | `f00-sha384sum --core fixture.txt` | 1.276 | **0.407** | **3.13×** | `f2578c293b7eeebf35402906e5e2fcd522b567687241b72950690c389f88baf83f9cd518c1fd67b3` |
| `sha512sum` | `f00-sha512sum --core fixture.txt` | 1.263 | **0.403** | **3.13×** | `a3282daa5cc665baa2b247ee17a0798f1d8028bbfb2107ea0df9493d0e57bfba10ba95d72d2550c2` |
| `b2sum` | `f00-b2sum --core fixture.txt` | 0.889 | **0.413** | **2.15×** | `915149393ea1091e4aa19ad9c68f980ebf83f5da2a576a20ca65fb001c685bf890523b442b840760` |
| `cksum` | `f00-cksum --core fixture.txt` | 1.262 | **0.375** | **3.37×** | `1448063438 22000 /tmp/f00-suite-bench.9b8wtaf8/fix.txt` |
| `sum` | `f00-sum --core fixture.txt` | 0.890 | **0.356** | **2.50×** | `9481 22 /tmp/f00-suite-bench.9b8wtaf8/fix.txt` |
| `ls` | `f00-ls --core -1 dir` | 1.058 | **0.452** | **2.34×** | `f01.txt f02.txt f03.txt f04.txt f05.txt f06.txt f07.txt f08.txt f09.txt f10.txt ` |
| `dir` | `f00-dir --core -1 dir` | 1.051 | **0.314** | **3.35×** | `f06.txt f02.txt f20.txt f14.txt f09.txt f13.txt f10.txt f17.txt f16.txt f08.txt ` |
| `vdir` | `f00-vdir --core -1 dir` | 1.051 | **0.316** | **3.32×** | `- f06.txt - f02.txt - f20.txt - f14.txt - f09.txt - f13.txt - f10.txt - f17.txt ` |
| `stat` | `f00-stat --core -c %s fixture.txt` | 1.003 | **0.300** | **3.35×** | `22000` |
| `realpath` | `f00-realpath --core .` | 0.842 | **0.377** | **2.24×** | `/home/runner/work/f00/f00/asm` |
| `readlink` | `f00-readlink --core /proc/self/exe` | 0.839 | **0.317** | **2.64×** | `/home/runner/work/f00/f00/asm/f00` |
| `df` | `f00-df --core -P /` | 0.959 | **0.353** | **2.72×** | `Filesystem 1K-blocks Used Available Use% Mounted on /dev/root 151263856 59008684` |
| `du` | `f00-du --core -s dir` | 0.906 | **0.353** | **2.57×** | `5 /tmp/f00-suite-bench.9b8wtaf8/dir` |
| `dircolors` | `f00-dircolors --core -p` | 0.818 | **0.325** | **2.52×** | `# Configuration file for dircolors, a utility to help you set the # LS_COLORS en` |
| `env` | `f00-env --core -i true` | 1.192 | **0.308** | **3.87×** | `` |
| `timeout` | `f00-timeout --core 5 true` | 1.499 | **0.426** | **3.52×** | `` |
| `nice` | `f00-nice --core true` | 1.254 | **0.306** | **4.09×** | `` |
| `nohup` | `f00-nohup --core true` | 1.244 | **0.297** | **4.19×** | `` |
| `sleep` | `f00-sleep --core 0` | 0.872 | **0.361** | **2.41×** | `` |
| `test` | `f00-test --core -f fixture.txt` | 0.803 | **0.304** | **2.64×** | `` |
| `pathchk` | `f00-pathchk --core ok-name` | 0.802 | **0.302** | **2.65×** | `` |
| `mktemp` | `f00-mktemp --core -u` | 0.822 | **0.308** | **2.67×** | `/tmp/tmp.m8X414` |
| `sync` | `f00-sync --core` | 0.863 | **0.336** | **2.57×** | `` |
| `uptime` | `f00-uptime --core` | 1.515 | **0.320** | **4.73×** | `up 4 minutes` |
| `hostid` | `f00-hostid --core` | 0.934 | **0.354** | **2.64×** | `db830370` |
| `logname` | `f00-logname --core` | 0.852 | **0.371** | **2.29×** | `runner` |
| `tty` | `f00-tty --core` | 0.813 | **0.240** | **3.38×** | `not a tty` |
| `groups` | `f00-groups --core` | 0.942 | **0.333** | **2.82×** | `adm users docker systemd-journal runner` |
| `arch` | `f00-arch --core` | 0.820 | **0.303** | **2.71×** | `x86_64` |
| `hostname` | `f00-hostname --core` | 0.572 | **0.301** | **1.90×** | `runnervmvrwv9` |
| `users` | `f00-users --core` | 0.851 | **0.307** | **2.77×** | `` |
| `who` | `f00-who --core` | 0.891 | **0.316** | **2.82×** | `` |
| `pinky` | `f00-pinky --core` | 0.862 | **0.312** | **2.76×** | `` |
| `fold` | `f00-fold --core -w 40 fixture.txt` | 0.898 | **0.388** | **2.31×** | `suite-bench line abcdefghijklmnopqrstuvw xyz 0123456789 suite-bench line abcdefg` |
| `fmt` | `f00-fmt --core -w 40 fixture.txt` | 1.008 | **0.374** | **2.70×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `expand` | `f00-expand --core fixture.txt` | 0.887 | **0.396** | **2.24×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `unexpand` | `f00-unexpand --core fixture.txt` | 0.865 | **0.421** | **2.05×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tac` | `f00-tac --core fixture.txt` | 0.867 | **0.392** | **2.21×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `rev` | `f00-rev --core fixture.txt` | 1.153 | **0.417** | **2.76×** | `9876543210 zyxwvutsrqponmlkjihgfedcba enil hcneb-etius 9876543210 zyxwvutsrqponm` |
| `ptx` | `f00-ptx --core -A fixture.txt` | 1.561 | **0.407** | **3.84×** | `suite bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite bench line abcdefgh` |
| `pr` | `f00-pr --core -t fixture.txt` | 1.066 | **0.389** | **2.74×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `shuf` | `f00-shuf --core fixture.txt` | 0.872 | **0.621** | **1.40×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tsort` | `f00-tsort --core` | 0.851 | **0.336** | **2.53×** | `a b c` |
| `tee` | `f00-tee --core tee.out` | 0.934 | **0.407** | **2.30×** | `tee data tee data tee data tee data tee data tee data tee data tee data tee data` |
| `split` | `f00-split --core -l 50 fixture.txt out` | 1.286 | **1.352** | **0.95×** | `` |
| `csplit` | `f00-csplit --core -f xx fixture 5` | 1.037 | **0.332** | **3.12×** | `` |
| `chmod` | `f00-chmod --core 644 fixture.txt` | 0.837 | **0.314** | **2.66×** | `` |
| `touch` | `f00-touch --core touched` | 0.823 | **0.357** | **2.30×** | `` |
| `truncate` | `f00-truncate --core -s 0 trunc` | 0.856 | **0.332** | **2.58×** | `` |
| `cp` | `f00-cp --core fixture.txt cp.out` | 1.212 | **0.400** | **3.03×** | `` |
| `dd` | `f00-dd --core if=fixture of=dd.out bs=4k count=1` | 0.907 | **0.398** | **2.28×** | `` |
| `install` | `f00-install --core -m 644 fixture inst.out` | 1.247 | **0.413** | **3.02×** | `` |
| `yes` | `f00-yes --core --version` | 0.826 | **0.244** | **3.38×** | `f00-yes (f00) 0.15.7 License: MIT · https://f00.sh` |
| `[` | `f00-[ --core -f fixture.txt` | 0.797 | **0.295** | **2.71×** | `` |

Full machine-readable data: [suite.json](suite.json)

