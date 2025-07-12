# 🚀 GigliOptix Browser Guide

## Overview

GigliOptix is a modern, reactive programming language that compiles to WebAssembly (WASM) for ultra-fast execution in web browsers. This guide will help you get started with running GigliOptix code in browsers.

## Quick Start

### 1. Using the Online Playground

The easiest way to try GigliOptix is through our online playground:

1. **Open the Playground**: Navigate to the web interface
2. **Choose an Example**: Select from pre-built examples (Hello World, Counter, Todo, Calculator, Weather, Game)
3. **Edit Code**: Modify the code in the editor
4. **Run**: Click the "Run Code" button or press `Ctrl+Enter`
5. **View Output**: See the results in the live output panel

### 2. Local Development Setup

For local development and distribution:

```bash
# Clone the repository
git clone https://github.com/giglioptix/giglioptix.git
cd giglioptix

# Build the compiler
cargo build --release

# Start the web server
python -m http.server 8000
# or
npx serve web/
```

Then open `http://localhost:8000` in your browser.

## Language Features

### Basic Syntax

```gx
// Hello World
fn main() {
    dom::set_inner_html("app", "<h1>Hello, GigliOptix!</h1>");
}
```

### Reactive Views

```gx
view Counter {
  cell count = 0

  on click: count += 1

  style:
    background: "#f0f0f0",
    padding: "20px",
    text_align: "center"

  render:
    div {
      h1("Counter: " + count)
      button(on_click: count += 1)("Increment")
    }
}
```

### State Management

```gx
view TodoApp {
  cell todos = []
  cell new_todo = ""

  fn add_todo() {
    if new_todo != "" {
      todos.push({
        id: Date::now(),
        text: new_todo,
        completed: false
      })
      new_todo = ""
    }
  }

  render:
    div {
      input(value: new_todo, on_input: new_todo = event.target.value)
      button(on_click: add_todo)("Add")

      for todo in todos {
        div(todo.text)
      }
    }
}
```

## Examples

### 1. Hello World
**File**: `examples/hello.gx`
```gx
fn main() {
    dom::set_inner_html("app", "<h1>Hello, Web!</h1>");
}
```

### 2. Counter
**File**: `examples/counter.gx`
```gx
view App {
  cell counter = 0
  on click: counter += 1

  style:
    background: "#000",
    color: if counter > 10 then "#f00" else "#0f0"

  render:
    text("Clicks: " + counter)
}
```

### 3. Todo List
**File**: `examples/todo.gx`
A complete todo application with add, toggle, and delete functionality.

### 4. Calculator
**File**: `examples/calculator.gx`
A fully functional calculator with grid layout and mathematical operations.

### 5. Weather App
**File**: `examples/weather.gx`
A weather application demonstrating API integration and data handling.

### 6. Snake Game
**File**: `examples/game.gx`
A complete Snake game with keyboard controls and collision detection.

## Compilation Process

### 1. Source Code → AST
The GigliOptix compiler first parses your `.gx` files into an Abstract Syntax Tree (AST).

### 2. AST → Intermediate Representation (IR)
The AST is transformed into a platform-agnostic intermediate representation.

### 3. IR → WebAssembly
The IR is compiled to WebAssembly for browser execution.

### 4. WASM → JavaScript Bridge
A JavaScript bridge provides DOM manipulation and browser APIs.

## Browser Integration

### HTML Setup

```html
<!DOCTYPE html>
<html>
<head>
    <title>GigliOptix App</title>
</head>
<body>
    <div id="app"></div>
    <script src="giglioptix.js"></script>
    <script>
        // Load and run your GigliOptix code
        GigliOptix.run(`
            view App {
                cell counter = 0
                render: div("Count: " + counter)
            }
        `);
    </script>
</body>
</html>
```

### JavaScript API

```javascript
// Initialize GigliOptix runtime
const gigli = new GigliOptix();

// Compile and run code
gigli.compile(code).then(module => {
    module.run();
});

// Load from file
gigli.loadFile('app.gx').then(module => {
    module.run();
});
```

## Distribution

### 1. Standalone Web App

Create a self-contained web application:

