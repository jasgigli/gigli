//! Core compiler components for the Gigli programming language
//!
//! This crate contains the fundamental components of the Gigli compiler:
//! - Abstract Syntax Tree (AST) definitions
//! - Lexical analyzer (lexer)
//! - Parser
//! - Semantic analyzer
//! - Intermediate Representation (IR)

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod semantic;
pub mod ir;

// Re-export commonly used types
pub use ast::*;
pub use ir::*;

// Re-export commonly used functions
pub use parser::parse_file;
pub use ir::generator::generate_ir;
