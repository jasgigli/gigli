//! Parser for GigliOptix source code
use crate::ast::{AST, Function, Stmt, Expr, View, Cell, Flow, FlowTrigger, StyleBlock, RenderBlock, RenderElement, EventHandler, BinaryOp, Token};
use crate::lexer::Lexer;
use std::collections::HashMap;
use std::fs;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let current_token = tokens.first().cloned();
        Parser {
            tokens,
            position: 0,
            current_token,
        }
    }

    pub fn parse(&mut self) -> Result<AST, String> {
        let mut functions = Vec::new();
        let mut views = Vec::new();
        let mut cells = Vec::new();
        let mut flows = Vec::new();

        while self.current_token.is_some() && self.current_token != Some(Token::EOF) {
            match &self.current_token {
                Some(Token::Fn) => {
                    functions.push(self.parse_function()?);
                }
                Some(Token::View) => {
                    views.push(self.parse_view()?);
                }
                Some(Token::Cell) => {
                    cells.push(self.parse_cell()?);
                }
                Some(Token::Flow) => {
                    flows.push(self.parse_flow()?);
                }
                _ => {
                    return Err(format!("Unexpected token: {:?}", self.current_token));
                }
            }
        }

        Ok(AST {
            functions,
            views,
            cells,
            flows,
        })
    }

    fn parse_function(&mut self) -> Result<Function, String> {
        self.expect(Token::Fn)?;
        let name = self.expect_identifier()?;
        self.expect(Token::LeftParen)?;

        let mut params = Vec::new();
        while self.current_token != Some(Token::RightParen) {
            params.push(self.expect_identifier()?);
            if self.current_token == Some(Token::Comma) {
                self.advance();
            }
        }
        self.expect(Token::RightParen)?;
        self.expect(Token::LeftBrace)?;

        let mut body = Vec::new();
        while self.current_token != Some(Token::RightBrace) {
            body.push(self.parse_statement()?);
        }
        self.expect(Token::RightBrace)?;

        Ok(Function { name, params, body })
    }

    fn parse_view(&mut self) -> Result<View, String> {
        self.expect(Token::View)?;
        let name = self.expect_identifier()?;
        self.expect(Token::LeftBrace)?;

        let mut cells = Vec::new();
        let mut flows = Vec::new();
        let mut style = None;
        let mut render = None;
        let mut event_handlers = Vec::new();

        while self.current_token != Some(Token::RightBrace) {
            match &self.current_token {
                Some(Token::Cell) => {
                    cells.push(self.parse_cell()?);
                }
                Some(Token::Flow) => {
                    flows.push(self.parse_flow()?);
                }
                Some(Token::Style) => {
                    style = Some(self.parse_style_block()?);
                }
                Some(Token::Render) => {
                    render = Some(self.parse_render_block()?);
                }
                Some(Token::On) => {
                    event_handlers.push(self.parse_event_handler()?);
                }
                _ => {
                    return Err(format!("Unexpected token in view: {:?}", self.current_token));
                }
            }
        }
        self.expect(Token::RightBrace)?;

        let render = render.ok_or("View must have a render block")?;

        Ok(View {
            name,
            cells,
            flows,
            style,
            render,
            event_handlers,
        })
    }

    fn parse_cell(&mut self) -> Result<Cell, String> {
        self.expect(Token::Cell)?;
        let name = self.expect_identifier()?;
        self.expect(Token::Assign)?;
        let initial_value = self.parse_expression()?;
        self.expect(Token::Semicolon)?;

        Ok(Cell {
            name,
            initial_value,
            is_mutable: true, // All cells are mutable in GigliOptix
        })
    }

    fn parse_flow(&mut self) -> Result<Flow, String> {
        self.expect(Token::Flow)?;
        let name = self.expect_identifier()?;
        self.expect(Token::Assign)?;
        let trigger = self.parse_flow_trigger()?;
        self.expect(Token::LeftBrace)?;

        let mut body = Vec::new();
        while self.current_token != Some(Token::RightBrace) {
            body.push(self.parse_statement()?);
        }
        self.expect(Token::RightBrace)?;

        Ok(Flow { name, trigger, body })
    }

    fn parse_flow_trigger(&mut self) -> Result<FlowTrigger, String> {
        match &self.current_token {
            Some(Token::On) => {
                self.advance();
                let event = self.expect_identifier()?;
                self.expect(Token::Colon)?;
                let target = self.expect_identifier()?;
                Ok(FlowTrigger::OnEvent { event, target })
            }
            Some(Token::Identifier(ident)) if ident == "interval" => {
                self.advance();
                self.expect(Token::LeftParen)?;
                let ms = self.expect_number()? as u64;
                self.expect(Token::RightParen)?;
                Ok(FlowTrigger::Interval { ms })
            }
            _ => Err(format!("Invalid flow trigger: {:?}", self.current_token)),
        }
    }

    fn parse_style_block(&mut self) -> Result<StyleBlock, String> {
        self.expect(Token::Style)?;
        self.expect(Token::Colon)?;

        let mut properties = HashMap::new();
        while self.current_token != Some(Token::Semicolon) {
            let property = self.expect_identifier()?;
            self.expect(Token::Colon)?;
            let value = self.parse_expression()?;
            properties.insert(property, value);

            if self.current_token == Some(Token::Comma) {
                self.advance();
            }
        }
        self.expect(Token::Semicolon)?;

        Ok(StyleBlock { properties })
    }

    fn parse_render_block(&mut self) -> Result<RenderBlock, String> {
        self.expect(Token::Render)?;
        self.expect(Token::Colon)?;

        let mut elements = Vec::new();
        while self.current_token != Some(Token::Semicolon) {
            elements.push(self.parse_render_element()?);
        }
        self.expect(Token::Semicolon)?;

        Ok(RenderBlock { elements })
    }

    fn parse_render_element(&mut self) -> Result<RenderElement, String> {
        match &self.current_token {
            Some(Token::StringLiteral(_)) | Some(Token::Identifier(_)) => {
                let expr = self.parse_expression()?;
                Ok(RenderElement::Text(expr))
            }
            Some(Token::If) => {
                self.advance();
                self.expect(Token::LeftParen)?;
                let condition = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                self.expect(Token::Then)?;

                let mut then_elements = Vec::new();
                while self.current_token != Some(Token::Else) && self.current_token != Some(Token::Semicolon) {
                    then_elements.push(self.parse_render_element()?);
                }

                let mut else_elements = None;
                if self.current_token == Some(Token::Else) {
                    self.advance();
                    let mut elements = Vec::new();
                    while self.current_token != Some(Token::Semicolon) {
                        elements.push(self.parse_render_element()?);
                    }
                    else_elements = Some(elements);
                }

                Ok(RenderElement::Conditional {
                    condition,
                    then: then_elements,
                    else_: else_elements,
                })
            }
            _ => Err(format!("Unexpected token in render element: {:?}", self.current_token)),
        }
    }

    fn parse_event_handler(&mut self) -> Result<EventHandler, String> {
        self.expect(Token::On)?;
        let event = self.expect_identifier()?;
        self.expect(Token::Colon)?;
        let target = self.expect_identifier()?;
        self.expect(Token::LeftBrace)?;

        let mut action = Vec::new();
        while self.current_token != Some(Token::RightBrace) {
            action.push(self.parse_statement()?);
        }
        self.expect(Token::RightBrace)?;

        Ok(EventHandler {
            event,
            target: Some(target),
            action,
        })
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        match &self.current_token {
            Some(Token::Let) => {
                self.advance();
                let name = self.expect_identifier()?;
                self.expect(Token::Assign)?;
                let value = self.parse_expression()?;
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Let { name, value })
            }
            Some(Token::Mut) => {
                self.advance();
                let name = self.expect_identifier()?;
                self.expect(Token::Assign)?;
                let value = self.parse_expression()?;
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Mut { name, value })
            }
            Some(Token::If) => {
                self.advance();
                self.expect(Token::LeftParen)?;
                let condition = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                self.expect(Token::LeftBrace)?;

                let mut then_body = Vec::new();
                while self.current_token != Some(Token::RightBrace) {
                    then_body.push(self.parse_statement()?);
                }
                self.expect(Token::RightBrace)?;

                let mut else_body = None;
                if self.current_token == Some(Token::Else) {
                    self.advance();
                    self.expect(Token::LeftBrace)?;
                    let mut body = Vec::new();
                    while self.current_token != Some(Token::RightBrace) {
                        body.push(self.parse_statement()?);
                    }
                    self.expect(Token::RightBrace)?;
                    else_body = Some(body);
                }

                Ok(Stmt::If { condition, then: then_body, else_: else_body })
            }
            Some(Token::Return) => {
                self.advance();
                let value = if self.current_token != Some(Token::Semicolon) {
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Return(value))
            }
            _ => {
                let expr = self.parse_expression()?;
                if self.current_token == Some(Token::Semicolon) {
                    self.advance();
                    Ok(Stmt::Expr(expr))
                } else {
                    Ok(Stmt::Expr(expr))
                }
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_binary_expression(0)
    }

    fn parse_binary_expression(&mut self, min_precedence: u8) -> Result<Expr, String> {
        let mut left = self.parse_unary_expression()?;

        while let Some(token) = &self.current_token {
            let precedence = self.get_operator_precedence(token);
            if precedence < min_precedence {
                break;
            }

            let op = self.parse_binary_operator(token)?;
            self.advance();
            let right = self.parse_binary_expression(precedence + 1)?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary_expression(&mut self) -> Result<Expr, String> {
        match &self.current_token {
            Some(Token::StringLiteral(s)) => {
                let value = s.clone();
                self.advance();
                Ok(Expr::StringLiteral(value))
            }
            Some(Token::NumberLiteral(n)) => {
                let value = *n;
                self.advance();
                Ok(Expr::NumberLiteral(value))
            }
            Some(Token::Identifier(ident)) => {
                let value = ident.clone();
                self.advance();

                // Check if it's a function call
                if self.current_token == Some(Token::LeftParen) {
                    self.advance();
                    let mut args = Vec::new();
                    while self.current_token != Some(Token::RightParen) {
                        args.push(self.parse_expression()?);
                        if self.current_token == Some(Token::Comma) {
                            self.advance();
                        }
                    }
                    self.expect(Token::RightParen)?;
                    Ok(Expr::Call { func: value, args })
                } else {
                    Ok(Expr::Identifier(value))
                }
            }
            Some(Token::LeftParen) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token in expression: {:?}", self.current_token)),
        }
    }

    fn parse_binary_operator(&self, token: &Token) -> Result<BinaryOp, String> {
        match token {
            Token::Plus => Ok(BinaryOp::Add),
            Token::Minus => Ok(BinaryOp::Subtract),
            Token::Star => Ok(BinaryOp::Multiply),
            Token::Slash => Ok(BinaryOp::Divide),
            Token::Percent => Ok(BinaryOp::Modulo),
            Token::Equal => Ok(BinaryOp::Equal),
            Token::NotEqual => Ok(BinaryOp::NotEqual),
            Token::LessThan => Ok(BinaryOp::LessThan),
            Token::LessThanEqual => Ok(BinaryOp::LessThanEqual),
            Token::GreaterThan => Ok(BinaryOp::GreaterThan),
            Token::GreaterThanEqual => Ok(BinaryOp::GreaterThanEqual),
            _ => Err(format!("Invalid binary operator: {:?}", token)),
        }
    }

    fn get_operator_precedence(&self, token: &Token) -> u8 {
        match token {
            Token::Plus | Token::Minus => 1,
            Token::Star | Token::Slash | Token::Percent => 2,
            Token::Equal | Token::NotEqual | Token::LessThan | Token::LessThanEqual | Token::GreaterThan | Token::GreaterThanEqual => 0,
            _ => 0,
        }
    }

    fn expect(&mut self, token: Token) -> Result<(), String> {
        if self.current_token == Some(token.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", token, self.current_token))
        }
    }

    fn expect_identifier(&mut self) -> Result<String, String> {
        match &self.current_token {
            Some(Token::Identifier(ident)) => {
                let value = ident.clone();
                self.advance();
                Ok(value)
            }
            _ => Err(format!("Expected identifier, got {:?}", self.current_token)),
        }
    }

    fn expect_number(&mut self) -> Result<f64, String> {
        match &self.current_token {
            Some(Token::NumberLiteral(n)) => {
                let value = *n;
                self.advance();
                Ok(value)
            }
            _ => Err(format!("Expected number, got {:?}", self.current_token)),
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_token = self.tokens.get(self.position).cloned();
    }
}

/// Very basic parser for demo purposes: parses a single function with a single call
pub fn parse_file(path: &str) -> AST {
    let source = fs::read_to_string(path).expect("Failed to read source file");
    println!("[Parser] Parsing {}", path);

    // Use the new parser for modern syntax
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("Failed to tokenize source");
    let mut parser = Parser::new(tokens);

    match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            println!("[Parser] Error: {}", e);
            // Fallback to old parser for backward compatibility
            parse_file_legacy(path)
        }
    }
}

