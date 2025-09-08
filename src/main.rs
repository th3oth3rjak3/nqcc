use std::path::PathBuf;

use clap::{ArgAction, Parser};

pub mod ast;
pub mod codegen;
pub mod driver;
pub mod lexer;
pub mod parser;

/// nqcc - The Not Quite C compiler written in Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
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

    /// Emit assembly but do not link (-S)
    #[arg(short = 'S', action = ArgAction::SetTrue)]
    emit_asm: bool,
}

fn main() {
    let cli = Cli::parse();
    driver::execute(cli)
}
