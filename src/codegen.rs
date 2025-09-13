use crate::asm::{self, AsmType, Operand, Register};
use crate::asm::{Instruction, Program, TopLevel};
use crate::ast::{self, Declaration, FunctionDeclaration};

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&mut self, program: &ast::Program) -> asm::Program {
        let mut stmts = Vec::new();

        for decl in &program.0 {
            let top_level = match decl {
                Declaration::FunDecl(fun) => self.generate_function(fun),
                _ => todo!("implement support for structs and variable declarations"),
            };

            stmts.push(top_level);
        }

        Program(stmts)
    }

    fn generate_function(&mut self, func_decl: &ast::FunctionDeclaration) -> TopLevel {
        let mut instructions = Vec::new();

        if let Some(body) = &func_decl.body {
            for item in &body.0 {
                match item {
                    ast::BlockItem::Stmt(stmt) => match stmt {
                        ast::Statement::ReturnStatement(expr_opt) => {
                            if let Some(expr) = expr_opt {
                                match expr {
                                    ast::Expression::ConstInt(val) => {
                                        instructions.push(Instruction::Mov {
                                            ty: AsmType::Longword,
                                            src: Operand::Imm((*val).into()),
                                            dst: Operand::Reg(Register::EAX),
                                        });
                                    }
                                    _ => todo!("handle other expressions"),
                                }
                            }
                            instructions.push(Instruction::Ret);
                        }
                        _ => todo!("handle other statements"),
                    },
                    _ => todo!("handle other block items"),
                }
            }
        }

        TopLevel::Function {
            name: func_decl.name.clone(),
            global: true,
            instructions,
        }
    }

    fn generate_variable() -> Vec<Instruction> {
        todo!()
    }
    fn generate_structure() -> Vec<Instruction> {
        todo!()
    }
}
