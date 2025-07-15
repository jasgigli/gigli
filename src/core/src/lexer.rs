//! Lexer for Gigli source code
use crate::ast::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.first().copied();
        Lexer {
            input: chars,
            position: 0,
            current_char,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while self.current_char.is_some() {
            // Skip whitespace
            while let Some(ch) = self.current_char {
                if ch.is_whitespace() {
                    self.advance();
                } else {
                    break;
                }
            }

            if let Some(ch) = self.current_char {
                // --- NEW: Recognize control flow block tokens ---
                if ch == '{' && self.peek() == Some('#') {
                    self.advance(); // skip '{'
                    self.advance(); // skip '#'
                    // Read block type (if, for, etc.)
                    let mut block_type = String::new();
                    while let Some(c) = self.current_char {
                        if c.is_alphabetic() {
                            block_type.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    match block_type.as_str() {
                        "if" => tokens.push(Token::HashIf),
                        "for" => tokens.push(Token::HashFor),
                        _ => return Err(format!("Unknown block type: {{#{}}}", block_type)),
                    }
                    continue;
                }
                if ch == '{' && self.peek() == Some(':') {
                    self.advance(); // skip '{'
                    self.advance(); // skip ':'
                    // Read 'else'
                    let mut else_kw = String::new();
                    while let Some(c) = self.current_char {
                        if c.is_alphabetic() {
                            else_kw.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    if else_kw == "else" {
                        tokens.push(Token::HashElse);
                        continue;
                    } else {
                        return Err(format!("Unknown block: {{:{} }}", else_kw));
                    }
                }
                if ch == '{' && self.peek() == Some('/') {
                    self.advance(); // skip '{'
                    self.advance(); // skip '/'
                    // Read block type (if, for, etc.)
                    let mut block_type = String::new();
                    while let Some(c) = self.current_char {
                        if c.is_alphabetic() {
                            block_type.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    match block_type.as_str() {
                        "if" => tokens.push(Token::ForwardSlashIf),
                        "for" => tokens.push(Token::ForwardSlashFor),
                        _ => return Err(format!("Unknown closing block: {{/{} }}", block_type)),
                    }
                    continue;
                }
                // --- END NEW ---
                match ch {
                    // Identifiers and keywords
                    'a'..='z' | 'A'..='Z' | '_' => {
                        tokens.push(self.read_identifier_or_keyword()?);
                    }
                    // Numbers
                    '0'..='9' => {
                        tokens.push(self.read_number()?);
                    }
                    // Strings
                    '"' => {
                        tokens.push(self.read_string()?);
                    }
                    // Operators and delimiters
                    '+' => {
                        if self.peek() == Some('=') {
                            self.advance();
                            tokens.push(Token::PlusAssign);
                        } else {
                            tokens.push(Token::Plus);
                        }
                        self.advance();
                    }
                    '-' => {
                        if self.peek() == Some('=') {
                            self.advance();
                            tokens.push(Token::MinusAssign);
                        } else if self.peek() == Some('>') {
                            self.advance();
                            tokens.push(Token::Arrow);
                        } else {
                            tokens.push(Token::Minus);
                        }
                        self.advance();
                    }
                    '*' => {
                        tokens.push(Token::Star);
                        self.advance();
                    }
                    '/' => {
                        // Check for comments
                        if self.peek() == Some('/') {
                            self.skip_line_comment();
                        } else {
                            tokens.push(Token::Slash);
                            self.advance();
                        }
                    }
                    '%' => {
                        tokens.push(Token::Percent);
                        self.advance();
                    }
                    '=' => {
                        if self.peek() == Some('=') {
                            self.advance();
                            tokens.push(Token::Equal);
                        } else {
                            tokens.push(Token::Assign);
                        }
                        self.advance();
                    }
                    '!' => {
                        if self.peek() == Some('=') {
                            self.advance();
                            tokens.push(Token::NotEqual);
                        } else {
                            return Err("Unexpected character '!'".to_string());
                        }
                        self.advance();
                    }
                    '<' => {
                        if self.peek() == Some('=') {
                            self.advance();
                            tokens.push(Token::LessThanEqual);
                        } else {
                            tokens.push(Token::LessThan);
                        }
                        self.advance();
                    }
                    '>' => {
                        if self.peek() == Some('=') {
                            self.advance();
                            tokens.push(Token::GreaterThanEqual);
                        } else {
                            tokens.push(Token::GreaterThan);
                        }
                        self.advance();
                    }
                    '(' => {
                        tokens.push(Token::LeftParen);
                        self.advance();
                    }
                    ')' => {
                        tokens.push(Token::RightParen);
                        self.advance();
                    }
                    '{' => {
                        tokens.push(Token::LeftBrace);
                        self.advance();
                    }
                    '}' => {
                        tokens.push(Token::RightBrace);
                        self.advance();
                    }
                    '[' => {
                        tokens.push(Token::LeftBracket);
                        self.advance();
                    }
                    ']' => {
                        tokens.push(Token::RightBracket);
                        self.advance();
                    }
                    ';' => {
                        tokens.push(Token::Semicolon);
                        self.advance();
                    }
                    ',' => {
                        tokens.push(Token::Comma);
                        self.advance();
                    }
                    '.' => {
                        tokens.push(Token::Dot);
                        self.advance();
                    }
                    ':' => {
                        tokens.push(Token::Colon);
                        self.advance();
                    }
                    _ => {
                        return Err(format!("Unexpected character '{}'", ch));
                    }
                }
            }
        }

        tokens.push(Token::EOF);
        Ok(tokens)
    }

    fn read_identifier_or_keyword(&mut self) -> Result<Token, String> {
        let mut identifier = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Check if it's a keyword
        match identifier.as_str() {
            "fn" => Ok(Token::Fn),
            "component" => Ok(Token::Component), // NEW
            "state" => Ok(Token::State),         // NEW
            "struct" => Ok(Token::Struct),       // NEW
            "enum" => Ok(Token::Enum),           // NEW
            "on" => Ok(Token::On),
            "style" => Ok(Token::Style),
            "if" => Ok(Token::If),
            "then" => Ok(Token::Then),
            "else" => Ok(Token::Else),
            "let" => Ok(Token::Let),
            "mut" => Ok(Token::Mut),
            "return" => Ok(Token::Return),
            _ => Ok(Token::Identifier(identifier)),
        }
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let mut number = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_digit(10) || ch == '.' {
                number.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        match number.parse::<f64>() {
            Ok(n) => Ok(Token::NumberLiteral(n)),
            Err(_) => Err(format!("Invalid number: {}", number)),
        }
    }

    fn read_string(&mut self) -> Result<Token, String> {
        let mut string = String::new();
        self.advance(); // Skip opening quote

        while let Some(ch) = self.current_char {
            match ch {
                '"' => {
                    self.advance(); // Skip closing quote
                    return Ok(Token::StringLiteral(string));
                }
                '\\' => {
                    self.advance();
                    if let Some(escaped) = self.current_char {
                        match escaped {
                            'n' => string.push('\n'),
                            't' => string.push('\t'),
                            'r' => string.push('\r'),
                            '\\' => string.push('\\'),
                            '"' => string.push('"'),
                            _ => return Err(format!("Invalid escape sequence \\{}", escaped)),
                        }
                        self.advance();
                    }
                }
                _ => {
                    string.push(ch);
                    self.advance();
                }
            }
        }

        Err("Unterminated string literal".to_string())
    }

    fn skip_line_comment(&mut self) {
        while let Some(ch) = self.current_char {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }
}

pub fn lexer_stub() {
    // This function is kept for backward compatibility
    println!("Lexer stub - use Lexer::new() instead");
}
