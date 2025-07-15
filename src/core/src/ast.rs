//! Abstract Syntax Tree (AST) definitions for Gigli

use std::collections::HashMap;

/// AST node for a program (list of modules, functions, classes and components)
#[derive(Debug)]
pub struct AST {
    pub modules: Vec<Module>,
    pub functions: Vec<Function>,
    pub classes: Vec<Class>,
    pub components: Vec<ComponentNode>, // NEW: replaces views
    pub imports: Vec<Import>,
}

/// AST node for a module
#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub items: Vec<ModuleItem>,
}

/// Module item types
#[derive(Debug)]
pub enum ModuleItem {
    Function(Function),
    Class(Class),
    Component(ComponentNode), // NEW: replaces View
    Constant(Constant),
}

/// AST node for a constant
#[derive(Debug)]
pub struct Constant {
    pub name: String,
    pub value: Expr,
    pub type_annotation: Option<Type>,
}

/// AST node for a class
#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub constructor: Option<Constructor>,
}

/// AST node for a field
#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub type_annotation: Option<Type>,
    pub initial_value: Option<Expr>,
    pub is_public: bool,
}

/// AST node for a method
#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Vec<Stmt>,
    pub is_public: bool,
}

/// AST node for a constructor
#[derive(Debug)]
pub struct Constructor {
    pub params: Vec<Parameter>,
    pub body: Vec<Stmt>,
}

/// AST node for a parameter
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<Type>,
    pub default_value: Option<Expr>,
    pub is_ref: bool,      // NEW: & reference
    pub is_mut_ref: bool,  // NEW: &mut reference
}

/// AST node for an import
#[derive(Debug)]
pub struct Import {
    pub module: String,
    pub items: Vec<String>,
    pub alias: Option<String>,
}

/// AST node for a function
#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Vec<Stmt>,
    pub is_public: bool,
    pub is_async: bool, // NEW: async fn support
}

/// AST node for a component (unified logic, markup, style)
#[derive(Debug, Clone)]
pub struct ComponentNode {
    pub name: String,
    pub state_vars: Vec<StateVar>,
    pub let_vars: Vec<LetVar>,
    pub functions: Vec<Function>,
    pub markup: Vec<MarkupNode>,
    pub style: Option<String>, // raw CSS block
}

impl ComponentNode {
    pub fn to_string_formatted(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("component {} {{\n", self.name));
        // TODO: Add formatted output for state, let, fn, markup, style
        s.push_str("}}\n");
        s
    }
}

/// AST node for a state variable (reactive)
#[derive(Debug, Clone)]
pub struct StateVar {
    pub name: String,
    pub type_annotation: Option<Type>,
    pub initial_value: Expr,
}

/// AST node for a let variable (derived, immutable or computed)
#[derive(Debug, Clone)]
pub struct LetVar {
    pub name: String,
    pub type_annotation: Option<Type>,
    pub value: Expr,
}

/// Markup node (HTML-like structure, including control flow blocks)
#[derive(Debug, Clone)]
pub enum MarkupNode {
    Element {
        tag: String,
        attributes: HashMap<String, Expr>,
        children: Vec<MarkupNode>,
    },
    Text(Expr),
    IfBlock(IfBlockNode),
    ForLoop(ForLoopBlockNode),
    // ... possibly more, e.g., ComponentInclude, etc.
}

/// If block node for {#if ...}{:else}{/if}
#[derive(Debug, Clone)]
pub struct IfBlockNode {
    pub condition: Expr,
    pub then_branch: Vec<MarkupNode>,
    pub else_branch: Option<Vec<MarkupNode>>,
}

/// For loop block node for {#for ...}{/for}
#[derive(Debug, Clone)]
pub struct ForLoopBlockNode {
    pub iterator: String,
    pub iterable: Expr,
    pub body: Vec<MarkupNode>,
}

/// AST node for a statement
#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Assign { target: String, value: Expr },
    If { condition: Expr, then: Vec<Stmt>, else_: Option<Vec<Stmt>> },
    Loop {
        init: Option<Box<Stmt>>,
        condition: Option<Expr>,
        update: Option<Box<Stmt>>,
        body: Vec<Stmt>
    },
    ForIn { variable: String, iterable: Expr, body: Vec<Stmt> },
    ForOf { variable: String, iterable: Expr, body: Vec<Stmt> },
    Return(Option<Expr>),
    StateVarDecl(StateVar), // NEW: state variable declaration
    LetVarDecl(LetVar),    // NEW: let variable declaration
    Block(Vec<Stmt>),
    Try { body: Vec<Stmt>, catch: Option<CatchBlock>, finally: Option<Vec<Stmt>> },
    Throw(Expr),
    Break(Option<String>), // label for labeled break
    Continue(Option<String>), // label for labeled continue
    Switch {
        expression: Expr,
        cases: Vec<SwitchCase>,
        default: Option<Vec<Stmt>>,
    },
    Reactive { name: String, expr: Expr }, // NEW: $: reactivity
    Comprehension { target: String, iter: Expr, filter: Option<Expr>, expr: Expr }, // NEW: list comprehensions
}

