//! IR generation for GigliOptix
use crate::ast::*;

#[derive(Debug)]
pub struct IRModule {
    pub functions: Vec<IRFunction>,
}

#[derive(Debug)]
pub struct IRFunction {
    pub name: String,
    pub body: Vec<IRStmt>,
}

#[derive(Debug)]
pub enum IRStmt {
    Call { func: String, args: Vec<IRExpr> },
    Assign { target: String, value: IRExpr }, // assignment
    Await(IRExpr), // async/await
    Reactive { name: String, expr: IRExpr }, // $: reactivity
    Comprehension { target: String, iter: IRExpr, filter: Option<IRExpr>, expr: IRExpr },
    Render(IRExpr), // UI render
    EventBind { target: String, event: String, handler: String }, // event binding
    DomOp { op: String, args: Vec<IRExpr> }, // DOM operation
    Return(Option<IRExpr>),
    // ... add more as needed ...
}

#[derive(Debug)]
pub enum IRExpr {
    StringLiteral(String),
    NumberLiteral(f64),
    Identifier(String),
    Await(Box<IRExpr>),
    Option(Box<IRExpr>),
    Result { ok: Box<IRExpr>, err: Box<IRExpr> },
    Comprehension { target: String, iter: Box<IRExpr>, filter: Option<Box<IRExpr>>, expr: Box<IRExpr> },
    DomRef(String), // reference to DOM node
    // ... add more as needed ...
}

pub fn generate_ir(ast: &AST) -> IRModule {
    let mut functions = Vec::new();

    // Convert functions
    for function in &ast.functions {
        functions.push(lower_function(function));
    }

    // Convert views
    for view in &ast.views {
        functions.push(lower_view(view));
    }

    // Convert flows
    for flow in &ast.flows {
        functions.push(lower_flow(flow));
    }

    // Convert classes
    for class in &ast.classes {
        functions.extend(lower_class(class));
    }

    IRModule { functions }
}

fn lower_function(f: &Function) -> IRFunction {
    let mut body = Vec::new();

    // Convert function body to statements
    for stmt in &f.body {
        body.push(lower_stmt(stmt));
    }

    IRFunction {
        name: format!("fn_{}", f.name),
        body,
    }
}

