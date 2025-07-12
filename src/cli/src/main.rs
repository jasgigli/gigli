//! Main entry point for the Gigli compiler CLI

use gigli_core::{parse_file, generate_ir};
use gigli_codegen_wasm::emit_wasm;

mod cli;
mod bundle;

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
        Some(("bundle", sub_m)) => {
            let input = sub_m.get_one::<String>("INPUT").unwrap();
            let output = sub_m.get_one::<String>("OUTPUT").map(|s| s.as_str()).unwrap_or("dist");
            println!("Bundling {input} for the web into {output} ...");

            // === 1. Parse source code ===
            let ast = parse_file(input);

            // === 2. Generate IR ===
            let ir = generate_ir(&ast);

            // === 3. Emit WASM ===
            let wasm_path = "main.wasm";
            emit_wasm(&ir, wasm_path);

            // === 4. Bundle for web ===
            bundle::bundle_for_web(wasm_path, output);
            println!("Bundle complete. Open {}/index.html in your browser.", output);
        }
        _ => {
            println!("No subcommand provided. Use --help for usage.");
        }
    }
}
