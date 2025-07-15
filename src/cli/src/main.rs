//! Main entry point for the Gigli compiler CLI

use gigli_core::ir::generator::{generate_ir, IRModule};
use gigli_codegen_wasm::emit_wasm;
use std::path::Path;
use std::process;
use std::path::PathBuf;

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
            let source = std::fs::read_to_string(input).unwrap();
            let mut lexer = gigli_core::lexer::Lexer::new(&source);
            let tokens = lexer.tokenize().unwrap();
            let mut parser = gigli_core::parser::Parser::new(tokens);
            let ast = parser.parse().unwrap();

            // === 2. Generate IR ===
            let ir = gigli_core::ir::generator::generate_ir(&ast);

            // === 3. Emit WASM ===
            let wasm_path = "main.wasm";
            gigli_codegen_wasm::emit_wasm(&ir, wasm_path);

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

fn start_dev_server(input: &str, host: &str, port: &str, open: bool) -> Result<(), Box<dyn std::error::Error>> {
    use std::process::{Command, Stdio};
    use std::thread;
    use std::time::Duration;
    use std::fs;
    use std::path::Path;

    // === 1. Parse source code ===
    let source = std::fs::read_to_string(input)?;
    let mut lexer = gigli_core::lexer::Lexer::new(&source);
    let tokens = lexer.tokenize()?;
    let mut parser = gigli_core::parser::Parser::new(tokens);
    let ast = parser.parse()?;

    // === 2. Generate IR ===
    let ir = gigli_core::ir::generator::generate_ir(&ast);

    // === 3. Emit WASM ===
    let out_dir = "dist";
    let wasm_path = Path::new(out_dir).join("main.wasm");
    fs::create_dir_all(out_dir)?;
    gigli_codegen_wasm::emit_wasm(&ir, wasm_path.to_str().unwrap());

    // === 4. Bundle for web ===
    if let Err(e) = std::panic::catch_unwind(|| {
        bundle::bundle_for_web(wasm_path.to_str().unwrap(), out_dir);
    }) {
        eprintln!("\n[Error] Failed to bundle for web: {:?}", e);
        eprintln!("This is often caused by the WASM file being locked. Please close any programs using dist/main.wasm and try again.");
        return Err("Failed to bundle for web".into());
    }

    // === 5. Start Node.js dev server ===
    let dev_server_filename = "dev-server.js";
    let dev_server_path_check = Path::new("dist").join(dev_server_filename);
    if !dev_server_path_check.exists() {
        return Err("dev-server.js not found in output directory".into());
    }

    // Spawn the Node.js server
    let port_num = port.parse::<u16>().unwrap_or(3000);
    let mut child = match Command::new("node")
        .arg(dev_server_filename) // Pass only the filename, as CWD is 'dist'
        .current_dir("dist")
        .env("PORT", port)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn() {
        Ok(child) => child,
        Err(e) => {
            eprintln!("\n[Error] Failed to start Node.js dev server: {}", e);
            eprintln!("Make sure Node.js is installed and available in your PATH.");
            return Err("Failed to start dev server".into());
        }
    };

    // Wait a moment for the server to start
    thread::sleep(Duration::from_millis(800));

    // Optionally open the browser
    if open {
        let url = format!("http://{}:{}", host, port_num);
        if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", "start", &url]).spawn().ok();
        } else if cfg!(target_os = "macos") {
            Command::new("open").arg(&url).spawn().ok();
        } else if cfg!(target_os = "linux") {
            Command::new("xdg-open").arg(&url).spawn().ok();
        }
    }

    println!("Development server running at http://{}:{}", host, port_num);
    println!("Press Ctrl+C to stop.");

    // Wait for the server process to exit
    let status = child.wait()?;
    if !status.success() {
        return Err("Dev server exited with error".into());
    }

    Ok(())
}

fn format_code(input: &str, check: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Formatting file: {}", input);
    let source = std::fs::read_to_string(input)?;

    // 1. Lexing
    let mut lexer = gigli_core::lexer::Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            println!("❌ Lexing error: {}", e);
            process::exit(1);
        }
    };

    // 2. Parsing
    let mut parser = gigli_core::parser::Parser::new(tokens);
    let _ast = match parser.parse() {
        Ok(a) => a,
        Err(e) => {
            println!("❌ Parsing error: {}", e);
            process::exit(1);
        }
    };

    if check {
        println!("✅ File is well-formed.");
    } else {
        // TODO: Implement pretty-printing of the AST
        println!("✅ File is well-formed. Pretty-printing coming soon!");
        // For now, just write the original source back
        // In a real implementation, we'd pretty-print the AST.
        // std::fs::write(input, source)?;
    }

    Ok(())
}

fn lint_code(input: &str, _fix: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Checking file: {}", input);
    let source = std::fs::read_to_string(input)?;

    // 1. Lexing
    let mut lexer = gigli_core::lexer::Lexer::new(&source);
    let tokens = lexer.tokenize()?;

    // 2. Parsing
    let mut parser = gigli_core::parser::Parser::new(tokens);
    let ast = parser.parse()?;

    // 3. Semantic Analysis
    let mut analyzer = gigli_core::semantic::SemanticAnalyzer::new();
    analyzer.analyze(&ast);

    if analyzer.errors.is_empty() {
        println!("✅ No errors found.");
    } else {
        println!("❌ Found {} errors:", analyzer.errors.len());
        for error in analyzer.errors {
            println!("  - {}", error);
        }
        process::exit(1);
    }

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

    let project_dir = dir.map_or(PathBuf::from(name), |d| PathBuf::from(d));
    if project_dir.exists() {
        return Err(format!("Directory '{}' already exists.", project_dir.display()).into());
    }
    fs::create_dir_all(&project_dir.join("src"))?;

    let gigli_toml = format!(r#"[project]
name = "{}"
version = "0.1.0"
"#, name);
    fs::write(project_dir.join("gigli.toml"), gigli_toml)?;

    let app_gx_content = r#"
component App {
    fn main() {
        <h1>Hello, world!</h1>
    }
}
"#;
    fs::write(project_dir.join("src/App.gx"), app_gx_content)?;

    println!("✅ Project '{}' created successfully.", name);
    println!("To get started, run:");
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
