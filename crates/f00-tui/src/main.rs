//! `f00-tui` — dual-pane interactive directory browser for f00.

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use f00_tui::{run_browser, BrowserOptions};

fn print_usage() {
    eprintln!(
        "\
f00-tui — interactive dual-pane directory browser

Usage:
  f00-tui [OPTIONS] [DIR]

Options:
  -A, --almost-all    Show hidden entries (except . and ..)
  -a, --all           Show all entries including . and ..
      --icons         Force icons on
      --no-icons      Force icons off
      --git           Enable git status annotations (default: on when built with git)
      --no-git        Disable git status annotations
  -h, --help          Show this help
  -V, --version       Show version

The dual-pane browser used to ship as `f00 --browse`. It is now a separate
binary so the main `f00` list CLI stays lean. Build the optional embedded
flag with: cargo build -p f00 --features tui
"
    );
}

fn main() -> ExitCode {
    let mut show_hidden = false;
    let mut icons = true;
    let mut git = cfg!(feature = "git");
    let mut start = PathBuf::from(".");
    let mut saw_path = false;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_usage();
                return ExitCode::SUCCESS;
            }
            "-V" | "--version" => {
                println!("f00-tui {}", env!("CARGO_PKG_VERSION"));
                return ExitCode::SUCCESS;
            }
            "-A" | "--almost-all" | "-a" | "--all" => show_hidden = true,
            "--icons" => icons = true,
            "--no-icons" => icons = false,
            "--git" => git = true,
            "--no-git" => git = false,
            "--" => {
                if let Some(p) = args.next() {
                    start = PathBuf::from(p);
                    saw_path = true;
                }
                break;
            }
            s if s.starts_with('-') => {
                eprintln!("f00-tui: unknown option: {s}");
                eprintln!("Try 'f00-tui --help' for more information.");
                return ExitCode::from(2);
            }
            s => {
                if saw_path {
                    eprintln!("f00-tui: unexpected argument: {s}");
                    return ExitCode::from(2);
                }
                start = PathBuf::from(s);
                saw_path = true;
            }
        }
    }

    // Remaining args after `--`
    for s in args {
        if saw_path {
            eprintln!("f00-tui: unexpected argument: {s}");
            return ExitCode::from(2);
        }
        start = PathBuf::from(s);
        saw_path = true;
    }

    match run_browser(
        &start,
        BrowserOptions {
            show_hidden,
            icons,
            git: git && cfg!(feature = "git"),
        },
    ) {
        Ok(code) => ExitCode::from(code as u8),
        Err(e) => {
            eprintln!("f00-tui: {e:#}");
            ExitCode::from(1)
        }
    }
}