/// Catch block for try-catch statements
#[derive(Debug, Clone)]
pub struct CatchBlock {
    pub error_var: String,
    pub body: Vec<Stmt>,
}

/// Switch case
#[derive(Debug, Clone)]
pub struct SwitchCase {
    pub value: Expr,
    pub body: Vec<Stmt>,
}

/// AST node for an expression
#[derive(Debug, Clone)]
pub enum Expr {
    // Literals
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    NullLiteral,
    UndefinedLiteral,
    ArrayLiteral(Vec<Expr>),
    ObjectLiteral(Vec<ObjectProperty>),

    // Variables and identifiers
    Identifier(String),
    CellAccess(String),

    // Binary operations
    BinaryOp { left: Box<Expr>, op: BinaryOp, right: Box<Expr> },

    // Unary operations
    UnaryOp { op: UnaryOp, operand: Box<Expr> },

    // Function calls
    Call { func: Box<Expr>, args: Vec<Expr> },

    // Method calls
    MethodCall { object: Box<Expr>, method: String, args: Vec<Expr> },

    // Conditional expressions
    If { condition: Box<Expr>, then: Box<Expr>, else_: Box<Expr> },

    // String concatenation
    Concat { left: Box<Expr>, right: Box<Expr> },

    // Property access
    PropertyAccess { object: Box<Expr>, property: String },

    // Array access
    ArrayAccess { array: Box<Expr>, index: Box<Expr> },

    // Template literals
    TemplateLiteral { parts: Vec<TemplatePart> },

    // Arrow functions
    ArrowFunction { params: Vec<Parameter>, body: Vec<Stmt> },

    // New expression (class instantiation)
    New { class: Box<Expr>, args: Vec<Expr> },

    // Type assertion
    TypeAssert { value: Box<Expr>, type_: Type },
    Await(Box<Expr>), // NEW: await expr
    Comprehension { target: String, iter: Box<Expr>, filter: Option<Box<Expr>>, expr: Box<Expr> }, // NEW: list comprehensions
}

/// Object property for object literals
#[derive(Debug, Clone)]
pub struct ObjectProperty {
    pub key: String,
    pub value: Expr,
    pub shorthand: bool,
}

/// Template literal part
#[derive(Debug, Clone)]
pub enum TemplatePart {
    String(String),
    Expression(Expr),
}

/// Binary operators
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    UnsignedRightShift,
}

/// Unary operators
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
    BitwiseNot,
    Increment,
    Decrement,
    TypeOf,
    Void,
    Delete,
}

/// Type system
#[derive(Debug, Clone)]
pub enum Type {
    String,
    Number,
    Boolean,
    Void,
    Any,
    Array(Box<Type>),
    Object(Vec<ObjectTypeProperty>),
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    Union(Vec<Type>),
    Generic { name: String, type_args: Vec<Type> },
    Custom(String),
    Option(Box<Type>), // NEW: Option<T>
    Result(Box<Type>, Box<Type>), // NEW: Result<T, E>
    Ref(Box<Type>),    // NEW: &T
    MutRef(Box<Type>), // NEW: &mut T
}

/// Object type property
#[derive(Debug, Clone)]
pub struct ObjectTypeProperty {
    pub name: String,
    pub type_: Type,
    pub optional: bool,
}

/// Token types for the lexer
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Fn,
    Class,
    Constructor,
    Extends,
    Super,
    This,
    New,
    Component, // NEW: component keyword
    State,     // NEW: state keyword
    Struct,    // NEW: struct keyword
    Enum,      // NEW: enum keyword
    On,        // event handler (on:event)
    Style,
    If,
    Then, // (may be removed later if not in new spec)
    Else,
    Let,
    Mut,
    Return,
    Try,
    Catch,
    Finally,
    Throw,
    Break,
    Continue,
    Switch,
    Case,
    Default,
    For,
    In,
    Of,
    While,
    Do,
    Loop,
    Import,
    Export,
    From,
    As,
    Module,
    Public,
    Private,
    Protected,
    Static,
    Abstract,
    Interface,
    Type,
    Const,
    Var,

    // Control flow blocks (NEW for v2.0)
    HashIf,            // {#if ...}
    HashFor,           // {#for ...}
    HashElse,          // {:else}
    ForwardSlashIf,    // {/if}
    ForwardSlashFor,   // {/for}

    // Identifiers and literals
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    TemplateLiteral(String),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    CaretAssign,
    And,
    Or,
    Not,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    Increment,
    Decrement,

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
    QuestionMark,
    DoubleColon,
    At,

    // Special
    EOF,
}