```bash
# Build the compiler
cargo build --release

# Create distribution package
mkdir dist
cp web/* dist/
cp target/release/giglioptix dist/
cp examples/*.gx dist/
```

### 2. CDN Distribution

For easy integration, you can host the compiler on a CDN:

```html
<script src="https://cdn.giglioptix.com/latest/giglioptix.js"></script>
```

### 3. NPM Package

```bash
npm install giglioptix
```

```javascript
import { GigliOptix } from 'giglioptix';

const app = new GigliOptix();
app.run(code);
```

## Performance

### WebAssembly Benefits

- **Near-native performance**: WASM runs at speeds close to native code
- **Small bundle size**: Efficient compilation results in smaller files
- **Cross-platform**: Works on all modern browsers
- **Security**: Sandboxed execution environment

### Optimization Tips

1. **Minimize DOM operations**: Use reactive updates instead of direct manipulation
2. **Lazy loading**: Load components only when needed
3. **Code splitting**: Split large applications into smaller modules
4. **Caching**: Cache compiled WASM modules

## Browser Support

### Supported Browsers

- **Chrome**: 57+
- **Firefox**: 52+
- **Safari**: 11+
- **Edge**: 79+

### Feature Detection

```javascript
if (typeof WebAssembly === 'object') {
    // WebAssembly is supported
    GigliOptix.init();
} else {
    // Fallback to JavaScript interpreter
    GigliOptix.init({ mode: 'js' });
}
```

## Development Tools

### 1. Language Server Protocol (LSP)

For IDE support, install the GigliOptix LSP:

```bash
cargo install giglioptix-lsp
```

### 2. Debugging

Enable debugging in your browser's developer tools:

```javascript
// Enable debug mode
GigliOptix.init({ debug: true });

// View compilation logs
console.log(GigliOptix.getCompilationLog());
```

### 3. Performance Profiling

```javascript
// Profile your application
const profile = GigliOptix.profile(code);
console.log('Compilation time:', profile.compilationTime);
console.log('Execution time:', profile.executionTime);
```

## Troubleshooting

### Common Issues

1. **CORS Errors**: Ensure your web server is properly configured
2. **WASM Loading**: Check that WebAssembly is supported in your browser
3. **Memory Issues**: Monitor memory usage in large applications
4. **Compilation Errors**: Check syntax and ensure all dependencies are available

### Debug Mode

Enable debug mode for detailed error messages:

```javascript
GigliOptix.init({
    debug: true,
    logLevel: 'verbose'
});
```

## Advanced Features

### 1. Custom Components

```gx
view CustomButton {
  cell text = "Click me"
  cell on_click = null

  render:
    button(
      style: "background: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 5px; cursor: pointer",
      on_click: if on_click then on_click() else {}
    )(text)
}
```

### 2. Styling System

```gx
view StyledComponent {
  style:
    background: "#ffffff",
    border: "1px solid #ddd",
    border_radius: "8px",
    padding: "20px",
    box_shadow: "0 2px 4px rgba(0,0,0,0.1)",
    font_family: "Arial, sans-serif",
    font_size: "16px"

  render:
    div("Styled content")
}
```

### 3. Event Handling

```gx
view EventExample {
  cell message = ""

  fn handle_click() {
    message = "Button clicked!"
  }

  fn handle_keydown(event) {
    if event.key == "Enter" {
      message = "Enter pressed!"
    }
  }

  render:
    div {
      button(on_click: handle_click)("Click me")
      input(on_keydown: handle_keydown)
      div(message)
    }
}
```

## Community and Resources

### Getting Help

- **Documentation**: [docs.giglioptix.com](https://docs.giglioptix.com)
- **GitHub**: [github.com/giglioptix/giglioptix](https://github.com/giglioptix/giglioptix)
- **Discord**: [discord.gg/giglioptix](https://discord.gg/giglioptix)
- **Stack Overflow**: Tag questions with `giglioptix`

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

### Examples Repository

Find more examples and templates at:
[github.com/giglioptix/examples](https://github.com/giglioptix/examples)

---

**Happy coding with GigliOptix! 🚀**

For the latest updates and features, visit our [official website](https://giglioptix.com).
