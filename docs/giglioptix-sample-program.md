# GigliOptix Sample Program: Feature Showcase

This sample demonstrates the core features of GigliOptix, with inline explanations.

---

## Counter & Todo App Component
```giglioptix
component FeatureShowcase {
    // TypeScript: Strong typing
    let count: int = 0
    let todos: List<string> = []
    let newTodo: string = ""

    // Svelte: Reactive declaration
    $: doubled = count * 2

    // Rust: Ownership & borrowing
    fn add_todo() {
        if newTodo.trim() != "" {
            todos.push(newTodo)
            newTodo = ""
        }
    }

    // Async/Await: Rust & JS
    async fn fetch_quote(): Result<string, string> {
        let response = await http.get("https://api.quotable.io/random")
        if response.status == 200 {
            return Ok(response.json().content)
        }
        return Err("Failed to fetch quote")
    }

    // Python: List comprehension
    let evenSquares = [x * x for x in range(10) if x % 2 == 0]

    <main>
        <h1>GigliOptix Feature Showcase</h1>
        <section>
            <h2>Counter</h2>
            <p>Count: {count} (Doubled: {doubled})</p>
            <button on:click={() => count += 1}>Increment</button>
        </section>
        <section>
            <h2>Todo List</h2>
            <input bind:value={newTodo} placeholder="Add a todo..." />
            <button on:click={add_todo}>Add</button>
            <ul>
                {for todo in todos}
                    <li>{todo}</li>
                {/for}
            </ul>
        </section>
        <section>
            <h2>Async Data Fetch</h2>
            <button on:click={async () => {
                let result = await fetch_quote()
                if result.is_ok() {
                    alert("Quote: " + result.unwrap())
                } else {
                    alert(result.unwrap_err())
                }
            }}>Fetch Random Quote</button>
        </section>
        <section>
            <h2>Pythonic List Comprehension</h2>
            <p>Even squares: {evenSquares.join(", ")}</p>
        </section>
    </main>

    style {
        main { max-width: 600px; margin: auto; font-family: sans-serif; }
        h1 { color: #2c3e50; }
        section { margin-bottom: 2em; }
        input, button { font-size: 1em; margin-right: 0.5em; }
        ul { padding-left: 1.5em; }
    }
}
```

---

### Feature Mapping
- **TypeScript**: Type annotations, generics, interfaces
- **Rust**: Ownership, borrowing, async/await, Result/Option types
- **Svelte**: Reactivity (`$:`), compiled UI, event bindings
- **Python**: List comprehensions, simple syntax
- **HTML/CSS**: Embedded markup and scoped styles
- **JavaScript**: Familiar event handling, async patterns
