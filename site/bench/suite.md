# Suite benchmarks (f00 vs GNU coreutils)

Generated: `2026-07-24T13:00:16Z` · N=15 median · warm-cache spawn-inclusive median

Host: x86_64 · Linux 6.17.0-1020-azure

| Tool | Command (f00) | GNU ms | f00 ms | Speedup | Sample output (f00) |
|------|---------------|-------:|-------:|--------:|---------------------|
| `true` | `f00-true --core` | 0.621 | **0.301** | **2.06×** | `` |
| `false` | `f00-false --core` | 0.617 | **0.301** | **2.05×** | `` |
| `basename` | `f00-basename --core /usr/bin/ls` | 0.883 | **0.296** | **2.98×** | `ls` |
| `dirname` | `f00-dirname --core /usr/bin/ls` | 0.836 | **0.253** | **3.31×** | `/usr/bin` |
| `echo` | `f00-echo --core hi` | 0.836 | **0.303** | **2.76×** | `hi` |
| `pwd` | `f00-pwd --core` | 0.824 | **0.306** | **2.69×** | `/home/runner/work/f00/f00` |
| `nproc` | `f00-nproc --core` | 0.833 | **0.252** | **3.30×** | `4` |
| `whoami` | `f00-whoami --core` | 0.899 | **0.260** | **3.46×** | `runner` |
| `uname` | `f00-uname --core -s` | 0.820 | **0.305** | **2.69×** | `Linux` |
| `id` | `f00-id --core -u` | 0.990 | **0.307** | **3.23×** | `1001` |
| `date` | `f00-date --core -u +%Y` | 0.867 | **0.309** | **2.81×** | `2026` |
| `printenv` | `f00-printenv --core PATH` | 0.837 | **0.309** | **2.71×** | `/snap/bin:/home/runner/.local/bin:/opt/pipx_bin:/home/runner/.cargo/bin:/home/ru` |
| `printf` | `f00-printf --core %s world` | 0.819 | **0.308** | **2.66×** | `world` |
| `factor` | `f00-factor --core 12` | 0.882 | **0.313** | **2.82×** | `12: 2 2 3` |
| `numfmt` | `f00-numfmt --core --to=si 1000` | 0.865 | **0.319** | **2.71×** | `1.0k` |
| `expr` | `f00-expr --core 1 + 1` | 0.892 | **0.314** | **2.85×** | `2` |
| `seq` | `f00-seq --core 1 5` | 0.830 | **0.308** | **2.70×** | `1 2 3 4 5` |
| `cat` | `f00-cat --core fixture.txt` | 0.857 | **0.289** | **2.96×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `wc` | `f00-wc --core -l fixture.txt` | 0.871 | **0.381** | **2.29×** | `400 /tmp/f00-suite-bench.dougzecz/fix.txt` |
| `head` | `f00-head --core -n 3 fixture.txt` | 0.845 | **0.312** | **2.71×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tail` | `f00-tail --core -n 3 fixture.txt` | 0.863 | **0.365** | **2.37×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `nl` | `f00-nl --core fixture.txt` | 0.923 | **0.396** | **2.33×** | `1 suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 2 suite-bench line abcd` |
| `od` | `f00-od --core -An -tx1 -N8 fixture.txt` | 0.857 | **0.325** | **2.63×** | `73 75 69 74 65 2d 62 65` |
| `cut` | `f00-cut --core -d: -f1 /etc/passwd` | 0.874 | **0.326** | **2.68×** | `root daemon bin sys sync games man lp mail news uucp proxy www-data backup list ` |
| `tr` | `f00-tr --core a-z A-Z` | 0.870 | **0.343** | **2.54×** | `HELLO` |
| `sort` | `f00-sort --core fixture.txt` | 1.362 | **0.763** | **1.79×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `uniq` | `f00-uniq --core a.txt` | 0.858 | **0.374** | **2.30×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789` |
| `paste` | `f00-paste --core a.txt b.txt` | 0.924 | **0.447** | **2.07×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `comm` | `f00-comm --core -12 a.txt b.txt` | 0.892 | **0.432** | **2.07×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `join` | `f00-join --core a.txt b.txt` | 14.611 | **0.453** | **32.29×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 line abcdefghijklmnopqrst` |
| `base64` | `f00-base64 --core fixture.txt` | 0.887 | **0.441** | **2.01×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `base32` | `f00-base32 --core fixture.txt` | 0.915 | **0.486** | **1.88×** | `ON2WS5DFFVRGK3TDNAQGY2LOMUQGCYTDMRSWMZ3INFVGW3DNNZXXA4LSON2HK5TXPB4XUIBQGEZD GNB` |
| `basenc` | `f00-basenc --core --base64 fixture.txt` | 0.914 | **0.438** | **2.08×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `md5sum` | `f00-md5sum --core fixture.txt` | 1.273 | **0.379** | **3.36×** | `a5e6b1aa8523bc01f561fcef58d16894 /tmp/f00-suite-bench.dougzecz/fix.txt` |
| `sha1sum` | `f00-sha1sum --core fixture.txt` | 1.268 | **0.409** | **3.10×** | `49f92a1f57c1a825b5ca5777c92d6e15ac26a8ea /tmp/f00-suite-bench.dougzecz/fix.txt` |
| `sha224sum` | `f00-sha224sum --core fixture.txt` | 1.244 | **0.442** | **2.81×** | `94c1dff65fd14336129da4555171327a3e1e3e684810af23fa13e7f3 /tmp/f00-suite-bench.do` |
| `sha256sum` | `f00-sha256sum --core fixture.txt` | 1.241 | **0.449** | **2.76×** | `7c28ea8726bc6923d5b38a6b6938ad5907c01dc6640e1645cf63cbf9df706132 /tmp/f00-suite-` |
| `sha384sum` | `f00-sha384sum --core fixture.txt` | 1.270 | **0.413** | **3.08×** | `f2578c293b7eeebf35402906e5e2fcd522b567687241b72950690c389f88baf83f9cd518c1fd67b3` |
| `sha512sum` | `f00-sha512sum --core fixture.txt` | 1.269 | **0.411** | **3.09×** | `a3282daa5cc665baa2b247ee17a0798f1d8028bbfb2107ea0df9493d0e57bfba10ba95d72d2550c2` |
| `b2sum` | `f00-b2sum --core fixture.txt` | 0.885 | **0.398** | **2.22×** | `915149393ea1091e4aa19ad9c68f980ebf83f5da2a576a20ca65fb001c685bf890523b442b840760` |
| `cksum` | `f00-cksum --core fixture.txt` | 1.267 | **0.375** | **3.38×** | `1448063438 22000 /tmp/f00-suite-bench.dougzecz/fix.txt` |
| `sum` | `f00-sum --core fixture.txt` | 0.902 | **0.355** | **2.54×** | `9481 22 /tmp/f00-suite-bench.dougzecz/fix.txt` |
| `ls` | `f00-ls --core -1 dir` | 1.079 | **0.448** | **2.41×** | `f01.txt f02.txt f03.txt f04.txt f05.txt f06.txt f07.txt f08.txt f09.txt f10.txt ` |
| `dir` | `f00-dir --core -1 dir` | 1.062 | **0.326** | **3.26×** | `f06.txt f02.txt f20.txt f14.txt f09.txt f13.txt f10.txt f17.txt f16.txt f08.txt ` |
| `vdir` | `f00-vdir --core -1 dir` | 1.071 | **0.323** | **3.32×** | `- f06.txt - f02.txt - f20.txt - f14.txt - f09.txt - f13.txt - f10.txt - f17.txt ` |
| `stat` | `f00-stat --core -c %s fixture.txt` | 1.016 | **0.309** | **3.29×** | `22000` |
| `realpath` | `f00-realpath --core .` | 0.825 | **0.386** | **2.14×** | `/home/runner/work/f00/f00/asm` |
| `readlink` | `f00-readlink --core /proc/self/exe` | 0.860 | **0.326** | **2.63×** | `/home/runner/work/f00/f00/asm/f00` |
| `df` | `f00-df --core -P /` | 0.968 | **0.358** | **2.71×** | `Filesystem 1K-blocks Used Available Use% Mounted on /dev/root 151263856 59919588` |
| `du` | `f00-du --core -s dir` | 0.912 | **0.364** | **2.51×** | `5 /tmp/f00-suite-bench.dougzecz/dir` |
| `dircolors` | `f00-dircolors --core -p` | 0.817 | **0.301** | **2.71×** | `# Configuration file for dircolors, a utility to help you set the # LS_COLORS en` |
| `env` | `f00-env --core -i true` | 1.222 | **0.309** | **3.96×** | `` |
| `timeout` | `f00-timeout --core 5 true` | 1.519 | **0.427** | **3.56×** | `` |
| `nice` | `f00-nice --core true` | 1.279 | **0.309** | **4.14×** | `` |
| `nohup` | `f00-nohup --core true` | 1.254 | **0.304** | **4.13×** | `` |
| `sleep` | `f00-sleep --core 0` | 0.879 | **0.361** | **2.43×** | `` |
| `test` | `f00-test --core -f fixture.txt` | 0.810 | **0.310** | **2.62×** | `` |
| `pathchk` | `f00-pathchk --core ok-name` | 0.797 | **0.301** | **2.64×** | `` |
| `mktemp` | `f00-mktemp --core -u` | 0.828 | **0.310** | **2.67×** | `/tmp/tmp.H4bjvu` |
| `sync` | `f00-sync --core` | 0.846 | **0.333** | **2.54×** | `` |
| `uptime` | `f00-uptime --core` | 1.533 | **0.322** | **4.76×** | `up 2 minutes` |
| `hostid` | `f00-hostid --core` | 0.944 | **0.360** | **2.62×** | `db830370` |
| `logname` | `f00-logname --core` | 0.838 | **0.375** | **2.24×** | `runner` |
| `tty` | `f00-tty --core` | 0.827 | **0.248** | **3.34×** | `not a tty` |
| `groups` | `f00-groups --core` | 0.931 | **0.328** | **2.84×** | `adm users docker systemd-journal runner` |
| `arch` | `f00-arch --core` | 0.822 | **0.309** | **2.66×** | `x86_64` |
| `hostname` | `f00-hostname --core` | 0.580 | **0.308** | **1.88×** | `runnervmvrwv9` |
| `users` | `f00-users --core` | 0.858 | **0.314** | **2.73×** | `` |
| `who` | `f00-who --core` | 0.855 | **0.308** | **2.78×** | `` |
| `pinky` | `f00-pinky --core` | 0.897 | **0.327** | **2.74×** | `` |
| `fold` | `f00-fold --core -w 40 fixture.txt` | 0.914 | **0.402** | **2.28×** | `suite-bench line abcdefghijklmnopqrstuvw xyz 0123456789 suite-bench line abcdefg` |
| `fmt` | `f00-fmt --core -w 40 fixture.txt` | 0.989 | **0.367** | **2.69×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `expand` | `f00-expand --core fixture.txt` | 0.903 | **0.385** | **2.35×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `unexpand` | `f00-unexpand --core fixture.txt` | 0.863 | **0.426** | **2.02×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tac` | `f00-tac --core fixture.txt` | 0.877 | **0.365** | **2.40×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `rev` | `f00-rev --core fixture.txt` | 1.156 | **0.399** | **2.90×** | `9876543210 zyxwvutsrqponmlkjihgfedcba enil hcneb-etius 9876543210 zyxwvutsrqponm` |
| `ptx` | `f00-ptx --core -A fixture.txt` | 1.576 | **0.415** | **3.80×** | `suite bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite bench line abcdefgh` |
| `pr` | `f00-pr --core -t fixture.txt` | 1.053 | **0.371** | **2.83×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `shuf` | `f00-shuf --core fixture.txt` | 0.879 | **0.596** | **1.47×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tsort` | `f00-tsort --core` | 0.854 | **0.342** | **2.50×** | `a b c` |
| `tee` | `f00-tee --core tee.out` | 0.931 | **0.415** | **2.24×** | `tee data tee data tee data tee data tee data tee data tee data tee data tee data` |
| `split` | `f00-split --core -l 50 fixture.txt out` | 1.309 | **1.420** | **0.92×** | `` |
| `csplit` | `f00-csplit --core -f xx fixture 5` | 1.058 | **0.337** | **3.14×** | `` |
| `chmod` | `f00-chmod --core 644 fixture.txt` | 0.847 | **0.317** | **2.67×** | `` |
| `touch` | `f00-touch --core touched` | 0.837 | **0.368** | **2.28×** | `` |
| `truncate` | `f00-truncate --core -s 0 trunc` | 0.830 | **0.327** | **2.54×** | `` |
| `cp` | `f00-cp --core fixture.txt cp.out` | 1.237 | **0.403** | **3.07×** | `` |
| `dd` | `f00-dd --core if=fixture of=dd.out bs=4k count=1` | 0.914 | **0.381** | **2.40×** | `` |
| `install` | `f00-install --core -m 644 fixture inst.out` | 1.215 | **0.417** | **2.91×** | `` |
| `yes` | `f00-yes --core --version` | 0.830 | **0.242** | **3.43×** | `f00-yes (f00) 0.15.5 License: MIT · https://f00.sh` |
| `[` | `f00-[ --core -f fixture.txt` | 0.812 | **0.298** | **2.72×** | `` |

Full machine-readable data: [suite.json](suite.json)

