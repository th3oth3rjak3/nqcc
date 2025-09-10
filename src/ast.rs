use crate::types;

#[derive(Debug, Clone)]
pub struct Identifier(pub String);

impl Identifier {
    pub fn name(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StorageClass {
    Static,
    Extern,
}

#[derive(Debug, Clone)]
pub enum Statement {
    ReturnStatement(Option<Expression>),
}

#[derive(Debug, Clone)]
pub enum Expression {
    ConstInt(i32),
    ConstLong(i64),
    ConstDouble(f64),
    StringConstant(String),
    If {
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternate: Option<Box<Statement>>,
    },
}

#[derive(Debug, Clone)]
pub struct Program(pub Vec<Declaration>);

#[derive(Debug, Clone)]
pub enum BlockItem {
    Stmt(Statement),
    Decl(Declaration),
}

#[derive(Debug, Clone)]
pub struct Block(pub Vec<BlockItem>);

#[derive(Debug, Clone)]
pub struct FunctionParam {
    pub typ: types::T,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub name: String,
    pub return_type: types::T,
    pub params: Vec<FunctionParam>,
    pub body: Option<Block>,
    pub storage_class: Option<StorageClass>,
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {}

#[derive(Debug, Clone)]
pub struct StructDeclaration {}

#[derive(Debug, Clone)]
pub enum Declaration {
    FunDecl(FunctionDeclaration),
    VarDecl(VariableDeclaration),
    StructDecl(StructDeclaration),
}