/// Legacy parser for backward compatibility
fn parse_file_legacy(path: &str) -> AST {
    let source = fs::read_to_string(path).expect("Failed to read source file");

    // This is a naive parser for: fn main() { dom::set_inner_html("app", "<h1>Hello, Web!</h1>"); }
    // It only works for this pattern and similar simple cases.
    let mut functions = Vec::new();
    if let Some(fn_start) = source.find("fn ") {
        if let Some(name_start) = source[fn_start+3..].find('(') {
            let name = source[fn_start+3..fn_start+3+name_start].trim().to_string();
            // Find the body
            if let Some(body_start) = source.find('{') {
                if let Some(body_end) = source.rfind('}') {
                    let body = &source[body_start+1..body_end];
                    // Only support a single call statement for now
                    let stmt = parse_stmt(body.trim());
                    let function = Function {
                        name,
                        params: Vec::new(),
                        body: vec![stmt],
                    };
                    functions.push(function);
                }
            }
        }
    }
    AST {
        functions,
        views: Vec::new(),
        cells: Vec::new(),
        flows: Vec::new(),
    }
}

fn parse_stmt(line: &str) -> Stmt {
    // Only support a function call with string args, e.g. dom::set_inner_html("app", "<h1>Hello, Web!</h1>");
    if let Some(paren) = line.find('(') {
        let func = line[..paren].replace("::", ".");
        let func = func.trim().to_string();
        let args_str = &line[paren+1..line.rfind(')').unwrap_or(line.len()-1)];
        let args: Vec<Expr> = args_str.split(',').map(|s| parse_expr(s.trim())).collect();
        Stmt::Expr(Expr::Call { func, args })
    } else {
        // Fallback: treat as identifier
        Stmt::Expr(Expr::Identifier(line.trim().to_string()))
    }
}

fn parse_expr(token: &str) -> Expr {
    if token.starts_with('"') && token.ends_with('"') {
        Expr::StringLiteral(token.trim_matches('"').to_string())
    } else {
        Expr::Identifier(token.to_string())
    }
}
