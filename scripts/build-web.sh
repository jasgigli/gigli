#!/bin/bash

# GigliOptix Web Distribution Build Script

set -e

echo "🚀 Building GigliOptix for web distribution..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the GigliOptix project root"
    exit 1
fi

# Create distribution directory
DIST_DIR="dist"
print_status "Creating distribution directory: $DIST_DIR"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Build the compiler
print_status "Building GigliOptix compiler..."
if cargo build --release; then
    print_success "Compiler built successfully"
else
    print_error "Failed to build compiler"
    exit 1
fi

# Copy web files
print_status "Copying web files..."
cp -r web/* "$DIST_DIR/"

# Copy examples
print_status "Copying examples..."
mkdir -p "$DIST_DIR/examples"
cp examples/*.gx "$DIST_DIR/examples/"

# Copy documentation
print_status "Copying documentation..."
mkdir -p "$DIST_DIR/docs"
cp docs/BROWSER_GUIDE.md "$DIST_DIR/docs/"
cp README.md "$DIST_DIR/"

# Create a simple server script
print_status "Creating server script..."
cat > "$DIST_DIR/server.py" << 'EOF'
#!/usr/bin/env python3
import http.server
import socketserver
import os
import sys

PORT = 8000

class MyHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        super().end_headers()

    def do_OPTIONS(self):
        self.send_response(200)
        self.end_headers()

if __name__ == "__main__":
    os.chdir(os.path.dirname(os.path.abspath(__file__)))

    with socketserver.TCPServer(("", PORT), MyHTTPRequestHandler) as httpd:
        print(f"🚀 GigliOptix Playground running at http://localhost:{PORT}")
        print("Press Ctrl+C to stop the server")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\n👋 Server stopped")
EOF

chmod +x "$DIST_DIR/server.py"

# Create a Node.js server script as well
print_status "Creating Node.js server script..."
cat > "$DIST_DIR/server.js" << 'EOF'
const http = require('http');
const fs = require('fs');
const path = require('path');

const PORT = 8000;

const mimeTypes = {
    '.html': 'text/html',
    '.js': 'text/javascript',
    '.css': 'text/css',
    '.json': 'application/json',
    '.png': 'image/png',
    '.jpg': 'image/jpg',
    '.gif': 'image/gif',
    '.svg': 'image/svg+xml',
    '.wav': 'audio/wav',
    '.mp4': 'video/mp4',
    '.woff': 'application/font-woff',
    '.ttf': 'application/font-ttf',
    '.eot': 'application/vnd.ms-fontobject',
    '.otf': 'application/font-otf',
    '.wasm': 'application/wasm'
};

const server = http.createServer((req, res) => {
    // Add CORS headers
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'GET, POST, OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type');

    if (req.method === 'OPTIONS') {
        res.writeHead(200);
        res.end();
        return;
    }

    let filePath = '.' + req.url;
    if (filePath === './') {
        filePath = './index.html';
    }

    const extname = String(path.extname(filePath)).toLowerCase();
    const mimeType = mimeTypes[extname] || 'application/octet-stream';

    fs.readFile(filePath, (error, content) => {
        if (error) {
            if (error.code === 'ENOENT') {
                res.writeHead(404);
                res.end('File not found');
            } else {
                res.writeHead(500);
                res.end('Server error: ' + error.code);
            }
        } else {
            res.writeHead(200, { 'Content-Type': mimeType });
            res.end(content, 'utf-8');
        }
    });
});

server.listen(PORT, () => {
    console.log(`🚀 GigliOptix Playground running at http://localhost:${PORT}`);
    console.log('Press Ctrl+C to stop the server');
});

server.on('error', (err) => {
    if (err.code === 'EADDRINUSE') {
        console.error(`Port ${PORT} is already in use. Please try a different port.`);
    } else {
        console.error('Server error:', err);
    }
});
EOF

# Create package.json for npm distribution
print_status "Creating package.json..."
cat > "$DIST_DIR/package.json" << 'EOF'
{
  "name": "giglioptix-playground",
  "version": "0.1.0",
  "description": "GigliOptix Programming Language Playground",
  "main": "server.js",
  "scripts": {
    "start": "node server.js",
    "serve": "python3 server.py",
    "dev": "python3 -m http.server 8000"
  },
  "keywords": [
    "giglioptix",
    "programming-language",
    "compiler",
    "wasm",
    "playground"
  ],
  "author": "GigliOptix Team",
  "license": "MIT",
  "engines": {
    "node": ">=14.0.0"
  }
}
EOF

# Create a README for the distribution
print_status "Creating distribution README..."
cat > "$DIST_DIR/README.md" << 'EOF'
# 🚀 GigliOptix Playground

Welcome to the GigliOptix Programming Language Playground!

## Quick Start

### Option 1: Python Server
```bash
python3 server.py
```

### Option 2: Node.js Server
```bash
npm install
npm start
```

### Option 3: Simple HTTP Server
```bash
python3 -m http.server 8000
```

Then open your browser and navigate to: http://localhost:8000

## Features

- **Interactive Code Editor**: Write and edit GigliOptix code with syntax highlighting
- **Live Preview**: See your code run in real-time
- **Example Gallery**: Explore pre-built examples
- **WebAssembly Compilation**: Ultra-fast execution in the browser

## Examples Included

- Hello World
- Counter App
- Todo List
- Calculator
- Weather App
- Snake Game

## Documentation

See `docs/BROWSER_GUIDE.md` for detailed documentation on using GigliOptix in browsers.

## Browser Support

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 79+

## License

MIT License - see LICENSE file for details.
EOF

# Create a simple installation script
print_status "Creating installation script..."
cat > "$DIST_DIR/install.sh" << 'EOF'
#!/bin/bash

echo "🚀 Installing GigliOptix Playground..."

# Check if Python is available
if command -v python3 &> /dev/null; then
    echo "✅ Python 3 found"
    echo "To start the playground, run: python3 server.py"
elif command -v python &> /dev/null; then
    echo "✅ Python found"
    echo "To start the playground, run: python server.py"
else
    echo "❌ Python not found. Please install Python 3."
fi

# Check if Node.js is available
if command -v node &> /dev/null; then
    echo "✅ Node.js found"
    echo "To start the playground with Node.js, run: npm install && npm start"
else
    echo "⚠️  Node.js not found. Python server will be used."
fi

echo ""
echo "🎉 Installation complete!"
echo "Open http://localhost:8000 in your browser to start coding."
EOF

chmod +x "$DIST_DIR/install.sh"

# Create a Windows batch file
print_status "Creating Windows batch file..."
cat > "$DIST_DIR/start.bat" << 'EOF'
@echo off
echo 🚀 Starting GigliOptix Playground...

REM Try Python first
python server.py 2>nul
if %errorlevel% neq 0 (
    python3 server.py 2>nul
    if %errorlevel% neq 0 (
        echo Python not found. Trying Node.js...
        node server.js 2>nul
        if %errorlevel% neq 0 (
            echo Neither Python nor Node.js found.
            echo Please install Python 3 or Node.js to run the playground.
            pause
        )
    )
)
EOF

print_success "Build completed successfully!"
echo ""
echo "📁 Distribution files created in: $DIST_DIR"
echo ""
echo "🚀 To start the playground:"
echo "   cd $DIST_DIR"
echo "   python3 server.py"
echo "   # or"
echo "   npm install && npm start"
echo ""
echo "🌐 Then open http://localhost:8000 in your browser"
echo ""
echo "📦 To distribute:"
echo "   zip -r giglioptix-playground.zip $DIST_DIR"
echo ""
print_success "Happy coding with GigliOptix! 🎉"
