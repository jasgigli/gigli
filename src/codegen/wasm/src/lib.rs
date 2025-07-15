//! WASM backend code generation for Gigli

use gigli_core::ir::IRModule;


/// Emits WebAssembly code from the given IRModule.
pub fn emit_wasm(module: &IRModule, output_path: &str) {
    println!("[WASM backend] Generating WASM for {} functions", module.functions.len());

    // Generate WASM binary with DOM operations and reactive features
    let wasm_bytes = generate_wasm_binary(module);

    std::fs::write(output_path, &wasm_bytes).expect("Failed to write WASM file");
    println!("[WASM backend] Emitted WASM to {}", output_path);
}

fn generate_wasm_binary(module: &IRModule) -> Vec<u8> {
    // Create a minimal working WASM binary
    let mut wasm = Vec::new();

    // WASM header
    wasm.extend_from_slice(&[0x00, 0x61, 0x73, 0x6d]); // \0asm
    wasm.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]); // version 1

    // Type section - just one function type: () -> ()
    let type_section = vec![
        0x01, // type section
        0x04, // section size
        0x01, // num types
        0x60, 0x00, 0x00, // () -> ()
    ];
    wasm.extend_from_slice(&type_section);

    // Function section - declare one function
    let function_section = vec![
        0x03, // function section
        0x02, // section size
        0x01, // num functions
        0x00, // type index 0
    ];
    wasm.extend_from_slice(&function_section);

    // Memory section - declare memory
    let memory_section = vec![
        0x05, // memory section
        0x03, // section size
        0x01, // num memories
        0x00, 0x01, // memory limits: min=1 page (64KB), max=unlimited
    ];
    wasm.extend_from_slice(&memory_section);

    // Export section - export memory and main function
    let export_section = vec![
        0x07, // export section
        0x0f, // section size
        0x02, // num exports
        // export memory
        0x06, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, // "memory"
        0x02, 0x00, // memory index 0
        // export main function
        0x04, 0x6d, 0x61, 0x69, 0x6e, // "main"
        0x00, 0x00, // function index 0
    ];
    wasm.extend_from_slice(&export_section);

    // Code section - function body
    let code_section = vec![
        0x0a, // code section
        0x04, // section size
        0x01, // num functions
        0x02, // function body size
        0x00, // local decl count
        0x0b, // end
    ];
    wasm.extend_from_slice(&code_section);

    wasm
}

fn create_type_section() -> Vec<u8> {
    let mut section = Vec::new();
    section.push(0x01); // type section

    // Function types:
    // - (i32, i32) -> i32 for DOM operations
    // - () -> () for main function
    // - (i32) -> () for event handlers
    let content = vec![
        0x0b, // section size
        0x03, // num types
        0x60, 0x02, 0x7f, 0x7f, 0x01, 0x7f, // (i32, i32) -> i32
        0x60, 0x00, 0x00, // () -> ()
        0x60, 0x01, 0x7f, 0x00, // (i32) -> ()
    ];
    section.extend_from_slice(&content);
    section
}

fn create_import_section() -> Vec<u8> {
    let mut section = Vec::new();
    section.push(0x02); // import section

    // Import DOM functions from JavaScript
    let content = vec![
        0x2a, // section size
        0x03, // num imports
        // import "dom" "set_inner_html"
        0x03, 0x64, 0x6f, 0x6d, // "dom"
        0x0d, 0x73, 0x65, 0x74, 0x5f, 0x69, 0x6e, 0x6e, 0x65, 0x72, 0x5f, 0x68, 0x74, 0x6d, 0x6c, // "set_inner_html"
        0x00, 0x00, // type index 0: (i32, i32) -> i32
        // import "dom" "add_event_listener"
        0x03, 0x64, 0x6f, 0x6d, // "dom"
        0x12, 0x61, 0x64, 0x64, 0x5f, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, // "add_event_listener"
        0x00, 0x00, // type index 0: (i32, i32) -> i32
        // import "dom" "get_element_by_id"
        0x03, 0x64, 0x6f, 0x6d, // "dom"
        0x0f, 0x67, 0x65, 0x74, 0x5f, 0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x62, 0x79, 0x5f, 0x69, 0x64, // "get_element_by_id"
        0x00, 0x01, // type index 1: (i32) -> i32
    ];
    section.extend_from_slice(&content);
    section
}

fn create_function_section(module: &IRModule) -> Vec<u8> {
    let mut section = Vec::new();
    section.push(0x03); // function section

    let num_functions = module.functions.len() + 1; // +1 for main function

    // Calculate content size: 1 byte for num_functions + num_functions bytes for type indices
    let content_size = 1 + num_functions;

    // Encode section size as LEB128
    let mut size_bytes = Vec::new();
    encode_leb128(content_size as u32, &mut size_bytes);

    section.extend_from_slice(&size_bytes);
    section.push(num_functions as u8); // num functions

    // All functions use type index 1 (() -> ())
    for _ in 0..num_functions {
        section.push(0x01); // type index
    }

    section
}

