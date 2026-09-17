use crate::token::{Token, TokenKind::*};

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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let char = self.consume();

        let Some(char) = char else {
            return Token { kind: EOF };
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

                return Token {
                    kind: Number(digit),
                };
            }

            b'+' => return Token { kind: Add },
            b'-' => return Token { kind: Sub },
            b'*' => return Token { kind: Mul },
            b'/' => return Token { kind: Div },

            _ => panic!("Unidentified token"),
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
