use crate::lexer::Lexer;
use crate::token::{Token, TokenKind::*};

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub curr_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let token = lexer.next_token();

        Self {
            lexer,
            curr_token: token,
        }
    }

    pub fn consume(&mut self) -> Token {
        let prev = self.curr_token;
        self.curr_token = self.lexer.next_token();

        prev
    }

    pub fn factor(&mut self) -> f64 {
        let res;
        let token = self.consume();

        match token.kind {
            Number(n) => {
                res = n;
            }
            LParen => {
                res = self.expr();
                let rparen = self.consume();

                if rparen.kind != RParen {
                    panic!("expected ')'");
                }
            }
            Sub => {
                res = -self.factor();
            }
            _ => panic!("unexpected token"),
        }

        res
    }

    pub fn term(&mut self) -> f64 {
        let mut res = self.factor();

        while self.curr_token.kind == Mul || self.curr_token.kind == Div {
            let operator = self.consume().kind;
            let rhs = self.factor();

            match operator {
                Mul => res *= rhs,
                Div => res /= rhs,
                _ => panic!("expected * or /"),
            }
        }

        res
    }

    pub fn expr(&mut self) -> f64 {
        let mut res = self.term();

        while self.curr_token.kind == Add || self.curr_token.kind == Sub {
            let operator = self.consume().kind;
            let rhs = self.term();

            match operator {
                Add => res += rhs,
                Sub => res -= rhs,
                _ => panic!("expected + or -"),
            }
        }

        res
    }
}
