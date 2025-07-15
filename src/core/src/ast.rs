//! Abstract Syntax Tree (AST) definitions for GigliOptix

use std::collections::HashMap;

/// AST node for a program (list of modules, functions, classes and views)
#[derive(Debug)]
pub struct AST {
    pub modules: Vec<Module>,
    pub functions: Vec<Function>,
    pub classes: Vec<Class>,
    pub views: Vec<View>,
    pub cells: Vec<Cell>,
    pub flows: Vec<Flow>,
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
    View(View),
    Cell(Cell),
    Flow(Flow),
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

/// AST node for a reactive cell (state container)
#[derive(Debug)]
pub struct Cell {
    pub name: String,
    pub initial_value: Expr,
    pub type_annotation: Option<Type>,
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
    OnMount,
    OnUnmount,
}

/// AST node for a view (declarative UI component)
#[derive(Debug)]
pub struct View {
    pub name: String,
    pub props: Vec<Parameter>,
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
    pub media_queries: Vec<MediaQuery>,
}

/// Media query for responsive design
#[derive(Debug)]
pub struct MediaQuery {
    pub condition: String,
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
    Element {
        tag: String,
        attributes: HashMap<String, Expr>,
        children: Vec<RenderElement>,
        key: Option<Expr>,
    },
    Conditional {
        condition: Expr,
        then: Vec<RenderElement>,
        else_: Option<Vec<RenderElement>>
    },
    Loop {
        iterator: String,
        items: Expr,
        body: Vec<RenderElement>,
        key: Option<Expr>,
    },
    Fragment(Vec<RenderElement>),
    Component {
        name: String,
        props: HashMap<String, Expr>,
        children: Vec<RenderElement>,
    },
}

/// Event handler for a view
#[derive(Debug)]
pub struct EventHandler {
    pub event: String,
    pub target: Option<String>,
    pub action: Vec<Stmt>,
    pub modifiers: Vec<String>, // e.g., "prevent", "stop", "once"
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
    Let { name: String, value: Expr, type_annotation: Option<Type> },
    Mut { name: String, value: Expr, type_annotation: Option<Type> },
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
