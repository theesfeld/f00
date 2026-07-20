//! Library surface for the **f00** CLI (testable without spawning a process).

pub mod cli;
pub mod config;
pub mod run;
pub mod update;

#[cfg(feature = "plugins")]
pub mod plugins_cmd;

pub use cli::Args;
pub use config::{
    apply_auto_gnu, invoked_as_ls, invoked_as_ls_from, load_user_config, merge_config_into_args,
    parse_config_str, platform_config_path, resolve_args, resolve_args_with_tty, FileConfig,
};
pub use run::{build_config, prepare_args, run, run_with_argv0};
