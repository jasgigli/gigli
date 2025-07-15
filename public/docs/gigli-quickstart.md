# Gigli Quick Start Guide

Welcome to Gigli! This guide will help you get started with your first project.

---

## 1. Installation
```bash
npm install -g gigli
```

---

## 2. Creating a New Project
```bash
gigli new my-app
cd my-app
```

---

## 3. Development Workflow
```bash
gigli dev      # Start development server with hot reload
gigli build    # Build for production
gigli deploy   # Deploy to your chosen platform
```

---

## 4. Example: Counter Component
```gigli
component Counter {
    let count: int = 0
    fn increment() { count += 1 }
    <button on:click={increment}>Clicked {count} times</button>
}
```

---

## 5. Example: Async Data Fetch
```gigli
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
- `src/` — Your Gigli source files
- `public/` — Static assets
- `gigli.config.json` — Project configuration

---

## 7. Next Steps
- Explore the [specification](./giglioptix-spec.md)
- Try building your own components
- Use the CLI for formatting, linting, and testing
- Join the community for support and updates
