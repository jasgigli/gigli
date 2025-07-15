//! Standard library: IO for Gigli

pub mod io {
    /// Reads the contents of a file into a string.
    pub fn read_to_string(_path: &str) -> String {
        // TODO: Implement for native targets; not available in WASM
        unimplemented!("File I/O is not available in WASM");
    }

    /// Writes a string to a file.
    pub fn write_string(_path: &str, _contents: &str) {
        // TODO: Implement for native targets; not available in WASM
        unimplemented!("File I/O is not available in WASM");
    }

    /// Reads a line from standard input.
    pub fn read_line() -> String {
        // TODO: Implement for native targets; not available in WASM
        unimplemented!("Stdin is not available in WASM");
    }

    /// Writes a string to standard output.
    pub fn print(_s: &str) {
        // TODO: Implement for native targets; in WASM, use console.log
        unimplemented!("Stdout is not available in WASM");
    }
}
