//! Semantic analysis for GigliOptix

use crate::ast::*;
use std::collections::HashMap;

pub struct SemanticAnalyzer {
    pub errors: Vec<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn analyze(&mut self, ast: &AST) {
        let mut global_vars = HashMap::new();
        for func in &ast.functions {
            self.check_function(func);
        }
        for stmt in &ast.cells {
            // Cells are global reactive state
            global_vars.insert(stmt.name.clone(), stmt.type_annotation.clone());
        }
        for view in &ast.views {
            for cell in &view.cells {
                global_vars.insert(cell.name.clone(), cell.type_annotation.clone());
            }
            for stmt in &view.flows {
                // TODO: Check flows
            }
            for stmt in &view.event_handlers {
                // TODO: Check event handlers
            }
        }
        // TODO: Add checks for classes, modules, etc.
    }

    fn check_function(&mut self, func: &Function) {
        let mut local_vars = HashMap::new();
        for param in &func.params {
            local_vars.insert(param.name.clone(), param.type_annotation.clone());
            // Ownership/borrowing: warn if both is_ref and is_mut_ref
            if param.is_ref && param.is_mut_ref {
                self.errors.push(format!("Parameter '{}' cannot be both & and &mut", param.name));
            }
        }
        if func.is_async {
            for stmt in &func.body {
                self.check_stmt(stmt, &mut local_vars, true);
            }
        } else {
            for stmt in &func.body {
                self.check_stmt(stmt, &mut local_vars, false);
            }
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt, vars: &mut HashMap<String, Option<Type>>, in_async: bool) {
        match stmt {
            Stmt::Expr(expr) => { self.check_expr(expr, vars, in_async); },
            Stmt::Return(Some(expr)) => { self.check_expr(expr, vars, in_async); },
            Stmt::Let { name, value, type_annotation } => {
                self.check_expr(value, vars, in_async);
                vars.insert(name.clone(), type_annotation.clone());
            },
            Stmt::Mut { name, value, type_annotation } => {
                self.check_expr(value, vars, in_async);
                vars.insert(name.clone(), type_annotation.clone());
            },
            Stmt::Reactive { name, expr } => {
                self.check_expr(expr, vars, in_async);
                if !vars.contains_key(name) {
                    self.errors.push(format!("Reactive variable '${}' not declared", name));
                }
            },
            Stmt::Comprehension { target, iter, filter, expr } => {
                self.check_expr(iter, vars, in_async);
                if let Some(f) = filter { self.check_expr(f, vars, in_async); }
                self.check_expr(expr, vars, in_async);
                vars.insert(target.clone(), None); // Assume type inference for now
            },
            Stmt::Block(stmts) => for s in stmts { self.check_stmt(s, vars, in_async); },
            // TODO: Add more statement checks (If, Loop, For, etc.)
            _ => {}
        }
    }

    fn check_expr(&mut self, expr: &Expr, vars: &mut HashMap<String, Option<Type>>, in_async: bool) {
        match expr {
            Expr::Await(inner) => {
                if !in_async {
                    self.errors.push("'await' used outside of async function".to_string());
                }
                self.check_expr(inner, vars, in_async);
            },
            Expr::Comprehension { target, iter, filter, expr } => {
                self.check_expr(iter, vars, in_async);
                if let Some(f) = filter { self.check_expr(f, vars, in_async); }
                self.check_expr(expr, vars, in_async);
                vars.insert(target.clone(), None);
            },
            Expr::Call { func, args } => {
                self.check_expr(func, vars, in_async);
                for arg in args { self.check_expr(arg, vars, in_async); }
            },
            Expr::Identifier(name) => {
                if !vars.contains_key(name) {
                    self.errors.push(format!("Use of undeclared variable '{}'", name));
                }
            },
            Expr::BinaryOp { left, right, .. } => {
                self.check_expr(left, vars, in_async);
                self.check_expr(right, vars, in_async);
            },
            Expr::UnaryOp { operand, .. } => self.check_expr(operand, vars, in_async),
            Expr::If { condition, then, else_ } => {
                self.check_expr(condition, vars, in_async);
                self.check_expr(then, vars, in_async);
                self.check_expr(else_, vars, in_async);
            },
            Expr::Option(_) | Expr::Result { .. } => {
                // TODO: Check Option/Result construction and usage
            },
            Expr::ArrayLiteral(items) => for item in items { self.check_expr(item, vars, in_async); },
            Expr::ObjectLiteral(props) => for prop in props { self.check_expr(&prop.value, vars, in_async); },
            // TODO: Add more expression checks as needed
            _ => {}
        }
    }
}

pub fn semantic_stub() {
    // TODO: Implement semantic analysis
}
