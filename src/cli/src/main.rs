//! Main entry point for the Gigli compiler CLI

use gigli_core::{parse_file};
use gigli_core::ir::generator::{generate_ir, IRModule};
use gigli_codegen_wasm::emit_wasm;
use std::path::Path;
use std::process;

mod cli;
mod bundle;

fn main() {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        Some(("build", sub_m)) => {
            let input = sub_m.get_one::<String>("INPUT").unwrap();
            let output = sub_m.get_one::<String>("OUTPUT").unwrap();
            let target = sub_m.get_one::<String>("TARGET").unwrap();
            let mode = sub_m.get_one::<String>("MODE").unwrap();
            let watch = sub_m.get_flag("WATCH");

            println!("Building project...");
            println!("  Input: {}", input);
            println!("  Output: {}", output);
            println!("  Target: {}", target);
            println!("  Mode: {}", mode);
            println!("  Watch: {}", watch);

            if let Err(e) = build_project(input, output, target, mode, watch) {
                eprintln!("Build failed: {}", e);
                process::exit(1);
            }
        }
        Some(("run", sub_m)) => {
            let input = sub_m.get_one::<String>("INPUT").unwrap();
            let port = sub_m.get_one::<String>("PORT").unwrap();
            let host = sub_m.get_one::<String>("HOST").unwrap();
            let open = sub_m.get_flag("OPEN");

            println!("Running project...");
            println!("  Input: {}", input);
            println!("  Server: {}:{}", host, port);
            println!("  Open browser: {}", open);

            if let Err(e) = run_project(input, host, port, open) {
                eprintln!("Run failed: {}", e);
                process::exit(1);
            }
        }
        Some(("dev", sub_m)) => {
            let input = sub_m.get_one::<String>("INPUT").map(|s| s.as_str()).unwrap_or("src/App.gx");
            let port = sub_m.get_one::<String>("PORT").unwrap();
            let host = sub_m.get_one::<String>("HOST").unwrap();
            let open = sub_m.get_flag("OPEN");

            println!("Starting development server...");
            println!("  Input: {}", input);
            println!("  Server: {}:{}", host, port);
            println!("  Open browser: {}", open);

            if let Err(e) = start_dev_server(input, host, port, open) {
                eprintln!("Development server failed: {}", e);
                process::exit(1);
            }
        }
        Some(("bundle", sub_m)) => {
            let input = sub_m.get_one::<String>("INPUT").unwrap();
            let output = sub_m.get_one::<String>("OUTPUT").unwrap();
            let minify = sub_m.get_flag("MINIFY");
            let source_map = sub_m.get_flag("SOURCE_MAP");

            println!("Bundling project for web deployment...");
            println!("  Input: {}", input);
            println!("  Output: {}", output);
            println!("  Minify: {}", minify);
            println!("  Source maps: {}", source_map);

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
        Some(("fmt", sub_m)) => {
            let input = sub_m.get_one::<String>("INPUT").unwrap();
            let check = sub_m.get_flag("CHECK");

            println!("Formatting code...");
            println!("  Input: {}", input);
            println!("  Check only: {}", check);

            if let Err(e) = format_code(input, check) {
                eprintln!("Format failed: {}", e);
                process::exit(1);
            }
        }
        Some(("lint", sub_m)) => {
            let input = sub_m.get_one::<String>("INPUT").unwrap();
            let fix = sub_m.get_flag("FIX");

            println!("Linting code...");
            println!("  Input: {}", input);
            println!("  Fix issues: {}", fix);

            if let Err(e) = lint_code(input, fix) {
                eprintln!("Lint failed: {}", e);
                process::exit(1);
            }
        }
        Some(("test", sub_m)) => {
            let input = sub_m.get_one::<String>("INPUT").unwrap();
            let watch = sub_m.get_flag("WATCH");
            let coverage = sub_m.get_flag("COVERAGE");

            println!("Running tests...");
            println!("  Input: {}", input);
            println!("  Watch mode: {}", watch);
            println!("  Coverage: {}", coverage);

            if let Err(e) = run_tests(input, watch, coverage) {
                eprintln!("Tests failed: {}", e);
                process::exit(1);
            }
        }
        Some(("init", sub_m)) | Some(("new", sub_m)) => {
            let name = sub_m.get_one::<String>("NAME").unwrap();
            let template = sub_m.get_one::<String>("TEMPLATE").unwrap();
            let dir = sub_m.get_one::<String>("DIR");

            println!("Initializing project...");
            println!("  Name: {}", name);
            println!("  Template: {}", template);
            if let Some(d) = dir {
                println!("  Directory: {}", d);
            }

            if let Err(e) = init_project(name, template, dir) {
                eprintln!("Init failed: {}", e);
                process::exit(1);
            }
        }
        Some(("install", sub_m)) => {
            let package = sub_m.get_one::<String>("PACKAGE");
            let global = sub_m.get_flag("GLOBAL");

            println!("Installing dependencies...");
            if let Some(p) = package {
                println!("  Package: {}", p);
            } else {
                println!("  All dependencies");
            }
            println!("  Global: {}", global);

            if let Err(e) = install_dependencies(package, global) {
                eprintln!("Install failed: {}", e);
                process::exit(1);
            }
        }
        Some(("publish", sub_m)) => {
            let input = sub_m.get_one::<String>("INPUT").unwrap();
            let dry_run = sub_m.get_flag("DRY_RUN");

            println!("Publishing package...");
            println!("  Input: {}", input);
            println!("  Dry run: {}", dry_run);

            if let Err(e) = publish_package(input, dry_run) {
                eprintln!("Publish failed: {}", e);
                process::exit(1);
            }
        }
        Some(("repl", sub_m)) => {
            let file = sub_m.get_one::<String>("FILE");

            println!("Starting REPL...");
            if let Some(f) = file {
                println!("  Loading file: {}", f);
            }

            if let Err(e) = start_repl(file) {
                eprintln!("REPL failed: {}", e);
                process::exit(1);
            }
        }
        Some(("version", _)) => {
            println!("Gigli Compiler v0.1.0");
            println!("Target: web, native, wasm");
            println!("License: MIT");
        }
        Some(("doctor", _)) => {
            println!("Checking system requirements...");
            if let Err(e) = check_system() {
                eprintln!("System check failed: {}", e);
                process::exit(1);
            }
        }
        _ => {
            println!("No subcommand provided. Use --help for usage.");
            process::exit(1);
        }
    }
}

