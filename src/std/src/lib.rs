//! Standard library for the Gigli programming language
//!
//! This crate provides the standard library functionality including:
//! - Browser APIs
//! - Core data structures and algorithms
//! - I/O operations
//! - System interfaces

pub mod browser;
pub mod list;
pub mod map;
pub mod option;
pub mod result;
pub mod io;
pub mod time;

// Re-export commonly used types
pub use browser::*;
pub use list::*;
pub use map::*;
pub use option::*;
pub use result::*;
