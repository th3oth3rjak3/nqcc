#[derive(Debug, Clone, Copy)]
pub enum AsmType {
    Byte,
    Longword,
    Quadword,
    Double,
    ByteArray { size: i64, alignment: i64 },
}

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Imm(i64),
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Mov {
        ty: AsmType,
        src: Operand,
        dst: Operand,
    },
    Ret,
}

#[derive(Debug, Clone)]
pub enum TopLevel {
    Function {
        name: String,
        global: bool,
        instructions: Vec<Instruction>,
    },
}

pub struct Program(pub Vec<TopLevel>);
