use std::{path::PathBuf, process::Command};

use crate::{Cli, lexer::Lexer};

pub fn execute(cli_args: Cli) {
    let preprocessed_file = preprocess_file(cli_args.filename.clone());
    let asm_file = run_compiler(cli_args, preprocessed_file);
    assemble_and_link(asm_file);
}

fn preprocess_file(filename: PathBuf) -> PathBuf {
    let input_file = std::fs::canonicalize(filename.clone()).expect("Filepath does not exist");
    let mut output_file = input_file.clone();
    if !output_file.set_extension("i") {
        eprintln!("Failed to set file extension for pre-processed file.");
        std::process::exit(1);
    }

    let mut child = Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(input_file)
        .arg("-o")
        .arg(&output_file)
        .spawn()
        .expect("preprocessor command failed to start");

    let exit_status = child
        .wait()
        .map(|status| status.code())
        .unwrap_or(Some(1))
        .unwrap();

    if exit_status != 0 {
        eprintln!("Preprocessor step failed.");
        std::process::exit(exit_status);
    }

    output_file
}

fn run_compiler(cli_args: Cli, preprocessed_file: PathBuf) -> PathBuf {
    _ = cli_args;
    let input_file = preprocessed_file.clone();
    let mut output_file = preprocessed_file.clone();
    if !output_file.set_extension("s") {
        eprintln!("Failed to set file extension for assembly file.");
        std::process::exit(1);
    }

    let source = match std::fs::read_to_string(input_file.clone()) {
        Ok(src) => src,
        Err(e) => {
            eprintln!("Failed to open source file for compilation: {:?}", e);
            std::process::exit(1);
        }
    };

    let source_chars = source.chars().collect::<Vec<_>>();
    let mut lexer = Lexer::new(&source_chars);
    let tokens = lexer.tokenize();

    if cli_args.lex {
        println!("TOKENS: {:#?}", &tokens);
        std::process::exit(0);
    }

    // TODO: actually use the compiler to produce asm, for now we stub out the contents of the asm
    let fake_asm: &str = r#"        .text
    .globl main
main:
        movl    $2, %eax
        ret

    .section .note.GNU-stack,"",@progbits

"#;
    if let Err(e) = std::fs::write(&output_file, fake_asm) {
        eprintln!("Failed to write asm: {e}");
        std::process::exit(1);
    }

    // Delete the preprocessed file
    if let Err(e) = std::fs::remove_file(input_file) {
        eprintln!("Failed to delete proprocessed file: {e}");
        std::process::exit(1);
    }

    output_file
}

fn assemble_and_link(asm_file: PathBuf) {
    let input_file = asm_file.clone();
    let mut output_file = input_file.clone();
    if !output_file.set_extension("") {
        eprintln!("Failed to set file extension for ouptut file.");
        std::process::exit(1);
    }

    let mut child = Command::new("gcc")
        .arg(input_file)
        .arg("-o")
        .arg(output_file)
        .spawn()
        .expect("assembly and linking command failed to start");

    let exit_status = child
        .wait()
        .map(|status| status.code())
        .unwrap_or(Some(1))
        .unwrap();

    if exit_status != 0 {
        eprintln!("Assembly and linking step failed.");
        std::process::exit(exit_status);
    }

    // Delete the asm file
    if let Err(e) = std::fs::remove_file(asm_file) {
        eprintln!("Failed to delete asm file: {e}");
        std::process::exit(1);
    }

    std::process::exit(0);
}
