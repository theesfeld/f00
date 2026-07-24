#!/usr/bin/env python3
"""Generate per-tool bench data for the website: tool, command, output, times.

Writes:
  site/bench/suite.json
  site/bench/suite.md

Usage:
  cd asm && make links
  python3 ../scripts/gen-suite-bench.py
  N=20 python3 ../scripts/gen-suite-bench.py
"""
from __future__ import annotations

import json
import os
import shlex
import statistics
import subprocess
import sys
import tempfile
import time
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ASM = ROOT / "asm"
F00 = ASM / "f00"
OUT_JSON = ROOT / "site" / "bench" / "suite.json"
OUT_MD = ROOT / "site" / "bench" / "suite.md"
README = ROOT / "README.md"
N = int(os.environ.get("N", "25"))

# Snapshot tools embedded in README (must match suite cases)
README_TOOLS = (
    "true",
    "basename",
    "nproc",
    "whoami",
    "cat",
    "wc",
    "md5sum",
    "sha256sum",
    "sort",
    "ls",
)

BENCH_START = "<!-- bench-table:start -->"
BENCH_END = "<!-- bench-table:end -->"


def find_gnu(name: str) -> str | None:
    for p in (f"/usr/bin/{name}", f"/bin/{name}"):
        if os.path.isfile(p) and os.access(p, os.X_OK):
            return p
    return None


def med(cmd: list[str], n: int = N, stdin: bytes | None = None) -> float:
    def _run() -> None:
        if stdin is None:
            subprocess.run(
                cmd,
                stdin=subprocess.DEVNULL,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
                check=False,
            )
        else:
            subprocess.run(
                cmd,
                input=stdin,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
                check=False,
            )

    for _ in range(3):
        _run()
    ts: list[float] = []
    for _ in range(n):
        t0 = time.perf_counter()
        _run()
        ts.append(time.perf_counter() - t0)
    return statistics.median(ts)


def capture(cmd: list[str], stdin: bytes | None = None, max_len: int = 160) -> str:
    try:
        if stdin is None:
            r = subprocess.run(
                cmd,
                stdin=subprocess.DEVNULL,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                check=False,
                timeout=5,
            )
        else:
            r = subprocess.run(
                cmd,
                input=stdin,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                check=False,
                timeout=5,
            )
        text = r.stdout.decode("utf-8", "replace").replace("\r", "")
        text = " ".join(text.split())
        if len(text) > max_len:
            text = text[: max_len - 1] + "…"
        return text
    except Exception as e:  # noqa: BLE001
        return f"({type(e).__name__})"


