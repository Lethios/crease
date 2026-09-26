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

    fn expect_identifier(&mut self, msg: &str) -> Result<String, Error> {
        let (line, column) = (self.curr_token.line, self.curr_token.column);

        match self.consume()?.kind {
            TokenKind::Identifier(value) => Ok(value),
            _ => Err(ParserError::new(UnexpectedToken, msg.to_string(), line, column).into()),
        }
    }

    fn expect_token(&mut self, kind: TokenKind, msg: &str) -> Result<Token, Error> {
        if self.curr_token.kind != kind {
            return Err(ParserError::new(
                UnexpectedToken,
                msg.to_string(),
                self.curr_token.line,
                self.curr_token.column,
            )
            .into());
        }

        self.consume()
    }

    fn construct_un_expr(&self, operator: UnaryOperators, operand: Expression) -> Expression {
        let res = UnaryOperation {
            operator,
            operand: Box::new(operand),
        };

        Expression::UnaryOperation(res)
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
            TokenKind::Set => self.declare_stmt()?,
            TokenKind::Identifier(_) => self.assign_stmt()?,
            TokenKind::Del => self.delete_stmt()?,
            TokenKind::In => self.input_stmt()?,
            TokenKind::Out => self.print_stmt()?,
            TokenKind::If => self.if_stmt()?,
            TokenKind::While => self.while_stmt()?,
            _ => {
                return Err(ParserError::new(
                    ExpectedStatement,
                    ("expected statement").to_string(),
                    self.curr_token.line,
                    self.curr_token.column,
                )
                .into());
            }
        };

        if self.curr_token.kind != TokenKind::Newline && self.curr_token.kind != TokenKind::EOF {
            return Err(ParserError::new(
                UnexpectedToken,
                ("expected newline or end of file after statement").to_string(),
                self.curr_token.line,
                self.curr_token.column,
            )
            .into());
        }

        Ok(res)
    }

    fn declare_stmt(&mut self) -> Result<Statement, Error> {
        self.consume()?;

        let value = self.expect_identifier("expected identifier after `set`")?;
        self.expect_token(TokenKind::Equals, "expected `=`")?;
        let expr = self.expr()?;

        Ok(Statement::DeclareStmt(DeclareStmt {
            iden: Identifier { value },
            expr,
        }))
    }

    fn assign_stmt(&mut self) -> Result<Statement, Error> {
        let value = self.expect_identifier("expected identifier")?;

        if self.curr_token.kind != TokenKind::Equals {
            self.expect_token(TokenKind::LBracket, "expected `[`")?;
            let idx = self.expr()?;
            self.expect_token(TokenKind::RBracket, "expected `]`")?;

            self.expect_token(TokenKind::Equals, "expected `=`")?;
            let expr = self.expr()?;

            return Ok(Statement::IndexAssignmentStmt(IndexAssignmentStmt {
                iden: Identifier { value },
                idx,
                expr,
            }));
        }

        self.expect_token(TokenKind::Equals, "expected `=`")?;
        let expr = self.expr()?;

        Ok(Statement::AssignmentStmt(AssignmentStmt {
            iden: Identifier { value },
            expr,
        }))
    }

    fn delete_stmt(&mut self) -> Result<Statement, Error> {
        self.consume()?;

        let value = self.expect_identifier("expected identifier")?;

        Ok(Statement::DeleteStmt(DeleteStmt {
            iden: Identifier { value },
        }))
    }

    fn input_stmt(&mut self) -> Result<Statement, Error> {
        self.consume()?;

        let value = self.expect_identifier("expected identifier")?;

        Ok(Statement::InputStmt(InputStmt {
            iden: Identifier { value },
        }))
    }

    fn print_stmt(&mut self) -> Result<Statement, Error> {
        self.consume()?;

        let expr = self.expr()?;

        Ok(Statement::PrintStmt(PrintStmt { expr }))
    }

    fn if_stmt(&mut self) -> Result<Statement, Error> {
        self.consume()?;

        let if_cond = self.expr()?;

        self.expect_token(TokenKind::Colon, "expected `:`")?;
        self.skip_newlines()?;

        let mut if_stmts = Vec::new();
        while self.curr_token.kind != TokenKind::Else && self.curr_token.kind != TokenKind::EndIf {
            if self.curr_token.kind == TokenKind::EOF {
                return Err(ParserError::new(
                    UnexpectedToken,
                    ("expected `endif` to close `if` statement").to_string(),
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

            self.expect_token(TokenKind::Colon, "expected `:`")?;
            self.skip_newlines()?;

            while self.curr_token.kind != TokenKind::EndIf {
                if self.curr_token.kind == TokenKind::EOF {
                    return Err(ParserError::new(
                        UnexpectedToken,
                        ("expected `endif` to close `if else` statement").to_string(),
                        self.curr_token.line,
                        self.curr_token.column,
                    )
                    .into());
                }
                else_stmts.push(self.statement()?);
                self.skip_newlines()?;
            }
        }

        self.expect_token(TokenKind::EndIf, "expected `endif`")?;

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

        self.expect_token(TokenKind::Colon, "expected `:`")?;
        self.skip_newlines()?;

        let mut while_stmts = Vec::new();
        while self.curr_token.kind != TokenKind::EndWhile {
            if self.curr_token.kind == TokenKind::EOF {
                return Err(ParserError::new(
                    UnexpectedToken,
                    ("expected `endwhile` to close `while` statement").to_string(),
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
                        ("expected either `==` or `!=`").to_string(),
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
                        ("expected either `<`, `>`, `<=` or `>=`").to_string(),
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
                        ("expected either `+` or `-`").to_string(),
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
                        ("expected either `*`, `/` or `%`").to_string(),
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
            TokenKind::IntegerLiteral(int) => {
                Ok(Expression::IntegerLiteral(IntegerLiteral { int }))
            }
            TokenKind::FloatLiteral(float) => Ok(Expression::FloatLiteral(FloatLiteral { float })),
            TokenKind::True => Ok(Expression::BooleanLiteral(BooleanLiteral { boolean: true })),
            TokenKind::False => Ok(Expression::BooleanLiteral(BooleanLiteral {
                boolean: false,
            })),
            TokenKind::StringLiteral(string) => {
                Ok(Expression::StringLiteral(StringLiteral { string }))
            }
            TokenKind::Identifier(value) => {
                if self.curr_token.kind != TokenKind::LBracket {
                    return Ok(Expression::Identifier(Identifier { value }));
                }

                self.expect_token(TokenKind::LBracket, "expected `[`")?;
                let expr = self.expr()?;
                self.expect_token(TokenKind::RBracket, "expected `]`")?;

                Ok(Expression::ArrayIndexing(ArrayIndexing {
                    iden: Identifier { value },
                    idx: Box::new(expr),
                }))
            }
            TokenKind::LBracket => Ok(self.array()?),
            TokenKind::LParen => {
                let res = self.expr()?;

                let rparen = self.consume()?;
                if rparen.kind != TokenKind::RParen {
                    return Err(ParserError::new(
                        MissingDelimiter,
                        ("missing `)` after `(`").to_string(),
                        rparen.line,
                        rparen.column,
                    )
                    .into());
                }

                Ok(res)
            }
            TokenKind::Plus => {
                let operand = self.factor()?;
                Ok(self.construct_un_expr(UnaryOperators::Add, operand))
            }
            TokenKind::Minus => {
                let operand = self.factor()?;
                Ok(self.construct_un_expr(UnaryOperators::Sub, operand))
            }
            TokenKind::Exclaim => {
                let operand = self.factor()?;
                Ok(self.construct_un_expr(UnaryOperators::Not, operand))
            }
            TokenKind::Int => {
                let operand = self.factor()?;
                Ok(self.construct_un_expr(UnaryOperators::ToInt, operand))
            }
            TokenKind::Float => {
                let operand = self.factor()?;
                Ok(self.construct_un_expr(UnaryOperators::ToFloat, operand))
            }
            TokenKind::Bool => {
                let operand = self.factor()?;
                Ok(self.construct_un_expr(UnaryOperators::ToBool, operand))
            }
            TokenKind::Str => {
                let operand = self.factor()?;
                Ok(self.construct_un_expr(UnaryOperators::ToStr, operand))
            }
            _ => Err(ParserError::new(
                UnidentifiedToken,
                ("expected expression").to_string(),
                token.line,
                token.column,
            )
            .into()),
        }
    }

    fn array(&mut self) -> Result<Expression, Error> {
        self.expect_token(TokenKind::LBracket, "expected `[`")?;

        let mut arr: Vec<Expression> = Vec::new();
        if self.curr_token.kind != TokenKind::RBracket {
            let expr = self.expr()?;
            arr.push(expr);

            while self.curr_token.kind != TokenKind::RBracket {
                self.expect_token(TokenKind::Comma, "expected `,`")?;
                let expr = self.expr()?;
                arr.push(expr);
            }
        }
        self.expect_token(TokenKind::RBracket, "expected `]`")?;

        Ok(Expression::ArrayLiteral(ArrayLiteral { arr }))
    }
}
