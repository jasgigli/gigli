//! Bundling logic for GigliOptix web output
use std::fs;
use std::path::Path;

/// Bundles compiled WASM, loader JS, and HTML template into the output directory.
pub fn bundle_for_web(wasm_path: &str, output_dir: &str) {
    // Ensure output directory exists
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    // Copy WASM file
    let wasm_filename = Path::new(wasm_path).file_name().unwrap();
    let wasm_dest = Path::new(output_dir).join(wasm_filename);
    fs::copy(wasm_path, &wasm_dest).expect("Failed to copy WASM file");

    // Write enhanced loader.js with DOM operations and reactive features
    let loader_js = r#"
// GigliOptix Runtime for WebAssembly
class GigliOptixRuntime {
    constructor() {
        this.cells = new Map(); // Reactive state containers
        this.flows = new Map(); // Reactive flows
        this.eventHandlers = new Map(); // Event handlers
        this.memory = null;
        this.instance = null;
    }

    // Initialize the runtime
    async init() {
        try {
            const response = await fetch('main.wasm');
            const bytes = await response.arrayBuffer();

            // Create import object with DOM operations
            const importObject = {
                dom: {
                    set_inner_html: (elementIdPtr, htmlPtr) => {
                        const elementId = this.readString(elementIdPtr);
                        const html = this.readString(htmlPtr);
                        const element = document.getElementById(elementId);
                        if (element) {
                            element.innerHTML = html;
                        }
                        return 0;
                    },

                    add_event_listener: (elementIdPtr, eventTypePtr) => {
                        const elementId = this.readString(elementIdPtr);
                        const eventType = this.readString(eventTypePtr);
                        const element = document.getElementById(elementId);
                        if (element) {
                            element.addEventListener(eventType, (event) => {
                                this.handleEvent(elementId, eventType, event);
                            });
                        }
                        return 0;
                    },

                    get_element_by_id: (elementIdPtr) => {
                        const elementId = this.readString(elementIdPtr);
                        const element = document.getElementById(elementId);
                        return element ? 1 : 0;
                    }
                }
            };

            // Instantiate WASM module
            const { instance } = await WebAssembly.instantiate(bytes, importObject);
            this.instance = instance;
            this.memory = instance.exports.memory;

            console.log('GigliOptix runtime initialized successfully');
            return true;
        } catch (error) {
            console.error('Failed to initialize GigliOptix runtime:', error);
            return false;
        }
    }

    // Read string from WASM memory
    readString(ptr) {
        if (!this.memory) return '';

        const view = new Uint8Array(this.memory.buffer);
        let str = '';
        let i = ptr;

        while (view[i] !== 0) {
            str += String.fromCharCode(view[i]);
            i++;
        }

        return str;
    }

    // Write string to WASM memory
    writeString(str) {
        if (!this.memory) return 0;

        const view = new Uint8Array(this.memory.buffer);
        const ptr = this.findFreeMemory(str.length + 1);

        for (let i = 0; i < str.length; i++) {
            view[ptr + i] = str.charCodeAt(i);
        }
        view[ptr + str.length] = 0; // null terminator

        return ptr;
    }

    // Find free memory location
    findFreeMemory(size) {
        // Simple implementation - just use a fixed offset for now
        return 1024; // Start after initial data
    }

    // Handle events from DOM
    handleEvent(elementId, eventType, event) {
        const handlerKey = `${elementId}_${eventType}`;
        const handler = this.eventHandlers.get(handlerKey);
        if (handler) {
            handler(event);
        }
    }

    // Create a reactive cell
    createCell(name, initialValue) {
        this.cells.set(name, initialValue);
        console.log(`Created cell '${name}' with value:`, initialValue);
    }

    // Get cell value
    getCell(name) {
        return this.cells.get(name);
    }

    // Set cell value
    setCell(name, value) {
        this.cells.set(name, value);
        this.triggerCellChange(name);
    }

    // Trigger cell change (notify dependent flows)
    triggerCellChange(cellName) {
        // Find and execute flows that depend on this cell
        for (const [flowName, flow] of this.flows) {
            if (flow.dependencies && flow.dependencies.includes(cellName)) {
                flow.execute();
            }
        }
    }

    // Create a reactive flow
    createFlow(name, trigger, dependencies, body) {
        this.flows.set(name, {
            trigger,
            dependencies: dependencies || [],
            body,
            execute: () => {
                console.log(`Executing flow '${name}'`);
                // Execute the flow body
                if (typeof body === 'function') {
                    body();
                }
            }
        });
    }

    // Run the main function
    run() {
        if (this.instance && this.instance.exports.main) {
            console.log('Running GigliOptix main function');
            this.instance.exports.main();
        } else {
            console.error('Main function not found in WASM module');
        }
    }
}

// Global runtime instance
window.gigliRuntime = new GigliOptixRuntime();

// Initialize and run
async function run() {
    console.log('Starting GigliOptix application...');

    const success = await window.gigliRuntime.init();
    if (success) {
        window.gigliRuntime.run();
    } else {
        // Fallback: show error message
        document.body.innerHTML = '<h1>Failed to load GigliOptix application</h1><p>Please check the console for details.</p>';
    }
}

// Start the application when the page loads
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', run);
} else {
    run();
}
"#;
    fs::write(Path::new(output_dir).join("loader.js"), loader_js).expect("Failed to write loader.js");

    // Write enhanced index.html with better styling and error handling
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>GigliOptix App</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            margin: 0;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }

        #app {
            background: white;
            border-radius: 12px;
            padding: 40px;
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
            max-width: 600px;
            width: 100%;
            text-align: center;
        }

        .loading {
            color: #666;
            font-size: 18px;
        }

        .error {
            color: #e74c3c;
            background: #fdf2f2;
            border: 1px solid #fecaca;
            border-radius: 8px;
            padding: 20px;
            margin: 20px 0;
        }

        button {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            padding: 12px 24px;
            border-radius: 8px;
            font-size: 16px;
            cursor: pointer;
            transition: transform 0.2s;
        }

        button:hover {
            transform: translateY(-2px);
        }

        h1 {
            color: #2d3748;
            margin-bottom: 20px;
        }
    </style>
