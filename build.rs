use std::env;
use std::path::PathBuf;

fn main() {
    // Tell Cargo to rerun this script if any of the following files change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/core/src/");
    println!("cargo:rerun-if-changed=src/codegen/llvm/src/");
    println!("cargo:rerun-if-changed=src/codegen/wasm/src/");

    // Set up LLVM configuration for inkwell
    if cfg!(feature = "llvm") {
        // This will be handled by inkwell's build script
        println!("cargo:rustc-cfg=feature=\"llvm\"");
    }

    // Set up WASM configuration
    if cfg!(target_arch = "wasm32") {
        println!("cargo:rustc-cfg=target_arch=\"wasm32\"");
    }

    // Generate version information
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.1.0".to_string());
    println!("cargo:rustc-env=GIGLI_VERSION={}", version);

    // Set up git commit hash if available
    if let Ok(output) = std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
    {
        if output.status.success() {
            let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("cargo:rustc-env=GIGLI_GIT_HASH={}", hash);
        }
    }
}
