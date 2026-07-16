use clap::Parser;
use f00_cli::cli::Args;
use f00_cli::config::invoked_as_ls;
use f00_cli::run;

fn main() {
    let args = Args::parse();
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