</head>
<body>
    <div id="app">
        <div class="loading">Loading GigliOptix application...</div>
    </div>

    <script src="loader.js"></script>

    <script>
        // Show loading state
        document.getElementById('app').innerHTML = '<div class="loading">Loading GigliOptix application...</div>';

        // Handle errors
        window.addEventListener('error', function(e) {
            document.getElementById('app').innerHTML = `
                <div class="error">
                    <h2>Application Error</h2>
                    <p>${e.message}</p>
                    <p>Check the console for more details.</p>
                </div>
            `;
        });
    </script>
</body>
</html>"#;
    fs::write(Path::new(output_dir).join("index.html"), html).expect("Failed to write index.html");

    // Create a simple development server script
    let dev_server_js = r#"
// Simple development server for GigliOptix
const http = require('http');
const fs = require('fs');
const path = require('path');

const server = http.createServer((req, res) => {
    let filePath = '.' + req.url;
    if (filePath === './') {
        filePath = './index.html';
    }

    const extname = path.extname(filePath);
    let contentType = 'text/html';

    switch (extname) {
        case '.js':
            contentType = 'text/javascript';
            break;
        case '.wasm':
            contentType = 'application/wasm';
            break;
        case '.css':
            contentType = 'text/css';
            break;
    }

    fs.readFile(filePath, (error, content) => {
        if (error) {
            res.writeHead(404);
            res.end('File not found');
        } else {
            res.writeHead(200, { 'Content-Type': contentType });
            res.end(content, 'utf-8');
        }
    });
});

const port = 8080;
server.listen(port, () => {
    console.log(`GigliOptix dev server running at http://localhost:${port}`);
    console.log('Press Ctrl+C to stop');
});
"#;
    fs::write(Path::new(output_dir).join("dev-server.js"), dev_server_js).expect("Failed to write dev-server.js");

    println!("Bundle complete! Files created in {}", output_dir);
    println!("To run the application:");
    println!("1. cd {}", output_dir);
    println!("2. node dev-server.js");
    println!("3. Open http://localhost:8080 in your browser");
}
