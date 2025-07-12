//! IR generation for GigliOptix
use crate::ast::{AST, Function, Stmt, Expr, View, Flow, BinaryOp};
use crate::ir::{IRModule, IRFunction, IRStmt, IRExpr};

/// Generates IR from AST
pub fn generate_ir(ast: &AST) -> IRModule {
    println!("[IR] Generating IR");
    let mut functions = Vec::new();

    // Generate IR for regular functions
    for f in &ast.functions {
        functions.push(lower_function(f));
    }

    // Generate IR for views (convert to functions)
    for view in &ast.views {
        functions.push(lower_view(view));
    }

    // Generate IR for flows (convert to functions)
    for flow in &ast.flows {
        functions.push(lower_flow(flow));
    }

    IRModule { functions }
}

fn lower_function(f: &Function) -> IRFunction {
    IRFunction {
        name: f.name.clone(),
        body: f.body.iter().map(|s| lower_stmt(s)).collect(),
    }
}

fn lower_view(view: &View) -> IRFunction {
    let mut body = Vec::new();

    // Initialize cells
    for cell in &view.cells {
        body.push(IRStmt::Call {
            func: "cell_create".to_string(),
            args: vec![
                IRExpr::StringLiteral(cell.name.clone()),
                lower_expr(&cell.initial_value),
            ],
        });
    }

    // Set up event handlers
    for handler in &view.event_handlers {
        body.push(IRStmt::Call {
            func: "add_event_listener".to_string(),
            args: vec![
                IRExpr::StringLiteral(handler.event.clone()),
                IRExpr::StringLiteral(handler.target.clone().unwrap_or_default()),
                IRExpr::StringLiteral(format!("{}_handler", handler.event)),
            ],
        });
    }

    // Generate render function
    body.push(IRStmt::Call {
        func: "render_view".to_string(),
        args: vec![
            IRExpr::StringLiteral(view.name.clone()),
            lower_render_block(&view.render),
        ],
    });

    IRFunction {
        name: format!("view_{}", view.name),
        body,
    }
}

fn lower_flow(flow: &Flow) -> IRFunction {
    let mut body = Vec::new();

    // Convert flow body to statements
    for stmt in &flow.body {
        body.push(lower_stmt(stmt));
    }

    IRFunction {
        name: format!("flow_{}", flow.name),
        body,
    }
}

fn lower_render_block(render: &crate::ast::RenderBlock) -> IRExpr {
    // For now, convert render block to a string representation
    let mut elements = Vec::new();
    for element in &render.elements {
        elements.push(lower_render_element(element));
    }

    // Join elements with newlines
    IRExpr::StringLiteral(elements.join("\n"))
}

fn lower_render_element(element: &crate::ast::RenderElement) -> String {
    match element {
        crate::ast::RenderElement::Text(expr) => {
            format!("{}", lower_expr_to_string(expr))
        }
        crate::ast::RenderElement::Element { tag, attributes, children } => {
            let mut attrs = Vec::new();
            for (key, value) in attributes {
                attrs.push(format!("{}=\"{}\"", key, lower_expr_to_string(value)));
            }
            let attr_str = attrs.join(" ");
            let children_str = children.iter().map(|c| lower_render_element(c)).collect::<Vec<_>>().join("");
            format!("<{} {}>{}</{}>", tag, attr_str, children_str, tag)
        }
        crate::ast::RenderElement::Conditional { condition, then, else_ } => {
            let condition_str = lower_expr_to_string(condition);
            let then_str = then.iter().map(|e| lower_render_element(e)).collect::<Vec<_>>().join("");
            let else_str = else_.as_ref().map(|elements| elements.iter().map(|e| lower_render_element(e)).collect::<Vec<_>>().join("")).unwrap_or_default();
            format!("if({}) {{ {} }} else {{ {} }}", condition_str, then_str, else_str)
        }
    }
}

fn lower_expr_to_string(expr: &Expr) -> String {
    match expr {
        Expr::StringLiteral(s) => s.clone(),
        Expr::NumberLiteral(n) => n.to_string(),
        Expr::BooleanLiteral(b) => b.to_string(),
        Expr::Identifier(s) => s.clone(),
        Expr::CellAccess(s) => format!("cell_{}", s),
        Expr::BinaryOp { left, op, right } => {
            let op_str = match op {
                BinaryOp::Add => "+",
                BinaryOp::Subtract => "-",
                BinaryOp::Multiply => "*",
                BinaryOp::Divide => "/",
                BinaryOp::Modulo => "%",
                BinaryOp::Equal => "==",
                BinaryOp::NotEqual => "!=",
                BinaryOp::LessThan => "<",
                BinaryOp::LessThanEqual => "<=",
                BinaryOp::GreaterThan => ">",
                BinaryOp::GreaterThanEqual => ">=",
                BinaryOp::And => "&&",
                BinaryOp::Or => "||",
            };
            format!("({} {} {})", lower_expr_to_string(left), op_str, lower_expr_to_string(right))
        }
        Expr::Call { func, args } => {
            let args_str = args.iter().map(|a| lower_expr_to_string(a)).collect::<Vec<_>>().join(", ");
            format!("{}({})", func, args_str)
        }
        Expr::If { condition, then, else_ } => {
            let else_str = lower_expr_to_string(else_);
            format!("if({}) {{ {} }} else {{ {} }}", lower_expr_to_string(condition), lower_expr_to_string(then), else_str)
        }
        Expr::Concat { left, right } => {
            format!("{}+{}", lower_expr_to_string(left), lower_expr_to_string(right))
        }
        Expr::PropertyAccess { object, property } => {
            format!("{}.{}", lower_expr_to_string(object), property)
        }
    }
}

