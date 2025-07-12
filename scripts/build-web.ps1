# GigliOptix Web Distribution Build Script (PowerShell)

param(
    [string]$DistDir = "dist"
)

Write-Host "🚀 Building GigliOptix for web distribution..." -ForegroundColor Blue

# Function to print colored output
function Write-Status {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Blue
}

function Write-Success {
    param([string]$Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARNING] $Message" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

# Check if we're in the right directory
if (-not (Test-Path "Cargo.toml")) {
    Write-Error "Please run this script from the GigliOptix project root"
    exit 1
}

# Create distribution directory
Write-Status "Creating distribution directory: $DistDir"
if (Test-Path $DistDir) {
    Remove-Item -Recurse -Force $DistDir
}
New-Item -ItemType Directory -Path $DistDir | Out-Null

# Build the compiler
Write-Status "Building GigliOptix compiler..."
try {
    cargo build --release
    Write-Success "Compiler built successfully"
} catch {
    Write-Error "Failed to build compiler"
    exit 1
}

# Copy web files
Write-Status "Copying web files..."
Copy-Item -Path "web\*" -Destination $DistDir -Recurse -Force

# Copy examples
Write-Status "Copying examples..."
New-Item -ItemType Directory -Path "$DistDir\examples" -Force | Out-Null
Copy-Item -Path "examples\*.gx" -Destination "$DistDir\examples\" -Force

# Copy documentation
Write-Status "Copying documentation..."
New-Item -ItemType Directory -Path "$DistDir\docs" -Force | Out-Null
Copy-Item -Path "docs\BROWSER_GUIDE.md" -Destination "$DistDir\docs\" -Force
Copy-Item -Path "README.md" -Destination "$DistDir\" -Force

# Create a simple server script
Write-Status "Creating server script..."
$serverPython = @"
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
"@

$serverPython | Out-File -FilePath "$DistDir\server.py" -Encoding UTF8

# Create a Node.js server script as well
Write-Status "Creating Node.js server script..."
$serverJS = @"
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
"@

$serverJS | Out-File -FilePath "$DistDir\server.js" -Encoding UTF8

# Create package.json for npm distribution
Write-Status "Creating package.json..."
$packageJson = @"
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
"@

$packageJson | Out-File -FilePath "$DistDir\package.json" -Encoding UTF8

# Create a README for the distribution
Write-Status "Creating distribution README..."
$readme = @"
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
"@

$readme | Out-File -FilePath "$DistDir\README.md" -Encoding UTF8

# Create a Windows batch file
Write-Status "Creating Windows batch file..."
$batchFile = @"
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
"@

$batchFile | Out-File -FilePath "$DistDir\start.bat" -Encoding ASCII

Write-Success "Build completed successfully!"
Write-Host ""
Write-Host "📁 Distribution files created in: $DistDir" -ForegroundColor Cyan
Write-Host ""
Write-Host "🚀 To start the playground:" -ForegroundColor Yellow
Write-Host "   cd $DistDir" -ForegroundColor White
Write-Host "   python3 server.py" -ForegroundColor White
Write-Host "   # or" -ForegroundColor White
Write-Host "   npm install && npm start" -ForegroundColor White
Write-Host ""
Write-Host "🌐 Then open http://localhost:8000 in your browser" -ForegroundColor Yellow
Write-Host ""
Write-Host "📦 To distribute:" -ForegroundColor Yellow
Write-Host "   Compress the $DistDir folder into a ZIP file" -ForegroundColor White
Write-Host ""
Write-Success "Happy coding with GigliOptix! 🎉"
