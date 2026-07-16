use std::env;
use std::io::{self, Write};

use clap::{CommandFactory, Parser};
use f00_cli::cli::Args;
use f00_cli::config::invoked_as_ls;
use f00_cli::run;
use f00_cli::update;

fn main() {
    // Subcommand-style entrypoints (also available as long flags).
    if let Some(cmd) = env::args().nth(1) {
        match cmd.as_str() {
            "update" => match update::perform_update() {
                Ok(_) => return,
                Err(err) => {
                    eprintln!("f00: update failed: {err:#}");
                    std::process::exit(2);
                }
            },
            "check-update" => {
                std::process::exit(update::print_check_update());
            }
            _ => {}
        }
    }

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

    if args.check_update {
        std::process::exit(update::print_check_update());
    }

    if args.update {
        match update::perform_update() {
            Ok(_) => return,
            Err(err) => {
                eprintln!("f00: update failed: {err:#}");
                std::process::exit(2);
            }
        }
    }

    if args.list_plugins {
        #[cfg(feature = "plugins")]
        {
            if let Err(err) = f00_cli::plugins_cmd::list_plugins() {
                eprintln!("f00: {err:#}");
                std::process::exit(2);
            }
            return;
        }
        #[cfg(not(feature = "plugins"))]
        {
            eprintln!("f00: plugins support not compiled in (build with --features plugins)");
            std::process::exit(2);
        }
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
