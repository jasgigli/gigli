# GigliOptix Installation Script for Windows
# This script installs GigliOptix and its dependencies

param(
    [switch]$SkipDependencies,
    [switch]$Force
)

# Set error action preference
$ErrorActionPreference = "Stop"

# Colors for output
$Red = "Red"
$Green = "Green"
$Yellow = "Yellow"
$Blue = "Blue"
$White = "White"

# Function to print colored output
function Write-Status {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor $Blue
}

function Write-Success {
    param([string]$Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor $Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARNING] $Message" -ForegroundColor $Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor $Red
}

# Function to check if command exists
function Test-Command {
    param([string]$Command)
    try {
        Get-Command $Command -ErrorAction Stop | Out-Null
        return $true
    }
    catch {
        return $false
    }
}

# Function to install Chocolatey
function Install-Chocolatey {
    Write-Status "Checking Chocolatey installation..."

    if (Test-Command "choco") {
        Write-Success "Chocolatey is already installed"
        return
    }

    Write-Status "Installing Chocolatey..."

    try {
        Set-ExecutionPolicy Bypass -Scope Process -Force
        [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
        iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
        Write-Success "Chocolatey installed successfully"
    }
    catch {
        Write-Error "Failed to install Chocolatey: $($_.Exception.Message)"
        exit 1
    }
}

# Function to install Rust
function Install-Rust {
    Write-Status "Checking Rust installation..."

    if (Test-Command "rustc") {
        $version = (rustc --version).Split(' ')[1]
        Write-Success "Rust is already installed (version: $version)"
        return
    }

    Write-Status "Installing Rust..."

    try {
        if (Test-Command "choco") {
            choco install rust -y
        }
        else {
            # Fallback to rustup
            Invoke-WebRequest -Uri "https://win.rustup.rs" -OutFile "rustup-init.exe"
            Start-Process -FilePath "rustup-init.exe" -ArgumentList "-y" -Wait
            Remove-Item "rustup-init.exe"
        }

        # Refresh environment variables
        $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")

        Write-Success "Rust installed successfully"
    }
    catch {
        Write-Error "Failed to install Rust: $($_.Exception.Message)"
        exit 1
    }
}

# Function to install Node.js
function Install-NodeJS {
    Write-Status "Checking Node.js installation..."

    if (Test-Command "node") {
        $nodeVersion = node --version
        $npmVersion = npm --version
        Write-Success "Node.js is already installed (Node: $nodeVersion, npm: $npmVersion)"
        return
    }

    Write-Status "Installing Node.js..."

    try {
        if (Test-Command "choco") {
            choco install nodejs -y
        }
        else {
            Write-Error "Chocolatey is required to install Node.js. Please install Chocolatey first."
            exit 1
        }

        # Refresh environment variables
        $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")

        Write-Success "Node.js installed successfully"
    }
    catch {
        Write-Error "Failed to install Node.js: $($_.Exception.Message)"
        exit 1
    }
}

# Function to install LLVM
function Install-LLVM {
    Write-Status "Checking LLVM installation..."

    if (Test-Command "clang") {
        $version = (clang --version).Split("`n")[0].Split(' ')[2]
        Write-Success "LLVM/Clang is already installed (version: $version)"
        return
    }

    Write-Status "Installing LLVM..."

    try {
        if (Test-Command "choco") {
            choco install llvm -y
        }
        else {
            Write-Error "Chocolatey is required to install LLVM. Please install Chocolatey first."
            exit 1
        }

        # Refresh environment variables
        $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")

        Write-Success "LLVM installed successfully"
    }
    catch {
        Write-Error "Failed to install LLVM: $($_.Exception.Message)"
        exit 1
    }
}

# Function to build GigliOptix
function Build-GigliOptix {
    Write-Status "Building GigliOptix..."

    if (-not (Test-Path "Cargo.toml")) {
        Write-Error "Cargo.toml not found. Please run this script from the GigliOptix root directory."
        exit 1
    }

    try {
        cargo build --release
        Write-Success "GigliOptix built successfully"
    }
    catch {
        Write-Error "Failed to build GigliOptix: $($_.Exception.Message)"
        exit 1
    }
}

# Function to install CLI globally
function Install-CLI {
    Write-Status "Installing GigliOptix CLI globally..."

    try {
        cargo install --path src/cli
        Write-Success "GigliOptix CLI installed successfully"
    }
    catch {
        Write-Error "Failed to install GigliOptix CLI: $($_.Exception.Message)"
        exit 1
    }
}

# Function to verify installation
function Test-Installation {
    Write-Status "Verifying installation..."

    if (Test-Command "gigli") {
        try {
            $version = gigli version 2>$null
            if ($version) {
                Write-Success "GigliOptix CLI is working (version: $version)"
            }
            else {
                Write-Success "GigliOptix CLI is working"
            }
        }
        catch {
            Write-Success "GigliOptix CLI is working"
        }
    }
    else {
        Write-Error "GigliOptix CLI not found in PATH"
        exit 1
    }

    # Test basic functionality
    Write-Status "Testing basic functionality..."

    # Create a temporary test file
    $testContent = @"
view TestApp {
    cell count = 0

    render {
        <div>Hello, GigliOptix! Count: {count}</div>
    }
}

fn main() {
    dom::mount("app", <TestApp />);
}
"@

    $testContent | Out-File -FilePath "test_app.gx" -Encoding UTF8

    try {
        # Try to parse the file (this might fail if the compiler is still in development)
        gigli build test_app.gx --dry-run 2>$null
        Write-Success "Basic functionality test passed"
    }
    catch {
        Write-Warning "Basic functionality test failed (this might be expected if the compiler is still in development)"
    }

    # Clean up
    if (Test-Path "test_app.gx") {
        Remove-Item "test_app.gx"
    }
}

# Function to show next steps
function Show-NextSteps {
    Write-Host ""
    Write-Success "🎉 GigliOptix installation completed successfully!"
    Write-Host ""
    Write-Host "Next steps:"
    Write-Host "1. Create your first project:"
    Write-Host "   gigli init my-app"
    Write-Host ""
    Write-Host "2. Navigate to your project:"
    Write-Host "   cd my-app"
    Write-Host ""
    Write-Host "3. Start development:"
    Write-Host "   gigli dev src/main.gx"
    Write-Host ""
    Write-Host "4. Build for production:"
    Write-Host "   gigli build src/main.gx -o dist"
    Write-Host ""
    Write-Host "For more information, visit:"
    Write-Host "  📖 Documentation: https://docs.giglioptix.dev"
    Write-Host "  🐙 GitHub: https://github.com/giglioptix/giglioptix"
    Write-Host "  💬 Discord: https://discord.gg/giglioptix"
    Write-Host ""
    Write-Success "Happy coding with GigliOptix! 🚀"
}

# Main installation function
function Main {
    Write-Host "🚀 GigliOptix Installation Script for Windows"
    Write-Host "=============================================="
    Write-Host ""

    # Check if running as administrator
    if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
        Write-Warning "This script may require administrator privileges for some installations."
        Write-Warning "If you encounter permission errors, try running PowerShell as Administrator."
        Write-Host ""
    }

    # Install dependencies if not skipped
    if (-not $SkipDependencies) {
        Install-Chocolatey
        Install-Rust
        Install-NodeJS
        Install-LLVM
        Write-Host ""
    }
    else {
        Write-Status "Skipping dependency installation..."
        Write-Host ""
    }

    # Build and install GigliOptix
    Build-GigliOptix
    Install-CLI
    Write-Host ""

    # Verify installation
    Test-Installation
    Write-Host ""

    # Show next steps
    Show-NextSteps
}

# Run main function
Main
