use crate::ast::{Expression, *};
use crate::error::{Error, ParserError, ParserErrorKind::*};
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

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

        while self.curr_token.kind != TokenKind::EOF {
            statements.push(self.statement()?);
            self.skip_newlines()?;
        }

        Ok(statements)
    }

    fn statement(&mut self) -> Result<Statement, Error> {
        let res = match self.curr_token.kind {
            TokenKind::Set => self.assign_stmt()?,
            TokenKind::Out => self.print_stmt()?,
            TokenKind::If => self.if_stmt()?,
            TokenKind::While => self.while_stmt()?,
            _ => {
                return Err(ParserError::new(
                    ExpectedStatement,
                    format!("expected statement"),
                    self.curr_token.line,
                    self.curr_token.column,
                )
                .into());
            }
        };

        if self.curr_token.kind != TokenKind::Newline && self.curr_token.kind != TokenKind::EOF {
            return Err(ParserError::new(
                UnexpectedToken,
                format!("expected newline or end of file after statement"),
                self.curr_token.line,
                self.curr_token.column,
            )
            .into());
        }

        Ok(res)
    }

    fn assign_stmt(&mut self) -> Result<Statement, Error> {
        self.consume()?;

        if !matches!(&self.curr_token.kind, TokenKind::Identifier(_)) {
            return Err(ParserError::new(
                UnexpectedToken,
                format!("expected identifier after `set`"),
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
                format!("expected `=`"),
                self.curr_token.line,
                self.curr_token.column,
            )
            .into());
        }
        self.consume()?;

        let expr = self.expr()?;
        Ok(Statement::Assignment(Assignment {
            iden: Identifier(iden),
            expr,
        }))
    }

    fn print_stmt(&mut self) -> Result<Statement, Error> {
        self.consume()?;

        let expr = self.expr()?;
        Ok(Statement::Print(Print(expr)))
    }

    fn if_stmt(&mut self) -> Result<Statement, Error> {
        self.consume()?;

        let if_cond = self.expr()?;

        if self.curr_token.kind != TokenKind::Colon {
            return Err(ParserError::new(
                UnidentifiedToken,
                format!("expected `:`"),
                self.curr_token.line,
                self.curr_token.column,
            )
            .into());
        }
        self.consume()?;
        self.skip_newlines()?;

        let mut if_stmts = Vec::new();
        while self.curr_token.kind != TokenKind::Else && self.curr_token.kind != TokenKind::EndIf {
            if self.curr_token.kind == TokenKind::EOF {
                return Err(ParserError::new(
                    UnexpectedToken,
                    format!("expected `endif` to close `if` statement"),
                    self.curr_token.line,
                    self.curr_token.column,
                )
                .into());
            }
            if_stmts.push(self.statement()?);
            self.skip_newlines()?;
        }

        let mut else_stmts = Vec::new();
        if self.curr_token.kind == TokenKind::Else {
            self.consume()?;

            if self.curr_token.kind != TokenKind::Colon {
                return Err(ParserError::new(
                    UnidentifiedToken,
                    format!("expected `:`"),
                    self.curr_token.line,
                    self.curr_token.column,
                )
                .into());
            }
            self.consume()?;
            self.skip_newlines()?;

            while self.curr_token.kind != TokenKind::EndIf {
                if self.curr_token.kind == TokenKind::EOF {
                    return Err(ParserError::new(
                        UnexpectedToken,
                        format!("expected `endif` to close `if else` statement"),
                        self.curr_token.line,
                        self.curr_token.column,
                    )
                    .into());
                }
                else_stmts.push(self.statement()?);
                self.skip_newlines()?;
            }
        }

        if self.curr_token.kind != TokenKind::EndIf {
            if self.curr_token.kind == TokenKind::EOF {
                return Err(ParserError::new(
                    UnidentifiedToken,
                    format!(
                        "expected `endif` to close `{}` statement",
                        if else_stmts.is_empty() {
                            "if"
                        } else {
                            "if else"
                        }
                    ),
                    self.curr_token.line,
                    self.curr_token.column,
                )
                .into());
            }

            return Err(ParserError::new(
                UnexpectedToken,
                format!("expected `endif`, found `{:?}`", self.curr_token.kind),
                self.curr_token.line,
                self.curr_token.column,
            )
            .into());
        }
        self.consume()?;

        let res = IfStmt {
            if_cond,
            if_stmts,
            else_stmts,
        };
        Ok(Statement::IfStmt(res))
    }

    fn while_stmt(&mut self) -> Result<Statement, Error> {
        self.consume()?;

        let while_cond = self.expr()?;

        if self.curr_token.kind != TokenKind::Colon {
            return Err(ParserError::new(
                UnexpectedToken,
                format!("expected `:`"),
                self.curr_token.line,
                self.curr_token.column,
            )
            .into());
        }
        self.consume()?;
        self.skip_newlines()?;

        let mut while_stmts = Vec::new();
        while self.curr_token.kind != TokenKind::EndWhile {
            if self.curr_token.kind == TokenKind::EOF {
                return Err(ParserError::new(
                    UnexpectedToken,
                    format!("expected `endwhile` to close `while` statement"),
                    self.curr_token.line,
                    self.curr_token.column,
                )
                .into());
            }
            while_stmts.push(self.statement()?);
            self.skip_newlines()?;
        }
        self.consume()?;

        let res = WhileStmt {
            while_cond,
            while_stmts,
        };
        Ok(Statement::WhileStmt(res))
    }

    fn expr(&mut self) -> Result<Expression, Error> {
        self.or_expr()
    }

    fn or_expr(&mut self) -> Result<Expression, Error> {
        let mut lhs = self.and_expr()?;

        while self.curr_token.kind == TokenKind::DPipe {
            self.consume()?;
            let rhs = self.and_expr()?;

            lhs = self.construct_bin_expr(BinaryOperators::Or, lhs, rhs)
        }

        Ok(lhs)
    }

    fn and_expr(&mut self) -> Result<Expression, Error> {
        let mut lhs = self.equality()?;

        while self.curr_token.kind == TokenKind::DAmpersand {
            self.consume()?;
            let rhs = self.equality()?;

            lhs = self.construct_bin_expr(BinaryOperators::And, lhs, rhs)
        }

        Ok(lhs)
    }

    fn equality(&mut self) -> Result<Expression, Error> {
        let mut lhs = self.comparison()?;

        while self.curr_token.kind == TokenKind::DEquals
            || self.curr_token.kind == TokenKind::NotEquals
        {
            let operator = self.consume()?;
            let rhs = self.comparison()?;

            match operator.kind {
                TokenKind::DEquals => {
                    lhs = self.construct_bin_expr(BinaryOperators::Equals, lhs, rhs)
                }
                TokenKind::NotEquals => {
                    lhs = self.construct_bin_expr(BinaryOperators::NotEquals, lhs, rhs)
                }
                _ => {
                    return Err(ParserError::new(
                        UnidentifiedToken,
                        format!("expected either `==` or `!=`"),
                        operator.line,
                        operator.column,
                    )
                    .into());
                }
            }
        }

        Ok(lhs)
    }

    fn comparison(&mut self) -> Result<Expression, Error> {
        let mut lhs = self.arithmetic()?;

        while self.curr_token.kind == TokenKind::LAngle
            || self.curr_token.kind == TokenKind::RAngle
            || self.curr_token.kind == TokenKind::LAngleEquals
            || self.curr_token.kind == TokenKind::RAngleEquals
        {
            let operator = self.consume()?;
            let rhs = self.arithmetic()?;

            match operator.kind {
                TokenKind::LAngle => {
                    lhs = self.construct_bin_expr(BinaryOperators::LThan, lhs, rhs)
                }
                TokenKind::RAngle => {
                    lhs = self.construct_bin_expr(BinaryOperators::GThan, lhs, rhs)
                }
                TokenKind::LAngleEquals => {
                    lhs = self.construct_bin_expr(BinaryOperators::LThanEquals, lhs, rhs)
                }

                TokenKind::RAngleEquals => {
                    lhs = self.construct_bin_expr(BinaryOperators::GThanEquals, lhs, rhs)
                }
                _ => {
                    return Err(ParserError::new(
                        UnidentifiedToken,
                        format!("expected either `<`, `>`, `<=` or `>=`"),
                        operator.line,
                        operator.column,
                    )
                    .into());
                }
            }
        }

        Ok(lhs)
    }

    fn arithmetic(&mut self) -> Result<Expression, Error> {
        let mut lhs = self.term()?;

        while self.curr_token.kind == TokenKind::Plus || self.curr_token.kind == TokenKind::Minus {
            let operator = self.consume()?;
            let rhs = self.term()?;

            match operator.kind {
                TokenKind::Plus => lhs = self.construct_bin_expr(BinaryOperators::Add, lhs, rhs),
                TokenKind::Minus => lhs = self.construct_bin_expr(BinaryOperators::Sub, lhs, rhs),
                _ => {
                    return Err(ParserError::new(
                        UnidentifiedToken,
                        format!("expected either `+` or `-`"),
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

        while self.curr_token.kind == TokenKind::Star
            || self.curr_token.kind == TokenKind::Slash
            || self.curr_token.kind == TokenKind::Percent
        {
            let operator = self.consume()?;
            let rhs = self.factor()?;

            match operator.kind {
                TokenKind::Star => lhs = self.construct_bin_expr(BinaryOperators::Mul, lhs, rhs),
                TokenKind::Slash => lhs = self.construct_bin_expr(BinaryOperators::Div, lhs, rhs),
                TokenKind::Percent => lhs = self.construct_bin_expr(BinaryOperators::Mod, lhs, rhs),
                _ => {
                    return Err(ParserError::new(
                        UnidentifiedToken,
                        format!("expecter either `*`, `/` or `%`"),
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
            TokenKind::Number(n) => {
                let res = Number(n);
                Ok(Expression::Number(res))
            }
            TokenKind::True => {
                let res = Boolean(true);
                Ok(Expression::Boolean(res))
            }
            TokenKind::False => {
                let res = Boolean(false);
                Ok(Expression::Boolean(res))
            }
            TokenKind::Identifier(i) => {
                let res = Identifier(i);
                Ok(Expression::Identifier(res))
            }
            TokenKind::LParen => {
                let res = self.expr()?;

                let rparen = self.consume()?;
                if rparen.kind != TokenKind::RParen {
                    return Err(ParserError::new(
                        MissingDelimiter,
                        format!("missing `)` after `(`"),
                        rparen.line,
                        rparen.column,
                    )
                    .into());
                }

                Ok(res)
            }
            TokenKind::Plus => {
                let res = UnaryOperation {
                    operator: UnaryOperators::Add,
                    operand: Box::new(self.factor()?),
                };
                Ok(Expression::UnaryOperation(res))
            }
            TokenKind::Minus => {
                let res = UnaryOperation {
                    operator: UnaryOperators::Sub,
                    operand: Box::new(self.factor()?),
                };
                Ok(Expression::UnaryOperation(res))
            }
            TokenKind::Exclaim => {
                let res = UnaryOperation {
                    operator: UnaryOperators::Not,
                    operand: Box::new(self.factor()?),
                };
                Ok(Expression::UnaryOperation(res))
            }
            _ => Err(ParserError::new(
                UnidentifiedToken,
                format!("expected expression"),
                token.line,
                token.column,
            )
            .into()),
        }
    }

    fn construct_bin_expr(
        &self,
        operator: BinaryOperators,
        lhs: Expression,
        rhs: Expression,
    ) -> Expression {
        let res = BinaryOperation {
            operator,
            left: Box::new(lhs),
            right: Box::new(rhs),
        };

        Expression::BinaryOperation(res)
    }
}
