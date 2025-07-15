#!/usr/bin/env pwsh

Write-Host "Setting up GigliOptix development environment..." -ForegroundColor Green

# Check if Rust is installed
if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
    Write-Host "Rust is not installed. Installing Rust..." -ForegroundColor Yellow
    Invoke-WebRequest -Uri "https://sh.rustup.rs" -OutFile "rustup-init.sh"
    bash rustup-init.sh -y
    Remove-Item "rustup-init.sh"
    $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
} else {
    Write-Host "Rust is already installed." -ForegroundColor Green
}

# Check if wasm-pack is installed
if (-not (Get-Command wasm-pack -ErrorAction SilentlyContinue)) {
    Write-Host "Installing wasm-pack..." -ForegroundColor Yellow
    cargo install wasm-pack
} else {
    Write-Host "wasm-pack is already installed." -ForegroundColor Green
}

# Check if LLVM is available
$llvmConfig = Get-Command llvm-config -ErrorAction SilentlyContinue
if (-not $llvmConfig) {
    Write-Host "LLVM not found. You may need to install LLVM for native compilation." -ForegroundColor Yellow
    Write-Host "Download from: https://releases.llvm.org/download.html" -ForegroundColor Yellow
    Write-Host "Set LLVM_SYS_130_PREFIX to your LLVM installation path" -ForegroundColor Yellow
} else {
    Write-Host "LLVM is available." -ForegroundColor Green
}

# Install development tools
Write-Host "Installing development tools..." -ForegroundColor Yellow
cargo install cargo-watch
cargo install cargo-audit

# Create necessary directories
Write-Host "Creating project directories..." -ForegroundColor Yellow
$directories = @(
    "docs",
    "dist",
    "build"
)

foreach ($dir in $directories) {
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
        Write-Host "Created directory: $dir" -ForegroundColor Green
    }
}

# Build the project
Write-Host "Building project..." -ForegroundColor Yellow
cargo build

if ($LASTEXITCODE -eq 0) {
    Write-Host "Setup completed successfully!" -ForegroundColor Green
    Write-Host "You can now run: .\scripts\build.ps1 -Help" -ForegroundColor Cyan
} else {
    Write-Host "Build failed. Please check the errors above." -ForegroundColor Red
    exit 1
}