def main() -> int:
    if not F00.is_file():
        print("build f00 first: cd asm && make", file=sys.stderr)
        return 1
    # ensure links
    subprocess.run(["make", "-C", str(ASM), "links"], check=False, stdout=subprocess.DEVNULL)

    work = Path(tempfile.mkdtemp(prefix="f00-suite-bench."))
    try:
        fix = work / "fix.txt"
        fix.write_text(
            ("suite-bench line abcdefghijklmnopqrstuvwxyz 0123456789\n" * 400),
            encoding="utf-8",
        )
        d = work / "dir"
        d.mkdir()
        for i in range(1, 21):
            (d / f"f{i:02d}.txt").write_text(f"e-{i:02d}\n", encoding="utf-8")
        a = work / "a.txt"
        b = work / "b.txt"
        a.write_text("".join(sorted(fix.read_text(encoding="utf-8").splitlines(True))), encoding="utf-8")
        b.write_text(a.read_text(encoding="utf-8"), encoding="utf-8")

        # name, display args, arg list, optional stdin, prep note
        cases: list[tuple[str, str, list[str], bytes | None]] = [
            ("true", "", [], None),
            ("false", "", [], None),
            ("basename", "/usr/bin/ls", ["/usr/bin/ls"], None),
            ("dirname", "/usr/bin/ls", ["/usr/bin/ls"], None),
            ("echo", "hi", ["hi"], None),
            ("pwd", "", [], None),
            ("nproc", "", [], None),
            ("whoami", "", [], None),
            ("uname", "-s", ["-s"], None),
            ("id", "-u", ["-u"], None),
            ("date", "-u +%Y", ["-u", "+%Y"], None),
            ("printenv", "PATH", ["PATH"], None),
            ("printf", "%s world", ["%s", "world"], None),
            ("factor", "12", ["12"], None),
            ("numfmt", "--to=si 1000", ["--to=si", "1000"], None),
            ("expr", "1 + 1", ["1", "+", "1"], None),
            ("seq", "1 5", ["1", "5"], None),
            ("cat", "fixture.txt", [str(fix)], None),
            ("wc", "-l fixture.txt", ["-l", str(fix)], None),
            ("head", "-n 3 fixture.txt", ["-n", "3", str(fix)], None),
            ("tail", "-n 3 fixture.txt", ["-n", "3", str(fix)], None),
            ("nl", "fixture.txt", [str(fix)], None),
            ("od", "-An -tx1 -N8 fixture.txt", ["-An", "-tx1", "-N8", str(fix)], None),
            ("cut", "-d: -f1 /etc/passwd", ["-d:", "-f1", "/etc/passwd"], None),
            ("tr", "a-z A-Z", ["a-z", "A-Z"], b"hello\n"),
            ("sort", "fixture.txt", [str(fix)], None),
            ("uniq", "a.txt", [str(a)], None),
            ("paste", "a.txt b.txt", [str(a), str(b)], None),
            ("comm", "-12 a.txt b.txt", ["-12", str(a), str(b)], None),
            ("join", "a.txt b.txt", [str(a), str(b)], None),
            ("base64", "fixture.txt", [str(fix)], None),
            ("base32", "fixture.txt", [str(fix)], None),
            ("basenc", "--base64 fixture.txt", ["--base64", str(fix)], None),
            ("md5sum", "fixture.txt", [str(fix)], None),
            ("sha1sum", "fixture.txt", [str(fix)], None),
            ("sha224sum", "fixture.txt", [str(fix)], None),
            ("sha256sum", "fixture.txt", [str(fix)], None),
            ("sha384sum", "fixture.txt", [str(fix)], None),
            ("sha512sum", "fixture.txt", [str(fix)], None),
            ("b2sum", "fixture.txt", [str(fix)], None),
            ("cksum", "fixture.txt", [str(fix)], None),
            ("sum", "fixture.txt", [str(fix)], None),
            ("ls", "-1 dir", ["-1", str(d)], None),
            ("dir", "-1 dir", ["-1", str(d)], None),
            ("vdir", "-1 dir", ["-1", str(d)], None),
            ("stat", "-c %s fixture.txt", ["-c", "%s", str(fix)], None),
            ("realpath", ".", [str(ASM)], None),
            ("readlink", "/proc/self/exe", ["/proc/self/exe"], None),
            ("df", "-P /", ["-P", "/"], None),
            ("du", "-s dir", ["-s", str(d)], None),
            ("dircolors", "-p", ["-p"], None),
            ("env", "-i true", ["-i", "true"], None),
            ("timeout", "5 true", ["5", "true"], None),
            ("nice", "true", ["true"], None),
            ("nohup", "true", ["true"], None),
            ("sleep", "0", ["0"], None),
            ("test", "-f fixture.txt", ["-f", str(fix)], None),
            ("pathchk", "ok-name", ["ok-name"], None),
            ("mktemp", "-u", ["-u"], None),
            ("sync", "", [], None),
            ("uptime", "", [], None),
            ("hostid", "", [], None),
            ("logname", "", [], None),
            ("tty", "", [], None),
            ("groups", "", [], None),
            ("arch", "", [], None),
            ("hostname", "", [], None),
            ("users", "", [], None),
            ("who", "", [], None),
            ("pinky", "", [], None),
            ("fold", "-w 40 fixture.txt", ["-w", "40", str(fix)], None),
            ("fmt", "-w 40 fixture.txt", ["-w", "40", str(fix)], None),
            ("expand", "fixture.txt", [str(fix)], None),
            ("unexpand", "fixture.txt", [str(fix)], None),
            ("tac", "fixture.txt", [str(fix)], None),
            ("rev", "fixture.txt", [str(fix)], None),
            ("ptx", "-A fixture.txt", ["-A", str(fix)], None),
            ("pr", "-t fixture.txt", ["-t", str(fix)], None),
            ("shuf", "fixture.txt", [str(fix)], None),
            ("tsort", "", [], b"a b\nb c\n"),
            ("tee", "tee.out", [str(work / "tee.out")], b"tee data\n" * 20),
            ("split", "-l 50 fixture.txt out", ["-l", "50", str(fix), str(work / "spl")], None),
            ("csplit", "-f xx fixture 5", ["-f", str(work / "xx"), str(fix), "5"], None),
            ("chmod", "644 fixture.txt", ["644", str(fix)], None),
            ("touch", "touched", [str(work / "touched")], None),
            ("truncate", "-s 0 trunc", ["-s", "0", str(work / "trunc")], None),
            ("cp", "fixture.txt cp.out", [str(fix), str(work / "cp.out")], None),
            ("dd", "if=fixture of=dd.out bs=4k count=1", [
                f"if={fix}", f"of={work / 'dd.out'}", "bs=4k", "count=1", "status=none"
            ], None),
            ("install", "-m 644 fixture inst.out", ["-m", "644", str(fix), str(work / "inst.out")], None),
            ("yes", "--version", ["--version"], None),
            # entry/help races for tools without a fair payload race
            ("[", "-f fixture.txt", ["-f", str(fix)], None),
        ]

        rows = []
        for name, disp_args, argl, stdin in cases:
            gnu_name = "test" if name == "[" else name
            gnu = find_gnu(gnu_name)
            if name == "[":
                link = ASM / "f00-["
                if not link.exists():
                    link = ASM / "f00-test"
            else:
                link = ASM / f"f00-{name}"
            if not link.exists() and name not in ("test", "["):
                continue
            if name == "test" and not (ASM / "f00-test").exists():
                continue

            f_bin = str(link) if link.exists() else str(F00)
            f_cmd = [f_bin, "--core", *argl]
            # display command strings
            f_disp = f"f00-{name} --core" + (f" {disp_args}" if disp_args else "")
            g_disp = f"{gnu_name}" + (f" {disp_args}" if disp_args else "")

            if not gnu:
                rows.append(
                    {
                        "tool": name,
                        "command_gnu": g_disp,
                        "command_f00": f_disp,
                        "output_gnu": "",
                        "output_f00": capture(f_cmd, stdin=stdin),
                        "time_gnu_ms": None,
                        "time_f00_ms": round(med(f_cmd, stdin=stdin) * 1000, 3),
                        "ratio": None,
                        "status": "skip-no-gnu",
                    }
                )
                continue

            g_cmd = [gnu, *argl]
            try:
                g_ms = med(g_cmd, stdin=stdin) * 1000
                f_ms = med(f_cmd, stdin=stdin) * 1000
                ratio = (g_ms / f_ms) if f_ms > 0 else None
                out_g = capture(g_cmd, stdin=stdin)
                out_f = capture(f_cmd, stdin=stdin)
                rows.append(
                    {
                        "tool": name,
                        "command_gnu": g_disp,
                        "command_f00": f_disp,
                        "output_gnu": out_g,
                        "output_f00": out_f,
                        "time_gnu_ms": round(g_ms, 3),
                        "time_f00_ms": round(f_ms, 3),
                        "ratio": round(ratio, 2) if ratio is not None else None,
                        "status": "ok",
                    }
                )
            except Exception as e:  # noqa: BLE001
                rows.append(
                    {
                        "tool": name,
                        "command_gnu": g_disp,
                        "command_f00": f_disp,
                        "output_gnu": "",
                        "output_f00": "",
                        "time_gnu_ms": None,
                        "time_f00_ms": None,
                        "ratio": None,
                        "status": f"error:{type(e).__name__}",
                    }
                )
            print(f"{name:16} done", flush=True)

        meta = {
            "generated_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            "host": os.uname().nodename,
            "machine": os.uname().machine,
            "system": f"{os.uname().sysname} {os.uname().release}",
            "n_runs": N,
            "method": "warm-cache spawn-inclusive median",
            "f00_version": subprocess.run(
                [str(F00), "--version"], capture_output=True, text=True, check=False
            ).stdout.splitlines()[0]
            if F00.is_file()
            else "unknown",
            "notes": "f00 timed as f00-TOOL --core; GNU as /usr/bin/TOOL. Times include process spawn.",
        }
        payload = {"meta": meta, "tools": rows}
        OUT_JSON.parent.mkdir(parents=True, exist_ok=True)
        OUT_JSON.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")

        lines = [
            "# Suite benchmarks (f00 vs GNU coreutils)",
            "",
            f"Generated: `{meta['generated_at']}` · N={N} median · {meta['method']}",
            "",
            f"Host: {meta['machine']} · {meta['system']}",
            "",
            "| Tool | Command (f00) | GNU ms | f00 ms | Speedup | Sample output (f00) |",
            "|------|---------------|-------:|-------:|--------:|---------------------|",
        ]
        for r in rows:
            if r["status"] != "ok":
                continue
            lines.append(
                f"| `{r['tool']}` | `{r['command_f00']}` | {r['time_gnu_ms']:.3f} | "
                f"**{r['time_f00_ms']:.3f}** | **{r['ratio']:.2f}×** | `{r['output_f00'][:80]}` |"
            )
        lines.append("")
        lines.append("Full machine-readable data: [suite.json](suite.json)")
        lines.append("")
        OUT_MD.write_text("\n".join(lines) + "\n", encoding="utf-8")
        update_readme_table(rows, meta)
        print(f"wrote {OUT_JSON}")
        print(f"wrote {OUT_MD}")
        ok = sum(1 for r in rows if r["status"] == "ok")
        print(f"tools ok: {ok}/{len(rows)}")
        return 0
    finally:
        import shutil

        shutil.rmtree(work, ignore_errors=True)


