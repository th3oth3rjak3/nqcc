use crate::asm::{self, AsmType, Operand, Register};
use crate::asm::{Instruction, Program, TopLevel};
use crate::ir::{self, Value};

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&mut self, program: &ir::Program) -> asm::Program {
        let mut stmts = Vec::new();

        for func in &program.0 {
            let top_level = self.generate_function(func);
            stmts.push(top_level);
        }

        Program(stmts)
    }

    fn generate_function(&mut self, func: &ir::Function) -> TopLevel {
        let mut instructions = Vec::new();

        if let Some(body) = &func.body {
            for instr in body {
                match instr {
                    ir::Instruction::Return(val) => {
                        // Move constant or variable into EAX
                        let op = match val {
                            Value::Constant(c) => Operand::Imm((*c).into()),
                            Value::Var(name) => Operand::Mem(name.clone()), // assumes a simple memory model for temps
                        };

                        instructions.push(Instruction::Mov {
                            ty: AsmType::Longword,
                            src: op,
                            dst: Operand::Reg(Register::EAX),
                        });

                        instructions.push(Instruction::Ret);
                    }

                    ir::Instruction::Unary { op, src, dst } => {
                        let src_op = match src {
                            Value::Constant(c) => Operand::Imm((*c).into()),
                            Value::Var(name) => Operand::Mem(name.clone()),
                        };
                        let dst_op = Operand::Mem(dst.name()); // implement name() for Value::Var

                        match op {
                            ir::UnaryOperator::Negate => {
                                instructions.push(Instruction::Mov {
                                    ty: AsmType::Longword,
                                    src: src_op,
                                    dst: dst_op.clone(),
                                });
                                instructions.push(Instruction::Neg {
                                    ty: AsmType::Longword,
                                    dst: dst_op,
                                });
                            }
                            ir::UnaryOperator::Complement => {
                                instructions.push(Instruction::Mov {
                                    ty: AsmType::Longword,
                                    src: src_op,
                                    dst: dst_op.clone(),
                                });
                                instructions.push(Instruction::Not {
                                    ty: AsmType::Longword,
                                    dst: dst_op,
                                });
                            }
                        }
                    }
                }
            }
        }

        TopLevel::Function {
            name: func.identifier.clone(),
            global: true,
            instructions,
        }
    }
}
