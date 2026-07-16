//! Library surface for the **f00** CLI (testable without spawning a process).

pub mod cli;
pub mod run;

pub use cli::Args;
pub use run::{build_config, run};
