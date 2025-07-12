//! Abstract Syntax Tree (AST) definitions for GigliOptix

use std::collections::HashMap;

/// AST node for a program (list of functions and views)
#[derive(Debug)]
pub struct AST {
    pub functions: Vec<Function>,
    pub views: Vec<View>,
    pub cells: Vec<Cell>,
    pub flows: Vec<Flow>,
}

/// AST node for a function
#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

/// AST node for a reactive cell (state container)
#[derive(Debug)]
pub struct Cell {
    pub name: String,
    pub initial_value: Expr,
    pub is_mutable: bool,
}

/// AST node for a reactive flow (time-based or event-driven logic)
#[derive(Debug)]
pub struct Flow {
    pub name: String,
    pub trigger: FlowTrigger,
    pub body: Vec<Stmt>,
}

/// Flow trigger types
#[derive(Debug)]
pub enum FlowTrigger {
    OnEvent { event: String, target: String },
    OnChange { cell: String },
    Interval { ms: u64 },
}

/// AST node for a view (declarative UI component)
#[derive(Debug)]
pub struct View {
    pub name: String,
    pub cells: Vec<Cell>,
    pub flows: Vec<Flow>,
    pub style: Option<StyleBlock>,
    pub render: RenderBlock,
    pub event_handlers: Vec<EventHandler>,
}

/// Style block for a view
#[derive(Debug)]
pub struct StyleBlock {
    pub properties: HashMap<String, Expr>,
}

/// Render block for a view
#[derive(Debug)]
pub struct RenderBlock {
    pub elements: Vec<RenderElement>,
}

/// Render element types
#[derive(Debug)]
pub enum RenderElement {
    Text(Expr),
    Element { tag: String, attributes: HashMap<String, Expr>, children: Vec<RenderElement> },
    Conditional { condition: Expr, then: Vec<RenderElement>, else_: Option<Vec<RenderElement>> },
}

/// Event handler for a view
#[derive(Debug)]
pub struct EventHandler {
    pub event: String,
    pub target: Option<String>,
    pub action: Vec<Stmt>,
}

/// AST node for a statement
#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Assign { target: String, value: Expr },
    If { condition: Expr, then: Vec<Stmt>, else_: Option<Vec<Stmt>> },
    Loop { body: Vec<Stmt> },
    Return(Option<Expr>),
    Let { name: String, value: Expr },
    Mut { name: String, value: Expr },
}

/// AST node for an expression
#[derive(Debug)]
pub enum Expr {
    // Literals
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),

    // Variables and identifiers
    Identifier(String),
    CellAccess(String),

    // Binary operations
    BinaryOp { left: Box<Expr>, op: BinaryOp, right: Box<Expr> },

    // Function calls
    Call { func: String, args: Vec<Expr> },

    // Conditional expressions
    If { condition: Box<Expr>, then: Box<Expr>, else_: Box<Expr> },

    // String concatenation
    Concat { left: Box<Expr>, right: Box<Expr> },

    // Property access
    PropertyAccess { object: Box<Expr>, property: String },
}

/// Binary operators
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    And,
    Or,
}

/// Token types for the lexer
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Fn,
    View,
    Cell,
    Flow,
    Watch,
    On,
    Style,
    Render,
    If,
    Then,
    Else,
    Let,
    Mut,
    Return,

    // Identifiers and literals
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(f64),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Assign,
    PlusAssign,
    MinusAssign,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    Colon,
    Arrow,

    // Special
    EOF,
}