fn lower_view(view: &View) -> IRFunction {
    let mut body = Vec::new();

    // Convert cells
    for cell in &view.cells {
        body.push(IRStmt::Call {
            func: "cell".to_string(),
            args: vec![
                IRExpr::StringLiteral(cell.name.clone()),
                lower_expr(&cell.initial_value),
            ],
        });
    }

    // Convert flows
    for flow in &view.flows {
        body.push(IRStmt::Call {
            func: "flow".to_string(),
            args: vec![
                IRExpr::StringLiteral(flow.name.clone()),
                IRExpr::StringLiteral(format!("{:?}", flow.trigger)),
            ],
        });
    }

    // Convert render block
    body.push(IRStmt::Call {
        func: "render".to_string(),
        args: vec![lower_render_block(&view.render)],
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

fn lower_class(class: &Class) -> Vec<IRFunction> {
    let mut functions = Vec::new();

    // Convert methods
    for method in &class.methods {
        functions.push(lower_function(&Function {
            name: format!("{}_{}", class.name, method.name),
            params: method.params.clone(),
            return_type: method.return_type.clone(),
            body: method.body.clone(),
            is_public: method.is_public,
            is_async: method.is_async,
        }));
    }

    // Convert constructor
    if let Some(constructor) = &class.constructor {
        functions.push(lower_function(&Function {
            name: format!("{}_constructor", class.name),
            params: constructor.params.clone(),
            return_type: None,
            body: constructor.body.clone(),
            is_public: true,
            is_async: false,
        }));
    }

    functions
}

fn lower_render_block(render: &RenderBlock) -> IRExpr {
    // For now, convert render block to a string representation
    let mut elements = Vec::new();
    for element in &render.elements {
        elements.push(lower_render_element(element));
    }

    // Join elements with newlines
    IRExpr::StringLiteral(elements.join("\n"))
}

fn lower_render_element(element: &RenderElement) -> String {
    match element {
        RenderElement::Text(expr) => {
            format!("{}", lower_expr_to_string(expr))
        }
        RenderElement::Element { tag, attributes, children, key: _ } => {
            let mut attrs = Vec::new();
            for (key, value) in attributes {
                attrs.push(format!("{}=\"{}\"", key, lower_expr_to_string(value)));
            }
            let attr_str = attrs.join(" ");
            let children_str = children.iter().map(|c| lower_render_element(c)).collect::<Vec<_>>().join("");
            format!("<{} {}>{}</{}>", tag, attr_str, children_str, tag)
        }
        RenderElement::Conditional { condition, then, else_ } => {
            let condition_str = lower_expr_to_string(condition);
            let then_str = then.iter().map(|e| lower_render_element(e)).collect::<Vec<_>>().join("");
            let else_str = else_.as_ref().map(|elements| elements.iter().map(|e| lower_render_element(e)).collect::<Vec<_>>().join("")).unwrap_or_default();
            format!("if({}) {{ {} }} else {{ {} }}", condition_str, then_str, else_str)
        }
        RenderElement::Loop { iterator, items, body, key: _ } => {
            let items_str = lower_expr_to_string(items);
            let body_str = body.iter().map(|e| lower_render_element(e)).collect::<Vec<_>>().join("");
            format!("for({} in {}) {{ {} }}", iterator, items_str, body_str)
        }
        RenderElement::Fragment(elements) => {
            elements.iter().map(|e| lower_render_element(e)).collect::<Vec<_>>().join("")
        }
        RenderElement::Component { name, props, children } => {
            let props_str = props.iter().map(|(k, v)| format!("{}={}", k, lower_expr_to_string(v))).collect::<Vec<_>>().join(" ");
            let children_str = children.iter().map(|e| lower_render_element(e)).collect::<Vec<_>>().join("");
            format!("<{} {}>{}</{}>", name, props_str, children_str, name)
        }
    }
}

fn lower_expr_to_string(expr: &Expr) -> String {
    match expr {
        Expr::StringLiteral(s) => s.clone(),
        Expr::NumberLiteral(n) => n.to_string(),
        Expr::BooleanLiteral(b) => b.to_string(),
        Expr::NullLiteral => "null".to_string(),
        Expr::UndefinedLiteral => "undefined".to_string(),
        Expr::Identifier(s) => s.clone(),
        Expr::CellAccess(s) => format!("cell_{}", s),
        Expr::BinaryOp { left, op, right } => {
            let op_str = match op {
                BinaryOp::Add => "+",
                BinaryOp::Subtract => "-",
                BinaryOp::Multiply => "*",
                BinaryOp::Divide => "/",
                BinaryOp::Modulo => "%",
                BinaryOp::Power => "**",
                BinaryOp::Equal => "==",
                BinaryOp::NotEqual => "!=",
                BinaryOp::StrictEqual => "===",
                BinaryOp::StrictNotEqual => "!==",
                BinaryOp::LessThan => "<",
                BinaryOp::LessThanEqual => "<=",
                BinaryOp::GreaterThan => ">",
                BinaryOp::GreaterThanEqual => ">=",
                BinaryOp::And => "&&",
                BinaryOp::Or => "||",
                BinaryOp::BitwiseAnd => "&",
                BinaryOp::BitwiseOr => "|",
                BinaryOp::BitwiseXor => "^",
                BinaryOp::LeftShift => "<<",
                BinaryOp::RightShift => ">>",
                BinaryOp::UnsignedRightShift => ">>>",
            };
            format!("({} {} {})", lower_expr_to_string(left), op_str, lower_expr_to_string(right))
        }
        Expr::UnaryOp { op, operand } => {
            let op_str = match op {
                UnaryOp::Plus => "+",
                UnaryOp::Minus => "-",
                UnaryOp::Not => "!",
                UnaryOp::BitwiseNot => "~",
                UnaryOp::Increment => "++",
                UnaryOp::Decrement => "--",
                UnaryOp::TypeOf => "typeof ",
                UnaryOp::Void => "void ",
                UnaryOp::Delete => "delete ",
            };
            format!("{}{}", op_str, lower_expr_to_string(operand))
        }
        Expr::Call { func, args } => {
            let args_str = args.iter().map(|a| lower_expr_to_string(a)).collect::<Vec<_>>().join(", ");
            format!("{}({})", lower_expr_to_string(func), args_str)
        }
        Expr::MethodCall { object, method, args } => {
            let args_str = args.iter().map(|a| lower_expr_to_string(a)).collect::<Vec<_>>().join(", ");
            format!("{}.{}({})", lower_expr_to_string(object), method, args_str)
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
        Expr::ArrayAccess { array, index } => {
            format!("{}[{}]", lower_expr_to_string(array), lower_expr_to_string(index))
        }
        Expr::TemplateLiteral { parts } => {
            let mut result = String::new();
            for part in parts {
                match part {
                    TemplatePart::String(s) => result.push_str(s),
                    TemplatePart::Expression(expr) => {
                        result.push_str(&format!("${{{}}}", lower_expr_to_string(expr)));
                    }
                }
            }
            format!("`{}`", result)
        }
        Expr::ArrowFunction { params, body } => {
            let params_str = params.iter().map(|p| p.name.clone()).collect::<Vec<_>>().join(", ");
            let body_str = body.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join("; ");
            format!("({}) => {{ {} }}", params_str, body_str)
        }
        Expr::New { class, args } => {
            let args_str = args.iter().map(|a| lower_expr_to_string(a)).collect::<Vec<_>>().join(", ");
            format!("new {}({})", lower_expr_to_string(class), args_str)
        }
        Expr::TypeAssert { value, type_ } => {
            format!("{} as {:?}", lower_expr_to_string(value), type_)
        }
        Expr::ArrayLiteral(elements) => {
            let elements_str = elements.iter().map(|e| lower_expr_to_string(e)).collect::<Vec<_>>().join(", ");
            format!("[{}]", elements_str)
        }
        Expr::ObjectLiteral(properties) => {
            let props_str = properties.iter().map(|p| {
                if p.shorthand {
                    p.key.clone()
                } else {
                    format!("{}: {}", p.key, lower_expr_to_string(&p.value))
                }
            }).collect::<Vec<_>>().join(", ");
            format!("{{ {} }}", props_str)
        }
        Expr::Await(inner) => IRExpr::Await(Box::new(lower_expr(inner))),
        Expr::Comprehension { target, iter, filter, expr } => IRExpr::Comprehension {
            target: target.clone(),
            iter: Box::new(lower_expr(iter)),
            filter: filter.as_ref().map(|f| Box::new(lower_expr(f))),
            expr: Box::new(lower_expr(expr)),
        },
        _ => unreachable!(),
    }
}

fn lower_stmt(s: &Stmt) -> IRStmt {
    match s {
        Stmt::Expr(e) => match e {
            Expr::Call { func, args } => IRStmt::Call {
                func: lower_expr_to_string(func),
                args: args.iter().map(|a| lower_expr(a)).collect(),
            },
            _ => IRStmt::Call {
                func: "expr".to_string(),
                args: vec![lower_expr(e)],
            },
        },
        Stmt::Assign { target, value } => IRStmt::Assign {
            target: target.clone(),
            value: lower_expr(value),
        },
        Stmt::If { condition, then, else_ } => IRStmt::Call {
            func: "if".to_string(),
            args: vec![
                lower_expr(condition),
                IRExpr::StringLiteral(then.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(";")),
                IRExpr::StringLiteral(else_.as_ref().map(|stmts| stmts.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(";")).unwrap_or_default()),
            ],
        },
        Stmt::Loop { init, condition, update, body } => IRStmt::Call {
            func: "loop".to_string(),
            args: vec![
                IRExpr::StringLiteral(init.as_ref().map(|s| format!("{:?}", s)).unwrap_or_default()),
                IRExpr::StringLiteral(condition.as_ref().map(|e| lower_expr_to_string(e)).unwrap_or_default()),
                IRExpr::StringLiteral(update.as_ref().map(|s| format!("{:?}", s)).unwrap_or_default()),
                IRExpr::StringLiteral(body.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(";")),
            ],
        },
        Stmt::ForIn { variable, iterable, body } => IRStmt::Call {
            func: "forin".to_string(),
            args: vec![
                IRExpr::StringLiteral(variable.clone()),
                lower_expr(iterable),
                IRExpr::StringLiteral(body.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(";")),
            ],
        },
        Stmt::ForOf { variable, iterable, body } => IRStmt::Call {
            func: "forof".to_string(),
            args: vec![
                IRExpr::StringLiteral(variable.clone()),
                lower_expr(iterable),
                IRExpr::StringLiteral(body.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(";")),
            ],
        },
        Stmt::Return(value) => IRStmt::Return(value.as_ref().map(|v| lower_expr(v))),
        Stmt::Let { name, value, type_annotation: _ } => IRStmt::Call {
            func: "let".to_string(),
            args: vec![
                IRExpr::StringLiteral(name.clone()),
                lower_expr(value),
            ],
        },
        Stmt::Mut { name, value, type_annotation: _ } => IRStmt::Call {
            func: "mut".to_string(),
            args: vec![
                IRExpr::StringLiteral(name.clone()),
                lower_expr(value),
            ],
        },
        Stmt::Block(statements) => IRStmt::Call {
            func: "block".to_string(),
            args: vec![
                IRExpr::StringLiteral(statements.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(";")),
            ],
        },
        Stmt::Try { body, catch, finally } => IRStmt::Call {
            func: "try".to_string(),
            args: vec![
                IRExpr::StringLiteral(body.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(";")),
                IRExpr::StringLiteral(catch.as_ref().map(|c| format!("catch({}) {{ {} }}", c.error_var, c.body.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(";"))).unwrap_or_default()),
                IRExpr::StringLiteral(finally.as_ref().map(|stmts| stmts.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(";")).unwrap_or_default()),
            ],
        },
        Stmt::Throw(expr) => IRStmt::Call {
            func: "throw".to_string(),
            args: vec![lower_expr(expr)],
        },
        Stmt::Break(label) => IRStmt::Call {
            func: "break".to_string(),
            args: vec![
                IRExpr::StringLiteral(label.as_ref().map(|s| s.clone()).unwrap_or_default()),
            ],
        },
        Stmt::Continue(label) => IRStmt::Call {
            func: "continue".to_string(),
            args: vec![
                IRExpr::StringLiteral(label.as_ref().map(|s| s.clone()).unwrap_or_default()),
            ],
        },
        Stmt::Switch { expression, cases, default } => IRStmt::Call {
            func: "switch".to_string(),
            args: vec![
                lower_expr(expression),
                IRExpr::StringLiteral(cases.iter().map(|c| format!("case {}: {{ {} }}", lower_expr_to_string(&c.value), c.body.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(";"))).collect::<Vec<_>>().join(";")),
                IRExpr::StringLiteral(default.as_ref().map(|stmts| stmts.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(";")).unwrap_or_default()),
            ],
        },
        Stmt::Reactive { name, expr } => IRStmt::Reactive {
            name: name.clone(),
            expr: lower_expr(expr),
        },
        Stmt::Comprehension { target, iter, filter, expr } => IRStmt::Comprehension {
            target: target.clone(),
            iter: lower_expr(iter),
            filter: filter.as_ref().map(|f| lower_expr(f)),
            expr: lower_expr(expr),
        },
        _ => unreachable!(),
    }
}

fn lower_expr(e: &Expr) -> IRExpr {
    match e {
        Expr::StringLiteral(s) => IRExpr::StringLiteral(s.clone()),
        Expr::NumberLiteral(n) => IRExpr::NumberLiteral(*n),
        Expr::BooleanLiteral(b) => IRExpr::StringLiteral(b.to_string()),
        Expr::NullLiteral => IRExpr::StringLiteral("null".to_string()),
        Expr::UndefinedLiteral => IRExpr::StringLiteral("undefined".to_string()),
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
                BinaryOp::Power => "**",
                BinaryOp::Equal => "==",
                BinaryOp::NotEqual => "!=",
                BinaryOp::StrictEqual => "===",
                BinaryOp::StrictNotEqual => "!==",
                BinaryOp::LessThan => "<",
                BinaryOp::LessThanEqual => "<=",
                BinaryOp::GreaterThan => ">",
                BinaryOp::GreaterThanEqual => ">=",
                BinaryOp::And => "&&",
                BinaryOp::Or => "||",
                BinaryOp::BitwiseAnd => "&",
                BinaryOp::BitwiseOr => "|",
                BinaryOp::BitwiseXor => "^",
                BinaryOp::LeftShift => "<<",
                BinaryOp::RightShift => ">>",
                BinaryOp::UnsignedRightShift => ">>>",
            },
            lower_expr_to_string(right)
        )),
        Expr::UnaryOp { op, operand } => IRExpr::StringLiteral(format!("{}{}",
            match op {
                UnaryOp::Plus => "+",
                UnaryOp::Minus => "-",
                UnaryOp::Not => "!",
                UnaryOp::BitwiseNot => "~",
                UnaryOp::Increment => "++",
                UnaryOp::Decrement => "--",
                UnaryOp::TypeOf => "typeof ",
                UnaryOp::Void => "void ",
                UnaryOp::Delete => "delete ",
            },
            lower_expr_to_string(operand)
        )),
        Expr::Call { func, args } => IRExpr::StringLiteral(format!("{}({})",
            lower_expr_to_string(func),
            args.iter().map(|a| lower_expr_to_string(a)).collect::<Vec<_>>().join(", ")
        )),
        Expr::MethodCall { object, method, args } => IRExpr::StringLiteral(format!("{}.{}({})",
            lower_expr_to_string(object),
            method,
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
        Expr::ArrayAccess { array, index } => IRExpr::StringLiteral(format!("{}[{}]",
            lower_expr_to_string(array),
            lower_expr_to_string(index)
        )),
        Expr::TemplateLiteral { parts } => IRExpr::StringLiteral(format!("`{}`",
            parts.iter().map(|p| match p {
                TemplatePart::String(s) => s.clone(),
                TemplatePart::Expression(expr) => format!("${{{}}}", lower_expr_to_string(expr)),
            }).collect::<Vec<_>>().join("")
        )),
        Expr::ArrowFunction { params, body } => IRExpr::StringLiteral(format!("({}) => {{ {} }}",
            params.iter().map(|p| p.name.clone()).collect::<Vec<_>>().join(", "),
            body.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join("; ")
        )),
        Expr::New { class, args } => IRExpr::StringLiteral(format!("new {}({})",
            lower_expr_to_string(class),
            args.iter().map(|a| lower_expr_to_string(a)).collect::<Vec<_>>().join(", ")
        )),
        Expr::TypeAssert { value, type_ } => IRExpr::StringLiteral(format!("{} as {:?}",
            lower_expr_to_string(value),
            type_
        )),
        Expr::ArrayLiteral(elements) => IRExpr::StringLiteral(format!("[{}]",
            elements.iter().map(|e| lower_expr_to_string(e)).collect::<Vec<_>>().join(", ")
        )),
        Expr::ObjectLiteral(properties) => IRExpr::StringLiteral(format!("{{ {} }}",
            properties.iter().map(|p| {
                if p.shorthand {
                    p.key.clone()
                } else {
                    format!("{}: {}", p.key, lower_expr_to_string(&p.value))
                }
            }).collect::<Vec<_>>().join(", ")
        )),
        Expr::Await(inner) => IRExpr::Await(Box::new(lower_expr(inner))),
        Expr::Comprehension { target, iter, filter, expr } => IRExpr::Comprehension {
            target: target.clone(),
            iter: Box::new(lower_expr(iter)),
            filter: filter.as_ref().map(|f| Box::new(lower_expr(f))),
            expr: Box::new(lower_expr(expr)),
        },
        _ => unreachable!(),
    }
}
