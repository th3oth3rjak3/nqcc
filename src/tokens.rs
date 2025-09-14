use crate::errors::CompilerError;

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
    ConstInt(i32),
    ConstLong(i64),
    ConstUInt(u32),
    ConstULong(u64),
    ConstDouble(f64),
    // StringLiteral(String),

    // Keywords
    Int,
    Void,
    Return,
    Struct,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    // LeftBracket,
    // RightBracket,
    Semicolon,
    Comma,
    // Operators
    // Plus,
    Minus,
    MinusMinus,
    Tilde,
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
    Error(CompilerError),
}
