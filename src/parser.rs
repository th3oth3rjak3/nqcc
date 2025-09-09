use crate::{
    ast::{Expression, FunctionDefinition, Identifier, Program, Statement},
    lexer::{Token, TokenKind},
};

#[derive(Debug, Clone)]
pub struct Parser {
    pub current: usize,
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { current: 0, tokens }
    }

    pub fn parse_program(&mut self) -> Program {
        self.expect(TokenKind::Int);
        self.expect(TokenKind::Identifier("main".to_string()));
        self.expect(TokenKind::LeftParen);
        self.expect(TokenKind::Void);
        self.expect(TokenKind::RightParen);
        self.expect(TokenKind::LeftBrace);
        let body = self.parse_statement();
        self.expect(TokenKind::RightBrace);
        self.expect(TokenKind::EOF);

        Program(FunctionDefinition {
            name: Identifier("main".to_string()),
            return_type: Identifier("int".to_string()),
            body,
            params: Vec::new(),
        })
    }

    fn parse_statement(&mut self) -> Statement {
        self.expect(TokenKind::Return);
        let expr = self.parse_expression();
        self.expect(TokenKind::Semicolon);

        Statement::ReturnStatement(Some(expr))
    }

    fn parse_expression(&mut self) -> Expression {
        let tok = self.take();
        match tok.kind {
            TokenKind::IntegerLiteral(val) => Expression::IntegerConstant(val),
            _ => {
                eprintln!("Expected: Integer; Got: {:#?}", tok.kind);
                std::process::exit(1);
            }
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Token {
        if kind != self.tokens[self.current].kind {
            eprintln!(
                "Expected: {:#?}; Got: {:#?}",
                kind, self.tokens[self.current].kind
            );
            std::process::exit(1);
        }

        self.take()
    }

    fn take(&mut self) -> Token {
        let tok = self.tokens[self.current].clone();
        self.current += 1;
        tok
    }
}