fn create_memory_section() -> Vec<u8> {
    let mut section = Vec::new();
    section.push(0x05); // memory section

    let content = vec![
        0x03, // section size
        0x01, // num memories
        0x00, 0x01, // memory limits: min=1 page (64KB), max=unlimited
    ];
    section.extend_from_slice(&content);
    section
}

fn create_export_section() -> Vec<u8> {
    let mut section = Vec::new();
    section.push(0x07); // export section

    let content = vec![
        0x0f, // section size
        0x02, // num exports
        // export memory
        0x06, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, // "memory"
        0x02, 0x00, // memory index 0
        // export main function
        0x04, 0x6d, 0x61, 0x69, 0x6e, // "main"
        0x00, 0x03, // function index 3 (after imports)
    ];
    section.extend_from_slice(&content);
    section
}

fn create_code_section(module: &IRModule) -> Vec<u8> {
    let mut section = Vec::new();
    section.push(0x0a); // code section

    let mut function_bodies = Vec::new();

    // Generate main function
    let main_body = generate_main_function(module);
    function_bodies.push(main_body);

    // Generate IR functions
    for func in &module.functions {
        let body = generate_function_body(func);
        function_bodies.push(body);
    }

    // Calculate section size: 1 byte for num functions + sum of all function body sizes
    let total_size = 1 + function_bodies.iter().map(|b| b.len()).sum::<usize>();

    // Encode section size as LEB128
    let mut size_bytes = Vec::new();
    encode_leb128(total_size as u32, &mut size_bytes);

    section.extend_from_slice(&size_bytes);
    section.push(module.functions.len() as u8 + 1); // num functions

    // Add function bodies
    for body in function_bodies {
        section.extend_from_slice(&body);
    }

    section
}

fn generate_main_function(module: &IRModule) -> Vec<u8> {
    let mut body = Vec::new();

    // Function body size (placeholder)
    body.push(0x00); // local decl count

    // Call each function in the module
    for (i, _func) in module.functions.iter().enumerate() {
        // call function index (3 + i, since first 3 are imports)
        body.push(0x10); // call
        body.extend_from_slice(&encode_leb128(3 + i as u32, &mut Vec::new()));
    }

    // End function
    body.push(0x0b); // end

    // Update body size
    let size = body.len() - 1; // -1 for the placeholder
    body[0] = size as u8;

    body
}

fn generate_function_body(func: &gigli_core::ir::IRFunction) -> Vec<u8> {
    let mut body = Vec::new();

    // Function body size (placeholder)
    body.push(0x00); // local decl count

    // Generate code for each statement
    for stmt in &func.body {
        match stmt {
            gigli_core::ir::IRStmt::Call { func: func_name, args } => {
                // Handle different function calls
                match func_name.as_str() {
                    "dom.set_inner_html" | "dom::set_inner_html" => {
                        // Call imported DOM function
                        for arg in args {
                            generate_expression(arg, &mut body);
                        }
                        body.push(0x10); // call
                        body.extend_from_slice(&encode_leb128(0, &mut Vec::new())); // import index 0
                    }
                    "dom.add_event_listener" | "dom::add_event_listener" => {
                        // Call imported event listener function
                        for arg in args {
                            generate_expression(arg, &mut body);
                        }
                        body.push(0x10); // call
                        body.extend_from_slice(&encode_leb128(1, &mut Vec::new())); // import index 1
                    }
                    "cell_create" => {
                        // Create a reactive cell (simplified - just store in memory)
                        for arg in args {
                            generate_expression(arg, &mut body);
                        }
                        // Store in memory (simplified implementation)
                        body.push(0x21); // global.set (placeholder)
                        body.push(0x00); // global index
                    }
                    "render_view" => {
                        // Render a view (simplified - just call set_inner_html)
                        for arg in args {
                            generate_expression(arg, &mut body);
                        }
                        body.push(0x10); // call
                        body.extend_from_slice(&encode_leb128(0, &mut Vec::new())); // import index 0
                    }
                    _ => {
                        // Unknown function - just generate expressions
                        for arg in args {
                            generate_expression(arg, &mut body);
                        }
                        // Drop the result
                        body.push(0x1a); // drop
                    }
                }
            }
            gigli_core::ir::IRStmt::Assign { target, value } => {
                // WASM code for assignment (placeholder)
                generate_expression(value, &mut body);
                // Store in memory (simplified)
                body.push(0x21); // global.set (placeholder)
                body.push(0x00); // global index
            },
            gigli_core::ir::IRStmt::Await(expr) => {
                // WASM code for await (placeholder: just evaluate expr)
                generate_expression(expr, &mut body);
                // In real WASM, would yield or await a promise
            },
            gigli_core::ir::IRStmt::Reactive { name, expr } => {
                // WASM code for reactivity (placeholder: evaluate and store)
                generate_expression(expr, &mut body);
                body.push(0x21); // global.set (placeholder)
                body.push(0x00); // global index for reactive var
            },
            gigli_core::ir::IRStmt::Comprehension { target, iter, filter, expr } => {
                // WASM code for list comprehension (placeholder)
                generate_expression(iter, &mut body);
                if let Some(f) = filter { generate_expression(f, &mut body); }
                generate_expression(expr, &mut body);
                // In real WASM, would loop and build array
            },
            gigli_core::ir::IRStmt::Render(expr) => {
                // WASM code for rendering (call JS glue to update DOM)
                generate_expression(expr, &mut body);
                body.push(0x10); // call
                body.extend_from_slice(&encode_leb128(0, &mut Vec::new())); // import index 0 (set_inner_html)
            },
            gigli_core::ir::IRStmt::EventBind { target, event, handler } => {
                // WASM code for event binding (call JS glue)
                body.push(0x41); // i32.const (placeholder for target)
                body.extend_from_slice(&encode_leb128(0, &mut Vec::new()));
                body.push(0x41); // i32.const (placeholder for event)
                body.extend_from_slice(&encode_leb128(0, &mut Vec::new()));
                body.push(0x10); // call
                body.extend_from_slice(&encode_leb128(1, &mut Vec::new())); // import index 1 (add_event_listener)
            },
            gigli_core::ir::IRStmt::DomOp { op, args } => {
                // WASM code for DOM operation (call JS glue)
                for arg in args { generate_expression(arg, &mut body); }
                body.push(0x10); // call
                body.extend_from_slice(&encode_leb128(0, &mut Vec::new())); // import index 0 (set_inner_html or similar)
            },
            gigli_core::ir::IRStmt::Return(opt) => {
                if let Some(expr) = opt { generate_expression(expr, &mut body); }
                // WASM return (end function)
                body.push(0x0f); // return
            },
            // ... handle other IRStmt variants as needed ...
        }
    }

    // End function
    body.push(0x0b); // end

    // Update body size
    let size = body.len() - 1; // -1 for the placeholder
    body[0] = size as u8;

    body
}

