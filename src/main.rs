pub mod ast;
pub mod codegen;
pub mod driver;
pub mod lexer;
pub mod parser;

fn main() {
    let args = std::env::args().skip(1).take(1).collect::<Vec<_>>();

    if args.len() != 1 {
        eprintln!("Usage: nqcc <filename.c>");
        std::process::exit(1);
    }

    driver::execute(args[0].clone())
}
