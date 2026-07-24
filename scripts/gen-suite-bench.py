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
N = int(os.environ.get("N", "25"))


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
            ("md5sum", "fixture.txt", [str(fix)], None),
            ("sha1sum", "fixture.txt", [str(fix)], None),
            ("sha256sum", "fixture.txt", [str(fix)], None),
            ("sha512sum", "fixture.txt", [str(fix)], None),
            ("cksum", "fixture.txt", [str(fix)], None),
            ("sum", "fixture.txt", [str(fix)], None),
            ("ls", "-1 dir", ["-1", str(d)], None),
            ("dir", "-1 dir", ["-1", str(d)], None),
            ("stat", "-c %s fixture.txt", ["-c", "%s", str(fix)], None),
            ("realpath", ".", [str(ASM)], None),
            ("df", "-P /", ["-P", "/"], None),
            ("du", "-s dir", ["-s", str(d)], None),
            ("dircolors", "-p", ["-p"], None),
            ("env", "-i true", ["-i", "true"], None),
            ("timeout", "5 true", ["5", "true"], None),
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
            ("fold", "-w 40 fixture.txt", ["-w", "40", str(fix)], None),
            ("fmt", "-w 40 fixture.txt", ["-w", "40", str(fix)], None),
            ("expand", "fixture.txt", [str(fix)], None),
            ("tac", "fixture.txt", [str(fix)], None),
            ("rev", "fixture.txt", [str(fix)], None),
            ("ptx", "-A fixture.txt", ["-A", str(fix)], None),
            ("pr", "-t fixture.txt", ["-t", str(fix)], None),
            ("shuf", "fixture.txt", [str(fix)], None),
            ("tsort", "", [], b"a b\nb c\n"),
            ("yes", "--version", ["--version"], None),
        ]

        rows = []
        for name, disp_args, argl, stdin in cases:
            gnu = find_gnu(name)
            link = ASM / f"f00-{name}"
            if not link.exists() and name != "test":
                # try bracket
                continue
            if name == "test" and not (ASM / "f00-test").exists():
                continue

            f_bin = str(ASM / f"f00-{name}") if (ASM / f"f00-{name}").exists() else str(F00)
            f_cmd = [f_bin, "--core", *argl]
            # display command strings
            f_disp = f"f00-{name} --core" + (f" {disp_args}" if disp_args else "")
            g_disp = f"{name}" + (f" {disp_args}" if disp_args else "")

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
        print(f"wrote {OUT_JSON}")
        print(f"wrote {OUT_MD}")
        ok = sum(1 for r in rows if r["status"] == "ok")
        print(f"tools ok: {ok}/{len(rows)}")
        return 0
    finally:
        import shutil

        shutil.rmtree(work, ignore_errors=True)


if __name__ == "__main__":
    sys.exit(main())