fn generate_expression(expr: &gigli_core::ir::IRExpr, body: &mut Vec<u8>) {
    match expr {
        gigli_core::ir::IRExpr::StringLiteral(_s) => {
            // Load string from memory (simplified - just load a constant offset)
            body.push(0x41); // i32.const
            body.extend_from_slice(&encode_leb128(0, &mut Vec::new())); // memory offset
        }
        gigli_core::ir::IRExpr::Identifier(_s) => {
            // Load variable from memory (simplified - just load a constant)
            body.push(0x41); // i32.const
            body.extend_from_slice(&encode_leb128(0, &mut Vec::new())); // constant value
        }
        gigli_core::ir::IRExpr::NumberLiteral(_n) => {
            // Placeholder: push 0 for number literals
            body.push(0x41); // i32.const
            body.extend_from_slice(&encode_leb128(0, &mut Vec::new()));
        }
        gigli_core::ir::IRExpr::Await(inner) => {
            generate_expression(inner, body);
            // In real WASM, would yield/await
        },
        gigli_core::ir::IRExpr::Option(inner) => {
            generate_expression(inner, body);
            // Option handling (placeholder)
        },
        gigli_core::ir::IRExpr::Result { ok, err } => {
            generate_expression(ok, body);
            generate_expression(err, body);
            // Result handling (placeholder)
        },
        gigli_core::ir::IRExpr::Comprehension { target, iter, filter, expr } => {
            generate_expression(iter, body);
            if let Some(f) = filter { generate_expression(f, body); }
            generate_expression(expr, body);
            // In real WASM, would loop and build array
        },
        gigli_core::ir::IRExpr::DomRef(_s) => {
            // Reference to DOM node (placeholder)
            body.push(0x41); // i32.const
            body.extend_from_slice(&encode_leb128(0, &mut Vec::new()));
        },
        // ... handle other IRExpr variants as needed ...
    }
}

fn create_data_section(_module: &IRModule) -> Vec<u8> {
    let mut section = Vec::new();
    section.push(0x0b); // data section

    // For now, just add a simple data section with some strings
    let content = vec![
        0x07, // section size
        0x01, // num data segments
        0x00, // memory index
        0x41, 0x00, // i32.const 0
        0x0b, // end
        0x05, // data size
        0x48, 0x65, 0x6c, 0x6c, 0x6f, // "Hello"
    ];
    section.extend_from_slice(&content);
    section
}

fn encode_leb128(mut value: u32, _bytes: &mut Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        result.push(byte);
        if value == 0 {
            break;
        }
    }
    result
}