def update_readme_table(rows: list[dict], meta: dict) -> None:
    """Refresh the README representative bench table between HTML markers."""
    if not README.is_file():
        return
    by_tool = {r["tool"]: r for r in rows if r.get("status") == "ok"}
    table_lines = [
        "| Tool | Command | GNU | f00 `--core` | vs GNU |",
        "|------|---------|-----|--------------|--------|",
    ]
    for name in README_TOOLS:
        r = by_tool.get(name)
        if not r:
            table_lines.append(f"| `{name}` | `f00-{name} --core` | — | — | see suite |")
            continue
        g = r.get("time_gnu_ms")
        f = r.get("time_f00_ms")
        ratio = r.get("ratio")
        g_s = f"{g:.2f} ms" if isinstance(g, (int, float)) else "—"
        f_s = f"**{f:.2f} ms**" if isinstance(f, (int, float)) else "—"
        r_s = f"**~{ratio:.1f}×**" if isinstance(ratio, (int, float)) else "see suite"
        cmd = r.get("command_f00") or f"f00-{name} --core"
        table_lines.append(f"| `{name}` | `{cmd}` | {g_s} | {f_s} | {r_s} |")

    stamp = (
        f"_CI / suite bench · `{meta.get('generated_at', '?')}` · "
        f"N={meta.get('n_runs', N)} median · {meta.get('machine', '?')} · "
        f"{meta.get('system', '?')}_"
    )
    block = (
        f"{BENCH_START}\n"
        f"{stamp}\n\n"
        + "\n".join(table_lines)
        + f"\n{BENCH_END}"
    )

    text = README.read_text(encoding="utf-8")
    if BENCH_START in text and BENCH_END in text:
        import re

        text2 = re.sub(
            re.escape(BENCH_START) + r".*?" + re.escape(BENCH_END),
            block,
            text,
            count=1,
            flags=re.S,
        )
    else:
        # Insert after "## Benchmarks" intro if markers missing
        import re

        m = re.search(r"(## Benchmarks\n(?:.*?\n)*?)(Representative results[^\n]*\n)", text)
        if m:
            text2 = text[: m.end()] + "\n" + block + "\n" + text[m.end() :]
        else:
            text2 = text + "\n\n## Benchmarks\n\n" + block + "\n"

    if text2 != text:
        README.write_text(text2, encoding="utf-8")
        print(f"updated {README} bench table")


if __name__ == "__main__":
    sys.exit(main())
