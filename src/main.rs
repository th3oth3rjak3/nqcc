use std::path::PathBuf;

use clap::{ArgAction, Parser};

pub mod asm;
pub mod ast;
pub mod code_emission;
pub mod codegen;
pub mod driver;
pub mod errors;
pub mod ir;
pub mod lexer;
pub mod parser;
pub mod tokens;
pub mod types;

const LONG_ABOUT: &'static str = r#"nqcc - The Not Quite C compiler written in Rust"#;

/// nqcc - The Not Quite C compiler written in Rust
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = Some(LONG_ABOUT))]
pub struct Cli {
    /// Input C file
    filename: PathBuf,

    /// Only lex the input and stop before parsing.
    #[arg(long, action = ArgAction::SetTrue)]
    lex: bool,

    /// Only lex and parse the input and stop before assembly generation.
    #[arg(long, action = ArgAction::SetTrue)]
    parse: bool,

    /// Only lex, parse, and run assembly generation but stop before
    /// code emission.
    #[arg(long, action = ArgAction::SetTrue)]
    codegen: bool,

    /// Only lex, parse, and generate a TACKY IR.
    #[arg(long, action = ArgAction::SetTrue)]
    tacky: bool,

    /// Emit assembly but do not link (-S)
    #[arg(short = 'S', action = ArgAction::SetTrue)]
    emit_asm: bool,
}

fn main() {
    let cli = Cli::parse();
    driver::execute(cli)
}
