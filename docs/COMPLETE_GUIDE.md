# 🚀 Complete GigliOptix Guide

## What is GigliOptix?

GigliOptix is a modern, reactive programming language designed for web development. It compiles to WebAssembly (WASM) for ultra-fast execution in browsers, providing a unified development experience with reactive UI components, state management, and cross-platform compatibility.

## 🎯 Key Features

- **Reactive Programming**: Automatic UI updates when data changes
- **WebAssembly Compilation**: Near-native performance in browsers
- **Modern Syntax**: Clean, readable code with type inference
- **Cross-Platform**: Write once, run everywhere
- **Built-in UI Components**: Declarative UI with styling system
- **State Management**: Simple and powerful state handling

## 📁 Project Structure

```
giglioptix/
├── examples/           # Example applications
│   ├── hello.gx       # Hello World
│   ├── counter.gx     # Counter app
│   ├── todo.gx        # Todo list
│   ├── calculator.gx  # Calculator
│   ├── weather.gx     # Weather app
│   └── game.gx        # Snake game
├── web/               # Web interface
│   ├── index.html     # Playground UI
│   └── app.js         # Playground logic
├── docs/              # Documentation
│   ├── BROWSER_GUIDE.md
│   └── COMPLETE_GUIDE.md
├── scripts/           # Build scripts
│   ├── build-web.sh   # Linux/Mac build
│   └── build-web.ps1  # Windows build
└── src/               # Compiler source code
```

## 🚀 Quick Start

### 1. Build the Compiler

**Windows (PowerShell):**
```powershell
.\scripts\build-web.ps1
```

**Linux/Mac:**
```bash
./scripts/build-web.sh
```

### 2. Start the Playground

**Option 1: Python Server**
```bash
cd dist
python3 server.py
```

**Option 2: Node.js Server**
```bash
cd dist
npm install
npm start
```

**Option 3: Simple HTTP Server**
```bash
cd dist
python3 -m http.server 8000
```

### 3. Open in Browser

Navigate to `http://localhost:8000` and start coding!

## 📚 Language Examples

### Hello World
```gx
fn main() {
    dom::set_inner_html("app", "<h1>Hello, Web!</h1>");
}
```

### Reactive Counter
```gx
view App {
  cell counter = 0

  on click: counter += 1

  style:
    background: "#f0f0f0",
    padding: "20px",
    text_align: "center"

  render:
    div {
      h1("Counter: " + counter)
      button(on_click: counter += 1)("Increment")
    }
}
```

### Todo List
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

## 🎮 Interactive Examples

### 1. **Hello World** (`examples/hello.gx`)
Simple DOM manipulation to display "Hello, Web!"

### 2. **Counter** (`examples/counter.gx`)
Reactive counter that changes color based on count value

### 3. **Todo List** (`examples/todo.gx`)
Complete todo application with add, toggle, and delete functionality

### 4. **Calculator** (`examples/calculator.gx`)
Fully functional calculator with grid layout and mathematical operations

### 5. **Weather App** (`examples/weather.gx`)
Weather application demonstrating API integration and data handling

### 6. **Snake Game** (`examples/game.gx`)
Complete Snake game with keyboard controls and collision detection

## 🛠️ Development Workflow

### 1. Write Code
Use the interactive editor in the playground or your preferred text editor.

### 2. Compile
The compiler automatically transforms your `.gx` code to WebAssembly.

### 3. Run
Execute the compiled code in the browser with instant feedback.

### 4. Debug
Use browser developer tools and the built-in error reporting.

## 🌐 Browser Integration

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
// Initialize runtime
const gigli = new GigliOptix();

// Compile and run
gigli.compile(code).then(module => {
    module.run();
});
```

## 📦 Distribution

### Standalone Package
The build script creates a complete distribution package in the `dist/` folder:

- **Web Interface**: Complete playground with code editor
- **Examples**: All example applications
- **Documentation**: Comprehensive guides
- **Server Scripts**: Python and Node.js servers
- **Installation Scripts**: Easy setup for users

### CDN Distribution
```html
<script src="https://cdn.giglioptix.com/latest/giglioptix.js"></script>
```

### NPM Package
```bash
npm install giglioptix
```

## 🔧 Advanced Features

### Custom Components
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

### Styling System
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

### Event Handling
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

## 🚀 Performance Benefits

### WebAssembly Advantages
- **Near-native speed**: Runs at speeds close to native code
- **Small bundle size**: Efficient compilation results in smaller files
- **Cross-platform**: Works on all modern browsers
- **Security**: Sandboxed execution environment

### Optimization Tips
1. **Minimize DOM operations**: Use reactive updates
2. **Lazy loading**: Load components only when needed
3. **Code splitting**: Split large applications
4. **Caching**: Cache compiled WASM modules

## 🌍 Browser Support

- **Chrome**: 57+
- **Firefox**: 52+
- **Safari**: 11+
- **Edge**: 79+

## 🛠️ Development Tools

### Language Server Protocol (LSP)
```bash
cargo install giglioptix-lsp
```

### Debugging
```javascript
// Enable debug mode
GigliOptix.init({ debug: true });

// View compilation logs
console.log(GigliOptix.getCompilationLog());
```

### Performance Profiling
```javascript
// Profile your application
const profile = GigliOptix.profile(code);
console.log('Compilation time:', profile.compilationTime);
console.log('Execution time:', profile.executionTime);
```

## 🔍 Troubleshooting

### Common Issues

1. **CORS Errors**: Ensure your web server is properly configured
2. **WASM Loading**: Check that WebAssembly is supported
3. **Memory Issues**: Monitor memory usage in large applications
4. **Compilation Errors**: Check syntax and dependencies

### Debug Mode
```javascript
GigliOptix.init({
    debug: true,
    logLevel: 'verbose'
});
```

## 📚 Learning Resources

### Documentation
- **Browser Guide**: `docs/BROWSER_GUIDE.md`
- **API Reference**: Available in the playground
- **Examples**: Interactive examples in the playground

### Community
- **GitHub**: [github.com/giglioptix/giglioptix](https://github.com/giglioptix/giglioptix)
- **Discord**: [discord.gg/giglioptix](https://discord.gg/giglioptix)
- **Stack Overflow**: Tag questions with `giglioptix`

## 🎯 Use Cases

### Web Applications
- Single Page Applications (SPAs)
- Interactive dashboards
- Real-time applications
- Progressive Web Apps (PWAs)

### Games
- Browser-based games
- Interactive simulations
- Educational applications

### Tools
- Code editors
- Data visualization tools
- Productivity applications

## 🔮 Future Roadmap

### Planned Features
- **Type System**: Advanced type checking and inference
- **Package Manager**: Dependency management system
- **Testing Framework**: Built-in testing capabilities
- **IDE Integration**: Enhanced editor support
- **Mobile Support**: React Native compilation target

### Community Contributions
- **Examples**: User-submitted examples and templates
- **Libraries**: Community-built libraries and components
- **Tools**: Development tools and utilities

## 📄 License

MIT License - see LICENSE file for details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📞 Support

- **Documentation**: [docs.giglioptix.com](https://docs.giglioptix.com)
- **Issues**: [GitHub Issues](https://github.com/giglioptix/giglioptix/issues)
- **Discussions**: [GitHub Discussions](https://github.com/giglioptix/giglioptix/discussions)

---

**Happy coding with GigliOptix! 🚀**

For the latest updates and features, visit our [official website](https://giglioptix.com).
