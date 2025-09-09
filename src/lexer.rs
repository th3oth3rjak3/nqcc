#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize) -> Self {
        Self { kind, start, end }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Dyanmically defined values
    Identifier(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    // StringLiteral(String),
    // BooleanLiteral(bool),

    // Keywords
    Int,
    Void,
    Return,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    // LeftBracket,
    // RightBracket,
    Semicolon,
    // Operators
    // Plus,
    // Minus,
    // Star,
    // Slash,
    // Equal,
    // EqualEqual,
    // BangEqual,
    // Bang,
    // Greater,
    // Less,
    // GreaterEqual,
    // LessEqual,

    // Special
    EOF,
    Error(String),
}

pub struct Lexer<'src> {
    source: &'src [char],
    line: usize,
    column: usize,
    offset: usize,
    line_offset: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src [char]) -> Self {
        Self {
            source,
            line: 1,
            column: 1,
            offset: 0,
            line_offset: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut out: Vec<Token> = Vec::new();

        while !self.is_at_end() {
            let tok = self.next_token();
            out.push(tok);
        }

        out
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let start = self.offset;

        let ch = self.peek();
        if ch == '\0' {
            return Token::new(TokenKind::EOF, start, start);
        }

        if is_letter_or_underscore(ch) {
            return self.handle_ident(start);
        }

        if is_digit(ch) {
            return self.handle_number(start);
        }

        match ch {
            '(' => {
                self.advance();
                return Token::new(TokenKind::LeftParen, start, self.offset);
            }
            ')' => {
                self.advance();
                return Token::new(TokenKind::RightParen, start, self.offset);
            }
            '{' => {
                self.advance();
                return Token::new(TokenKind::LeftBrace, start, self.offset);
            }
            '}' => {
                self.advance();
                return Token::new(TokenKind::RightBrace, start, self.offset);
            }
            ';' => {
                self.advance();
                return Token::new(TokenKind::Semicolon, start, self.offset);
            }
            _ => {
                self.advance();
                return Token::new(
                    TokenKind::Error(format!("Unexpected character: '{ch}'")),
                    start,
                    self.offset,
                );
            }
        }
    }

    fn advance(&mut self) {
        if self.peek() == '\n' {
            self.offset += 1;
            self.line_offset = self.offset;
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
            self.offset += 1;
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.offset >= self.source.len()
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\t' | '\r' | '\n' => self.advance(),
                _ => return,
            }
        }
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source[self.offset]
    }

    fn peek_next(&mut self) -> char {
        if self.offset + 1 >= self.source.len() {
            return '\0';
        }

        self.source[self.offset + 1]
    }

    fn handle_ident(&mut self, start: usize) -> Token {
        while !self.is_at_end() && valid_ident_char(self.peek()) {
            self.advance();
        }

        let name = self.source[start..self.offset].iter().collect::<String>();
        match name.as_str() {
            "int" => Token::new(TokenKind::Int, start, self.offset),
            "void" => Token::new(TokenKind::Void, start, self.offset),
            "return" => Token::new(TokenKind::Return, start, self.offset),
            _ => Token::new(TokenKind::Identifier(name), start, self.offset),
        }
    }

    fn handle_number(&mut self, start: usize) -> Token {
        while !self.is_at_end() && is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() != '.' && !is_digit(self.peek_next()) {
            let literal: String = self.source[start..self.offset].iter().collect();
            match literal.parse() {
                Ok(val) => {
                    return Token::new(TokenKind::IntegerLiteral(val), start, self.offset);
                }
                Err(e) => {
                    return Token::new(TokenKind::Error(e.to_string()), start, self.offset);
                }
            };
        }

        self.advance(); // skip '.'

        while !self.is_at_end() && is_digit(self.peek_next()) {
            self.advance();
        }

        let literal: String = self.source[start..self.offset].iter().collect();
        match literal.parse() {
            Ok(val) => {
                return Token::new(TokenKind::FloatLiteral(val), start, self.offset);
            }
            Err(e) => {
                return Token::new(TokenKind::Error(e.to_string()), start, self.offset);
            }
        };
    }
}

fn valid_ident_char(ch: char) -> bool {
    is_letter_or_underscore(ch) || is_digit(ch)
}

fn is_letter_or_underscore(ch: char) -> bool {
    match ch {
        '_' => true,
        _ => is_letter(ch),
    }
}

fn is_letter(ch: char) -> bool {
    match ch {
        'a'..'z' => true,
        'A'..'Z' => true,
        _ => false,
    }
}

fn is_digit(ch: char) -> bool {
    match ch {
        '0'..'9' => true,
        _ => false,
    }
}
