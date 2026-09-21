use crate::ast::{self, Expression};
use crate::error::{Error, ParserError, ParserErrorKind::*};
use crate::lexer::Lexer;
use crate::token::{
    Token,
    TokenKind::{self, *},
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

    pub fn parse(&mut self) -> Result<Vec<ast::Statement>, Error> {
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

    fn program(&mut self) -> Result<Vec<ast::Statement>, Error> {
        let mut statements = Vec::new();
        self.skip_newlines()?;

        while self.curr_token.kind != EOF {
            statements.push(self.statement()?);
            self.skip_newlines()?;
        }

        Ok(statements)
    }

    fn statement(&mut self) -> Result<ast::Statement, Error> {
        let res = match self.curr_token.kind {
            TokenKind::Set => self.assignment()?,
            TokenKind::Out => self.print()?,
            TokenKind::If => self.if_else()?,
            _ => {
                return Err(ParserError::new(
                    ExpectedStatement,
                    self.curr_token.line,
                    self.curr_token.column,
                )
                .into());
            }
        };

        if self.curr_token.kind != TokenKind::Newline && self.curr_token.kind != TokenKind::EOF {
            return Err(ParserError::new(
                UnexpectedToken,
                self.curr_token.line,
                self.curr_token.column,
            )
            .into());
        }

        Ok(res)
    }

    fn assignment(&mut self) -> Result<ast::Statement, Error> {
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
        Ok(ast::Statement::Assignment(ast::Assignment {
            iden: ast::Identifier(iden),
            expr,
        }))
    }

    fn print(&mut self) -> Result<ast::Statement, Error> {
        self.consume()?;

        let expr = self.expr()?;
        Ok(ast::Statement::Print(ast::Print(expr)))
    }

    fn if_else(&mut self) -> Result<ast::Statement, Error> {
        self.consume()?;

        let if_cond = self.expr()?;

        if self.curr_token.kind != TokenKind::Colon {
            return Err(ParserError::new(
                UnidentifiedToken,
                self.curr_token.line,
                self.curr_token.column,
            )
            .into());
        }
        self.consume()?;

        let mut if_stmt = Vec::new();
        while self.curr_token.kind != TokenKind::Else && self.curr_token.kind != TokenKind::EndIf {
            if_stmt.push(self.statement()?);
        }

        let mut else_stmt = Vec::new();
        if self.curr_token.kind == Else {
            self.consume()?;

            if self.curr_token.kind != TokenKind::Colon {
                return Err(ParserError::new(
                    UnidentifiedToken,
                    self.curr_token.line,
                    self.curr_token.column,
                )
                .into());
            }
            self.consume()?;

            while self.curr_token.kind != TokenKind::EndIf {
                else_stmt.push(self.statement()?);
            }
        }

        if self.curr_token.kind != TokenKind::EndIf {
            return Err(ParserError::new(
                UnidentifiedToken,
                self.curr_token.line,
                self.curr_token.column,
            )
            .into());
        }
        self.consume()?;

        let res = ast::IfStmt {
            if_cond,
            if_stmt,
            else_stmt,
        };
        Ok(ast::Statement::IfStmt(res))
    }

    fn expr(&mut self) -> Result<ast::Expression, Error> {
        self.or_expr()
    }

    fn or_expr(&mut self) -> Result<ast::Expression, Error> {
        let mut lhs = self.and_expr()?;

        while self.curr_token.kind == DPipe {
            self.consume()?;
            let rhs = self.and_expr()?;

            lhs = self.construct_bin_expr(ast::BinaryOperators::Or, lhs, rhs)
        }

        Ok(lhs)
    }

    fn and_expr(&mut self) -> Result<ast::Expression, Error> {
        let mut lhs = self.equality()?;

        while self.curr_token.kind == DAmpersand {
            self.consume()?;
            let rhs = self.equality()?;

            lhs = self.construct_bin_expr(ast::BinaryOperators::And, lhs, rhs)
        }

        Ok(lhs)
    }

    fn equality(&mut self) -> Result<ast::Expression, Error> {
        let mut lhs = self.comparison()?;

        while self.curr_token.kind == DEquals || self.curr_token.kind == NotEquals {
            let operator = self.consume()?;
            let rhs = self.comparison()?;

            match operator.kind {
                DEquals => lhs = self.construct_bin_expr(ast::BinaryOperators::Equals, lhs, rhs),
                NotEquals => {
                    lhs = self.construct_bin_expr(ast::BinaryOperators::NotEquals, lhs, rhs)
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

    fn comparison(&mut self) -> Result<ast::Expression, Error> {
        let mut lhs = self.arithmetic()?;

        while self.curr_token.kind == LAngle
            || self.curr_token.kind == RAngle
            || self.curr_token.kind == LAngleEquals
            || self.curr_token.kind == RAngleEquals
        {
            let operator = self.consume()?;
            let rhs = self.arithmetic()?;

            match operator.kind {
                LAngle => lhs = self.construct_bin_expr(ast::BinaryOperators::LThan, lhs, rhs),
                RAngle => lhs = self.construct_bin_expr(ast::BinaryOperators::GThan, lhs, rhs),
                LAngleEquals => {
                    lhs = self.construct_bin_expr(ast::BinaryOperators::LThanEquals, lhs, rhs)
                }

                RAngleEquals => {
                    lhs = self.construct_bin_expr(ast::BinaryOperators::GThanEquals, lhs, rhs)
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

    fn arithmetic(&mut self) -> Result<ast::Expression, Error> {
        let mut lhs = self.term()?;

        while self.curr_token.kind == Plus || self.curr_token.kind == Minus {
            let operator = self.consume()?;
            let rhs = self.term()?;

            match operator.kind {
                Plus => lhs = self.construct_bin_expr(ast::BinaryOperators::Add, lhs, rhs),
                Minus => lhs = self.construct_bin_expr(ast::BinaryOperators::Sub, lhs, rhs),
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

    fn term(&mut self) -> Result<ast::Expression, Error> {
        let mut lhs = self.factor()?;

        while self.curr_token.kind == Star
            || self.curr_token.kind == Slash
            || self.curr_token.kind == Percent
        {
            let operator = self.consume()?;
            let rhs = self.factor()?;

            match operator.kind {
                Star => lhs = self.construct_bin_expr(ast::BinaryOperators::Mul, lhs, rhs),
                Slash => lhs = self.construct_bin_expr(ast::BinaryOperators::Div, lhs, rhs),
                Percent => lhs = self.construct_bin_expr(ast::BinaryOperators::Mod, lhs, rhs),
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

    fn factor(&mut self) -> Result<ast::Expression, Error> {
        let token = self.consume()?;

        match token.kind {
            Number(n) => {
                let res = ast::Number(n);
                Ok(ast::Expression::Number(res))
            }
            True => {
                let res = ast::Boolean(true);
                Ok(ast::Expression::Boolean(res))
            }
            False => {
                let res = ast::Boolean(false);
                Ok(ast::Expression::Boolean(res))
            }
            Identifier(i) => {
                let res = ast::Identifier(i);
                Ok(ast::Expression::Identifier(res))
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
            Plus => {
                let res = ast::UnaryOperation {
                    operator: ast::UnaryOperators::Add,
                    operand: Box::new(self.factor()?),
                };
                Ok(ast::Expression::UnaryOperation(res))
            }
            Minus => {
                let res = ast::UnaryOperation {
                    operator: ast::UnaryOperators::Sub,
                    operand: Box::new(self.factor()?),
                };
                Ok(ast::Expression::UnaryOperation(res))
            }
            Exclaim => {
                let res = ast::UnaryOperation {
                    operator: ast::UnaryOperators::Not,
                    operand: Box::new(self.factor()?),
                };
                Ok(ast::Expression::UnaryOperation(res))
            }
            _ => Err(ParserError::new(UnidentifiedToken, token.line, token.column).into()),
        }
    }

    fn construct_bin_expr(
        &self,
        operator: ast::BinaryOperators,
        lhs: Expression,
        rhs: Expression,
    ) -> ast::Expression {
        let res = ast::BinaryOperation {
            operator,
            left: Box::new(lhs),
            right: Box::new(rhs),
        };

        ast::Expression::BinaryOperation(res)
    }
}
