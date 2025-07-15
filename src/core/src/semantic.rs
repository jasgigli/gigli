//! Semantic analysis for Gigli

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
        for component in &ast.components {
            self.check_component(component, &mut global_vars);
        }
        // TODO: Add checks for classes, modules, etc.
    }

    fn check_component(&mut self, component: &ComponentNode, global_vars: &mut HashMap<String, Option<Type>>) {
        let mut local_vars = global_vars.clone();
        // Register state vars (reactive)
        for state in &component.state_vars {
            local_vars.insert(state.name.clone(), state.type_annotation.clone());
        }
        // Register let vars (derived)
        for letv in &component.let_vars {
            // Check if let depends on any state var (reactivity)
            let mut depends_on_state = false;
            self.check_expr_reactivity(&letv.value, &local_vars, &component.state_vars, &mut depends_on_state);
            if depends_on_state {
                // Mark as derived reactive (could store this info in a real implementation)
            }
            local_vars.insert(letv.name.clone(), letv.type_annotation.clone());
        }
        // Check functions
        for func in &component.functions {
            self.check_function(func);
        }
        // Check markup
        for node in &component.markup {
            self.check_markup(node, &local_vars);
        }
    }

    fn check_markup(&mut self, node: &MarkupNode, vars: &HashMap<String, Option<Type>>) {
        match node {
            MarkupNode::Element { tag:_, attributes, children } => {
                for expr in attributes.values() {
                    self.check_expr(expr, &mut vars.clone(), false);
                }
                for child in children {
                    self.check_markup(child, vars);
                }
            }
            MarkupNode::Text(expr) => {
                self.check_expr(expr, &mut vars.clone(), false);
            }
            MarkupNode::IfBlock(ifblock) => {
                self.check_expr(&ifblock.condition, &mut vars.clone(), false);
                for n in &ifblock.then_branch {
                    self.check_markup(n, vars);
                }
                if let Some(else_branch) = &ifblock.else_branch {
                    for n in else_branch {
                        self.check_markup(n, vars);
                    }
                }
            }
            MarkupNode::ForLoop(forblock) => {
                self.check_expr(&forblock.iterable, &mut vars.clone(), false);
                let mut loop_vars = vars.clone();
                loop_vars.insert(forblock.iterator.clone(), None);
                for n in &forblock.body {
                    self.check_markup(n, &loop_vars);
                }
            }
        }
    }

    /// Recursively check if an expression depends on any state variable
    fn check_expr_reactivity(&mut self, expr: &Expr, vars: &HashMap<String, Option<Type>>, state_vars: &[StateVar], found: &mut bool) {
        match expr {
            Expr::Identifier(name) => {
                if state_vars.iter().any(|s| &s.name == name) {
                    *found = true;
                }
            }
            Expr::BinaryOp { left, right, .. } => {
                self.check_expr_reactivity(left, vars, state_vars, found);
                self.check_expr_reactivity(right, vars, state_vars, found);
            }
            Expr::UnaryOp { operand, .. } => {
                self.check_expr_reactivity(operand, vars, state_vars, found);
            }
            Expr::Call { func, args } => {
                self.check_expr_reactivity(func, vars, state_vars, found);
                for arg in args {
                    self.check_expr_reactivity(arg, vars, state_vars, found);
                }
            }
            Expr::ArrayLiteral(items) => {
                for item in items {
                    self.check_expr_reactivity(item, vars, state_vars, found);
                }
            }
            Expr::ObjectLiteral(props) => {
                for prop in props {
                    self.check_expr_reactivity(&prop.value, vars, state_vars, found);
                }
            }
            _ => {}
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt, vars: &mut HashMap<String, Option<Type>>, in_async: bool) {
        match stmt {
            Stmt::Expr(expr) => { self.check_expr(expr, vars, in_async); },
            Stmt::Return(Some(expr)) => { self.check_expr(expr, vars, in_async); },
            Stmt::StateVarDecl(state) => {
                self.check_expr(&state.initial_value, vars, in_async);
                vars.insert(state.name.clone(), state.type_annotation.clone());
            },
            Stmt::LetVarDecl(letv) => {
                self.check_expr(&letv.value, vars, in_async);
                if vars.contains_key(&letv.name) {
                    self.errors.push(format!("Cannot reassign to immutable let variable '{}'.", letv.name));
                }
                vars.insert(letv.name.clone(), letv.type_annotation.clone());
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
            // Option/Result support can be added here in the future
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
