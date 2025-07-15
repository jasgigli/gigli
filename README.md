# GigliOptix Programming Language

> **The Future of Unified, Reactive, Ultra-Fast Software Development**

---
   gigli init my-app
   cd my-app
   gigli dev src/main.gx
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

### Installation

```bash
# Install GigliOptix CLI globally
npm install -g @giglioptix/cli

# Or build from source
git clone https://github.com/giglioptix/giglioptix.git
cd giglioptix
cargo install --path src/cli
```

### Create Your First Project

```bash
# Create a new project
gigli init my-app

# Navigate to project
cd my-app

# Start development server
gigli dev src/main.gx

# Build for production
gigli build src/main.gx -o dist

# Run in browser
gigli run src/main.gx
```

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

## ⚙️ Key Language Features

| Feature | Description | Example |
|---------|-------------|---------|
| **Reactive Cells** | Automatic state management | `cell count = 0` |
| **Flows** | Lifecycle and event handling | `flow onMount { init() }` |
| **Views** | Declarative UI components | `view Button { ... }` |
| **Classes** | Object-oriented programming | `class Service { ... }` |
| **Types** | Strong type system | `type User = { id: number }` |
| **Imports** | Module system | `import { dom } from "std/web"` |
| **Async/Await** | Modern async programming | `async fn loadData() { ... }` |
| **Template Literals** | String interpolation | `` `Hello ${name}` `` |
| **JSX-like Syntax** | Declarative UI | `<div class="app">{content}</div>` |

---

## 🛠️ Development Workflow

### 1. Development Mode
```bash
gigli dev src/main.gx --port 3000 --open
```
- Hot reload on file changes
- Live error reporting
- Browser auto-refresh

### 2. Building for Production
```bash
gigli build src/main.gx -o dist --mode release --minify
```
- Optimized WebAssembly output
- Minified JavaScript
- Source maps for debugging

### 3. Testing
```bash
gigli test src/ --watch --coverage
```
- Unit and integration tests
- Coverage reporting
- Watch mode for TDD

### 4. Code Quality
```bash
gigli fmt src/ --check
gigli lint src/ --fix
```
- Automatic code formatting
- Linting with auto-fix
- Consistent code style

---

## 📦 Project Structure

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

## 🔧 CLI Commands

| Command | Description |
|---------|-------------|
| `gigli init <name>` | Create new project |
| `gigli dev <file>` | Start development server |
| `gigli build <file>` | Build for production |
| `gigli run <file>` | Compile and run |
| `gigli bundle <file>` | Bundle for web deployment |
| `gigli fmt <path>` | Format code |
| `gigli lint <path>` | Lint code |
| `gigli test <path>` | Run tests |
| `gigli repl` | Start interactive REPL |

---

## 🎯 Use Cases

| Domain | GigliOptix Role | Traditional Stack |
|--------|----------------|-------------------|
| **Web Development** | Single language for everything | HTML + CSS + JS + React |
| **SaaS Applications** | Unified frontend and backend | React + Node.js + Express |
| **UI Prototyping** | Instant visual behavior | Figma + Code |
| **IoT Dashboards** | Reactive, real-time UIs | React + WebSocket |
| **CLI Tools** | Cross-platform utilities | Python + Click |

---

## 🧪 Examples

Check out the `examples/` directory for complete applications:

- **`simple-counter.gx`** - Basic reactive counter
- **`modern-app.gx`** - Full-featured todo app with API
- **`game.gx`** - Simple browser game
- **`calculator.gx`** - Interactive calculator
- **`weather.gx`** - Weather dashboard

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
git clone https://github.com/giglioptix/giglioptix.git
cd giglioptix

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

## 📄 License

MIT License (c) 2025 GigliOptix Authors

---

## 💬 Community

- **GitHub**: [github.com/giglioptix](https://github.com/giglioptix)
- **Discord**: [discord.gg/giglioptix](https://discord.gg/giglioptix)
- **Twitter**: [@giglioptix](https://twitter.com/giglioptix)
- **Documentation**: [docs.giglioptix.dev](https://docs.giglioptix.dev)

---

> *"Code that lives, flows, and heals."*

**Ready to build the future of web development?** [Get started now →](https://docs.giglioptix.dev/getting-started)
