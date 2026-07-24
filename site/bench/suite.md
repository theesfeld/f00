# Suite benchmarks (f00 vs GNU coreutils)

Generated: `2026-07-24T12:12:42Z` · N=15 median · warm-cache spawn-inclusive median

Host: x86_64 · Linux 6.17.0-1020-azure

| Tool | Command (f00) | GNU ms | f00 ms | Speedup | Sample output (f00) |
|------|---------------|-------:|-------:|--------:|---------------------|
| `true` | `f00-true --core` | 0.591 | **0.287** | **2.06×** | `` |
| `false` | `f00-false --core` | 0.536 | **0.249** | **2.15×** | `` |
| `basename` | `f00-basename --core /usr/bin/ls` | 0.758 | **0.253** | **2.99×** | `ls` |
| `dirname` | `f00-dirname --core /usr/bin/ls` | 0.763 | **0.251** | **3.05×** | `/usr/bin` |
| `echo` | `f00-echo --core hi` | 0.771 | **0.340** | **2.27×** | `hi` |
| `pwd` | `f00-pwd --core` | 0.780 | **0.317** | **2.46×** | `/home/runner/work/f00/f00` |
| `nproc` | `f00-nproc --core` | 0.782 | **0.253** | **3.09×** | `4` |
| `whoami` | `f00-whoami --core` | 0.839 | **0.266** | **3.16×** | `runner` |
| `uname` | `f00-uname --core -s` | 0.782 | **0.333** | **2.35×** | `Linux` |
| `id` | `f00-id --core -u` | 0.958 | **0.348** | **2.76×** | `1001` |
| `date` | `f00-date --core -u +%Y` | 0.804 | **0.319** | **2.52×** | `2026` |
| `printenv` | `f00-printenv --core PATH` | 0.766 | **0.312** | **2.45×** | `/snap/bin:/home/runner/.local/bin:/opt/pipx_bin:/home/runner/.cargo/bin:/home/ru` |
| `printf` | `f00-printf --core %s world` | 0.760 | **0.314** | **2.42×** | `world` |
| `factor` | `f00-factor --core 12` | 0.816 | **0.321** | **2.54×** | `12: 2 2 3` |
| `numfmt` | `f00-numfmt --core --to=si 1000` | 0.794 | **0.327** | **2.43×** | `1.0k` |
| `expr` | `f00-expr --core 1 + 1` | 0.845 | **0.334** | **2.53×** | `2` |
| `seq` | `f00-seq --core 1 5` | 0.775 | **0.312** | **2.48×** | `1 2 3 4 5` |
| `cat` | `f00-cat --core fixture.txt` | 0.795 | **0.293** | **2.72×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `wc` | `f00-wc --core -l fixture.txt` | 0.806 | **0.347** | **2.33×** | `400 /tmp/f00-suite-bench.wdu8_9b7/fix.txt` |
| `head` | `f00-head --core -n 3 fixture.txt` | 0.793 | **0.370** | **2.14×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tail` | `f00-tail --core -n 3 fixture.txt` | 0.844 | **0.410** | **2.06×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `nl` | `f00-nl --core fixture.txt` | 0.878 | **0.412** | **2.13×** | `1 suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 2 suite-bench line abcd` |
| `od` | `f00-od --core -An -tx1 -N8 fixture.txt` | 0.805 | **0.327** | **2.46×** | `73 75 69 74 65 2d 62 65` |
| `cut` | `f00-cut --core -d: -f1 /etc/passwd` | 0.784 | **0.325** | **2.41×** | `root daemon bin sys sync games man lp mail news uucp proxy www-data backup list ` |
| `tr` | `f00-tr --core a-z A-Z` | 0.814 | **0.347** | **2.35×** | `HELLO` |
| `sort` | `f00-sort --core fixture.txt` | 1.295 | **0.712** | **1.82×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `uniq` | `f00-uniq --core a.txt` | 0.832 | **0.361** | **2.30×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789` |
| `paste` | `f00-paste --core a.txt b.txt` | 0.894 | **0.489** | **1.83×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `comm` | `f00-comm --core -12 a.txt b.txt` | 0.875 | **0.451** | **1.94×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `join` | `f00-join --core a.txt b.txt` | 15.059 | **0.494** | **30.46×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 line abcdefghijklmnopqrst` |
| `base64` | `f00-base64 --core fixture.txt` | 0.849 | **0.476** | **1.78×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `base32` | `f00-base32 --core fixture.txt` | 0.838 | **0.521** | **1.61×** | `ON2WS5DFFVRGK3TDNAQGY2LOMUQGCYTDMRSWMZ3INFVGW3DNNZXXA4LSON2HK5TXPB4XUIBQGEZD GNB` |
| `basenc` | `f00-basenc --core --base64 fixture.txt` | 0.854 | **0.485** | **1.76×** | `c3VpdGUtYmVuY2ggbGluZSBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5eiAwMTIzNDU2Nzg5CnN1 aXR` |
| `md5sum` | `f00-md5sum --core fixture.txt` | 1.199 | **0.382** | **3.14×** | `a5e6b1aa8523bc01f561fcef58d16894 /tmp/f00-suite-bench.wdu8_9b7/fix.txt` |
| `sha1sum` | `f00-sha1sum --core fixture.txt` | 1.247 | **0.464** | **2.69×** | `49f92a1f57c1a825b5ca5777c92d6e15ac26a8ea /tmp/f00-suite-bench.wdu8_9b7/fix.txt` |
| `sha224sum` | `f00-sha224sum --core fixture.txt` | 1.169 | **0.444** | **2.64×** | `94c1dff65fd14336129da4555171327a3e1e3e684810af23fa13e7f3 /tmp/f00-suite-bench.wd` |
| `sha256sum` | `f00-sha256sum --core fixture.txt` | 1.161 | **0.442** | **2.63×** | `7c28ea8726bc6923d5b38a6b6938ad5907c01dc6640e1645cf63cbf9df706132 /tmp/f00-suite-` |
| `sha384sum` | `f00-sha384sum --core fixture.txt` | 1.189 | **0.429** | **2.77×** | `f2578c293b7eeebf35402906e5e2fcd522b567687241b72950690c389f88baf83f9cd518c1fd67b3` |
| `sha512sum` | `f00-sha512sum --core fixture.txt` | 1.191 | **0.411** | **2.90×** | `a3282daa5cc665baa2b247ee17a0798f1d8028bbfb2107ea0df9493d0e57bfba10ba95d72d2550c2` |
| `b2sum` | `f00-b2sum --core fixture.txt` | 0.849 | **0.411** | **2.07×** | `915149393ea1091e4aa19ad9c68f980ebf83f5da2a576a20ca65fb001c685bf890523b442b840760` |
| `cksum` | `f00-cksum --core fixture.txt` | 1.181 | **0.380** | **3.11×** | `1448063438 22000 /tmp/f00-suite-bench.wdu8_9b7/fix.txt` |
| `sum` | `f00-sum --core fixture.txt` | 0.846 | **0.349** | **2.42×** | `9481 22 /tmp/f00-suite-bench.wdu8_9b7/fix.txt` |
| `ls` | `f00-ls --core -1 dir` | 1.007 | **0.509** | **1.98×** | `f01.txt f02.txt f03.txt f04.txt f05.txt f06.txt f07.txt f08.txt f09.txt f10.txt ` |
| `dir` | `f00-dir --core -1 dir` | 0.988 | **0.331** | **2.98×** | `f06.txt f02.txt f20.txt f14.txt f09.txt f13.txt f10.txt f17.txt f16.txt f08.txt ` |
| `vdir` | `f00-vdir --core -1 dir` | 0.999 | **0.328** | **3.05×** | `- f06.txt - f02.txt - f20.txt - f14.txt - f09.txt - f13.txt - f10.txt - f17.txt ` |
| `stat` | `f00-stat --core -c %s fixture.txt` | 0.942 | **0.307** | **3.07×** | `22000` |
| `realpath` | `f00-realpath --core .` | 0.786 | **0.432** | **1.82×** | `/home/runner/work/f00/f00/asm` |
| `readlink` | `f00-readlink --core /proc/self/exe` | 0.775 | **0.330** | **2.35×** | `/home/runner/work/f00/f00/asm/f00` |
| `df` | `f00-df --core -P /` | 0.912 | **0.359** | **2.54×** | `Filesystem 1K-blocks Used Available Use% Mounted on /dev/root 151263856 59919604` |
| `du` | `f00-du --core -s dir` | 0.848 | **0.361** | **2.35×** | `5 /tmp/f00-suite-bench.wdu8_9b7/dir` |
| `dircolors` | `f00-dircolors --core -p` | 0.764 | **0.310** | **2.46×** | `# Configuration file for dircolors, a utility to help you set the # LS_COLORS en` |
| `env` | `f00-env --core -i true` | 1.142 | **0.321** | **3.56×** | `` |
| `timeout` | `f00-timeout --core 5 true` | 1.435 | **0.430** | **3.34×** | `` |
| `nice` | `f00-nice --core true` | 1.191 | **0.333** | **3.57×** | `` |
| `nohup` | `f00-nohup --core true` | 1.189 | **0.324** | **3.67×** | `` |
| `sleep` | `f00-sleep --core 0` | 0.896 | **0.442** | **2.02×** | `` |
| `test` | `f00-test --core -f fixture.txt` | 0.765 | **0.308** | **2.48×** | `` |
| `pathchk` | `f00-pathchk --core ok-name` | 0.760 | **0.312** | **2.44×** | `` |
| `mktemp` | `f00-mktemp --core -u` | 0.791 | **0.312** | **2.53×** | `/tmp/tmp.qv4HqC` |
| `sync` | `f00-sync --core` | 0.813 | **0.342** | **2.38×** | `` |
| `uptime` | `f00-uptime --core` | 1.406 | **0.319** | **4.41×** | `up 0 minutes` |
| `hostid` | `f00-hostid --core` | 0.883 | **0.387** | **2.28×** | `db830370` |
| `logname` | `f00-logname --core` | 0.782 | **0.369** | **2.12×** | `runner` |
| `tty` | `f00-tty --core` | 0.786 | **0.250** | **3.15×** | `not a tty` |
| `groups` | `f00-groups --core` | 0.869 | **0.326** | **2.66×** | `adm users docker systemd-journal runner` |
| `arch` | `f00-arch --core` | 0.765 | **0.301** | **2.54×** | `x86_64` |
| `hostname` | `f00-hostname --core` | 0.540 | **0.314** | **1.72×** | `runnervmvrwv9` |
| `users` | `f00-users --core` | 0.799 | **0.311** | **2.57×** | `` |
| `who` | `f00-who --core` | 0.803 | **0.342** | **2.35×** | `` |
| `pinky` | `f00-pinky --core` | 0.810 | **0.317** | **2.55×** | `` |
| `fold` | `f00-fold --core -w 40 fixture.txt` | 0.858 | **0.418** | **2.05×** | `suite-bench line abcdefghijklmnopqrstuvw xyz 0123456789 suite-bench line abcdefg` |
| `fmt` | `f00-fmt --core -w 40 fixture.txt` | 0.942 | **0.373** | **2.52×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `expand` | `f00-expand --core fixture.txt` | 0.851 | **0.398** | **2.14×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `unexpand` | `f00-unexpand --core fixture.txt` | 0.820 | **0.442** | **1.85×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tac` | `f00-tac --core fixture.txt` | 0.820 | **0.377** | **2.17×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `rev` | `f00-rev --core fixture.txt` | 1.092 | **0.405** | **2.69×** | `9876543210 zyxwvutsrqponmlkjihgfedcba enil hcneb-etius 9876543210 zyxwvutsrqponm` |
| `ptx` | `f00-ptx --core -A fixture.txt` | 1.567 | **0.445** | **3.52×** | `suite bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite bench line abcdefgh` |
| `pr` | `f00-pr --core -t fixture.txt` | 1.023 | **0.377** | **2.72×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `shuf` | `f00-shuf --core fixture.txt` | 0.829 | **0.547** | **1.52×** | `suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789 suite-bench line abcdefgh` |
| `tsort` | `f00-tsort --core` | 0.801 | **0.344** | **2.33×** | `a b c` |
| `tee` | `f00-tee --core tee.out` | 0.872 | **0.398** | **2.19×** | `tee data tee data tee data tee data tee data tee data tee data tee data tee data` |
| `split` | `f00-split --core -l 50 fixture.txt out` | 1.177 | **1.351** | **0.87×** | `` |
| `csplit` | `f00-csplit --core -f xx fixture 5` | 0.979 | **0.345** | **2.84×** | `` |
| `chmod` | `f00-chmod --core 644 fixture.txt` | 0.776 | **0.324** | **2.39×** | `` |
| `touch` | `f00-touch --core touched` | 0.791 | **0.388** | **2.04×** | `` |
| `truncate` | `f00-truncate --core -s 0 trunc` | 0.799 | **0.345** | **2.31×** | `` |
| `cp` | `f00-cp --core fixture.txt cp.out` | 1.130 | **0.403** | **2.80×** | `` |
| `dd` | `f00-dd --core if=fixture of=dd.out bs=4k count=1` | 0.848 | **0.389** | **2.18×** | `` |
| `install` | `f00-install --core -m 644 fixture inst.out` | 1.123 | **0.405** | **2.77×** | `` |
| `yes` | `f00-yes --core --version` | 0.770 | **0.252** | **3.06×** | `f00-yes (f00) 0.15.3 License: MIT · https://f00.sh` |
| `[` | `f00-[ --core -f fixture.txt` | 0.748 | **0.307** | **2.44×** | `` |

Full machine-readable data: [suite.json](suite.json)

