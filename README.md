# Gigli Programming Language (v0.2.0)

**Gigli** is a modern, unified programming language for building high-performance, reactive web applications. It combines logic, markup, and styling into a single, elegant component format. By compiling to WebAssembly (WASM), Gigli delivers exceptional speed and safety, aiming to drastically simplify the web development workflow.

## Guiding Principles

1.  **Unified & Simple:** A single `.gx` file defines a complete component. No more context-switching between HTML, CSS, and JavaScript.
2.  **Ergonomic & Readable:** The syntax is designed to be intuitive and minimize boilerplate, making code a joy to write and maintain.
3.  **Reactive by Default:** State management is a core language feature, not a library. The UI automatically reacts to state changes.
4.  **Performant & Safe:** Leveraging a Rust-based compiler and WASM target, Gigli provides compile-time safety guarantees and near-native performance.

---

## ⚡ Quick Start

1.  **Install the CLI:**
    ```bash
    # Coming soon to a package manager near you!
    # For now, build from source:
    git clone https://github.com/your-repo/gigli.git
    cd gigli
    cargo install --path src/cli
    ```

2.  **Create a new project:**
    ```bash
    gigli new my-app
    cd my-app
    ```

3.  **Start the dev server:**
    ```bash
    gigli dev
    ```
    This starts a hot-reloading development server and opens your new app in the browser.

---

## 📖 Gigli by Example: A Modern Todo App

This example demonstrates how all the features of Gigli work together in a single file.

**File: `src/components/TodoApp.gx`**
```gigli
// Import other components (example)
// import { Button, Input } from "../shared/forms.gx"

// Define our data structure
struct TodoItem {
    id: int,
    text: string,
    completed: bool,
}

component TodoApp {
    // --- Logic & State ---
    state todos: List<TodoItem> = []
    state newTodoText: string = ""

    // Derived reactive state: re-calculates automatically
    let remainingCount = todos.filter(|t| !t.completed).len()

    fn addTodo() {
        if newTodoText.trim() == "" { return }

        let newTodo = TodoItem {
            id: Date.now(), // Assume a built-in Date API
            text: newTodoText,
            completed: false,
        }

        todos.push(newTodo)
        newTodoText = "" // Clear the input automatically
    }

    fn toggleTodo(id: int) {
        for todo in &mut todos {
            if todo.id == id {
                todo.completed = !todo.completed
                break
            }
        }
    }

    // --- Markup (View) ---
    <div class="app-container">
        <header>
            <h1>Gigli Todos</h1>
            <h2>{remainingCount} items remaining</h2>
        </header>

        <form class="add-todo-form" on:submit:preventDefault={addTodo}>
            <input
                placeholder="What needs to be done?"
                bind:value={newTodoText}
            />
            <button type="submit">Add Todo</button>
        </form>

        <ul class="todo-list">
            {#for todo in todos}
                <li
                    class:completed={todo.completed}
                    on:click={() => toggleTodo(todo.id)}
                >
                    {todo.text}
                </li>
            {/for}
        </ul>
    </div>

    // --- Style (Scoped by default) ---
    style {
        .app-container {
            max-width: 500px;
            margin: 2rem auto;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        }
        .todo-list li {
            padding: 12px;
            border-bottom: 1px solid #eee;
            cursor: pointer;
            transition: all 0.2s ease;
        }
        .todo-list li:hover {
            background-color: #f9f9f9;
        }
        .todo-list li.completed {
            text-decoration: line-through;
            color: #aaa;
        }
    }
}
```

---

## 🛠️ Compiler & Tooling

*   **Compiler:** The Rust-based `gigli` compiler performs advanced optimizations like dead-code elimination, tree-shaking, and minification to produce a highly compact and efficient WASM binary.
*   **CLI:** The `gigli` command-line tool provides a seamless developer experience:
    *   `gigli new <name>`: Scaffolds a new project.
    *   `gigli dev`: Starts a hot-reloading development server.
    *   `gigli build`: Creates an optimized production build.
    *   `gigli check`: Type-checks the project without compiling.
    *   `gigli fmt`: Formats all `.gx` files in the project.
*   **LSP:** A dedicated Language Server Protocol implementation provides real-time diagnostics, autocompletion, and type information in modern code editors.
