//! Main entry point for the GigliOptix compiler CLI
mod cli;

fn main() {
    let matches = cli::build_cli().get_matches();
    match matches.subcommand() {
        Some(("build", _)) => {
            println!("Building project...");
            // TODO: Call build logic
        }
        Some(("run", _)) => {
            println!("Running project...");
            // TODO: Call run logic
        }
        _ => {
            println!("No subcommand provided. Use --help for usage.");
        }
    }
}
