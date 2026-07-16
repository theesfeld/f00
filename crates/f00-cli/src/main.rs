use std::io::{self, Write};

use clap::{CommandFactory, Parser};
use f00_cli::cli::Args;
use f00_cli::config::invoked_as_ls;
use f00_cli::run;

fn main() {
    let args = Args::parse();

    if let Some(shell) = args.generate_completions {
        let mut cmd = Args::command();
        let name = cmd.get_name().to_string();
        clap_complete::generate(shell, &mut cmd, name, &mut io::stdout());
        return;
    }

    if args.generate_man {
        let cmd = Args::command();
        let man = clap_mangen::Man::new(cmd);
        let mut buf = Vec::new();
        if let Err(err) = man.render(&mut buf) {
            eprintln!("f00: failed to generate man page: {err}");
            std::process::exit(2);
        }
        if let Err(err) = io::stdout().write_all(&buf) {
            eprintln!("f00: {err}");
            std::process::exit(2);
        }
        return;
    }

    let as_ls = invoked_as_ls();
    match run::run_with_argv0(args, as_ls) {
        Ok(code) => {
            if code != 0 {
                std::process::exit(code);
            }
        }
        Err(err) => {
            eprintln!("f00: {err:#}");
            // Serious trouble (I/O on stdout, bad config, etc.)
            std::process::exit(2);
        }
    }
}
