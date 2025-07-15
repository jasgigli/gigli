//! Bundling logic for Gigli web output
use std::fs;
use std::path::Path;

/// Bundles compiled WASM, loader JS, and HTML template into the output directory.
pub fn bundle_for_web(wasm_path: &str, output_dir: &str) {
    // Ensure output directory exists
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    // WASM file should already be present in output_dir
    let wasm_filename = Path::new(wasm_path).file_name().unwrap();
    let wasm_dest = Path::new(output_dir).join(wasm_filename);
    if !wasm_dest.exists() {
        panic!("WASM file not found in output directory: {}", wasm_dest.display());
    }

    // Write enhanced loader.js with DOM operations and reactive features
    let loader_js = r#"
// Gigli v2.0 Runtime for WebAssembly
class GigliRuntime {
    constructor() {
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

                    add_event_listener: (elementIdPtr, eventTypePtr, handlerFuncIndex) => {
                        const elementId = this.readString(elementIdPtr);
                        const eventType = this.readString(eventTypePtr);
                        const element = document.getElementById(elementId);
                        if (element) {
                            const handler = this.instance.exports.__indirect_function_table.get(handlerFuncIndex);
                            element.addEventListener(eventType, handler);
                        }
                        return 0;
                    },

                    get_element_by_id: (elementIdPtr) => {
                        const elementId = this.readString(elementIdPtr);
                        const element = document.getElementById(elementId);
                        return element ? 1 : 0;
                    },
                    // New DOM manipulation for reactive updates
                    update_text: (nodeIdPtr, textPtr) => {
                        const nodeId = this.readString(nodeIdPtr);
                        const text = this.readString(textPtr);
                        const node = document.getElementById(nodeId);
                        if (node) node.textContent = text;
                    },
                    update_attribute: (nodeIdPtr, attrPtr, valuePtr) => {
                        const nodeId = this.readString(nodeIdPtr);
                        const attr = this.readString(attrPtr);
                        const value = this.readString(valuePtr);
                        const node = document.getElementById(nodeId);
                        if (node) node.setAttribute(attr, value);
                    },
                }
            };

            // Instantiate WASM module
            const { instance } = await WebAssembly.instantiate(bytes, importObject);
            this.instance = instance;
            this.memory = instance.exports.memory;

            console.log('Gigli v2.0 runtime initialized successfully');
            return true;
        } catch (error) {
            console.error('Failed to initialize Gigli runtime:', error);
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

    // Find free memory location (simple stub)
    findFreeMemory(size) {
        return 1024; // Start after initial data
    }

    // Run the main function
    run() {
        if (this.instance && this.instance.exports.main) {
            console.log('Running Gigli main function');
            this.instance.exports.main();
        } else {
            console.error('Main function not found in WASM module');
        }
    }
}

// Global runtime instance
window.gigliRuntime = new GigliRuntime();

// Initialize and run
async function run() {
    console.log('Starting Gigli application...');

    const success = await window.gigliRuntime.init();
    if (success) {
        window.gigliRuntime.run();
    } else {
        // Fallback: show error message
        document.body.innerHTML = '<h1>Failed to load Gigli application</h1><p>Please check the console for details.</p>';
    }
}

// Start the application when the page loads
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', run);
} else {
    run();
}
"#;

    let loader_path = Path::new(output_dir).join("loader.js");
    fs::write(&loader_path, loader_js).expect("Failed to write loader.js");
    println!("Generated loader.js at {}", loader_path.display());

    // Generate a simple index.html
    let html_content = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Gigli App</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <div id="app-root"></div>
    <script src="loader.js"></script>
</body>
</html>
"#;
    let html_path = Path::new(output_dir).join("index.html");
    fs::write(&html_path, html_content).expect("Failed to write index.html");
    println!("Generated index.html at {}", html_path.display());

    // Generate a simple style.css
    let css_content = r#"
body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
    padding: 2rem;
    background-color: #f4f4f9;
}
#app-root {
    max-width: 800px;
    margin: 0 auto;
    background: white;
    padding: 2rem;
    border-radius: 8px;
    box-shadow: 0 4px 6px rgba(0,0,0,0.1);
}
"#;
    let css_path = Path::new(output_dir).join("style.css");
    fs::write(&css_path, css_content).expect("Failed to write style.css");
    println!("Generated style.css at {}", css_path.display());
}
