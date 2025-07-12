#!/usr/bin/env pwsh

param(
    [string]$Target = "all",
    [switch]$Release,
    [switch]$Clean,
    [switch]$Test,
    [switch]$Format,
    [switch]$Clippy,
    [switch]$Help
)

if ($Help) {
    Write-Host @"
GigliOptix Build Script

Usage: .\scripts\build.ps1 [options]

Options:
    -Target <target>     Build target (all, core, cli, lsp, wasm, llvm)
    -Release            Build in release mode
    -Clean              Clean build artifacts
    -Test               Run tests
    -Format             Format code
    -Clippy             Run clippy
    -Help               Show this help

Examples:
    .\scripts\build.ps1                    # Build all in debug mode
    .\scripts\build.ps1 -Release           # Build all in release mode
    .\scripts\build.ps1 -Target cli        # Build only CLI
    .\scripts\build.ps1 -Test              # Run all tests
    .\scripts\build.ps1 -Format -Clippy    # Format and lint code
"@
    exit 0
}

# Set build mode
$BuildMode = if ($Release) { "--release" } else { "" }

# Clean if requested
if ($Clean) {
    Write-Host "Cleaning build artifacts..." -ForegroundColor Yellow
    cargo clean
}

# Format if requested
if ($Format) {
    Write-Host "Formatting code..." -ForegroundColor Yellow
    cargo fmt
}

# Run clippy if requested
if ($Clippy) {
    Write-Host "Running clippy..." -ForegroundColor Yellow
    cargo clippy $BuildMode
}

# Build based on target
switch ($Target.ToLower()) {
    "all" {
        Write-Host "Building all crates..." -ForegroundColor Green
        cargo build $BuildMode
    }
    "core" {
        Write-Host "Building core crate..." -ForegroundColor Green
        cargo build $BuildMode -p gigli-core
    }
    "cli" {
        Write-Host "Building CLI crate..." -ForegroundColor Green
        cargo build $BuildMode -p gigli-cli
    }
    "lsp" {
        Write-Host "Building LSP crate..." -ForegroundColor Green
        cargo build $BuildMode -p gigli-lsp
    }
    "wasm" {
        Write-Host "Building WASM backend..." -ForegroundColor Green
        cargo build $BuildMode -p gigli-codegen-wasm
    }
    "llvm" {
        Write-Host "Building LLVM backend..." -ForegroundColor Green
        cargo build $BuildMode -p gigli-codegen-llvm
    }
    default {
        Write-Host "Unknown target: $Target" -ForegroundColor Red
        Write-Host "Use -Help for available options" -ForegroundColor Yellow
        exit 1
    }
}

# Run tests if requested
if ($Test) {
    Write-Host "Running tests..." -ForegroundColor Green
    cargo test $BuildMode
}

Write-Host "Build completed successfully!" -ForegroundColor Green
