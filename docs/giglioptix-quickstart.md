# GigliOptix Quick Start Guide

Welcome to GigliOptix! This guide will help you get started with your first project.

---

## 1. Installation
```bash
npm install -g giglioptix
```

---

## 2. Creating a New Project
```bash
giglioptix new my-app
cd my-app
```

---

## 3. Development Workflow
```bash
giglioptix dev      # Start development server with hot reload
giglioptix build    # Build for production
giglioptix deploy   # Deploy to your chosen platform
```

---

## 4. Example: Counter Component
```giglioptix
component Counter {
    let count: int = 0
    fn increment() { count += 1 }
    <button on:click={increment}>Clicked {count} times</button>
}
```

---

## 5. Example: Async Data Fetch
```giglioptix
component DataFetcher {
    let data: Option<string> = None
    async fn load() {
        let response = await http.get("https://api.example.com/data")
        data = Some(response.text())
    }
    <button on:click={load}>Load Data</button>
    {if data != None}
        <p>Data: {data.unwrap()}</p>
    {/if}
}
```

---

## 6. Project Structure
- `src/` — Your GigliOptix source files
- `public/` — Static assets
- `gigli.config.json` — Project configuration

---

## 7. Next Steps
- Explore the [specification](./giglioptix-spec.md)
- Try building your own components
- Use the CLI for formatting, linting, and testing
- Join the community for support and updates
