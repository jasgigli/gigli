//! CLI argument parsing for GigliOptix
use clap::{Command};

pub fn build_cli() -> Command {
    Command::new("giglioptix")
        .about("GigliOptix Compiler CLI")
        .subcommand(Command::new("build").about("Build a GigliOptix project"))
        .subcommand(Command::new("run").about("Run a GigliOptix program"))
}
