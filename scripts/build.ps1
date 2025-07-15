#!/usr/bin/env pwsh

# Clean up previous build artifacts, binaries, packages, and demo app
Write-Host "[0/8] Cleaning previous build, binaries, npm packages, and demo app..."

# Remove my-app if exists
if (Test-Path "my-app") { Remove-Item -Recurse -Force "my-app" }

# Remove gigli-npm/bin/gigli.exe if exists
$binExe = "gigli-npm/bin/gigli.exe"
if (Test-Path $binExe) { Remove-Item -Force $binExe }

# Remove all .tgz files in gigli-npm
Get-ChildItem -Path "gigli-npm" -Filter "*.tgz" | Remove-Item -Force

# Remove Cargo.lock if exists
if (Test-Path "Cargo.lock") { Remove-Item -Force "Cargo.lock" }

# Remove /target directory if exists
if (Test-Path "target") { Remove-Item -Recurse -Force "target" }

# Build the Gigli compiler and automate the full developer flow

Write-Host "[1/8] Building Gigli compiler with cargo..."
cargo build --release
if ($LASTEXITCODE -ne 0) { Write-Error "Cargo build failed."; exit 1 }

$exePath = "target/release/gigli.exe"
$destPath = "gigli-npm/bin/gigli.exe"

Write-Host "[2/8] Copying gigli.exe to gigli-npm/bin..."
if (!(Test-Path $exePath)) { Write-Error "gigli.exe not found at $exePath"; exit 1 }
Copy-Item $exePath $destPath -Force

Write-Host "[3/8] Packing npm module in gigli-npm..."
Push-Location gigli-npm
npm pack
if ($LASTEXITCODE -ne 0) { Write-Error "npm pack failed."; Pop-Location; exit 1 }
$tgz = Get-ChildItem *.tgz | Select-Object -First 1
Pop-Location

if (-not $tgz) { Write-Error "No .tgz file found in gigli-npm after npm pack."; exit 1 }

Write-Host "[4/8] Uninstalling previous global gigli npm package (if any)..."
npm uninstall -g gigli

Write-Host "[5/8] Installing npm package globally..."
npm install -g "./gigli-npm/$($tgz.Name)"
if ($LASTEXITCODE -ne 0) { Write-Error "npm install -g failed."; exit 1 }

Write-Host "[6/8] Initializing new Gigli project in root as 'my-app'..."
gigli init my-app
if ($LASTEXITCODE -ne 0) { Write-Error "gigli init failed."; exit 1 }

Write-Host "[7/8] Starting development server in my-app..."
Set-Location my-app
gigli dev &
Start-Sleep -Seconds 2

$localhostUrl = "http://localhost:3000"
Write-Host "[8/8] Development server started. Click to open: $localhostUrl"
Write-Host "(If not clickable, copy and paste into your browser.)"

# Optionally, open browser automatically (uncomment if desired)
# Start-Process $localhostUrl
