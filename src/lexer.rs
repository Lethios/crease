use crate::{
    error::{LexerError, LexerErrorKind::UnidentifiedCharacter},
    token::{
        Token,
        TokenKind::{self, *},
    },
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
                let mut seen_decimal = false;

                while let Some(d) = self.peek() {
                    if d.is_ascii_digit() {
                        self.consume();
                    } else if d == b'.' {
                        self.consume();
                        if !seen_decimal {
                            seen_decimal = true;
                        } else {
                            return Err(LexerError::new(
                                UnidentifiedCharacter,
                                (self.idx, self.idx),
                            ));
                        }
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

            b'=' => Ok(self.construct_token(Equals)),
            b';' => Ok(self.construct_token(Semicolon)),

            b'>' => {
                if self.input[self.idx] == b'>' {
                    self.consume();
                    return Ok(self.construct_token(Print));
                }

                Err(LexerError::new(UnidentifiedCharacter, (self.idx, self.idx)))
            }

            b'(' => Ok(self.construct_token(LParen)),
            b')' => Ok(self.construct_token(RParen)),

            b'+' => Ok(self.construct_token(Add)),
            b'-' => Ok(self.construct_token(Sub)),
            b'*' => Ok(self.construct_token(Mul)),
            b'/' => Ok(self.construct_token(Div)),

            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let start = self.idx - 1;

                while let Some(char) = self.peek() {
                    if char.is_ascii_alphanumeric() || char == b'_' {
                        self.consume();
                    } else {
                        break;
                    }
                }

                let end = self.idx - 1;

                let temp = str::from_utf8(&self.input[start..=end]).unwrap();
                let s = temp.parse::<String>().unwrap();

                let kind = if s == "set" { Set } else { Identifier(s) };
                Ok(Token {
                    kind,
                    span: (start, end),
                })
            }

            _ => Err(LexerError::new(UnidentifiedCharacter, (self.idx, self.idx))),
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

    fn construct_token(&self, kind: TokenKind) -> Token {
        Token {
            kind,
            span: (self.idx, self.idx),
        }
    }
}
