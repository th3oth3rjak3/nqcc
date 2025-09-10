use crate::{
    ast::{
        Block, BlockItem, Declaration, Expression, FunctionDeclaration, FunctionParam, Program,
        Statement,
    },
    errors::CompilerError,
    tokens::{Token, TokenKind},
    types,
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

    pub fn parse_program(&mut self) -> Result<Program, CompilerError> {
        let mut decls: Vec<Declaration> = Vec::new();

        while !self.is_at_end() {
            let decl = self.parse_declaration()?;
            decls.push(decl);
        }

        Ok(Program(decls))
    }

    fn parse_declaration(&mut self) -> Result<Declaration, CompilerError> {
        let tok = self.peek();

        match tok.kind {
            TokenKind::Int | TokenKind::Void => {
                // Skip looking at the next token because it's probably an ident, but it doesn't
                // matter at the moment.
                // If the token after that is a (, then it's a function, if not, it must be some kind of var declaration
                let next = self.peek_at(self.current + 2)?;

                if matches!(next.kind, TokenKind::LeftParen) {
                    // It's definitely (probably) a function (we hope)
                    return self.parse_func_decl();
                }

                // it must be a var declaration
                self.parse_var_decl()
            }
            TokenKind::Struct => self.parse_struct_decl(),
            _ => todo!("handle parsing var declarations probably"),
        }
    }

    fn parse_struct_decl(&mut self) -> Result<Declaration, CompilerError> {
        todo!()
    }

    fn parse_func_decl(&mut self) -> Result<Declaration, CompilerError> {
        // first get the type
        let type_token = self.take()?;

        let typ = match type_token.kind {
            TokenKind::Int => types::T::Int,
            _ => {
                return Err(CompilerError::ParseError {
                    message: "type not found where expected".into(),
                });
            }
        };

        let ident_token = self.take()?;
        let ident = match ident_token.kind {
            TokenKind::Identifier(name) => name,
            _ => {
                return Err(CompilerError::ParseError {
                    message: "function name not found where expected".into(),
                });
            }
        };

        self.expect(TokenKind::LeftParen)?;

        let mut params: Vec<FunctionParam> = Vec::new();

        while !self.is_at_end() && self.peek().kind != TokenKind::RightParen {
            let type_token = self.take()?;

            let typ = match type_token.kind {
                TokenKind::Int => types::T::Int,
                TokenKind::Void => types::T::Void,
                _ => {
                    return Err(CompilerError::ParseError {
                        message: "type not found where expected".into(),
                    });
                }
            };

            // special case for void like this int main(void) { return 2; } // void is the only type that has no name.
            if typ == types::T::Void {
                let param = FunctionParam {
                    typ,
                    name: "".into(),
                };
                params.push(param);

                break;
            }

            // Wasn't a single void param, so we must need to parse the ident still.
            let next = self.take()?;

            if let TokenKind::Identifier(name) = next.kind {
                let param = FunctionParam {
                    typ,
                    name: name.into(),
                };

                params.push(param);
            } else {
                return Err(CompilerError::ParseError {
                    message: "invalid parameter declaration".into(),
                });
            }

            if matches!(self.peek().kind, TokenKind::Comma) {
                self.take()?;
            }
        }

        self.expect(TokenKind::RightParen)?;

        let body = self.parse_block()?;

        let func = FunctionDeclaration {
            return_type: typ,
            name: ident,
            params,
            body,
            storage_class: None,
        };

        Ok(Declaration::FunDecl(func))
    }

    fn parse_var_decl(&mut self) -> Result<Declaration, CompilerError> {
        todo!()
    }

    fn parse_block(&mut self) -> Result<Option<Block>, CompilerError> {
        let mut items: Vec<BlockItem> = Vec::new();
        self.expect(TokenKind::LeftBrace)?;

        while !self.is_at_end() && self.peek().kind != TokenKind::RightBrace {
            let item = self.parse_block_item()?;
            items.push(item);
        }
        // parse all block items

        self.expect(TokenKind::RightBrace)?;

        Ok(Some(Block(items)))
    }

    fn parse_block_item(&mut self) -> Result<BlockItem, CompilerError> {
        let stmt = self.parse_statement()?;
        Ok(BlockItem::Stmt(stmt))
    }

    fn parse_statement(&mut self) -> Result<Statement, CompilerError> {
        self.expect(TokenKind::Return)?;
        let expr = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;

        Ok(Statement::ReturnStatement(Some(expr)))
    }

    fn parse_expression(&mut self) -> Result<Expression, CompilerError> {
        let tok = self.take()?;
        match tok.kind {
            TokenKind::ConstInt(val) => Ok(Expression::ConstInt(val)),
            _ => {
                let err = format!("Expected: Integer; Got: {:#?}", tok.kind);
                return Err(CompilerError::ParseError { message: err });
            }
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, CompilerError> {
        if kind != self.tokens[self.current].kind {
            let err = CompilerError::ParseError {
                message: format!(
                    "Expected: {:#?}; Got: {:#?}",
                    kind, self.tokens[self.current].kind
                ),
            };
            return Err(err);
        }

        self.take()
    }

    fn take(&mut self) -> Result<Token, CompilerError> {
        let tok = self.tokens[self.current].clone();
        self.current += 1;
        Ok(tok)
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    fn peek_at(&mut self, location: usize) -> Result<Token, CompilerError> {
        if location >= self.tokens.len() {
            return Err(CompilerError::ParseError {
                message: "unexpected end of file".into(),
            });
        }

        Ok(self.tokens[location].clone())
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.tokens.len() || self.peek().kind == TokenKind::EOF
    }
}
