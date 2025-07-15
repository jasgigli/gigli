# Gigli Programming Language

> **The Future of Unified, Reactive, Ultra-Fast Software Development**

---

## 🚀 Overview
Gigli is a next-generation, unified programming language and toolchain for building ultra-fast, reactive, and maintainable software for web, native, and beyond. The Gigli CLI enables you to scaffold, develop, build, and deploy Gigli projects with ease.

---

## ✨ Features
- Unified language for frontend, backend, and system programming
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

Or build from source:

```bash
git clone https://github.com/jasgigli/gigli.git
cd gigli
cargo install --path src/cli
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
version = "0.1.0"
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

## 🔣 Modern Syntax Examples

### Simple Counter App

```gx
view Counter {
    cell count = 0
    cell name = "World"

    on click: count += 1
    on doubleClick: count = 0

    cell isEven = count % 2 === 0
    cell message = `Hello, ${name}! Count: ${count}`

    style {
        textAlign: "center",
        padding: "20px",
        backgroundColor: if isEven then "#e8f5e8" else "#fff3cd"
    }

    render {
        <div class="counter">
            <h1>{message}</h1>
            <div class="count">{count}</div>
            <button onClick={() => count += 1}>
                Increment
            </button>
        </div>
    }
}
```

### Modern Web App with Classes

```gx
// Type definitions
type User = {
    id: number,
    name: string,
    email: string
};

// Service class
class UserService {
    private baseUrl: string;

    constructor(baseUrl: string = "https://api.example.com") {
        this.baseUrl = baseUrl;
    }

    async getUsers(): Promise<User[]> {
        const response = await http.get(`${this.baseUrl}/users`);
        return response.json();
    }
}

// State management
class UserStore {
    cell users = [];
    cell loading = false;
    cell error = null;

    constructor(private service: UserService) {}

    async loadUsers() {
        this.loading = true;
        try {
            this.users = await this.service.getUsers();
        } catch (err) {
            this.error = err.message;
        } finally {
            this.loading = false;
        }
    }
}

// UI Component
view UserList {
    cell store = new UserStore(new UserService());

    flow onMount {
        store.loadUsers();
    }

    render {
        <div class="user-list">
            {store.loading && <div>Loading...</div>}
            {store.error && <div class="error">Error: {store.error}</div>}
            {store.users.map(user => (
                <div key={user.id} class="user-item">
                    <h3>{user.name}</h3>
                    <p>{user.email}</p>
                </div>
            ))}
        </div>
    }
}
```

---

## 📦 Project Structure Example

```
my-app/
├── src/
│   ├── main.gx              # Entry point
│   ├── components/          # UI components
│   │   ├── Header.gx
│   │   ├── Footer.gx
│   │   └── MainContent.gx
│   ├── services/           # Business logic
│   │   └── ApiService.gx
│   ├── types/              # Type definitions
│   │   └── models.gx
│   └── utils/              # Utility functions
│       └── helpers.gx
├── public/                 # Static assets
├── dist/                   # Build output
├── tests/                  # Test files
├── gigli.config.json       # Project configuration
└── package.json           # Dependencies
```

---

## 📈 Roadmap

### ✅ Completed
- [x] Core language syntax and AST
- [x] Lexer and parser implementation
- [x] Basic CLI interface
- [x] WASM compilation backend
- [x] Project templates

### 🚧 In Progress
- [ ] Type system implementation
- [ ] Standard library modules
- [ ] Development server with hot reload
- [ ] Language Server Protocol (LSP)

### 🔮 Planned
- [ ] Visual debugger
- [ ] Package registry
- [ ] Full LLVM backend
- [ ] Mobile compilation targets
- [ ] IDE extensions

---

## 🤝 Contributing

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Setup

```bash
# Clone and setup
git clone https://github.com/jasgigli/gigli.git
cd gigli

# Install dependencies
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

---

## 📝 License

MIT License (c) 2025 Gigli Authors

---

## 💬 Community

- **GitHub**: [github.com/jasgigli/gigli](https://github.com/jasgigli/gigli)
- **Discord**: [discord.gg/gigli](https://discord.gg/gigli)
- **Twitter**: [@gigli](https://twitter.com/gigli)
- **Documentation**: [gigli.vercel.app](https://gigli.vercel.app)

---

> *"Code that lives, flows, and heals."*

**Ready to build the future of web development?** [Get started now →](https://gigli.vercel.app/getting-started)
