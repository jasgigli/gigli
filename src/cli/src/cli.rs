//! CLI argument parsing for GigliOptix
use clap::{Command};

pub fn build_cli() -> Command {
    Command::new("giglioptix")
        .about("GigliOptix Compiler CLI")
        .subcommand(Command::new("build").about("Build a GigliOptix project"))
        .subcommand(Command::new("run").about("Run a GigliOptix program"))
        .subcommand(Command::new("bundle")
            .about("Compile and bundle a GigliOptix project for the web")
            .arg(clap::arg!(<INPUT> "Input .gx file").required(true))
            .arg(clap::arg!([OUTPUT] "Output directory").required(false)))
}
