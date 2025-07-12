# GigliOptix Programming Language

> **The Future of Unified, Reactive, Ultra-Fast Software Development**

---

## 🧠 Executive Summary

**GigliOptix** is a next-generation, compiled programming language designed to unify the **frontend, backend, system programming, and reactive UI development** into a single paradigm. It introduces a **state-first, visually-integrated, self-healing architecture** that eliminates the boundaries between markup, style, logic, and runtime. GigliOptix is engineered to **replace traditional stacks** like HTML, CSS, JavaScript, React, Python, Rust, and Go with a **zero-boilerplate, high-performance reactive compiler model** that targets both **native binaries and WebAssembly**.

---

## 🌍 Why GigliOptix?

Today's developers juggle:

* HTML + CSS + JS for UI
* React/Angular for frontend logic
* Rust/Go/Python for backend
* Complex state management libraries
* Separate DSLs for design, data, and control flow

**GigliOptix ends this fragmentation.** It introduces a **unified language** for describing **state**, **logic**, **UI**, **timing**, **events**, and **style** in one place, with:

* Instant reactivity without frameworks
* Native visual-state debugging
* Self-healing execution flow (via `expect/recover` constructs)
* Reactive-first syntax (`cell`, `flow`, `watch`)
* Full compilation to native or WASM for maximum performance

---

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- LLVM 13+ (for native compilation)
- Node.js 16+ (for web tools)

### Setup

```bash
# Clone the repository
git clone https://github.com/giglioptix/giglioptix.git
cd giglioptix

# Setup development environment
.\scripts\setup.ps1

# Build the project
.\scripts\build.ps1 -Release
```

### Development

```bash
# Build specific components
.\scripts\build.ps1 -Target cli
.\scripts\build.ps1 -Target wasm

# Run tests
.\scripts\build.ps1 -Test

# Format and lint
.\scripts\build.ps1 -Format -Clippy

# Run CLI
cargo run -p gigli-cli -- --help
```

---

## 🏗️ Project Structure

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
├── docs/                  # Documentation
└── Cargo.toml            # Workspace configuration
```

For detailed structure information, see [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md).

---

## 🔣 Example Syntax

```gigli
view App {
  cell counter = 0

  on click: counter += 1

  style:
    background: "#000"
    color: if counter > 10 then "#f00" else "#0f0"

  render:
    text("Clicks: " + counter)
}
```

This code defines **UI, state, event logic, and style** in a single unit, all reactive by default.

---

## ⚙️ Key Constructs

| Keyword              | Description                                        |
| -------------------- | -------------------------------------------------- |
| `cell`               | Reactive state container                           |
| `flow`               | Time-based or event-driven reactive logic          |
| `watch`              | Conditional state observation and triggers         |
| `view`               | Declarative UI and logic component                 |
| `expect` / `recover` | Built-in error handling and self-healing execution |

---

## 🛠️ Compiler Architecture

* **Lexer & Parser**: Converts `.gx` files into AST
* **Static Type Checker** (planned): Ensures safety at compile time
* **IR Generation**: Intermediate representation for optimization
* **Backends**:
  * LLVM: native binaries
  * WASM: browser compatibility
  * Bytecode VM (planned)

---

## 📦 Tooling and Ecosystem

* `gigli build` – Compile `.gx` to binary/WASM
* `gigli dev` – Hot reload + visual state debugger
* `gigli fmt` – Code formatter
* `gigli pkg` – Native package manager

---

## 📌 Use Cases

| Domain                 | GigliOptix Role                     |
| ---------------------- | ----------------------------------- |
| Web Development        | Replaces HTML, CSS, JS, React       |
| SaaS/Backends          | Unified APIs and server logic       |
| UI Prototyping         | Instant visual behavior definition  |
| IoT / Embedded         | Fast, reactive, small runtime apps  |
| Automation / Scripting | Live CLI + fault-tolerant workflows |

---

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test -p gigli-core
cargo test --test lexer_tests
cargo test --test parser_tests

# Run benchmarks
cargo bench
```

---

## 📈 Roadmap

* [x] Syntax spec & lexer
* [x] Project structure & build system
* [ ] MVP interpreter in Rust
* [ ] WASM backend (GigliOptix apps in browser!)
* [ ] REPL + Visual Debugger
* [ ] Gigli Playground (web-based editor)
* [ ] Package Registry
* [ ] Full LLVM backend

---

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust coding standards
- Add tests for new features
- Update documentation
- Run `.\scripts\build.ps1 -Format -Clippy -Test` before submitting

---

## 📄 License

MIT License (c) 2025 GigliOptix Authors

---

## 💬 Join the Movement

We are building the **first language designed for unified, reactive development across all platforms**. Join us in rewriting the future of code.

> GitHub: [github.com/giglioptix](https://github.com/giglioptix)

---

> *"Code that lives, flows, and heals."*
