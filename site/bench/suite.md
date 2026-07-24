# Suite benchmarks (f00 vs GNU coreutils)

**Overall: 2.7× faster than GNU coreutils overall** (166% faster overall; geo mean of per-tool speedups)

Generated: `2026-07-24T14:28:05Z` · N=15 median · warm-cache spawn-inclusive median

Host: x86_64 · Linux 6.17.0-1020-azure

Tools timed: 91 · wins: 90 · median 2.63× · total-time 2.949×

| Tool | Command (f00) | GNU ms | f00 ms | Speedup | Sample output (f00) |
|------|---------------|-------:|-------:|--------:|---------------------|
| `true` | `f00-true --core` | 0.552 | **0.256** | **2.15×** | `` |
| `false` | `f00-false --core` | 0.538 | **0.246** | **2.19×** | `` |
| `basename` | `f00-basename --core /usr/bin/ls` | 0.814 | **0.260** | **3.13×** | `ls` |
| `dirname` | `f00-dirname --core /usr/bin/ls` | 0.808 | **0.298** | **2.72×** | `/usr/bin` |
| `echo` | `f00-echo --core hi` | 0.865 | **0.374** | **2.32×** | `hi` |
| `pwd` | `f00-pwd --core` | 0.874 | **0.357** | **2.45×** | `/home/runner/work/f00/f00` |
| `nproc` | `f00-nproc --core` | 0.895 | **0.263** | **3.40×** | `4` |
| `whoami` | `f00-whoami --core` | 0.897 | **0.266** | **3.38×** | `runner` |
| `uname` | `f00-uname --core -s` | 0.813 | **0.306** | **2.66×** | `Linux` |
| `id` | `f00-id --core -u` | 0.972 | **0.309** | **3.15×** | `1001` |
| `date` | `f00-date --core -u +%Y` | 0.847 | **0.310** | **2.73×** | `2026` |
| `printenv` | `f00-printenv --core PATH` | 0.819 | **0.316** | **2.59×** | `/snap/bin:/home/runner/.local/bin:/opt/pipx_bin:/home/runner/.cargo/bin:/home/ru` |
| `printf` | `f00-printf --core %s world` | 0.825 | **0.307** | **2.69×** | `world` |
| `factor` | `f00-factor --core 12` | 0.857 | **0.308** | **2.78×** | `12: 2 2 3` |
| `numfmt` | `f00-numfmt --core --to=si 1000` | 0.825 | **0.314** | **2.63×** | `1.0k` |
| `expr` | `f00-expr --core 1 + 1` | 0.884 | **0.311** | **2.84×** | `2` |
| `seq` | `f00-seq --core 1 5` | 0.816 | **0.306** | **2.66×** | `1 2 3 4 5` |
| `cat` | `f00-cat --core fixture.txt` | 0.834 | **0.292** | **2.86×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `wc` | `f00-wc --core -l fixture.txt` | 0.863 | **0.359** | **2.40×** | `400 /tmp/f00-suite-bench.4fps3b14/fix.txt` |
| `head` | `f00-head --core -n 3 fixture.txt` | 0.847 | **0.321** | **2.64×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tail` | `f00-tail --core -n 3 fixture.txt` | 0.843 | **0.356** | **2.37×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `nl` | `f00-nl --core fixture.txt` | 0.895 | **0.401** | **2.23×** | `1 suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 2 suite-bench line abcd` |
| `od` | `f00-od --core -An -tx1 -N8 fixture.txt` | 0.847 | **0.318** | **2.66×** | `73 75 69 74 65 2d 62 65` |
| `cut` | `f00-cut --core -d: -f1 /etc/passwd` | 0.832 | **0.317** | **2.63×** | `root daemon bin sys sync games man lp mail news uucp proxy www-data backup list ` |
| `tr` | `f00-tr --core a-z A-Z` | 0.851 | **0.329** | **2.58×** | `HELLO` |
| `sort` | `f00-sort --core fixture.txt` | 1.356 | **0.868** | **1.56×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `uniq` | `f00-uniq --core a.txt` | 0.844 | **0.372** | **2.27×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789` |
| `paste` | `f00-paste --core a.txt b.txt` | 0.920 | **0.455** | **2.02×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `comm` | `f00-comm --core -12 a.txt b.txt` | 0.898 | **0.437** | **2.06×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `join` | `f00-join --core a.txt b.txt` | 15.019 | **0.465** | **32.30×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 line abcdefghijklmnopqrst` |
| `base64` | `f00-base64 --core fixture.txt` | 0.894 | **0.475** | **1.88×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `base32` | `f00-base32 --core fixture.txt` | 0.894 | **0.514** | **1.74×** | `ON2WS5DFFVRGK3TDNAQGY2LOMUQGCYTDMRSWMZ3INFVGW3DNNZXXA4LSON2HK5TXPB4XUIBQGEZD GNB` |
| `basenc` | `f00-basenc --core --base64 fixture.txt` | 0.886 | **0.473** | **1.87×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `md5sum` | `f00-md5sum --core fixture.txt` | 1.262 | **0.386** | **3.27×** | `a5e6b1aa8523bc01f561fcef58d16894 /tmp/f00-suite-bench.4fps3b14/fix.txt` |
| `sha1sum` | `f00-sha1sum --core fixture.txt` | 1.242 | **0.422** | **2.94×** | `49f92a1f57c1a825b5ca5777c92d6e15ac26a8ea /tmp/f00-suite-bench.4fps3b14/fix.txt` |
| `sha224sum` | `f00-sha224sum --core fixture.txt` | 1.215 | **0.449** | **2.70×** | `94c1dff65fd14336129da4555171327a3e1e3e684810af23fa13e7f3 /tmp/f00-suite-bench.4f` |
| `sha256sum` | `f00-sha256sum --core fixture.txt` | 1.239 | **0.449** | **2.76×** | `7c28ea8726bc6923d5b38a6b6938ad5907c01dc6640e1645cf63cbf9df706132 /tmp/f00-suite-` |
| `sha384sum` | `f00-sha384sum --core fixture.txt` | 1.264 | **0.417** | **3.03×** | `f2578c293b7eeebf35402906e5e2fcd522b567687241b72950690c389f88baf83f9cd518c1fd67b3` |
| `sha512sum` | `f00-sha512sum --core fixture.txt` | 1.240 | **0.415** | **2.99×** | `a3282daa5cc665baa2b247ee17a0798f1d8028bbfb2107ea0df9493d0e57bfba10ba95d72d2550c2` |
| `b2sum` | `f00-b2sum --core fixture.txt` | 0.884 | **0.398** | **2.22×** | `915149393ea1091e4aa19ad9c68f980ebf83f5da2a576a20ca65fb001c685bf890523b442b840760` |
| `cksum` | `f00-cksum --core fixture.txt` | 1.228 | **0.376** | **3.27×** | `1448063438 22000 /tmp/f00-suite-bench.4fps3b14/fix.txt` |
| `sum` | `f00-sum --core fixture.txt` | 0.873 | **0.357** | **2.44×** | `9481 22 /tmp/f00-suite-bench.4fps3b14/fix.txt` |
| `ls` | `f00-ls --core -1 dir` | 1.036 | **0.418** | **2.48×** | `f01.txt f02.txt f03.txt f04.txt f05.txt f06.txt f07.txt f08.txt f09.txt f10.txt ` |
| `dir` | `f00-dir --core -1 dir` | 1.055 | **0.330** | **3.20×** | `f06.txt f02.txt f20.txt f14.txt f09.txt f13.txt f10.txt f17.txt f16.txt f08.txt ` |
| `vdir` | `f00-vdir --core -1 dir` | 1.044 | **0.323** | **3.23×** | `- f06.txt - f02.txt - f20.txt - f14.txt - f09.txt - f13.txt - f10.txt - f17.txt ` |
| `stat` | `f00-stat --core -c %s fixture.txt` | 0.999 | **0.317** | **3.15×** | `22000` |
| `realpath` | `f00-realpath --core .` | 0.832 | **0.382** | **2.18×** | `/home/runner/work/f00/f00/asm` |
| `readlink` | `f00-readlink --core /proc/self/exe` | 0.838 | **0.376** | **2.23×** | `/home/runner/work/f00/f00/asm/f00` |
| `df` | `f00-df --core -P /` | 0.951 | **0.363** | **2.62×** | `Filesystem 1K-blocks Used Available Use% Mounted on /dev/root 151263856 59009096` |
| `du` | `f00-du --core -s dir` | 0.909 | **0.363** | **2.51×** | `5 /tmp/f00-suite-bench.4fps3b14/dir` |
| `dircolors` | `f00-dircolors --core -p` | 0.808 | **0.307** | **2.64×** | `# Configuration file for dircolors, a utility to help you set the # LS_COLORS en` |
| `env` | `f00-env --core -i true` | 1.179 | **0.321** | **3.67×** | `` |
| `timeout` | `f00-timeout --core 5 true` | 1.524 | **0.424** | **3.59×** | `` |
| `nice` | `f00-nice --core true` | 1.250 | **0.315** | **3.96×** | `` |
| `nohup` | `f00-nohup --core true` | 1.265 | **0.309** | **4.09×** | `` |
| `sleep` | `f00-sleep --core 0` | 0.890 | **0.365** | **2.43×** | `` |
| `test` | `f00-test --core -f fixture.txt` | 0.797 | **0.306** | **2.60×** | `` |
| `pathchk` | `f00-pathchk --core ok-name` | 0.803 | **0.321** | **2.50×** | `` |
| `mktemp` | `f00-mktemp --core -u` | 0.823 | **0.358** | **2.30×** | `/tmp/tmp.5W89Tf` |
| `sync` | `f00-sync --core` | 0.850 | **0.339** | **2.51×** | `` |
| `uptime` | `f00-uptime --core` | 1.517 | **0.321** | **4.73×** | `up 0 minutes` |
| `hostid` | `f00-hostid --core` | 0.927 | **0.361** | **2.57×** | `db830370` |
| `logname` | `f00-logname --core` | 0.839 | **0.373** | **2.25×** | `runner` |
| `tty` | `f00-tty --core` | 0.830 | **0.249** | **3.33×** | `not a tty` |
| `groups` | `f00-groups --core` | 0.931 | **0.335** | **2.78×** | `adm users docker systemd-journal runner` |
| `arch` | `f00-arch --core` | 0.808 | **0.307** | **2.63×** | `x86_64` |
| `hostname` | `f00-hostname --core` | 0.562 | **0.322** | **1.74×** | `runnervmvrwv9` |
| `users` | `f00-users --core` | 0.850 | **0.319** | **2.67×** | `` |
| `who` | `f00-who --core` | 0.863 | **0.329** | **2.63×** | `` |
| `pinky` | `f00-pinky --core` | 0.867 | **0.315** | **2.76×** | `` |
| `fold` | `f00-fold --core -w 40 fixture.txt` | 0.891 | **0.391** | **2.28×** | `suite-bench line abcdefghijklmnopqrstuvw xyz 0123456789 suite-bench line abcdefg` |
| `fmt` | `f00-fmt --core -w 40 fixture.txt` | 0.988 | **0.366** | **2.70×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `expand` | `f00-expand --core fixture.txt` | 0.882 | **0.377** | **2.34×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `unexpand` | `f00-unexpand --core fixture.txt` | 0.862 | **0.439** | **1.96×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tac` | `f00-tac --core fixture.txt` | 0.866 | **0.383** | **2.26×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `rev` | `f00-rev --core fixture.txt` | 1.140 | **0.407** | **2.80×** | `9876543210 zyxwvutsrqponmlkjihgfedcba enil hcneb-etius 9876543210 zyxwvutsrqponm` |
| `ptx` | `f00-ptx --core -A fixture.txt` | 1.583 | **0.412** | **3.84×** | `suite bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite bench line abcdefgh` |
| `pr` | `f00-pr --core -t fixture.txt` | 1.052 | **0.382** | **2.75×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `shuf` | `f00-shuf --core fixture.txt` | 0.883 | **0.615** | **1.44×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tsort` | `f00-tsort --core` | 0.854 | **0.329** | **2.60×** | `a b c` |
| `tee` | `f00-tee --core tee.out` | 0.928 | **0.413** | **2.25×** | `tee data tee data tee data tee data tee data tee data tee data tee data tee data` |
| `split` | `f00-split --core -l 50 fixture.txt out` | 1.278 | **1.425** | **0.90×** | `` |
| `csplit` | `f00-csplit --core -f xx fixture 5` | 1.045 | **0.328** | **3.19×** | `` |
| `chmod` | `f00-chmod --core 644 fixture.txt` | 0.832 | **0.318** | **2.62×** | `` |
| `touch` | `f00-touch --core touched` | 0.830 | **0.370** | **2.25×** | `` |
| `truncate` | `f00-truncate --core -s 0 trunc` | 0.838 | **0.335** | **2.50×** | `` |
| `cp` | `f00-cp --core fixture.txt cp.out` | 1.208 | **0.418** | **2.89×** | `` |
| `dd` | `f00-dd --core if=fixture of=dd.out bs=4k count=1` | 0.922 | **0.383** | **2.41×** | `` |
| `install` | `f00-install --core -m 644 fixture inst.out` | 1.227 | **0.410** | **2.99×** | `` |
| `yes` | `f00-yes --core --version` | 0.811 | **0.246** | **3.30×** | `f00-yes (f00) 0.15.9 License: MIT · https://f00.sh` |
| `[` | `f00-[ --core -f fixture.txt` | 0.802 | **0.316** | **2.54×** | `` |

Full machine-readable data: [suite.json](suite.json)

