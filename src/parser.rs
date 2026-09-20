use crate::ast::Statement;
use crate::error::ParserErrorKind::{
    ExpectedStatement, MissingDelimiter, MissingSemicolon, UnexpectedToken, UnidentifiedToken,
};
use crate::error::{Error, ParserError};
use crate::lexer::Lexer;
use crate::token::{
    Token,
    TokenKind::{self, *},
};
use crate::{
    ast,
    ast::{BinaryOperators, Expression, UnaryOperators},
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Result<Self, Error> {
        match lexer.next_token() {
            Ok(token) => Ok(Self {
                lexer,
                curr_token: token,
            }),
            Err(e) => Err(Error::Lex(e)),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, Error> {
        self.program()
    }

    fn consume(&mut self) -> Result<Token, Error> {
        let prev = std::mem::replace(&mut self.curr_token, self.lexer.next_token()?);

        Ok(prev)
    }

    fn skip_newlines(&mut self) -> Result<(), Error> {
        while self.curr_token.kind == TokenKind::Newline {
            self.consume()?;
        }

        Ok(())
    }

    fn program(&mut self) -> Result<Vec<Statement>, Error> {
        let mut statements = Vec::new();
        self.skip_newlines()?;

        while self.curr_token.kind != EOF {
            statements.push(self.statement()?);
            self.skip_newlines()?;
        }

        Ok(statements)
    }

    fn statement(&mut self) -> Result<Statement, Error> {
        let res = match self.curr_token.kind {
            TokenKind::Set => self.assignment()?,
            TokenKind::Out => self.print()?,
            _ => {
                return Err(ParserError::new(
                    ExpectedStatement,
                    self.curr_token.line,
                    self.curr_token.column,
                )
                .into());
            }
        };

        if self.curr_token.kind != TokenKind::Semicolon {
            return Err(ParserError::new(
                MissingSemicolon,
                self.curr_token.line,
                self.curr_token.column,
            )
            .into());
        }
        self.consume()?;

        Ok(res)
    }

    fn assignment(&mut self) -> Result<Statement, Error> {
        self.consume()?;

        if !matches!(&self.curr_token.kind, TokenKind::Identifier(_)) {
            return Err(ParserError::new(
                UnexpectedToken,
                self.curr_token.line,
                self.curr_token.column,
            )
            .into());
        }
        let iden = match self.consume()?.kind {
            TokenKind::Identifier(s) => s,
            _ => unreachable!(),
        };

        if self.curr_token.kind != TokenKind::Equals {
            return Err(ParserError::new(
                UnexpectedToken,
                self.curr_token.line,
                self.curr_token.column,
            )
            .into());
        }
        self.consume()?;

        let expr = self.expr()?;
        Ok(Statement::Assignment(ast::Assignment {
            iden: ast::Identifier(iden),
            expr,
        }))
    }

    fn print(&mut self) -> Result<Statement, Error> {
        self.consume()?;

        let expr = self.expr()?;
        Ok(Statement::Print(ast::Print(expr)))
    }

    fn expr(&mut self) -> Result<Expression, Error> {
        let mut lhs = self.term()?;

        while self.curr_token.kind == Add || self.curr_token.kind == Sub {
            let operator = self.consume()?;
            let rhs = self.term()?;

            match operator.kind {
                Add => {
                    let res = ast::BinaryOperation {
                        operator: BinaryOperators::Add,
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    };
                    lhs = Expression::BinaryOperation(res);
                }
                Sub => {
                    let res = ast::BinaryOperation {
                        operator: BinaryOperators::Sub,
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    };
                    lhs = Expression::BinaryOperation(res);
                }
                _ => {
                    return Err(ParserError::new(
                        UnidentifiedToken,
                        operator.line,
                        operator.column,
                    )
                    .into());
                }
            }
        }

        Ok(lhs)
    }

    fn term(&mut self) -> Result<Expression, Error> {
        let mut lhs = self.factor()?;

        while self.curr_token.kind == Mul
            || self.curr_token.kind == Div
            || self.curr_token.kind == Percent
        {
            let operator = self.consume()?;
            let rhs = self.factor()?;

            match operator.kind {
                Mul => {
                    let res = ast::BinaryOperation {
                        operator: BinaryOperators::Mul,
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    };
                    lhs = Expression::BinaryOperation(res);
                }
                Div => {
                    let res = ast::BinaryOperation {
                        operator: BinaryOperators::Div,
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    };
                    lhs = Expression::BinaryOperation(res);
                }
                Percent => {
                    let res = ast::BinaryOperation {
                        operator: BinaryOperators::Mod,
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    };
                    lhs = Expression::BinaryOperation(res);
                }
                _ => {
                    return Err(ParserError::new(
                        UnidentifiedToken,
                        operator.line,
                        operator.column,
                    )
                    .into());
                }
            }
        }

        Ok(lhs)
    }

    fn factor(&mut self) -> Result<Expression, Error> {
        let token = self.consume()?;

        match token.kind {
            Number(n) => {
                let res = ast::Number(n);
                Ok(Expression::Number(res))
            }
            Identifier(i) => {
                let res = ast::Identifier(i);
                Ok(Expression::Identifier(res))
            }
            LParen => {
                let res = self.expr()?;

                let rparen = self.consume()?;
                if rparen.kind != RParen {
                    return Err(
                        ParserError::new(MissingDelimiter, rparen.line, rparen.column).into(),
                    );
                }

                Ok(res)
            }
            Add => {
                let res = ast::UnaryOperation {
                    operator: UnaryOperators::Add,
                    operand: Box::new(self.factor()?),
                };
                Ok(Expression::UnaryOperation(res))
            }
            Sub => {
                let res = ast::UnaryOperation {
                    operator: UnaryOperators::Sub,
                    operand: Box::new(self.factor()?),
                };
                Ok(Expression::UnaryOperation(res))
            }
            _ => Err(ParserError::new(UnidentifiedToken, token.line, token.column).into()),
        }
    }
}
