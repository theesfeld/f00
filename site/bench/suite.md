# Suite benchmarks (f00 vs GNU coreutils)

**Overall: 2.5× faster than GNU coreutils overall** (148% faster overall; geo mean of per-tool speedups)

Generated: `2026-07-24T13:50:45Z` · N=15 median · warm-cache spawn-inclusive median

Host: x86_64 · Linux 6.17.0-1020-azure

Tools timed: 91 · wins: 90 · median 2.44× · total-time 2.774×

| Tool | Command (f00) | GNU ms | f00 ms | Speedup | Sample output (f00) |
|------|---------------|-------:|-------:|--------:|---------------------|
| `true` | `f00-true --core` | 0.540 | **0.264** | **2.05×** | `` |
| `false` | `f00-false --core` | 0.584 | **0.259** | **2.25×** | `` |
| `basename` | `f00-basename --core /usr/bin/ls` | 0.776 | **0.260** | **2.98×** | `ls` |
| `dirname` | `f00-dirname --core /usr/bin/ls` | 0.829 | **0.310** | **2.68×** | `/usr/bin` |
| `echo` | `f00-echo --core hi` | 0.823 | **0.360** | **2.29×** | `hi` |
| `pwd` | `f00-pwd --core` | 0.837 | **0.366** | **2.28×** | `/home/runner/work/f00/f00` |
| `nproc` | `f00-nproc --core` | 0.798 | **0.260** | **3.07×** | `4` |
| `whoami` | `f00-whoami --core` | 0.834 | **0.264** | **3.16×** | `runner` |
| `uname` | `f00-uname --core -s` | 0.762 | **0.317** | **2.40×** | `Linux` |
| `id` | `f00-id --core -u` | 0.937 | **0.322** | **2.91×** | `1001` |
| `date` | `f00-date --core -u +%Y` | 0.805 | **0.330** | **2.44×** | `2026` |
| `printenv` | `f00-printenv --core PATH` | 0.763 | **0.310** | **2.46×** | `/snap/bin:/home/runner/.local/bin:/opt/pipx_bin:/home/runner/.cargo/bin:/home/ru` |
| `printf` | `f00-printf --core %s world` | 0.779 | **0.319** | **2.44×** | `world` |
| `factor` | `f00-factor --core 12` | 0.832 | **0.322** | **2.58×** | `12: 2 2 3` |
| `numfmt` | `f00-numfmt --core --to=si 1000` | 0.782 | **0.330** | **2.37×** | `1.0k` |
| `expr` | `f00-expr --core 1 + 1` | 0.843 | **0.322** | **2.62×** | `2` |
| `seq` | `f00-seq --core 1 5` | 0.788 | **0.317** | **2.48×** | `1 2 3 4 5` |
| `cat` | `f00-cat --core fixture.txt` | 0.797 | **0.296** | **2.69×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `wc` | `f00-wc --core -l fixture.txt` | 0.813 | **0.381** | **2.13×** | `400 /tmp/f00-suite-bench.ta3c03f4/fix.txt` |
| `head` | `f00-head --core -n 3 fixture.txt` | 0.807 | **0.335** | **2.41×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tail` | `f00-tail --core -n 3 fixture.txt` | 0.830 | **0.374** | **2.22×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `nl` | `f00-nl --core fixture.txt` | 0.906 | **0.411** | **2.21×** | `1 suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 2 suite-bench line abcd` |
| `od` | `f00-od --core -An -tx1 -N8 fixture.txt` | 0.826 | **0.373** | **2.22×** | `73 75 69 74 65 2d 62 65` |
| `cut` | `f00-cut --core -d: -f1 /etc/passwd` | 0.821 | **0.363** | **2.26×** | `root daemon bin sys sync games man lp mail news uucp proxy www-data backup list ` |
| `tr` | `f00-tr --core a-z A-Z` | 0.827 | **0.346** | **2.39×** | `HELLO` |
| `sort` | `f00-sort --core fixture.txt` | 1.322 | **0.741** | **1.78×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `uniq` | `f00-uniq --core a.txt` | 0.832 | **0.367** | **2.27×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789` |
| `paste` | `f00-paste --core a.txt b.txt` | 0.903 | **0.492** | **1.84×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `comm` | `f00-comm --core -12 a.txt b.txt` | 0.877 | **0.479** | **1.83×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `join` | `f00-join --core a.txt b.txt` | 15.101 | **0.506** | **29.82×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 line abcdefghijklmnopqrst` |
| `base64` | `f00-base64 --core fixture.txt` | 0.861 | **0.511** | **1.69×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `base32` | `f00-base32 --core fixture.txt` | 0.862 | **0.554** | **1.56×** | `ON2WS5DFFVRGK3TDNAQGY2LOMUQGCYTDMRSWMZ3INFVGW3DNNZXXA4LSON2HK5TXPB4XUIBQGEZD GNB` |
| `basenc` | `f00-basenc --core --base64 fixture.txt` | 0.861 | **0.509** | **1.69×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `md5sum` | `f00-md5sum --core fixture.txt` | 1.210 | **0.399** | **3.03×** | `a5e6b1aa8523bc01f561fcef58d16894 /tmp/f00-suite-bench.ta3c03f4/fix.txt` |
| `sha1sum` | `f00-sha1sum --core fixture.txt` | 1.173 | **0.442** | **2.65×** | `49f92a1f57c1a825b5ca5777c92d6e15ac26a8ea /tmp/f00-suite-bench.ta3c03f4/fix.txt` |
| `sha224sum` | `f00-sha224sum --core fixture.txt` | 1.198 | **0.456** | **2.63×** | `94c1dff65fd14336129da4555171327a3e1e3e684810af23fa13e7f3 /tmp/f00-suite-bench.ta` |
| `sha256sum` | `f00-sha256sum --core fixture.txt` | 1.171 | **0.452** | **2.59×** | `7c28ea8726bc6923d5b38a6b6938ad5907c01dc6640e1645cf63cbf9df706132 /tmp/f00-suite-` |
| `sha384sum` | `f00-sha384sum --core fixture.txt` | 1.186 | **0.423** | **2.80×** | `f2578c293b7eeebf35402906e5e2fcd522b567687241b72950690c389f88baf83f9cd518c1fd67b3` |
| `sha512sum` | `f00-sha512sum --core fixture.txt` | 1.187 | **0.416** | **2.85×** | `a3282daa5cc665baa2b247ee17a0798f1d8028bbfb2107ea0df9493d0e57bfba10ba95d72d2550c2` |
| `b2sum` | `f00-b2sum --core fixture.txt` | 0.842 | **0.416** | **2.02×** | `915149393ea1091e4aa19ad9c68f980ebf83f5da2a576a20ca65fb001c685bf890523b442b840760` |
| `cksum` | `f00-cksum --core fixture.txt` | 1.213 | **0.388** | **3.12×** | `1448063438 22000 /tmp/f00-suite-bench.ta3c03f4/fix.txt` |
| `sum` | `f00-sum --core fixture.txt` | 0.842 | **0.361** | **2.33×** | `9481 22 /tmp/f00-suite-bench.ta3c03f4/fix.txt` |
| `ls` | `f00-ls --core -1 dir` | 0.996 | **0.451** | **2.21×** | `f01.txt f02.txt f03.txt f04.txt f05.txt f06.txt f07.txt f08.txt f09.txt f10.txt ` |
| `dir` | `f00-dir --core -1 dir` | 1.002 | **0.354** | **2.83×** | `f06.txt f02.txt f20.txt f14.txt f09.txt f13.txt f10.txt f17.txt f16.txt f08.txt ` |
| `vdir` | `f00-vdir --core -1 dir` | 1.009 | **0.331** | **3.05×** | `- f06.txt - f02.txt - f20.txt - f14.txt - f09.txt - f13.txt - f10.txt - f17.txt ` |
| `stat` | `f00-stat --core -c %s fixture.txt` | 0.970 | **0.329** | **2.95×** | `22000` |
| `realpath` | `f00-realpath --core .` | 0.790 | **0.395** | **2.00×** | `/home/runner/work/f00/f00/asm` |
| `readlink` | `f00-readlink --core /proc/self/exe` | 0.799 | **0.351** | **2.28×** | `/home/runner/work/f00/f00/asm/f00` |
| `df` | `f00-df --core -P /` | 0.909 | **0.370** | **2.46×** | `Filesystem 1K-blocks Used Available Use% Mounted on /dev/root 151263856 59919540` |
| `du` | `f00-du --core -s dir` | 0.881 | **0.392** | **2.25×** | `5 /tmp/f00-suite-bench.ta3c03f4/dir` |
| `dircolors` | `f00-dircolors --core -p` | 0.783 | **0.319** | **2.46×** | `# Configuration file for dircolors, a utility to help you set the # LS_COLORS en` |
| `env` | `f00-env --core -i true` | 1.169 | **0.339** | **3.45×** | `` |
| `timeout` | `f00-timeout --core 5 true` | 1.439 | **0.434** | **3.32×** | `` |
| `nice` | `f00-nice --core true` | 1.217 | **0.325** | **3.75×** | `` |
| `nohup` | `f00-nohup --core true` | 1.209 | **0.312** | **3.88×** | `` |
| `sleep` | `f00-sleep --core 0` | 0.893 | **0.433** | **2.06×** | `` |
| `test` | `f00-test --core -f fixture.txt` | 0.786 | **0.318** | **2.47×** | `` |
| `pathchk` | `f00-pathchk --core ok-name` | 0.781 | **0.320** | **2.44×** | `` |
| `mktemp` | `f00-mktemp --core -u` | 0.795 | **0.323** | **2.46×** | `/tmp/tmp.CLy05i` |
| `sync` | `f00-sync --core` | 0.809 | **0.341** | **2.37×** | `` |
| `uptime` | `f00-uptime --core` | 1.430 | **0.333** | **4.29×** | `up 2 minutes` |
| `hostid` | `f00-hostid --core` | 0.876 | **0.371** | **2.36×** | `db830370` |
| `logname` | `f00-logname --core` | 0.787 | **0.376** | **2.09×** | `runner` |
| `tty` | `f00-tty --core` | 0.779 | **0.260** | **2.99×** | `not a tty` |
| `groups` | `f00-groups --core` | 0.889 | **0.357** | **2.49×** | `adm users docker systemd-journal runner` |
| `arch` | `f00-arch --core` | 0.780 | **0.328** | **2.38×** | `x86_64` |
| `hostname` | `f00-hostname --core` | 0.574 | **0.327** | **1.75×** | `runnervmvrwv9` |
| `users` | `f00-users --core` | 0.812 | **0.329** | **2.47×** | `` |
| `who` | `f00-who --core` | 0.820 | **0.337** | **2.43×** | `` |
| `pinky` | `f00-pinky --core` | 0.829 | **0.327** | **2.53×** | `` |
| `fold` | `f00-fold --core -w 40 fixture.txt` | 0.869 | **0.403** | **2.16×** | `suite-bench line abcdefghijklmnopqrstuvw xyz 0123456789 suite-bench line abcdefg` |
| `fmt` | `f00-fmt --core -w 40 fixture.txt` | 0.946 | **0.382** | **2.48×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `expand` | `f00-expand --core fixture.txt` | 0.852 | **0.435** | **1.96×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `unexpand` | `f00-unexpand --core fixture.txt` | 0.837 | **0.459** | **1.83×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tac` | `f00-tac --core fixture.txt` | 0.833 | **0.369** | **2.26×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `rev` | `f00-rev --core fixture.txt` | 1.102 | **0.432** | **2.55×** | `9876543210 zyxwvutsrqponmlkjihgfedcba enil hcneb-etius 9876543210 zyxwvutsrqponm` |
| `ptx` | `f00-ptx --core -A fixture.txt` | 1.581 | **0.427** | **3.70×** | `suite bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite bench line abcdefgh` |
| `pr` | `f00-pr --core -t fixture.txt` | 1.020 | **0.380** | **2.69×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `shuf` | `f00-shuf --core fixture.txt` | 0.826 | **0.554** | **1.49×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tsort` | `f00-tsort --core` | 0.813 | **0.343** | **2.37×** | `a b c` |
| `tee` | `f00-tee --core tee.out` | 0.873 | **0.418** | **2.09×** | `tee data tee data tee data tee data tee data tee data tee data tee data tee data` |
| `split` | `f00-split --core -l 50 fixture.txt out` | 1.180 | **1.389** | **0.85×** | `` |
| `csplit` | `f00-csplit --core -f xx fixture 5` | 0.990 | **0.332** | **2.98×** | `` |
| `chmod` | `f00-chmod --core 644 fixture.txt` | 0.794 | **0.324** | **2.45×** | `` |
| `touch` | `f00-touch --core touched` | 0.784 | **0.385** | **2.04×** | `` |
| `truncate` | `f00-truncate --core -s 0 trunc` | 0.795 | **0.343** | **2.32×** | `` |
| `cp` | `f00-cp --core fixture.txt cp.out` | 1.159 | **0.414** | **2.80×** | `` |
| `dd` | `f00-dd --core if=fixture of=dd.out bs=4k count=1` | 0.883 | **0.426** | **2.07×** | `` |
| `install` | `f00-install --core -m 644 fixture inst.out` | 1.181 | **0.477** | **2.47×** | `` |
| `yes` | `f00-yes --core --version` | 0.803 | **0.255** | **3.15×** | `f00-yes (f00) 0.15.9 License: MIT · https://f00.sh` |
| `[` | `f00-[ --core -f fixture.txt` | 0.784 | **0.324** | **2.42×** | `` |

Full machine-readable data: [suite.json](suite.json)

