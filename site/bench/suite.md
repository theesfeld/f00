# Suite benchmarks (f00 vs GNU coreutils)

Generated: `2026-07-24T12:01:21Z` · N=15 median · warm-cache spawn-inclusive median

Host: x86_64 · Linux 6.17.0-1020-azure

| Tool | Command (f00) | GNU ms | f00 ms | Speedup | Sample output (f00) |
|------|---------------|-------:|-------:|--------:|---------------------|
| `true` | `f00-true --core` | 0.555 | **0.249** | **2.23×** | `` |
| `false` | `f00-false --core` | 0.563 | **0.247** | **2.28×** | `` |
| `basename` | `f00-basename --core /usr/bin/ls` | 0.822 | **0.315** | **2.61×** | `ls` |
| `dirname` | `f00-dirname --core /usr/bin/ls` | 0.881 | **0.288** | **3.05×** | `/usr/bin` |
| `echo` | `f00-echo --core hi` | 0.867 | **0.360** | **2.41×** | `hi` |
| `pwd` | `f00-pwd --core` | 0.872 | **0.348** | **2.50×** | `/home/runner/work/f00/f00` |
| `nproc` | `f00-nproc --core` | 0.882 | **0.282** | **3.12×** | `4` |
| `whoami` | `f00-whoami --core` | 0.889 | **0.256** | **3.48×** | `runner` |
| `uname` | `f00-uname --core -s` | 0.814 | **0.304** | **2.68×** | `Linux` |
| `id` | `f00-id --core -u` | 0.996 | **0.309** | **3.22×** | `1001` |
| `date` | `f00-date --core -u +%Y` | 0.891 | **0.306** | **2.91×** | `2026` |
| `printenv` | `f00-printenv --core PATH` | 0.814 | **0.309** | **2.64×** | `/snap/bin:/home/runner/.local/bin:/opt/pipx_bin:/home/runner/.cargo/bin:/home/ru` |
| `printf` | `f00-printf --core %s world` | 0.833 | **0.300** | **2.78×** | `world` |
| `factor` | `f00-factor --core 12` | 0.874 | **0.315** | **2.77×** | `12: 2 2 3` |
| `numfmt` | `f00-numfmt --core --to=si 1000` | 0.828 | **0.311** | **2.66×** | `1.0k` |
| `expr` | `f00-expr --core 1 + 1` | 0.889 | **0.312** | **2.85×** | `2` |
| `seq` | `f00-seq --core 1 5` | 0.822 | **0.301** | **2.73×** | `1 2 3 4 5` |
| `cat` | `f00-cat --core fixture.txt` | 0.839 | **0.286** | **2.94×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `wc` | `f00-wc --core -l fixture.txt` | 0.858 | **0.342** | **2.51×** | `400 /tmp/f00-suite-bench.27f0dse5/fix.txt` |
| `head` | `f00-head --core -n 3 fixture.txt` | 0.826 | **0.311** | **2.66×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tail` | `f00-tail --core -n 3 fixture.txt` | 0.907 | **0.352** | **2.58×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `nl` | `f00-nl --core fixture.txt` | 0.908 | **0.379** | **2.40×** | `1 suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 2 suite-bench line abcd` |
| `od` | `f00-od --core -An -tx1 -N8 fixture.txt` | 0.850 | **0.322** | **2.64×** | `73 75 69 74 65 2d 62 65` |
| `cut` | `f00-cut --core -d: -f1 /etc/passwd` | 0.853 | **0.324** | **2.63×** | `root daemon bin sys sync games man lp mail news uucp proxy www-data backup list ` |
| `tr` | `f00-tr --core a-z A-Z` | 0.858 | **0.327** | **2.62×** | `HELLO` |
| `sort` | `f00-sort --core fixture.txt` | 1.348 | **0.783** | **1.72×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `uniq` | `f00-uniq --core a.txt` | 0.856 | **0.354** | **2.42×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789` |
| `paste` | `f00-paste --core a.txt b.txt` | 0.930 | **0.456** | **2.04×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `comm` | `f00-comm --core -12 a.txt b.txt` | 0.909 | **0.453** | **2.01×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `join` | `f00-join --core a.txt b.txt` | 15.058 | **0.456** | **33.06×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 line abcdefghijklmnopqrst` |
| `base64` | `f00-base64 --core fixture.txt` | 0.892 | **0.464** | **1.92×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `base32` | `f00-base32 --core fixture.txt` | 0.897 | **0.487** | **1.84×** | `ON2WS5DFFVRGK3TDNAQGY2LOMUQGCYTDMRSWMZ3INFVGW3DNNZXXA4LSON2HK5TXPB4XUIBQGEZD GNB` |
| `basenc` | `f00-basenc --core --base64 fixture.txt` | 0.890 | **0.504** | **1.77×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `md5sum` | `f00-md5sum --core fixture.txt` | 1.274 | **0.387** | **3.29×** | `a5e6b1aa8523bc01f561fcef58d16894 /tmp/f00-suite-bench.27f0dse5/fix.txt` |
| `sha1sum` | `f00-sha1sum --core fixture.txt` | 1.238 | **0.400** | **3.09×** | `49f92a1f57c1a825b5ca5777c92d6e15ac26a8ea /tmp/f00-suite-bench.27f0dse5/fix.txt` |
| `sha224sum` | `f00-sha224sum --core fixture.txt` | 1.231 | **0.441** | **2.79×** | `94c1dff65fd14336129da4555171327a3e1e3e684810af23fa13e7f3 /tmp/f00-suite-bench.27` |
| `sha256sum` | `f00-sha256sum --core fixture.txt` | 1.230 | **0.439** | **2.80×** | `7c28ea8726bc6923d5b38a6b6938ad5907c01dc6640e1645cf63cbf9df706132 /tmp/f00-suite-` |
| `sha384sum` | `f00-sha384sum --core fixture.txt` | 1.253 | **0.410** | **3.06×** | `f2578c293b7eeebf35402906e5e2fcd522b567687241b72950690c389f88baf83f9cd518c1fd67b3` |
| `sha512sum` | `f00-sha512sum --core fixture.txt` | 1.263 | **0.423** | **2.99×** | `a3282daa5cc665baa2b247ee17a0798f1d8028bbfb2107ea0df9493d0e57bfba10ba95d72d2550c2` |
| `b2sum` | `f00-b2sum --core fixture.txt` | 0.888 | **0.396** | **2.24×** | `915149393ea1091e4aa19ad9c68f980ebf83f5da2a576a20ca65fb001c685bf890523b442b840760` |
| `cksum` | `f00-cksum --core fixture.txt` | 1.260 | **0.432** | **2.91×** | `1448063438 22000 /tmp/f00-suite-bench.27f0dse5/fix.txt` |
| `sum` | `f00-sum --core fixture.txt` | 0.880 | **0.350** | **2.52×** | `9481 22 /tmp/f00-suite-bench.27f0dse5/fix.txt` |
| `ls` | `f00-ls --core -1 dir` | 1.051 | **0.415** | **2.53×** | `f01.txt f02.txt f03.txt f04.txt f05.txt f06.txt f07.txt f08.txt f09.txt f10.txt ` |
| `dir` | `f00-dir --core -1 dir` | 1.042 | **0.321** | **3.25×** | `f06.txt f02.txt f20.txt f14.txt f09.txt f13.txt f10.txt f17.txt f16.txt f08.txt ` |
| `vdir` | `f00-vdir --core -1 dir` | 1.045 | **0.318** | **3.28×** | `- f06.txt - f02.txt - f20.txt - f14.txt - f09.txt - f13.txt - f10.txt - f17.txt ` |
| `stat` | `f00-stat --core -c %s fixture.txt` | 1.001 | **0.302** | **3.31×** | `22000` |
| `realpath` | `f00-realpath --core .` | 0.814 | **0.382** | **2.13×** | `/home/runner/work/f00/f00/asm` |
| `readlink` | `f00-readlink --core /proc/self/exe` | 0.848 | **0.320** | **2.65×** | `/home/runner/work/f00/f00/asm/f00` |
| `df` | `f00-df --core -P /` | 0.968 | **0.372** | **2.61×** | `Filesystem 1K-blocks Used Available Use% Mounted on /dev/root 151263856 59920800` |
| `du` | `f00-du --core -s dir` | 0.907 | **0.365** | **2.49×** | `5 /tmp/f00-suite-bench.27f0dse5/dir` |
| `dircolors` | `f00-dircolors --core -p` | 0.832 | **0.303** | **2.74×** | `# Configuration file for dircolors, a utility to help you set the # LS_COLORS en` |
| `env` | `f00-env --core -i true` | 1.203 | **0.315** | **3.82×** | `` |
| `timeout` | `f00-timeout --core 5 true` | 1.533 | **0.426** | **3.60×** | `` |
| `nice` | `f00-nice --core true` | 1.253 | **0.308** | **4.07×** | `` |
| `nohup` | `f00-nohup --core true` | 1.257 | **0.307** | **4.10×** | `` |
| `sleep` | `f00-sleep --core 0` | 0.884 | **0.387** | **2.28×** | `` |
| `test` | `f00-test --core -f fixture.txt` | 0.818 | **0.306** | **2.67×** | `` |
| `pathchk` | `f00-pathchk --core ok-name` | 0.877 | **0.310** | **2.83×** | `` |
| `mktemp` | `f00-mktemp --core -u` | 0.832 | **0.305** | **2.73×** | `/tmp/tmp.4mDbnj` |
| `sync` | `f00-sync --core` | 0.854 | **0.345** | **2.48×** | `` |
| `uptime` | `f00-uptime --core` | 1.492 | **0.335** | **4.45×** | `up 0 minutes` |
| `hostid` | `f00-hostid --core` | 0.930 | **0.362** | **2.57×** | `db830370` |
| `logname` | `f00-logname --core` | 0.867 | **0.365** | **2.37×** | `runner` |
| `tty` | `f00-tty --core` | 0.811 | **0.240** | **3.38×** | `not a tty` |
| `groups` | `f00-groups --core` | 0.928 | **0.320** | **2.90×** | `adm users docker systemd-journal runner` |
| `arch` | `f00-arch --core` | 0.797 | **0.306** | **2.61×** | `x86_64` |
| `hostname` | `f00-hostname --core` | 0.582 | **0.295** | **1.97×** | `runnervmvrwv9` |
| `users` | `f00-users --core` | 0.833 | **0.305** | **2.73×** | `` |
| `who` | `f00-who --core` | 0.846 | **0.304** | **2.78×** | `` |
| `pinky` | `f00-pinky --core` | 0.846 | **0.305** | **2.77×** | `` |
| `fold` | `f00-fold --core -w 40 fixture.txt` | 0.884 | **0.390** | **2.26×** | `suite-bench line abcdefghijklmnopqrstuvw xyz 0123456789 suite-bench line abcdefg` |
| `fmt` | `f00-fmt --core -w 40 fixture.txt` | 0.980 | **0.362** | **2.70×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `expand` | `f00-expand --core fixture.txt` | 0.884 | **0.382** | **2.32×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `unexpand` | `f00-unexpand --core fixture.txt` | 0.855 | **0.426** | **2.01×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tac` | `f00-tac --core fixture.txt` | 0.857 | **0.362** | **2.37×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `rev` | `f00-rev --core fixture.txt` | 1.141 | **0.396** | **2.88×** | `9876543210 zyxwvutsrqponmlkjihgfedcba enil hcneb-etius 9876543210 zyxwvutsrqponm` |
| `ptx` | `f00-ptx --core -A fixture.txt` | 1.549 | **0.407** | **3.81×** | `suite bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite bench line abcdefgh` |
| `pr` | `f00-pr --core -t fixture.txt` | 1.035 | **0.362** | **2.86×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `shuf` | `f00-shuf --core fixture.txt` | 0.854 | **0.583** | **1.47×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tsort` | `f00-tsort --core` | 0.831 | **0.326** | **2.55×** | `a b c` |
| `tee` | `f00-tee --core tee.out` | 0.940 | **0.406** | **2.32×** | `tee data tee data tee data tee data tee data tee data tee data tee data tee data` |
| `split` | `f00-split --core -l 50 fixture.txt out` | 1.285 | **1.397** | **0.92×** | `` |
| `csplit` | `f00-csplit --core -f xx fixture 5` | 1.057 | **0.329** | **3.21×** | `` |
| `chmod` | `f00-chmod --core 644 fixture.txt` | 0.843 | **0.320** | **2.64×** | `` |
| `touch` | `f00-touch --core touched` | 0.829 | **0.362** | **2.29×** | `` |
| `truncate` | `f00-truncate --core -s 0 trunc` | 0.833 | **0.321** | **2.60×** | `` |
| `cp` | `f00-cp --core fixture.txt cp.out` | 1.238 | **0.411** | **3.01×** | `` |
| `dd` | `f00-dd --core if=fixture of=dd.out bs=4k count=1` | 0.916 | **0.385** | **2.38×** | `` |
| `install` | `f00-install --core -m 644 fixture inst.out` | 1.254 | **0.428** | **2.93×** | `` |
| `yes` | `f00-yes --core --version` | 0.837 | **0.247** | **3.38×** | `f00-yes (f00) 0.15.2 License: MIT · https://f00.sh` |
| `[` | `f00-[ --core -f fixture.txt` | 0.819 | **0.299** | **2.74×** | `` |

Full machine-readable data: [suite.json](suite.json)

