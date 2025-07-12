//! Intermediate Representation (IR) module for GigliOptix

pub mod generator;

/// IR module: list of IR functions
pub struct IRModule {
    pub functions: Vec<IRFunction>,
}

/// IR for a function
pub struct IRFunction {
    pub name: String,
    pub body: Vec<IRStmt>,
}

/// IR for a statement
pub enum IRStmt {
    Call { func: String, args: Vec<IRExpr> },
}

/// IR for an expression
pub enum IRExpr {
    StringLiteral(String),
    Identifier(String),
}
