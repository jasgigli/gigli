# Gigli Programming Language Specification (v0.2.0)

## Overview

**Gigli** is a modern, unified programming language for building high-performance, reactive web applications. It combines logic, markup, and styling into a single, elegant component format. By compiling to WebAssembly (WASM), Gigli delivers exceptional speed and safety, aiming to drastically simplify the web development workflow.

## Guiding Principles

1.  **Unified & Simple:** A single `.gx` file defines a complete component. No more context-switching between HTML, CSS, and JavaScript.
2.  **Ergonomic & Readable:** The syntax is designed to be intuitive and minimize boilerplate, making code a joy to write and maintain.
3.  **Reactive by Default:** State management is a core language feature, not a library. The UI automatically reacts to state changes.
4.  **Performant & Safe:** Leveraging a Rust-based compiler and WASM target, Gigli provides compile-time safety guarantees and near-native performance.

---

## 1. Core Syntax

### 1.1. Variables and Constants

Gigli is a statically-typed language with powerful type inference.

-   `let`: Defines an immutable constant.
-   `mut`: Defines a mutable variable.

```gigli
let PI: float = 3.14159
let name = "Gigli" // Type `string` is inferred

mut counter: int = 0
counter = 10
```

### 1.2. Functions

Functions are declared with the `fn` keyword. Type annotations are required for parameters and return values. `void` is used for functions that do not return a value.

```gigli
fn add(a: int, b: int): int {
    return a + b
}

// Lambdas use a concise syntax
let multiply = |a, b| a * b
```

### 1.3. Control Flow

Gigli supports standard control flow statements.

```gigli
if user.isAuthenticated {
    print("Welcome!")
} else {
    print("Please log in.")
}

// `match` provides powerful pattern matching
match response.statusCode {
    200 => print("Success"),
    404 => print("Not Found"),
    _   => print("An error occurred"), // `_` is a wildcard
}
```

### 1.4. Error Handling

Gigli uses `Result<T, E>` and `Option<T>` for robust error and nullability handling, eliminating null pointer errors.

```gigli
fn safe_divide(a: int, b: int): Result<int, string> {
    if b == 0 {
        return Err("Division by zero")
    }
    return Ok(a / b)
}

let result = safe_divide(10, 2) // result is Ok(5)
```

---

## 2. Component Model

The component is the fundamental building block of a Gigli application. A component is defined in a `.gx` file and encapsulates its logic, markup, and styles.

### 2.1. Component Structure

A component consists of three optional sections in a single file:
1.  **Logic (Script):** The code at the top of the file.
2.  **Markup (View):** The HTML-like structure of the component.
3.  **Style:** The component-scoped CSS.

```gigli
// 1. Logic Section
// ...

// 2. Markup Section
// ...

// 3. Style Section
style {
    /* ... */
}
```

### 2.2. State Management

Reactivity is at the heart of Gigli.

*   `state`: Declares a reactive state variable. Any modification to a `state` variable will automatically trigger a UI update.
*   `let`: Declares a derived reactive value. It automatically re-calculates whenever the `state` variables it depends on change.

```gigli
component Counter {
    // Reactive state: changing this re-renders the UI
    state count: int = 0

    // Derived state: re-calculates when `count` changes
    let isEven = count % 2 == 0
    let message = `The count is {count}`

    fn increment() {
        count += 1
    }
}
```

### 2.3. Markup Syntax

The markup section uses an HTML-like syntax for defining the component's structure.

*   **Expressions:** Use `{...}` to embed dynamic values and expressions.
*   **Event Handling:** Use `on:event={handler}`. Event modifiers are chained with colons, e.g., `on:submit:preventDefault={...}`.
*   **Two-Way Data Binding:** Use `bind:attribute={state_variable}` to link a state variable to an input's attribute.
*   **Control Flow:** Use special `{#...}` blocks for rendering logic.

```gigli
// Conditional Rendering
{#if user.loggedIn}
    <button on:click={logout}>Log Out</button>
{:else}
    <p>Please log in.</p>
{/if}

// List Rendering
<ul>
    {#for item in items}
        <li>{item.name}</li>
    {/for}
</ul>
```

### 2.4. Styling

The `style` block contains standard CSS. All styles are **scoped by default** to the component, meaning they won't leak out and affect other components.

```gigli
style {
    /* This `h1` style only applies to h1 tags inside this component */
    h1 {
        color: #4A5568; /* A nice dark gray */
        font-size: 2rem;
    }
}
```

---

## 3. Type System

*   **Primitives:** `int`, `float`, `bool`, `string`, `char`.
*   **Data Structures:**
    *   `struct`: For creating complex data types.
    *   `enum`: For defining a type with a fixed set of variants.
*   **Collections:** `List<T>`, `Map<K, V>`.
*   **Special Types:** `Option<T>`, `Result<T, E>`.
*   **Generics & Union Types:** Supported for building flexible and reusable code.

```gigli
// A custom data structure
struct User {
    id: int,
    name: string,
    isActive: bool,
}

// A type that can be one of several variants
enum Status {
    Loading,
    Success(data: string),
    Error(message: string),
}
```

---

## 4. Memory Management & Concurrency

*   **Ownership Model:** Inspired by Rust, Gigli uses an ownership system with compile-time checks to ensure memory safety without a garbage collector. For UI development, the framework manages most memory concerns automatically.
*   **Borrowing:** Use `&` for immutable and `&mut` for mutable references to pass data without transferring ownership.
*   **Concurrency:** `async`/`await` is built-in for handling asynchronous operations like API calls in a non-blocking way.

```gigli
async fn fetchUser(id: int): Result<User, string> {
    let response = await http.get(`/api/users/{id}`)
    if response.ok {
        return Ok(response.json())
    } else {
        return Err("Failed to fetch user")
    }
}
```

---

## 5. Full Example: A Modern Todo App

This example demonstrates how all the features of Gigli work together.

**File: `src/components/TodoApp.gx`**
```gigli
// Import other components
import { Button, Input } from "../shared/forms.gx"

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

    // Derived reactive state
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
            <Input
                placeholder="What needs to be done?"
                bind:value={newTodoText}
            />
            <Button type="submit">Add Todo</Button>
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

    // --- Style (Scoped) ---
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

## 6. Compiler & Tooling

*   **Compiler:** The Rust-based `gigli` compiler performs advanced optimizations like dead-code elimination, tree-shaking, and minification to produce a highly compact and efficient WASM binary.
*   **CLI:** The `gigli` command-line tool provides a seamless developer experience:
    *   `gigli new <name>`: Scaffolds a new project.
    *   `gigli dev`: Starts a hot-reloading development server.
    *   `gigli build`: Creates an optimized production build.
    *   `gigli check`: Type-checks the project without compiling.
    *   `gigli fmt`: Formats all `.gx` files in the project.
*   **LSP:** A dedicated Language Server Protocol implementation provides real-time diagnostics, autocompletion, and type information in modern code editors.
