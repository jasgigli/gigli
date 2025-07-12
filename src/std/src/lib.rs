//! Standard library for the Gigli programming language
//!
//! This crate provides the standard library functionality including:
//! - Browser APIs
//! - Core data structures and algorithms
//! - I/O operations
//! - System interfaces

pub mod browser;

// Re-export commonly used types
pub use browser::*;
