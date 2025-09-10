use crate::{
    errors::CompilerError,
    tokens::{Token, TokenKind},
};

pub struct Lexer<'src> {
    source: &'src [char],
    line: usize,
    column: usize,
    offset: usize,
    line_offset: usize,
    errors: Vec<CompilerError>,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src [char]) -> Self {
        Self {
            source,
            line: 1,
            column: 1,
            offset: 0,
            line_offset: 0,
            errors: Vec::new(),
        }
    }

    pub fn errors(&self) -> Vec<CompilerError> {
        self.errors.iter().cloned().collect()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut out: Vec<Token> = Vec::new();

        while !self.is_at_end() {
            let tok = self.next_token();
            match tok {
                Ok(tok) => {
                    out.push(tok.clone());
                    if tok.kind == TokenKind::EOF {
                        break;
                    }
                }
                Err(e) => self.errors.push(e),
            }
        }

        out
    }

    fn next_token(&mut self) -> Result<Token, CompilerError> {
        self.skip_whitespace();

        let start = self.offset;

        let ch = self.peek();
        if ch == '\0' {
            return self.make_token(TokenKind::EOF, start, false);
        }

        if is_letter_or_underscore(ch) {
            return Ok(self.handle_ident(start));
        }

        if is_digit(ch) {
            return self.handle_number(start);
        }

        match ch {
            '(' => self.make_token(TokenKind::LeftParen, start, true),
            ')' => self.make_token(TokenKind::RightParen, start, true),
            '{' => self.make_token(TokenKind::LeftBrace, start, true),
            '}' => self.make_token(TokenKind::RightBrace, start, true),
            ';' => self.make_token(TokenKind::Semicolon, start, true),
            _ => self.make_error(format!("unexpected character '{ch}'"), true),
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

    fn make_token(
        &mut self,
        kind: TokenKind,
        start: usize,
        advance: bool,
    ) -> Result<Token, CompilerError> {
        if advance {
            self.advance();
        }

        Ok(Token::new(kind, start, self.offset))
    }

    fn make_error(&mut self, message: String, advance: bool) -> Result<Token, CompilerError> {
        let err = CompilerError::LexError {
            message: message,
            line: self.line,
            column: self.column,
        };

        if advance {
            self.advance();
        }

        Err(err)
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

    fn handle_number(&mut self, start: usize) -> Result<Token, CompilerError> {
        while !self.is_at_end() && is_digit(self.peek()) {
            self.advance();
        }

        if is_letter_or_underscore(self.peek()) {
            return self.make_error("invalid identifier".into(), false);
        }

        if self.peek() != '.' && !is_digit(self.peek_next()) {
            let literal: String = self.source[start..self.offset].iter().collect();
            return match literal.parse() {
                Ok(val) => self.make_token(TokenKind::ConstInt(val), start, false),
                Err(e) => self.make_error(format!("Error parsing integer literal: {e}"), false),
            };
        }

        self.advance(); // skip '.'

        while !self.is_at_end() && is_digit(self.peek_next()) {
            self.advance();
        }

        if is_letter_or_underscore(self.peek()) {
            return self.make_error("invalid identifier".into(), false);
        }

        let literal: String = self.source[start..self.offset].iter().collect();
        match literal.parse() {
            Ok(val) => self.make_token(TokenKind::ConstDouble(val), start, false),
            Err(e) => self.make_error(format!("Error parsing double literal: {e}"), false),
        }
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
