use clap::Parser;
use f00_cli::cli::Args;
use f00_cli::run;

fn main() {
    let args = Args::parse();
    if let Err(err) = run::run(args) {
        eprintln!("f00: {err:#}");
        std::process::exit(1);
    }
}
