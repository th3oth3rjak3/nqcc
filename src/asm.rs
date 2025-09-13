#[derive(Debug, Clone, Copy)]
pub enum Register {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RSP,
    RBP,
    EAX,
}

#[derive(Debug, Clone, Copy)]
pub enum AsmType {
    /// 1 byte (8 bits)
    Byte,

    /// 4 bytes (32 bits)
    Longword,

    /// 8 bytes (64 bits)
    Quadword,

    /// 8 bytes (64 bits), typically used for floating-point doubles
    Double,

    /// An array of bytes with a specific size and alignment
    /// `size` = number of bytes in the array
    /// `alignment` = required memory alignment in bytes
    ByteArray { size: i64, alignment: i64 },
}

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Imm(i64),
    Reg(Register),
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

#[derive(Debug, Clone)]
pub struct Program(pub Vec<TopLevel>);