fn build_project(_input: &str, _output: &str, _target: &str, _mode: &str, _watch: bool) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement build logic
    println!("Build functionality coming soon!");
    Ok(())
}

fn run_project(_input: &str, _host: &str, _port: &str, _open: bool) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement run logic
    println!("Run functionality coming soon!");
    Ok(())
}

fn start_dev_server(_input: &str, _host: &str, _port: &str, _open: bool) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement dev server logic
    println!("Development server functionality coming soon!");
    Ok(())
}

fn format_code(_input: &str, _check: bool) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement code formatting
    println!("Code formatting functionality coming soon!");
    Ok(())
}

fn lint_code(_input: &str, _fix: bool) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement code linting
    println!("Code linting functionality coming soon!");
    Ok(())
}

fn run_tests(_input: &str, _watch: bool, _coverage: bool) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement test running
    println!("Test running functionality coming soon!");
    Ok(())
}

fn init_project(name: &str, _template: &str, dir: Option<&String>) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use std::path::Path;

    let project_dir = if let Some(d) = dir {
        Path::new(d).join(name)
    } else {
        Path::new(name).to_path_buf()
    };
    if project_dir.exists() {
        return Err(format!("Directory '{}' already exists", project_dir.display()).into());
    }
    fs::create_dir_all(project_dir.join("src"))?;
    fs::create_dir_all(project_dir.join("build"))?;

    // Write starter App.gx
    let app_gx = r#"component App {
    let count: int = 0

    fn increment() {
        count += 1
    }

    <main>
        <h1>Hello, Gigli!</h1>
        <button on:click={increment}>Count: {count}</button>
    </main>

    style {
        main { text-align: center; }
        button { font-size: 1.5em; }
    }
}
"#;
    fs::write(project_dir.join("src/App.gx"), app_gx)?;

    // Write config
    let config = r#"[project]
name = "App"
version = "0.1.0"
"#;
    fs::write(project_dir.join("gigli.toml"), config)?;

    // Write README
    let readme = format!("# {}\n\nCreated with Gigli CLI\n", name);
    fs::write(project_dir.join("README.md"), readme)?;

    println!("Project '{}' created successfully!", name);
    println!("Next steps:");
    println!("  cd {}", project_dir.display());
    println!("  gigli dev");
    Ok(())
}

fn install_dependencies(_package: Option<&String>, _global: bool) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement dependency installation
    println!("Dependency installation functionality coming soon!");
    Ok(())
}

fn publish_package(_input: &str, _dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement package publishing
    println!("Package publishing functionality coming soon!");
    Ok(())
}

fn start_repl(_file: Option<&String>) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement REPL
    println!("REPL functionality coming soon!");
    Ok(())
}

fn check_system() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement system check
    println!("System check functionality coming soon!");
    Ok(())
}