fn lower_stmt(s: &Stmt) -> IRStmt {
    match s {
        Stmt::Expr(e) => match e {
            Expr::Call { func, args } => IRStmt::Call {
                func: func.clone(),
                args: args.iter().map(|a| lower_expr(a)).collect(),
            },
            _ => IRStmt::Call {
                func: "expr".to_string(),
                args: vec![lower_expr(e)],
            },
        },
        Stmt::Assign { target, value } => IRStmt::Call {
            func: "assign".to_string(),
            args: vec![
                IRExpr::StringLiteral(target.clone()),
                lower_expr(value),
            ],
        },
        Stmt::If { condition, then, else_ } => IRStmt::Call {
            func: "if".to_string(),
            args: vec![
                lower_expr(condition),
                IRExpr::StringLiteral(then.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(";")),
                IRExpr::StringLiteral(else_.as_ref().map(|stmts| stmts.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(";")).unwrap_or_default()),
            ],
        },
        Stmt::Loop { body } => IRStmt::Call {
            func: "loop".to_string(),
            args: vec![
                IRExpr::StringLiteral(body.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(";")),
            ],
        },
        Stmt::Return(value) => IRStmt::Call {
            func: "return".to_string(),
            args: value.as_ref().map(|v| vec![lower_expr(v)]).unwrap_or_default(),
        },
        Stmt::Let { name, value } => IRStmt::Call {
            func: "let".to_string(),
            args: vec![
                IRExpr::StringLiteral(name.clone()),
                lower_expr(value),
            ],
        },
        Stmt::Mut { name, value } => IRStmt::Call {
            func: "mut".to_string(),
            args: vec![
                IRExpr::StringLiteral(name.clone()),
                lower_expr(value),
            ],
        },
    }
}

fn lower_expr(e: &Expr) -> IRExpr {
    match e {
        Expr::StringLiteral(s) => IRExpr::StringLiteral(s.clone()),
        Expr::NumberLiteral(n) => IRExpr::StringLiteral(n.to_string()),
        Expr::BooleanLiteral(b) => IRExpr::StringLiteral(b.to_string()),
        Expr::Identifier(s) => IRExpr::Identifier(s.clone()),
        Expr::CellAccess(s) => IRExpr::Identifier(format!("cell_{}", s)),
        Expr::BinaryOp { left, op, right } => IRExpr::StringLiteral(format!("({} {} {})",
            lower_expr_to_string(left),
            match op {
                BinaryOp::Add => "+",
                BinaryOp::Subtract => "-",
                BinaryOp::Multiply => "*",
                BinaryOp::Divide => "/",
                BinaryOp::Modulo => "%",
                BinaryOp::Equal => "==",
                BinaryOp::NotEqual => "!=",
                BinaryOp::LessThan => "<",
                BinaryOp::LessThanEqual => "<=",
                BinaryOp::GreaterThan => ">",
                BinaryOp::GreaterThanEqual => ">=",
                BinaryOp::And => "&&",
                BinaryOp::Or => "||",
            },
            lower_expr_to_string(right)
        )),
        Expr::Call { func, args } => IRExpr::StringLiteral(format!("{}({})",
            func,
            args.iter().map(|a| lower_expr_to_string(a)).collect::<Vec<_>>().join(", ")
        )),
        Expr::If { condition, then, else_ } => IRExpr::StringLiteral(format!("if({}) {{ {} }} else {{ {} }}",
            lower_expr_to_string(condition),
            lower_expr_to_string(then),
            lower_expr_to_string(else_)
        )),
        Expr::Concat { left, right } => IRExpr::StringLiteral(format!("{}+{}",
            lower_expr_to_string(left),
            lower_expr_to_string(right)
        )),
        Expr::PropertyAccess { object, property } => IRExpr::StringLiteral(format!("{}.{}",
            lower_expr_to_string(object),
            property
        )),
    }
}
