# GigliOptix Programming Language Specification (Draft)

## Overview
GigliOptix is a unified programming language for modern web development, blending the strengths of TypeScript, Rust, Svelte, Python, HTML, and CSS. It compiles to WebAssembly (WASM) for high performance, safety, and seamless browser integration.

---

## 1. Syntax & Semantics

### 1.1. Variables & Types
```giglioptix
let count: int = 0
let name: string = "GigliOptix"
let items: List<string> = ["a", "b", "c"]
let data: Option<int> = None
```

### 1.2. Functions
```giglioptix
fn add(a: int, b: int): int {
    return a + b
}

fn greet(name: string): void {
    print("Hello, {name}!")
}
```

### 1.3. Ownership & Borrowing (Rust-inspired)
```giglioptix
fn process(data: &mut List<int>) {
    data.push(42)
}
```

### 1.4. Async/Await (Concurrency)
```giglioptix
async fn fetch_data(url: string): Result<Response, Error> {
    let response = await http.get(url)
    return response
}
```

### 1.5. Reactivity (Svelte-like)
```giglioptix
let count: int = 0

$: doubled = count * 2  // Reactive: updates when count changes
```

### 1.6. UI Components (HTML/CSS-like)
```giglioptix
component Counter {
    let count: int = 0

    fn increment() {
        count += 1
    }

    <div class="counter">
        <h1>Count: {count}</h1>
        <button on:click={increment}>Increment</button>
    </div>

    style {
        .counter {
            color: blue;
            font-size: 2em;
        }
    }
}
```

### 1.7. Pythonic Features
```giglioptix
let squares = [x * x for x in range(10) if x % 2 == 0]
```

### 1.8. Error Handling
```giglioptix
fn safe_divide(a: int, b: int): Result<int, string> {
    if b == 0 {
        return Err("Division by zero")
    }
    return Ok(a / b)
}
```

---

## 2. Type System
- **Primitives**: int, float, bool, string, char
- **Collections**: List<T>, Map<K, V>, Option<T>, Result<T, E>
- **Interfaces & Generics**: Like TypeScript
- **Union Types**: `let value: int | string`
- **Type inference**: Optional, for concise code

---

## 3. Memory Safety
- **Ownership**: Each value has a single owner.
- **Borrowing**: `&` (immutable), `&mut` (mutable) references.
- **No GC**: Compile-time checks prevent leaks and data races.

---

## 4. Concurrency
- **Async/await**: For non-blocking I/O and UI.
- **Thread-safe constructs**: Channels, mutexes, etc., inspired by Rust.

---

## 5. UI & Styling
- **Embedded HTML**: For declarative UI.
- **Scoped CSS**: Via `style { ... }` blocks.
- **Event bindings**: `on:event={handler}` syntax.

---

## 6. Scripting
- **Pythonic constructs**: List comprehensions, concise lambdas, dynamic typing (opt-in).

---

## 7. Compilation & WASM
- Compiles to WebAssembly for browser execution.
- Optimized for size and speed.
- Minimal runtime overhead.

---

## 8. Error Handling & Tooling
- **Clear error messages**: Inspired by TypeScript and Rust.
- **Linting & formatting**: CLI support for code quality.
- **IDE integration**: Language server for autocompletion, diagnostics.

---

## 9. Example: Todo App
```giglioptix
component TodoApp {
    let todos: List<string> = []
    let newTodo: string = ""

    fn add_todo() {
        if newTodo.trim() != "" {
            todos.push(newTodo)
            newTodo = ""
        }
    }

    <main>
        <h1>Todo List</h1>
        <input bind:value={newTodo} placeholder="Add a todo..." />
        <button on:click={add_todo}>Add</button>
        <ul>
            {for todo in todos}
                <li>{todo}</li>
            {/for}
        </ul>
    </main>

    style {
        main { max-width: 400px; margin: auto; }
        h1 { color: #2c3e50; }
        input, button { font-size: 1em; }
    }
}
```

---

## 10. Compiler Optimization Recommendations
- Dead code elimination
- Tree shaking
- Inlining small functions
- Minification of WASM output
- Memory layout optimization
- Compile reactivity to minimal update code
- Scoped CSS compilation

---

## 11. CLI Enhancement Suggestions
- Hot reload for development
- Error overlays in browser
- Bundle analysis and reporting
- Component/template scaffolding
- Integrated testing
- Format/lint commands

---

## 12. Quick Start Guide
```bash
# Install CLI
npm install -g giglioptix

# Create a new project
giglioptix new my-app
cd my-app

giglioptix dev      # Development mode
giglioptix build    # Production build
giglioptix deploy   # Deploy
```

---

## 13. Testing & Validation
- Unit and UI tests
- WASM validation in browser
- CLI command checks
- Cross-browser testing

---

## 14. Next Steps
- Review and refine syntax/features
- Implement and test sample programs
- Optimize compiler for WASM
- Enhance CLI for developer experience
- Expand documentation
