//! Parser for Gigli source code
use crate::ast::*;
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
        let mut parser = Parser {
            tokens,
            position: 0,
            current_token: None,
        };
        parser.advance();
        parser
    }

    pub fn parse(&mut self) -> Result<AST, String> {
        let mut functions = Vec::new();
        let mut components = Vec::new();
        let mut classes = Vec::new();
        let mut modules = Vec::new();
        let mut imports = Vec::new();

        while self.current_token.is_some() {
            match &self.current_token {
                Some(Token::Fn) => {
                    functions.push(self.parse_function()?);
                }
                Some(Token::Component) => {
                    components.push(self.parse_component()?);
                }
                Some(Token::Class) => {
                    classes.push(self.parse_class()?);
                }
                Some(Token::Module) => {
                    modules.push(self.parse_module()?);
                }
                Some(Token::Import) => {
                    imports.push(self.parse_import()?);
                }
                Some(Token::EOF) => break,
                _ => {
                    return Err(format!("Unexpected token: {:?}", self.current_token));
                }
            }
        }

        Ok(AST {
            functions,
            components,
            classes,
            modules,
            imports,
        })
    }

    fn parse_function(&mut self) -> Result<Function, String> {
        let mut is_async = false;
        if self.current_token == Some(Token::Identifier("async".to_string())) {
            is_async = true;
            self.advance();
        }
        self.expect(Token::Fn)?;
        let name = self.expect_identifier()?;
        self.expect(Token::LeftParen)?;

        let mut params = Vec::new();
        while self.current_token != Some(Token::RightParen) {
            params.push(self.parse_parameter()?);
            if self.current_token == Some(Token::Comma) {
                self.advance();
            }
        }
        self.expect(Token::RightParen)?;

        let mut return_type = None;
        if self.current_token == Some(Token::Colon) {
            self.advance();
            return_type = Some(self.parse_type()?);
        }

        self.expect(Token::LeftBrace)?;

        let mut body = Vec::new();
        while self.current_token != Some(Token::RightBrace) {
            body.push(self.parse_statement()?);
        }
        self.expect(Token::RightBrace)?;

        Ok(Function {
            name,
            params,
            return_type,
            body,
            is_public: true, // Default to public for now
            is_async,
        })
    }

    fn parse_parameter(&mut self) -> Result<Parameter, String> {
        let mut is_ref = false;
        let mut is_mut_ref = false;
        if self.current_token == Some(Token::And) {
            self.advance();
            if self.current_token == Some(Token::Mut) {
                is_mut_ref = true;
                self.advance();
            } else {
                is_ref = true;
            }
        }
        let name = self.expect_identifier()?;

        let mut type_annotation = None;
        if self.current_token == Some(Token::Colon) {
            self.advance();
            type_annotation = Some(self.parse_type()?);
        }

        let mut default_value = None;
        if self.current_token == Some(Token::Assign) {
            self.advance();
            default_value = Some(self.parse_expression()?);
        }

        Ok(Parameter {
            name,
            type_annotation,
            default_value,
            is_ref,
            is_mut_ref,
        })
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        if self.current_token == Some(Token::And) {
            self.advance();
            if self.current_token == Some(Token::Mut) {
                self.advance();
                return Ok(Type::MutRef(Box::new(self.parse_type()?)));
            } else {
                return Ok(Type::Ref(Box::new(self.parse_type()?)));
            }
        }
        match &self.current_token {
            Some(Token::Identifier(name)) => {
                let name_clone = name.clone();
                self.advance();
                match name_clone.as_str() {
                    "string" => Ok(Type::String),
                    "number" => Ok(Type::Number),
                    "boolean" => Ok(Type::Boolean),
                    "void" => Ok(Type::Void),
                    "any" => Ok(Type::Any),
                    "Option" => {
                        self.expect(Token::LessThan)?;
                        let inner = self.parse_type()?;
                        self.expect(Token::GreaterThan)?;
                        Ok(Type::Option(Box::new(inner)))
                    },
                    "Result" => {
                        self.expect(Token::LessThan)?;
                        let ok = self.parse_type()?;
                        self.expect(Token::Comma)?;
                        let err = self.parse_type()?;
                        self.expect(Token::GreaterThan)?;
                        Ok(Type::Result(Box::new(ok), Box::new(err)))
                    },
                    _ => Ok(Type::Custom(name_clone)),
                }
            }
            _ => Err(format!("Expected type, got: {:?}", self.current_token)),
        }
    }

    fn parse_class(&mut self) -> Result<Class, String> {
        self.expect(Token::Class)?;
        let name = self.expect_identifier()?;
        self.expect(Token::LeftBrace)?;

        let mut fields = Vec::new();
        let mut methods = Vec::new();
        let mut constructor = None;

        while self.current_token != Some(Token::RightBrace) {
            match &self.current_token {
                Some(Token::Constructor) => {
                    constructor = Some(self.parse_constructor()?);
                }
                Some(Token::Fn) => {
                    methods.push(self.parse_method()?);
                }
                _ => {
                    fields.push(self.parse_field()?);
                }
            }
        }
        self.expect(Token::RightBrace)?;

        Ok(Class {
            name,
            fields,
            methods,
            constructor,
        })
    }

    fn parse_field(&mut self) -> Result<Field, String> {
        let mut is_public = true;
        if self.current_token == Some(Token::Private) {
            self.advance();
            is_public = false;
        }

        let name = self.expect_identifier()?;

        let mut type_annotation = None;
        if self.current_token == Some(Token::Colon) {
            self.advance();
            type_annotation = Some(self.parse_type()?);
        }

        let mut initial_value = None;
        if self.current_token == Some(Token::Assign) {
            self.advance();
            initial_value = Some(self.parse_expression()?);
        }

        self.expect(Token::Semicolon)?;

        Ok(Field {
            name,
            type_annotation,
            initial_value,
            is_public,
        })
    }

    fn parse_method(&mut self) -> Result<Method, String> {
        let mut is_public = true;
        if self.current_token == Some(Token::Private) {
            self.advance();
            is_public = false;
        }

        self.expect(Token::Fn)?;
        let name = self.expect_identifier()?;
        self.expect(Token::LeftParen)?;

        let mut params = Vec::new();
        while self.current_token != Some(Token::RightParen) {
            params.push(self.parse_parameter()?);
            if self.current_token == Some(Token::Comma) {
                self.advance();
            }
        }
        self.expect(Token::RightParen)?;

        let mut return_type = None;
        if self.current_token == Some(Token::Colon) {
            self.advance();
            return_type = Some(self.parse_type()?);
        }

        self.expect(Token::LeftBrace)?;

        let mut body = Vec::new();
        while self.current_token != Some(Token::RightBrace) {
            body.push(self.parse_statement()?);
        }
        self.expect(Token::RightBrace)?;

        Ok(Method {
            name,
            params,
            return_type,
            body,
            is_public,
        })
    }

    fn parse_constructor(&mut self) -> Result<Constructor, String> {
        self.expect(Token::Constructor)?;
        self.expect(Token::LeftParen)?;

        let mut params = Vec::new();
        while self.current_token != Some(Token::RightParen) {
            params.push(self.parse_parameter()?);
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

        Ok(Constructor { params, body })
    }

    fn parse_component(&mut self) -> Result<ComponentNode, String> {
        self.expect(Token::Component)?;
        let name = self.expect_identifier()?;
        self.expect(Token::LeftBrace)?;

        let mut state_vars = Vec::new();
        let mut let_vars = Vec::new();
        let mut functions = Vec::new();
        let mut markup = Vec::new();
        let mut style = None;

        while self.current_token != Some(Token::RightBrace) {
            match &self.current_token {
                Some(Token::State) => {
                    state_vars.push(self.parse_state_var()?);
                }
                Some(Token::Let) => {
                    let_vars.push(self.parse_let_var()?);
                }
                Some(Token::Fn) => {
                    functions.push(self.parse_function()?);
                }
                Some(Token::Style) => {
                    style = Some(self.parse_style_block_raw()?);
                }
                // Markup parsing: parse until the end of the component block
                _ => {
                    markup.append(&mut self.parse_markup()?);
                }
            }
        }
        self.expect(Token::RightBrace)?;

        Ok(ComponentNode {
            name,
            state_vars,
            let_vars,
            functions,
            markup,
            style,
        })
    }

    /// Parse a sequence of markup nodes (HTML-like, text, or control flow blocks)
    fn parse_markup(&mut self) -> Result<Vec<MarkupNode>, String> {
        let mut nodes = Vec::new();
        while let Some(token) = &self.current_token {
            match token {
                Token::Identifier(_) | Token::StringLiteral(_) => {
                    nodes.push(self.parse_markup_text_or_element()?);
                }
                Token::HashIf => {
                    nodes.push(MarkupNode::IfBlock(self.parse_if_block()?));
                }
                Token::HashFor => {
                    nodes.push(MarkupNode::ForLoop(self.parse_for_block()?));
                }
                Token::RightBrace | Token::ForwardSlashIf | Token::ForwardSlashFor | Token::HashElse => {
                    // End of this markup context
                    break;
                }
                _ => {
                    // Skip unrecognized tokens in markup for now
                    self.advance();
                }
            }
        }
        Ok(nodes)
    }

    /// Parse a text node or an HTML-like element
    fn parse_markup_text_or_element(&mut self) -> Result<MarkupNode, String> {
        match &self.current_token {
            Some(Token::Identifier(tag)) => {
                // Parse as an element: <tag ...>...</tag>
                let tag_name = tag.clone();
                self.advance();
                let mut attributes = std::collections::HashMap::new();
                // Parse attributes (identifier = expr pairs)
                while let Some(Token::Identifier(attr)) = &self.current_token {
                    let attr_name = attr.clone();
                    self.advance();
                    if self.current_token == Some(Token::Assign) {
                        self.advance();
                        let value = self.parse_expression()?;
                        attributes.insert(attr_name, value);
                    } else {
                        // Boolean attribute
                        attributes.insert(attr_name, Expr::BooleanLiteral(true));
                    }
                }
                // Children (nested markup)
                let children = if let Some(Token::LeftBrace) = &self.current_token {
                    self.advance();
                    let children = self.parse_markup()?;
                    self.expect(Token::RightBrace)?;
                    children
                } else {
                    Vec::new()
                };
                Ok(MarkupNode::Element {
                    tag: tag_name,
                    attributes,
                    children,
                })
            }
            Some(Token::StringLiteral(s)) => {
                let expr = Expr::StringLiteral(s.clone());
                self.advance();
                Ok(MarkupNode::Text(expr))
            }
            _ => Err(format!("Expected markup element or text, got: {:?}", self.current_token)),
        }
    }

    /// Parse an {#if ...} ... {:else} ... {/if} block
    fn parse_if_block(&mut self) -> Result<IfBlockNode, String> {
        self.expect(Token::HashIf)?;
        let condition = self.parse_expression()?;
        let then_branch = self.parse_markup()?;
        let else_branch = if self.current_token == Some(Token::HashElse) {
            self.advance();
            Some(self.parse_markup()?)
        } else {
            None
        };
        self.expect(Token::ForwardSlashIf)?;
        Ok(IfBlockNode {
            condition,
            then_branch,
            else_branch,
        })
    }

    /// Parse a {#for item in items} ... {/for} block
    fn parse_for_block(&mut self) -> Result<ForLoopBlockNode, String> {
        self.expect(Token::HashFor)?;
        let iterator = self.expect_identifier()?;
        self.expect(Token::In)?;
        let iterable = self.parse_expression()?;
        let body = self.parse_markup()?;
        self.expect(Token::ForwardSlashFor)?;
        Ok(ForLoopBlockNode {
            iterator,
            iterable,
            body,
        })
    }

    fn parse_state_var(&mut self) -> Result<StateVar, String> {
        self.expect(Token::State)?;
        let name = self.expect_identifier()?;
        let mut type_annotation = None;
        if self.current_token == Some(Token::Colon) {
            self.advance();
            type_annotation = Some(self.parse_type()?);
        }
        self.expect(Token::Assign)?;
        let initial_value = self.parse_expression()?;
        self.expect(Token::Semicolon)?;
        Ok(StateVar { name, type_annotation, initial_value })
    }

    fn parse_let_var(&mut self) -> Result<LetVar, String> {
        self.expect(Token::Let)?;
        let name = self.expect_identifier()?;
        let mut type_annotation = None;
        if self.current_token == Some(Token::Colon) {
            self.advance();
            type_annotation = Some(self.parse_type()?);
        }
        self.expect(Token::Assign)?;
        let value = self.parse_expression()?;
        self.expect(Token::Semicolon)?;
        Ok(LetVar { name, type_annotation, value })
    }

    fn parse_style_block_raw(&mut self) -> Result<String, String> {
        self.expect(Token::Style)?;
        // For now, just collect everything until the next right brace as a raw string
        let mut css = String::new();
        if self.current_token == Some(Token::LeftBrace) {
            self.advance();
            while self.current_token != Some(Token::RightBrace) && self.current_token != Some(Token::EOF) {
                // This is a stub: in a real parser, we'd handle nested braces and parse CSS properly
                if let Some(Token::Identifier(s)) = &self.current_token {
                    css.push_str(s);
                    css.push(' ');
                }
                self.advance();
            }
            self.expect(Token::RightBrace)?;
        }
        Ok(css)
    }

    fn parse_module(&mut self) -> Result<Module, String> {
        self.expect(Token::Module)?;
        let name = self.expect_identifier()?;
        self.expect(Token::LeftBrace)?;

        let mut items = Vec::new();
        while self.current_token != Some(Token::RightBrace) {
            match &self.current_token {
                Some(Token::Fn) => {
                    items.push(ModuleItem::Function(self.parse_function()?));
                }
                Some(Token::Class) => {
                    items.push(ModuleItem::Class(self.parse_class()?));
                }
                Some(Token::Component) => {
                    items.push(ModuleItem::Component(self.parse_component()?));
                }
                _ => {
                    return Err(format!("Unexpected token in module: {:?}", self.current_token));
                }
            }
        }
        self.expect(Token::RightBrace)?;

        Ok(Module { name, items })
    }

    fn parse_import(&mut self) -> Result<Import, String> {
        self.expect(Token::Import)?;
        self.expect(Token::LeftBrace)?;

        let mut items = Vec::new();
        while self.current_token != Some(Token::RightBrace) {
            items.push(self.expect_identifier()?);
            if self.current_token == Some(Token::Comma) {
                self.advance();
            }
        }
        self.expect(Token::RightBrace)?;
        self.expect(Token::From)?;

        let module = self.expect_identifier()?;

        let mut alias = None;
        if self.current_token == Some(Token::As) {
            self.advance();
            alias = Some(self.expect_identifier()?);
        }

        self.expect(Token::Semicolon)?;

        Ok(Import { module, items, alias })
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        if self.current_token == Some(Token::Identifier("$".to_string())) {
            self.advance();
            self.expect(Token::Colon)?;
            let name = self.expect_identifier()?;
            self.expect(Token::Assign)?;
            let expr = self.parse_expression()?;
            self.expect(Token::Semicolon)?;
            return Ok(Stmt::Reactive { name, expr });
        }
        match &self.current_token {
            Some(Token::Let) => {
                self.advance();
                let name = self.expect_identifier()?;

                let mut type_annotation = None;
                if self.current_token == Some(Token::Colon) {
                    self.advance();
                    type_annotation = Some(self.parse_type()?);
                }

                self.expect(Token::Assign)?;
                let value = self.parse_expression()?;
                self.expect(Token::Semicolon)?;

                Ok(Stmt::Let { name, value, type_annotation })
            }
            Some(Token::Mut) => {
                self.advance();
                let name = self.expect_identifier()?;

                let mut type_annotation = None;
                if self.current_token == Some(Token::Colon) {
                    self.advance();
                    type_annotation = Some(self.parse_type()?);
                }

                self.expect(Token::Assign)?;
                let value = self.parse_expression()?;
                self.expect(Token::Semicolon)?;

                Ok(Stmt::Mut { name, value, type_annotation })
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

                Ok(Stmt::If {
                    condition,
                    then: then_body,
                    else_: else_body,
                })
            }
            Some(Token::Loop) => {
                self.advance();
                self.expect(Token::LeftBrace)?;

                let mut body = Vec::new();
                while self.current_token != Some(Token::RightBrace) {
                    body.push(self.parse_statement()?);
                }
                self.expect(Token::RightBrace)?;

                Ok(Stmt::Loop {
                    init: None,
                    condition: None,
                    update: None,
                    body,
                })
            }
            _ => {
                let expr = self.parse_expression()?;
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        if self.current_token == Some(Token::Identifier("await".to_string())) {
            self.advance();
            let expr = self.parse_expression()?;
            return Ok(Expr::Await(Box::new(expr)));
        }
        // List comprehension: [expr for var in iter if cond]
        if self.current_token == Some(Token::LeftBracket) {
            self.advance();
            let expr = self.parse_expression()?;
            if self.current_token == Some(Token::For) {
                self.advance();
                let target = self.expect_identifier()?;
                self.expect(Token::In)?;
                let iter = self.parse_expression()?;
                let mut filter = None;
                if self.current_token == Some(Token::If) {
                    self.advance();
                    filter = Some(self.parse_expression()?);
                }
                self.expect(Token::RightBracket)?;
                return Ok(Expr::Comprehension {
                    target,
                    iter: Box::new(iter),
                    filter: filter.map(Box::new),
                    expr: Box::new(expr),
                });
            } else {
                // Not a comprehension, fallback to array literal
                // ... fallback logic ...
            }
        }
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
            Some(Token::Minus) => {
                self.advance();
                let operand = self.parse_unary_expression()?;
                Ok(Expr::UnaryOp {
                    op: UnaryOp::Minus,
                    operand: Box::new(operand),
                })
            }
            Some(Token::Not) => {
                self.advance();
                let operand = self.parse_unary_expression()?;
                Ok(Expr::UnaryOp {
                    op: UnaryOp::Not,
                    operand: Box::new(operand),
                })
            }
            Some(Token::Identifier(_)) => {
                let func = Box::new(self.parse_primary_expression()?);
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
                    Ok(Expr::Call { func, args })
                } else {
                    Ok(*func)
                }
            }
            _ => self.parse_primary_expression(),
        }
    }

    fn parse_primary_expression(&mut self) -> Result<Expr, String> {
        match &self.current_token {
            Some(Token::NumberLiteral(n)) => {
                let value = *n;
                self.advance();
                Ok(Expr::NumberLiteral(value))
            }
            Some(Token::StringLiteral(s)) => {
                let value = s.clone();
                self.advance();
                Ok(Expr::StringLiteral(value))
            }
            Some(Token::BooleanLiteral(b)) => {
                let value = *b;
                self.advance();
                Ok(Expr::BooleanLiteral(value))
            }
            Some(Token::Identifier(name)) => {
                let value = name.clone();
                self.advance();
                Ok(Expr::Identifier(value))
            }
            Some(Token::LeftParen) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token: {:?}", self.current_token)),
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
            Token::And => Ok(BinaryOp::And),
            Token::Or => Ok(BinaryOp::Or),
            _ => Err(format!("Invalid binary operator: {:?}", token)),
        }
    }

    fn get_operator_precedence(&self, token: &Token) -> u8 {
        match token {
            Token::Or => 1,
            Token::And => 2,
            Token::Equal | Token::NotEqual => 3,
            Token::LessThan | Token::LessThanEqual | Token::GreaterThan | Token::GreaterThanEqual => 4,
            Token::Plus | Token::Minus => 5,
            Token::Star | Token::Slash | Token::Percent => 6,
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
            Some(Token::Identifier(name)) => {
                let value = name.clone();
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
        self.current_token = if self.position < self.tokens.len() {
            Some(self.tokens[self.position].clone())
        } else {
            None
        };
    }

    fn peek(&self) -> Option<&Token> {
        if self.position + 1 < self.tokens.len() {
            Some(&self.tokens[self.position + 1])
        } else {
            None
        }
    }
}

pub fn parse_file(path: &str) -> AST {
    // For now, return a simple AST for testing
    AST {
        functions: vec![],
        views: vec![],
        cells: vec![],
        flows: vec![],
        classes: vec![],
        modules: vec![],
        imports: vec![],
    }
}

// Legacy parser for backward compatibility
fn parse_file_legacy(path: &str) -> AST {
    // Simple legacy parser that creates a basic AST
    let mut functions = Vec::new();
    let mut views = Vec::new();
    let mut cells = Vec::new();
    let mut flows = Vec::new();

    // Parse the file content
    if let Ok(content) = std::fs::read_to_string(path) {
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("fn ") {
                functions.push(parse_function_legacy(line));
            } else if line.starts_with("view ") {
                views.push(parse_view_legacy(line));
            }
        }
    }

    AST {
        functions,
        views,
        cells,
        flows,
        classes: vec![],
        modules: vec![],
        imports: vec![],
    }
}

fn parse_function_legacy(line: &str) -> Function {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let name = parts.get(1).unwrap_or(&"main").to_string();

    Function {
        name,
        params: vec![],
        return_type: None,
        body: vec![Stmt::Expr(Expr::StringLiteral("Hello, World!".to_string()))],
        is_public: true,
        is_async: false,
    }
}

fn parse_view_legacy(line: &str) -> View {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let name = parts.get(1).unwrap_or(&"App").to_string();

    View {
        name,
        props: vec![],
        cells: vec![],
        flows: vec![],
        style: None,
        render: RenderBlock { elements: vec![] },
        event_handlers: vec![],
    }
}

fn parse_stmt(line: &str) -> Stmt {
    Stmt::Expr(parse_expr(line))
}

fn parse_expr(token: &str) -> Expr {
    Expr::StringLiteral(token.to_string())
}
