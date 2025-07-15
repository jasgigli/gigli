#!/bin/bash

# GigliOptix Installation Script
# This script installs GigliOptix and its dependencies

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to detect OS
detect_os() {
    case "$(uname -s)" in
        Linux*)     echo "linux";;
        Darwin*)    echo "macos";;
        CYGWIN*)    echo "windows";;
        MINGW*)     echo "windows";;
        MSYS*)      echo "windows";;
        *)          echo "unknown";;
    esac
}

# Function to detect architecture
detect_arch() {
    case "$(uname -m)" in
        x86_64)     echo "x64";;
        aarch64)    echo "arm64";;
        armv7l)     echo "arm";;
        *)          echo "unknown";;
    esac
}

# Function to install Rust
install_rust() {
    print_status "Checking Rust installation..."

    if command_exists rustc && command_exists cargo; then
        local rust_version=$(rustc --version | cut -d' ' -f2)
        print_success "Rust is already installed (version: $rust_version)"
        return 0
    fi

    print_status "Installing Rust..."

    if command_exists curl; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        print_success "Rust installed successfully"
    else
        print_error "curl is required to install Rust. Please install curl first."
        exit 1
    fi
}

# Function to install Node.js
install_nodejs() {
    print_status "Checking Node.js installation..."

    if command_exists node && command_exists npm; then
        local node_version=$(node --version)
        local npm_version=$(npm --version)
        print_success "Node.js is already installed (Node: $node_version, npm: $npm_version)"
        return 0
    fi

    print_status "Installing Node.js..."

    local os=$(detect_os)

    if [ "$os" = "macos" ]; then
        if command_exists brew; then
            brew install node
        else
            print_error "Homebrew is required to install Node.js on macOS. Please install Homebrew first."
            exit 1
        fi
    elif [ "$os" = "linux" ]; then
        if command_exists curl; then
            curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
            sudo apt-get install -y nodejs
        else
            print_error "curl is required to install Node.js. Please install curl first."
            exit 1
        fi
    elif [ "$os" = "windows" ]; then
        print_warning "Please install Node.js manually from https://nodejs.org/"
        print_warning "After installation, run this script again."
        exit 1
    else
        print_error "Unsupported operating system: $os"
        exit 1
    fi

    print_success "Node.js installed successfully"
}

# Function to install LLVM
install_llvm() {
    print_status "Checking LLVM installation..."

    if command_exists clang; then
        local clang_version=$(clang --version | head -n1 | cut -d' ' -f3)
        print_success "LLVM/Clang is already installed (version: $clang_version)"
        return 0
    fi

    print_status "Installing LLVM..."

    local os=$(detect_os)

    if [ "$os" = "macos" ]; then
        if command_exists brew; then
            brew install llvm
        else
            print_error "Homebrew is required to install LLVM on macOS. Please install Homebrew first."
            exit 1
        fi
    elif [ "$os" = "linux" ]; then
        sudo apt-get update
        sudo apt-get install -y clang llvm-dev
    elif [ "$os" = "windows" ]; then
        print_warning "Please install LLVM manually from https://releases.llvm.org/"
        print_warning "After installation, run this script again."
        exit 1
    else
        print_error "Unsupported operating system: $os"
        exit 1
    fi

    print_success "LLVM installed successfully"
}

# Function to build GigliOptix
build_giglioptix() {
    print_status "Building GigliOptix..."

    if [ ! -f "Cargo.toml" ]; then
        print_error "Cargo.toml not found. Please run this script from the GigliOptix root directory."
        exit 1
    fi

    cargo build --release

    if [ $? -eq 0 ]; then
        print_success "GigliOptix built successfully"
    else
        print_error "Failed to build GigliOptix"
        exit 1
    fi
}

# Function to install CLI globally
install_cli() {
    print_status "Installing GigliOptix CLI globally..."

    # Install the CLI crate globally
    cargo install --path src/cli

    if [ $? -eq 0 ]; then
        print_success "GigliOptix CLI installed successfully"
    else
        print_error "Failed to install GigliOptix CLI"
        exit 1
    fi
}

# Function to verify installation
verify_installation() {
    print_status "Verifying installation..."

    if command_exists gigli; then
        local version=$(gigli version 2>/dev/null || echo "unknown")
        print_success "GigliOptix CLI is working (version: $version)"
    else
        print_error "GigliOptix CLI not found in PATH"
        exit 1
    fi

    # Test basic functionality
    print_status "Testing basic functionality..."

    # Create a temporary test file
    cat > test_app.gx << 'EOF'
view TestApp {
    cell count = 0

    render {
        <div>Hello, GigliOptix! Count: {count}</div>
    }
}

fn main() {
    dom::mount("app", <TestApp />);
}
EOF

    # Try to parse the file
    if gigli build test_app.gx --dry-run 2>/dev/null; then
        print_success "Basic functionality test passed"
    else
        print_warning "Basic functionality test failed (this might be expected if the compiler is still in development)"
    fi

    # Clean up
    rm -f test_app.gx
}

# Function to show next steps
show_next_steps() {
    echo
    print_success "🎉 GigliOptix installation completed successfully!"
    echo
    echo "Next steps:"
    echo "1. Create your first project:"
    echo "   gigli init my-app"
    echo
    echo "2. Navigate to your project:"
    echo "   cd my-app"
    echo
    echo "3. Start development:"
    echo "   gigli dev src/main.gx"
    echo
    echo "4. Build for production:"
    echo "   gigli build src/main.gx -o dist"
    echo
    echo "For more information, visit:"
    echo "  📖 Documentation: https://docs.giglioptix.dev"
    echo "  🐙 GitHub: https://github.com/giglioptix/giglioptix"
    echo "  💬 Discord: https://discord.gg/giglioptix"
    echo
    print_success "Happy coding with GigliOptix! 🚀"
}

# Main installation function
main() {
    echo "🚀 GigliOptix Installation Script"
    echo "=================================="
    echo

    # Detect system
    local os=$(detect_os)
    local arch=$(detect_arch)

    print_status "Detected OS: $os"
    print_status "Detected Architecture: $arch"
    echo

    # Install dependencies
    install_rust
    install_nodejs
    install_llvm

    echo

    # Build and install GigliOptix
    build_giglioptix
    install_cli

    echo

    # Verify installation
    verify_installation

    echo

    # Show next steps
    show_next_steps
}

# Run main function
main "$@"
