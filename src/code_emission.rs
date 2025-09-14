use crate::asm::{self, Instruction, TopLevel};

#[derive(Debug, Clone, Copy)]
pub struct CodeEmitter;

impl CodeEmitter {
    pub fn new() -> Self {
        Self
    }

    pub fn emit(&mut self, program: &asm::Program) -> String {
        let mut output = String::new();

        for top_level in &program.0 {
            let result = match top_level {
                TopLevel::Function {
                    name,
                    global,
                    instructions,
                } => self.emit_function(
                    name.to_owned(),
                    *global,
                    instructions.iter().cloned().collect(),
                ),
            };

            output.push_str(&result);
        }

        let executable_stack_section = r#".section .note.GNU-stack,"",@progbits"#;
        output.push_str("\n");
        output.push_str(executable_stack_section);
        output.push_str("\n");

        output
    }

    fn emit_function(
        &mut self,
        name: String,
        global: bool,
        instructions: Vec<Instruction>,
    ) -> String {
        let mut output = String::new();

        // add global directive when appropriate.
        if global {
            output.push_str(&format!(".globl {name}\n"));
        }

        // function label
        output.push_str(&format!("{name}:\n"));

        // instruction emission
        for instr in instructions {
            let instr_str = self.emit_instruction(&instr);
            output.push_str(&format!("    {instr_str}\n"));
        }

        output
    }

    fn emit_instruction(&mut self, instr: &asm::Instruction) -> String {
        match instr {
            asm::Instruction::Mov { ty, src, dst } => {
                let mnemonic = match ty {
                    asm::AsmType::Byte => "movb",
                    asm::AsmType::Longword => "movl",
                    asm::AsmType::Quadword => "movq",
                    asm::AsmType::Double => "movsd",
                    asm::AsmType::ByteArray { .. } => todo!("ByteArray not yet supported"),
                };

                let src_str = match src {
                    asm::Operand::Imm(val) => format!("${val}"),
                    asm::Operand::Reg(r) => format!("%{:?}", r).to_lowercase(),
                    asm::Operand::Mem(name) => format!("-{}(%rbp)", name), // simple stack offset model for temps
                };

                let dst_str = match dst {
                    asm::Operand::Reg(r) => format!("%{:?}", r).to_lowercase(),
                    asm::Operand::Mem(name) => format!("-{}(%rbp)", name),
                    _ => panic!("Unsupported destination operand"),
                };

                format!("{mnemonic} {src_str}, {dst_str}")
            }

            asm::Instruction::Neg { ty, dst } => {
                let mnemonic = match ty {
                    asm::AsmType::Byte => "negb",
                    asm::AsmType::Longword => "negl",
                    asm::AsmType::Quadword => "negq",
                    asm::AsmType::Double => panic!("Cannot negate double"),
                    asm::AsmType::ByteArray { .. } => panic!("Unsupported"),
                };

                let dst_str = match dst {
                    asm::Operand::Reg(r) => format!("%{:?}", r).to_lowercase(),
                    asm::Operand::Mem(name) => format!("-{}(%rbp)", name),
                    _ => panic!("Unsupported operand for neg"),
                };

                format!("{mnemonic} {dst_str}")
            }

            asm::Instruction::Not { ty, dst } => {
                let mnemonic = match ty {
                    asm::AsmType::Byte => "notb",
                    asm::AsmType::Longword => "notl",
                    asm::AsmType::Quadword => "notq",
                    asm::AsmType::Double => panic!("Cannot bitwise complement double"),
                    asm::AsmType::ByteArray { .. } => panic!("Unsupported"),
                };

                let dst_str = match dst {
                    asm::Operand::Reg(r) => format!("%{:?}", r).to_lowercase(),
                    asm::Operand::Mem(name) => format!("-{}(%rbp)", name),
                    _ => panic!("Unsupported operand for not"),
                };

                format!("{mnemonic} {dst_str}")
            }

            asm::Instruction::Ret => "ret".into(),
        }
    }
}
