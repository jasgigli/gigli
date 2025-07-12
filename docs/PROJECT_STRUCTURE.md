# GigliOptix Project Structure

This document describes the organized and maintainable structure of the GigliOptix programming language project.

## Overview

The project follows a modular workspace structure with clear separation of concerns:

```
giglioptix/
├── src/                    # Source code
│   ├── core/              # Core language implementation
│   ├── cli/               # Command-line interface
│   ├── lsp/               # Language Server Protocol
│   ├── codegen/           # Code generation backends
│   │   ├── llvm/          # LLVM backend
│   │   └── wasm/          # WebAssembly backend
│   ├── runtime/           # Runtime implementations
│   │   └── js/            # JavaScript runtime
│   └── std/               # Standard library
├── tests/                 # Test files
├── examples/              # Example programs
├── web/                   # Web tools and playground
├── scripts/               # Build and utility scripts
├── public/                # Public assets
├── docs/                  # Documentation (planned)
└── Cargo.toml            # Workspace configuration
```

## Module Descriptions

### Core (`src/core/`)
- **Purpose**: Core language implementation
- **Contains**: Lexer, parser, AST, semantic analysis, IR generation
- **Dependencies**: Minimal external dependencies
- **Exports**: Language primitives, AST nodes, parsing functions

### CLI (`src/cli/`)
- **Purpose**: Command-line interface for the compiler
- **Contains**: Main binary, CLI argument parsing, build orchestration
- **Dependencies**: All other crates, clap for CLI
- **Exports**: `gigli` binary

### LSP (`src/lsp/`)
- **Purpose**: Language Server Protocol implementation
- **Contains**: LSP server, language features (completion, diagnostics)
- **Dependencies**: Core crate, tower-lsp
- **Exports**: `gigli-lsp` binary

### Code Generation (`src/codegen/`)

#### LLVM Backend (`src/codegen/llvm/`)
- **Purpose**: Generate native binaries via LLVM
- **Contains**: LLVM IR generation, optimization passes
- **Dependencies**: Core crate, inkwell
- **Exports**: LLVM code generation functions

#### WASM Backend (`src/codegen/wasm/`)
- **Purpose**: Generate WebAssembly modules
- **Contains**: WASM generation, browser integration
- **Dependencies**: Core crate, wasm-bindgen
- **Exports**: WASM code generation functions

### Runtime (`src/runtime/`)

#### JavaScript Runtime (`src/runtime/js/`)
- **Purpose**: JavaScript runtime for browser execution
- **Contains**: JS interop, DOM manipulation, event handling
- **Dependencies**: Core crate, wasm-bindgen, web-sys
- **Exports**: JavaScript runtime functions

### Standard Library (`src/std/`)
- **Purpose**: Standard library functions and modules
- **Contains**: Built-in functions, browser APIs, utilities
- **Dependencies**: Core crate, runtime crates
- **Exports**: Standard library modules

## Build System

### Workspace Configuration
- **Root Cargo.toml**: Defines workspace, shared dependencies, metadata
- **Individual Cargo.toml**: Each crate has its own configuration
- **Build.rs**: Handles build-time configuration and code generation

### Dependencies
- **Workspace Dependencies**: Shared across all crates
- **Crate Dependencies**: Specific to each crate's needs
- **Feature Flags**: Enable/disable optional functionality

## Development Workflow

### Building
```bash
# Build all crates
cargo build

# Build specific crate
cargo build -p gigli-cli

# Build with optimizations
cargo build --release
```

### Testing
```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p gigli-core

# Run integration tests
cargo test --test "*"
```

### Development
```bash
# Run CLI
cargo run -p gigli-cli -- [args]

# Run LSP
cargo run -p gigli-lsp

# Check formatting
cargo fmt

# Run clippy
cargo clippy
```

## File Naming Conventions

- **Rust files**: `snake_case.rs`
- **Directories**: `snake_case/`
- **Crates**: `kebab-case` (in Cargo.toml), `snake_case` (in code)
- **Binaries**: `kebab-case`
- **Tests**: `test_name.rs` or `mod test_name` in source files

## Benefits of This Structure

1. **Modularity**: Each component is self-contained
2. **Maintainability**: Clear separation of concerns
3. **Scalability**: Easy to add new backends or features
4. **Testing**: Isolated testing per module
5. **Documentation**: Clear organization for documentation
6. **Dependencies**: Minimal, well-defined dependency graph
