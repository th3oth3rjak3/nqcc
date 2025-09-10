use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum CompilerError {
    LexError {
        message: String,
        line: usize,
        column: usize,
    },
    ParseError {
        message: String,
    },
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilerError::LexError {
                message,
                line,
                column,
            } => f.write_fmt(format_args!("[{}:{}] Error: {}", line, column, message)),
            CompilerError::ParseError { message } => f.write_str(message),
        }
    }
}
