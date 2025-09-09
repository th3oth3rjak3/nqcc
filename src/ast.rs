#[derive(Debug, Clone)]
pub struct Identifier(pub String);

impl Identifier {
    pub fn name(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: Identifier,
    pub return_type: Identifier,
    pub body: Statement,
    pub params: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    ReturnStatement(Option<Expression>),
}

#[derive(Debug, Clone)]
pub enum Expression {
    IntegerConstant(i64),
    FloatConstant(f64),
    BooleanConstant(bool),
    StringConstant(String),
    If {
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternate: Option<Box<Statement>>,
    },
}

#[derive(Debug, Clone)]
pub struct Program(pub FunctionDefinition);

#[derive(Debug, Clone)]
pub enum AstNode {
    Function(FunctionDefinition),
    Statement(Statement),
    Expression(Expression),
}
