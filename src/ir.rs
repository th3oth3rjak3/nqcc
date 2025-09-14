use crate::ast::{self};

#[derive(Debug, Clone)]
pub struct Program(pub Vec<Function>);

#[derive(Debug, Clone)]
pub struct Function {
    pub identifier: String,
    pub body: Option<Vec<Instruction>>,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Return(Value),
    Unary {
        op: UnaryOperator,
        src: Value,
        dst: Value,
    },
}

#[derive(Debug, Clone)]
pub enum Value {
    Constant(i32),
    Var(String),
}

impl Value {
    pub fn name(&self) -> String {
        match self {
            Value::Var(s) => s.clone(),
            Value::Constant(_) => panic!("Cannot get name of a constant"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOperator {
    Complement,
    Negate,
}

#[derive(Debug, Clone, Copy)]
pub struct TackyGenerator {
    next_temp: usize,
}

impl TackyGenerator {
    pub fn new() -> Self {
        Self { next_temp: 0 }
    }

    pub fn emit_tacky_program(&mut self, program: &ast::Program) -> Program {
        let mut functions = Vec::new();

        for decl in &program.0 {
            match decl {
                ast::Declaration::FunDecl(func_decl) => {
                    let func = self.generate_function(func_decl);
                    functions.push(func);
                }
                ast::Declaration::VarDecl(_) | ast::Declaration::StructDecl(_) => {
                    // For now, we ignore these in our minimal subset
                    continue;
                }
            }
        }

        Program(functions)
    }

    pub fn generate_function(&mut self, func: &ast::FunctionDeclaration) -> Function {
        let mut instructions = Vec::new();

        if let Some(body) = &func.body {
            for item in &body.0 {
                match item {
                    ast::BlockItem::Stmt(stmt) => {
                        self.emit_statement(stmt.clone(), &mut instructions)
                    }
                    ast::BlockItem::Decl(_) => panic!("Variable declarations not handled yet"),
                }
            }
        }

        Function {
            identifier: func.name.clone(),
            body: Some(instructions),
        }
    }

    pub fn emit_statement(&mut self, stmt: ast::Statement, instructions: &mut Vec<Instruction>) {
        match stmt {
            ast::Statement::ReturnStatement(Some(expr)) => {
                let val = self.emit_tacky(expr, instructions);
                instructions.push(Instruction::Return(val));
            }
            _ => panic!("Unsupported statement type"),
        }
    }

    pub fn emit_tacky(
        &mut self,
        expr: ast::Expression,
        instructions: &mut Vec<Instruction>,
    ) -> Value {
        match expr {
            ast::Expression::ConstInt(val) => Value::Constant(val),

            ast::Expression::Unary { operator, expr } => {
                let src = self.emit_tacky(*expr, instructions);
                let dst_name = self.make_temp();
                let dst = Value::Var(dst_name.clone());

                instructions.push(Instruction::Unary {
                    op: self.convert_op(operator),
                    src,
                    dst: dst.clone(),
                });

                dst
            }

            ast::Expression::Grouping(inner) => {
                // Simply unwrap the grouping
                self.emit_tacky(*inner, instructions)
            }

            _ => panic!("Unsupported expression type: {:?}", expr),
        }
    }

    fn make_temp(&mut self) -> String {
        let tmp = format!("tmp.{}", self.next_temp);
        self.next_temp += 1;
        tmp
    }

    fn convert_op(&self, op: ast::UnaryOperator) -> UnaryOperator {
        match op {
            ast::UnaryOperator::Negate => UnaryOperator::Negate,
            ast::UnaryOperator::Complement => UnaryOperator::Complement,
        }
    }
}
