# Gigli CLI (npm)

> **The official CLI for the Gigli Programming Language**

---

## 🚀 Overview
Gigli is a next-generation, unified programming language and toolchain for building ultra-fast, reactive, and maintainable software for web, native, and beyond. The Gigli CLI enables you to scaffold, develop, build, and deploy Gigli projects with ease.

---

## ✨ Features
- Cross-platform CLI (Windows, macOS, Linux)
- Project scaffolding and templates
- Hot-reload development server
- WASM and native builds
- Built-in formatter, linter, and test runner
- Zero-config, batteries-included

---

## 📦 Installation

Install the Gigli CLI globally via npm:

```bash
npm install -g gigli
```

Or update to the latest version:

```bash
npm update -g gigli
```

---

## ⚡ Quick Start

Create a new Gigli project and start developing:

```bash
gigli init my-app
cd my-app
gigli dev
```

This will scaffold a new project, start the development server, and open your app in the browser.

---

## 🛠️ CLI Usage

| Command                        | Description                                 |
|--------------------------------|---------------------------------------------|
| `gigli init <name>`            | Create a new project                        |
| `gigli dev`                    | Start development server with hot reload     |
| `gigli build`                  | Build for production (WASM/native)           |
| `gigli run <file>`             | Compile and run a Gigli file                 |
| `gigli bundle`                 | Bundle for web deployment                    |
| `gigli fmt <path>`             | Format code                                  |
| `gigli lint <path>`            | Lint code                                    |
| `gigli test <path>`            | Run tests                                    |
| `gigli repl`                   | Start interactive REPL                       |

For all options, run:

```bash
gigli --help
```

---

## 📁 Project Structure

A typical Gigli project looks like:

```
my-app/
├── src/                # Gigli source files (.gx)
│   ├── App.gx          # Main app component
│   └── ...
├── public/             # Static assets
├── dist/               # Build output
├── gigli.toml          # Project configuration
├── package.json        # (optional) JS dependencies
└── README.md           # Project docs
```

---

## ⚙️ Configuration

Project settings are managed in `gigli.toml`:

```toml
[project]
name = "my-app"
version = "0.2.0"
description = "My first Gigli app"

[build]
target = "web"      # or "native"
optimization = "release"

[dev]
port = 3000
host = "localhost"
auto_reload = true
```

---

## 🧑‍💻 Advanced Usage
- **Custom templates:** `gigli init my-app -t <template>`
- **Build for native:** `gigli build --target native`
- **Watch mode:** `gigli build --watch`
- **Format & lint:** `gigli fmt src/ && gigli lint src/`
- **Run tests:** `gigli test src/`
- **REPL:** `gigli repl`

---

## 🩺 Troubleshooting
- **Binary not found:** Ensure your platform is supported (Windows, macOS, Linux, x64/arm64).
- **Permission denied:** Try running with elevated permissions or check your PATH.
- **Other issues:** See [GitHub Issues](https://github.com/jasgigli/gigli/issues) or join the community.

---

## 🤝 Contributing
We welcome contributions! See the [Gigli repository](https://github.com/jasgigli/gigli) for guidelines, issue tracking, and source code.

---

## 📚 Resources
- [Official Docs](https://gigli.vercel.app)
- [GitHub](https://github.com/jasgigli/gigli)
- [Discord](https://discord.gg/gigli)

---

## 📝 License
MIT
