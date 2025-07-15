//! Standard library: Time for Gigli

pub mod time {
    /// Returns the current system time as a string.
    pub fn now() -> String {
        // TODO: Implement for WASM/JS interop
        unimplemented!("Time is not available in WASM");
    }

    /// Sleeps for the given number of milliseconds.
    pub fn sleep(_ms: u64) {
        // TODO: Implement for native targets; not available in WASM
        unimplemented!("Sleep is not available in WASM");
    }

    /// Formats a timestamp as a string.
    pub fn format(_timestamp: u64, _fmt: &str) -> String {
        // TODO: Implement for native targets; not available in WASM
        unimplemented!("Date formatting is not available in WASM");
    }
}
