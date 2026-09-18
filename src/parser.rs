use crate::lexer::Lexer;
use crate::token::{Token, TokenKind::*};
use crate::{
    ast,
    ast::{BinaryOperators, Expression, UnaryOperators},
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let token = lexer.next_token();

        Self {
            lexer,
            curr_token: token,
        }
    }

    fn consume(&mut self) -> Token {
        let prev = self.curr_token;
        self.curr_token = self.lexer.next_token();

        prev
    }

    fn factor(&mut self) -> Expression {
        let token = self.consume();

        match token.kind {
            Number(n) => {
                let res = ast::Number(n);
                return Expression::Number(res);
            }
            LParen => {
                let res = self.expr();

                let rparen = self.consume();
                if rparen.kind != RParen {
                    panic!("expected ')'");
                }

                return res;
            }
            Sub => {
                let res = ast::UnaryOperation {
                    operator: UnaryOperators::Sub,
                    operand: Box::new(self.factor()),
                };
                return Expression::UnaryOperation(res);
            }
            _ => panic!("unexpected token"),
        }
    }

    fn term(&mut self) -> Expression {
        let mut lhs = self.factor();

        while self.curr_token.kind == Mul || self.curr_token.kind == Div {
            let operator = self.consume().kind;
            let rhs = self.factor();

            match operator {
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
                _ => panic!("expected * or /"),
            }
        }

        lhs
    }

    pub fn expr(&mut self) -> Expression {
        let mut lhs = self.term();

        while self.curr_token.kind == Add || self.curr_token.kind == Sub {
            let operator = self.consume().kind;
            let rhs = self.term();

            match operator {
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
                _ => panic!("expected + or -"),
            }
        }

        lhs
    }
}
