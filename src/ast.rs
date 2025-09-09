pub struct Identifier {
    pub name: String,
}

pub enum Function {
    FunctionLiteral {
        name: Identifier,
        return_type: Identifier,
        body: Statement,
        params: Vec<Expression>,
    },
}

pub enum Statement {
    ReturnStatement(Option<Expression>),
}

pub enum Expression {
    IntegerConstant(i64),
    If {
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternate: Option<Box<Statement>>,
    },
}

pub enum AstNode {
    Program(Function),
    Function(Function),
    Statement(Statement),
    Expression(Expression),
}
