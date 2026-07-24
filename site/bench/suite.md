# Suite benchmarks (f00 vs GNU coreutils)

**Overall: 2.7× faster than GNU coreutils overall** (168% faster overall; geo mean of per-tool speedups)

Generated: `2026-07-24T14:15:55Z` · N=15 median · warm-cache spawn-inclusive median

Host: x86_64 · Linux 6.17.0-1020-azure

Tools timed: 91 · wins: 90 · median 2.63× · total-time 2.986×

| Tool | Command (f00) | GNU ms | f00 ms | Speedup | Sample output (f00) |
|------|---------------|-------:|-------:|--------:|---------------------|
| `true` | `f00-true --core` | 0.543 | **0.245** | **2.21×** | `` |
| `false` | `f00-false --core` | 0.539 | **0.249** | **2.17×** | `` |
| `basename` | `f00-basename --core /usr/bin/ls` | 0.839 | **0.256** | **3.27×** | `ls` |
| `dirname` | `f00-dirname --core /usr/bin/ls` | 0.802 | **0.247** | **3.25×** | `/usr/bin` |
| `echo` | `f00-echo --core hi` | 0.810 | **0.337** | **2.40×** | `hi` |
| `pwd` | `f00-pwd --core` | 0.872 | **0.367** | **2.38×** | `/home/runner/work/f00/f00` |
| `nproc` | `f00-nproc --core` | 0.832 | **0.254** | **3.28×** | `4` |
| `whoami` | `f00-whoami --core` | 0.954 | **0.315** | **3.02×** | `runner` |
| `uname` | `f00-uname --core -s` | 0.832 | **0.316** | **2.63×** | `Linux` |
| `id` | `f00-id --core -u` | 1.008 | **0.319** | **3.16×** | `1001` |
| `date` | `f00-date --core -u +%Y` | 0.856 | **0.322** | **2.66×** | `2026` |
| `printenv` | `f00-printenv --core PATH` | 0.810 | **0.315** | **2.57×** | `/snap/bin:/home/runner/.local/bin:/opt/pipx_bin:/home/runner/.cargo/bin:/home/ru` |
| `printf` | `f00-printf --core %s world` | 0.821 | **0.308** | **2.67×** | `world` |
| `factor` | `f00-factor --core 12` | 0.876 | **0.304** | **2.88×** | `12: 2 2 3` |
| `numfmt` | `f00-numfmt --core --to=si 1000` | 0.828 | **0.302** | **2.74×** | `1.0k` |
| `expr` | `f00-expr --core 1 + 1` | 0.894 | **0.302** | **2.96×** | `2` |
| `seq` | `f00-seq --core 1 5` | 0.830 | **0.366** | **2.27×** | `1 2 3 4 5` |
| `cat` | `f00-cat --core fixture.txt` | 0.851 | **0.293** | **2.91×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `wc` | `f00-wc --core -l fixture.txt` | 0.850 | **0.349** | **2.44×** | `400 /tmp/f00-suite-bench.ozstnps0/fix.txt` |
| `head` | `f00-head --core -n 3 fixture.txt` | 0.830 | **0.313** | **2.65×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tail` | `f00-tail --core -n 3 fixture.txt` | 0.866 | **0.355** | **2.43×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `nl` | `f00-nl --core fixture.txt` | 0.892 | **0.369** | **2.42×** | `1 suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 2 suite-bench line abcd` |
| `od` | `f00-od --core -An -tx1 -N8 fixture.txt` | 0.842 | **0.319** | **2.64×** | `73 75 69 74 65 2d 62 65` |
| `cut` | `f00-cut --core -d: -f1 /etc/passwd` | 0.828 | **0.312** | **2.66×** | `root daemon bin sys sync games man lp mail news uucp proxy www-data backup list ` |
| `tr` | `f00-tr --core a-z A-Z` | 0.834 | **0.324** | **2.57×** | `HELLO` |
| `sort` | `f00-sort --core fixture.txt` | 1.348 | **0.751** | **1.79×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `uniq` | `f00-uniq --core a.txt` | 0.858 | **0.355** | **2.41×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789` |
| `paste` | `f00-paste --core a.txt b.txt` | 0.934 | **0.467** | **2.00×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `comm` | `f00-comm --core -12 a.txt b.txt` | 0.912 | **0.461** | **1.98×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `join` | `f00-join --core a.txt b.txt` | 15.032 | **0.468** | **32.13×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 line abcdefghijklmnopqrst` |
| `base64` | `f00-base64 --core fixture.txt` | 0.893 | **0.481** | **1.86×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `base32` | `f00-base32 --core fixture.txt` | 0.889 | **0.502** | **1.77×** | `ON2WS5DFFVRGK3TDNAQGY2LOMUQGCYTDMRSWMZ3INFVGW3DNNZXXA4LSON2HK5TXPB4XUIBQGEZD GNB` |
| `basenc` | `f00-basenc --core --base64 fixture.txt` | 0.894 | **0.476** | **1.88×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `md5sum` | `f00-md5sum --core fixture.txt` | 1.269 | **0.381** | **3.33×** | `a5e6b1aa8523bc01f561fcef58d16894 /tmp/f00-suite-bench.ozstnps0/fix.txt` |
| `sha1sum` | `f00-sha1sum --core fixture.txt` | 1.230 | **0.404** | **3.05×** | `49f92a1f57c1a825b5ca5777c92d6e15ac26a8ea /tmp/f00-suite-bench.ozstnps0/fix.txt` |
| `sha224sum` | `f00-sha224sum --core fixture.txt` | 1.230 | **0.447** | **2.75×** | `94c1dff65fd14336129da4555171327a3e1e3e684810af23fa13e7f3 /tmp/f00-suite-bench.oz` |
| `sha256sum` | `f00-sha256sum --core fixture.txt` | 1.236 | **0.441** | **2.80×** | `7c28ea8726bc6923d5b38a6b6938ad5907c01dc6640e1645cf63cbf9df706132 /tmp/f00-suite-` |
| `sha384sum` | `f00-sha384sum --core fixture.txt` | 1.251 | **0.406** | **3.08×** | `f2578c293b7eeebf35402906e5e2fcd522b567687241b72950690c389f88baf83f9cd518c1fd67b3` |
| `sha512sum` | `f00-sha512sum --core fixture.txt` | 1.243 | **0.409** | **3.04×** | `a3282daa5cc665baa2b247ee17a0798f1d8028bbfb2107ea0df9493d0e57bfba10ba95d72d2550c2` |
| `b2sum` | `f00-b2sum --core fixture.txt` | 0.881 | **0.399** | **2.21×** | `915149393ea1091e4aa19ad9c68f980ebf83f5da2a576a20ca65fb001c685bf890523b442b840760` |
| `cksum` | `f00-cksum --core fixture.txt` | 1.254 | **0.377** | **3.33×** | `1448063438 22000 /tmp/f00-suite-bench.ozstnps0/fix.txt` |
| `sum` | `f00-sum --core fixture.txt` | 0.879 | **0.347** | **2.53×** | `9481 22 /tmp/f00-suite-bench.ozstnps0/fix.txt` |
| `ls` | `f00-ls --core -1 dir` | 1.062 | **0.420** | **2.53×** | `f01.txt f02.txt f03.txt f04.txt f05.txt f06.txt f07.txt f08.txt f09.txt f10.txt ` |
| `dir` | `f00-dir --core -1 dir` | 1.039 | **0.316** | **3.29×** | `f06.txt f02.txt f20.txt f14.txt f09.txt f13.txt f10.txt f17.txt f16.txt f08.txt ` |
| `vdir` | `f00-vdir --core -1 dir` | 1.041 | **0.321** | **3.24×** | `- f06.txt - f02.txt - f20.txt - f14.txt - f09.txt - f13.txt - f10.txt - f17.txt ` |
| `stat` | `f00-stat --core -c %s fixture.txt` | 1.000 | **0.305** | **3.28×** | `22000` |
| `realpath` | `f00-realpath --core .` | 0.808 | **0.383** | **2.11×** | `/home/runner/work/f00/f00/asm` |
| `readlink` | `f00-readlink --core /proc/self/exe` | 0.820 | **0.386** | **2.12×** | `/home/runner/work/f00/f00/asm/f00` |
| `df` | `f00-df --core -P /` | 0.954 | **0.363** | **2.63×** | `Filesystem 1K-blocks Used Available Use% Mounted on /dev/root 151263856 59008924` |
| `du` | `f00-du --core -s dir` | 0.906 | **0.367** | **2.47×** | `5 /tmp/f00-suite-bench.ozstnps0/dir` |
| `dircolors` | `f00-dircolors --core -p` | 0.826 | **0.314** | **2.63×** | `# Configuration file for dircolors, a utility to help you set the # LS_COLORS en` |
| `env` | `f00-env --core -i true` | 1.200 | **0.309** | **3.89×** | `` |
| `timeout` | `f00-timeout --core 5 true` | 1.489 | **0.424** | **3.51×** | `` |
| `nice` | `f00-nice --core true` | 1.258 | **0.334** | **3.77×** | `` |
| `nohup` | `f00-nohup --core true` | 1.265 | **0.305** | **4.14×** | `` |
| `sleep` | `f00-sleep --core 0` | 0.891 | **0.371** | **2.40×** | `` |
| `test` | `f00-test --core -f fixture.txt` | 0.813 | **0.302** | **2.69×** | `` |
| `pathchk` | `f00-pathchk --core ok-name` | 0.796 | **0.303** | **2.63×** | `` |
| `mktemp` | `f00-mktemp --core -u` | 0.829 | **0.359** | **2.31×** | `/tmp/tmp.ryim9f` |
| `sync` | `f00-sync --core` | 0.835 | **0.329** | **2.54×** | `` |
| `uptime` | `f00-uptime --core` | 1.480 | **0.331** | **4.48×** | `up 1 minute` |
| `hostid` | `f00-hostid --core` | 0.914 | **0.352** | **2.59×** | `db830370` |
| `logname` | `f00-logname --core` | 0.817 | **0.359** | **2.27×** | `runner` |
| `tty` | `f00-tty --core` | 0.798 | **0.244** | **3.27×** | `not a tty` |
| `groups` | `f00-groups --core` | 0.946 | **0.328** | **2.88×** | `adm users docker systemd-journal runner` |
| `arch` | `f00-arch --core` | 0.797 | **0.303** | **2.63×** | `x86_64` |
| `hostname` | `f00-hostname --core` | 0.567 | **0.304** | **1.87×** | `runnervmvrwv9` |
| `users` | `f00-users --core` | 0.861 | **0.313** | **2.76×** | `` |
| `who` | `f00-who --core` | 0.853 | **0.310** | **2.75×** | `` |
| `pinky` | `f00-pinky --core` | 0.853 | **0.316** | **2.70×** | `` |
| `fold` | `f00-fold --core -w 40 fixture.txt` | 0.913 | **0.381** | **2.40×** | `suite-bench line abcdefghijklmnopqrstuvw xyz 0123456789 suite-bench line abcdefg` |
| `fmt` | `f00-fmt --core -w 40 fixture.txt` | 0.962 | **0.355** | **2.71×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `expand` | `f00-expand --core fixture.txt` | 0.879 | **0.385** | **2.28×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `unexpand` | `f00-unexpand --core fixture.txt` | 0.850 | **0.412** | **2.06×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tac` | `f00-tac --core fixture.txt` | 0.852 | **0.371** | **2.30×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `rev` | `f00-rev --core fixture.txt` | 1.154 | **0.408** | **2.83×** | `9876543210 zyxwvutsrqponmlkjihgfedcba enil hcneb-etius 9876543210 zyxwvutsrqponm` |
| `ptx` | `f00-ptx --core -A fixture.txt` | 1.567 | **0.398** | **3.94×** | `suite bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite bench line abcdefgh` |
| `pr` | `f00-pr --core -t fixture.txt` | 1.055 | **0.368** | **2.86×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `shuf` | `f00-shuf --core fixture.txt` | 0.869 | **0.590** | **1.47×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tsort` | `f00-tsort --core` | 0.842 | **0.325** | **2.59×** | `a b c` |
| `tee` | `f00-tee --core tee.out` | 0.911 | **0.411** | **2.22×** | `tee data tee data tee data tee data tee data tee data tee data tee data tee data` |
| `split` | `f00-split --core -l 50 fixture.txt out` | 1.207 | **1.332** | **0.91×** | `` |
| `csplit` | `f00-csplit --core -f xx fixture 5` | 1.029 | **0.327** | **3.15×** | `` |
| `chmod` | `f00-chmod --core 644 fixture.txt` | 0.809 | **0.315** | **2.57×** | `` |
| `touch` | `f00-touch --core touched` | 0.808 | **0.366** | **2.21×** | `` |
| `truncate` | `f00-truncate --core -s 0 trunc` | 0.815 | **0.322** | **2.53×** | `` |
| `cp` | `f00-cp --core fixture.txt cp.out` | 1.232 | **0.421** | **2.93×** | `` |
| `dd` | `f00-dd --core if=fixture of=dd.out bs=4k count=1` | 0.947 | **0.396** | **2.39×** | `` |
| `install` | `f00-install --core -m 644 fixture inst.out` | 1.201 | **0.413** | **2.91×** | `` |
| `yes` | `f00-yes --core --version` | 0.810 | **0.252** | **3.21×** | `f00-yes (f00) 0.15.9 License: MIT · https://f00.sh` |
| `[` | `f00-[ --core -f fixture.txt` | 0.815 | **0.310** | **2.63×** | `` |

Full machine-readable data: [suite.json](suite.json)

