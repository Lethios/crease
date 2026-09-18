use crate::{
    error::{LexerError, LexerErrorKind::UnidentifiedCharacter},
    token::{Token, TokenKind::*},
};

pub struct Lexer<'a> {
    pub input: &'a [u8],
    pub idx: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.as_bytes(),
            idx: 0,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();
        let char = self.consume();

        let Some(char) = char else {
            return Ok(Token {
                kind: EOF,
                span: (self.idx, self.idx),
            });
        };

        match char {
            b'0'..=b'9' => {
                let start = self.idx - 1;

                while let Some(d) = self.peek() {
                    if d.is_ascii_digit() {
                        self.consume();
                    } else {
                        break;
                    }
                }
                let end = self.idx - 1;

                let temp = str::from_utf8(&self.input[start..=end]).unwrap();
                let digit = temp.parse::<f64>().unwrap();

                Ok(Token {
                    kind: Number(digit),
                    span: (start, end),
                })
            }

            b'(' => Ok(Token {
                kind: LParen,
                span: (self.idx, self.idx),
            }),
            b')' => Ok(Token {
                kind: RParen,
                span: (self.idx, self.idx),
            }),

            b'+' => Ok(Token {
                kind: Add,
                span: (self.idx, self.idx),
            }),
            b'-' => Ok(Token {
                kind: Sub,
                span: (self.idx, self.idx),
            }),
            b'*' => Ok(Token {
                kind: Mul,
                span: (self.idx, self.idx),
            }),
            b'/' => Ok(Token {
                kind: Div,
                span: (self.idx, self.idx),
            }),

            _ => Err(LexerError {
                kind: UnidentifiedCharacter,
                span: (self.idx, self.idx),
            }),
        }
    }

    fn consume(&mut self) -> Option<u8> {
        let char = self.input.get(self.idx).copied();
        if char.is_some() {
            self.idx += 1;
        }

        char
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.idx).copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(char) = self.peek() {
            if char.is_ascii_whitespace() {
                self.consume();
            } else {
                break;
            }
        }
    }
}
