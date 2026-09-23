use crate::error::{LexerError, LexerErrorKind::*};
use crate::token::{
    Token,
    TokenKind::{self, *},
};

pub struct Lexer<'a> {
    pub input: &'a [u8],
    pub idx: usize,
    pub line: usize,
    pub column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.as_bytes(),
            idx: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        loop {
            self.skip_whitespace();

            let start_line = self.line;
            let start_col = self.column;

            let char = self.consume();

            let Some(char) = char else {
                return Ok(self.construct_token(EOF, start_line, start_col));
            };

            match char {
                b'0'..=b'9' => {
                    let start = self.idx - 1;
                    let mut seen_decimal = false;

                    while let Some(d) = self.peek() {
                        if d.is_ascii_digit() {
                            self.consume();
                        } else if d == b'.' {
                            if seen_decimal {
                                // flag double decimal point 0..
                                return Err(LexerError::new(
                                    UnexpectedCharacter,
                                    ("expected digit after `.`").to_string(),
                                    self.line,
                                    self.column,
                                ));
                            }
                            self.consume();
                            seen_decimal = true;
                        } else {
                            break;
                        }
                    }
                    let end = self.idx - 1;

                    if self.input.get(end) == Some(&b'.') {
                        // reject number ending with .
                        return Err(LexerError::new(
                            InvalidNumber,
                            ("expected digit after `.`").to_string(),
                            start_line,
                            start_col,
                        ));
                    }

                    #[expect(
                        clippy::unwrap_in_result,
                        clippy::unwrap_used,
                        clippy::indexing_slicing,
                        reason = "&str guarantees utf8"
                    )]
                    let temp = str::from_utf8(&self.input[start..=end]).unwrap();
                    #[expect(
                        clippy::unwrap_used,
                        clippy::unwrap_in_result,
                        reason = "valid f64 is guaranteed"
                    )]
                    let num = temp.parse::<f64>().unwrap();

                    break Ok(self.construct_token(Number(num), start_line, start_col));
                }

                b'=' => {
                    if let Some(char) = self.peek()
                        && char == b'='
                    {
                        self.consume();
                        break Ok(self.construct_token(DEquals, start_line, start_col));
                    }
                    break Ok(self.construct_token(Equals, start_line, start_col));
                }

                b'(' => break Ok(self.construct_token(LParen, start_line, start_col)),
                b')' => break Ok(self.construct_token(RParen, start_line, start_col)),

                b'+' => break Ok(self.construct_token(Plus, start_line, start_col)),
                b'-' => break Ok(self.construct_token(Minus, start_line, start_col)),
                b'*' => break Ok(self.construct_token(Star, start_line, start_col)),
                b'/' => break Ok(self.construct_token(Slash, start_line, start_col)),
                b'%' => break Ok(self.construct_token(Percent, start_line, start_col)),

                b'<' => {
                    if let Some(char) = self.peek()
                        && char == b'='
                    {
                        self.consume();
                        break Ok(self.construct_token(LAngleEquals, start_line, start_col));
                    }

                    break Ok(self.construct_token(LAngle, start_line, start_col));
                }
                b'>' => {
                    if let Some(char) = self.peek()
                        && char == b'='
                    {
                        self.consume();
                        break Ok(self.construct_token(RAngleEquals, start_line, start_col));
                    }

                    break Ok(self.construct_token(RAngle, start_line, start_col));
                }
                b'!' => {
                    if let Some(char) = self.peek()
                        && char == b'='
                    {
                        self.consume();
                        break Ok(self.construct_token(NotEquals, start_line, start_col));
                    }

                    break Ok(self.construct_token(Exclaim, start_line, start_col));
                }
                b'&' => {
                    if let Some(char) = self.peek()
                        && char == b'&'
                    {
                        self.consume();
                        break Ok(self.construct_token(DAmpersand, start_line, start_col));
                    } else {
                        break Err(LexerError::new(
                            UnidentifiedCharacter,
                            ("expected `&&`").to_string(),
                            start_line,
                            start_col,
                        ));
                    }
                }
                b'|' => {
                    if let Some(char) = self.peek()
                        && char == b'|'
                    {
                        self.consume();
                        break Ok(self.construct_token(DPipe, start_line, start_col));
                    } else {
                        break Err(LexerError::new(
                            UnidentifiedCharacter,
                            ("expected `||`").to_string(),
                            start_line,
                            start_col,
                        ));
                    }
                }

                b':' => break Ok(self.construct_token(Colon, start_line, start_col)),

                b'"' => {
                    // store idx of char after "
                    let start = self.idx;
                    let mut end = self.idx;

                    while let Some(char) = self.peek() {
                        if char == b'"' {
                            end = self.idx;
                            break;
                        } else {
                            self.consume();
                        }
                    }

                    if let Some(char) = self.peek()
                        && char == b'"'
                    {
                        self.consume();
                    } else {
                        break Err(LexerError::new(
                            InvalidString,
                            ("unterminated string").to_string(),
                            start_line,
                            start_col,
                        ));
                    }

                    #[expect(
                        clippy::unwrap_in_result,
                        clippy::unwrap_used,
                        clippy::indexing_slicing,
                        reason = "&str guarantees utf8"
                    )]
                    let temp = str::from_utf8(&self.input[start..end]).unwrap();
                    let string = temp.to_string();

                    break Ok(self.construct_token(String(string), start_line, start_col));
                }

                b'#' => {
                    while let Some(char) = self.peek() {
                        if char != b'\n' {
                            self.consume();
                        } else {
                            break;
                        }
                    }

                    continue;
                }

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
                    #[expect(
                        clippy::unwrap_in_result,
                        clippy::unwrap_used,
                        clippy::indexing_slicing,
                        reason = "&str guarantees utf8"
                    )]
                    let s = str::from_utf8(&self.input[start..=end]).unwrap();

                    let kind = match s {
                        "set" => Set,
                        "out" => Out,
                        "if" => If,
                        "else" => Else,
                        "endif" => EndIf,
                        "while" => While,
                        "endwhile" => EndWhile,
                        "true" => True,
                        "false" => False,
                        _ => Identifier(s.to_string()),
                    };

                    break Ok(self.construct_token(kind, start_line, start_col));
                }

                b'\n' => {
                    let newline = self.construct_token(Newline, start_line, start_col);
                    self.line += 1;
                    self.column = 1;

                    break Ok(newline);
                }

                _ => {
                    break Err(LexerError::new(
                        UnidentifiedCharacter,
                        format!("unidentified character {:?}", char as char),
                        start_line,
                        start_col,
                    ));
                }
            }
        }
    }

    fn consume(&mut self) -> Option<u8> {
        let char = self.input.get(self.idx).copied();
        if char.is_some() {
            self.idx += 1;
            self.column += 1;
        }

        char
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.idx).copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(char) = self.peek() {
            if char != b'\n' && char.is_ascii_whitespace() {
                self.consume();
            } else {
                break;
            }
        }
    }

    fn construct_token(&self, kind: TokenKind, line: usize, column: usize) -> Token {
        Token { kind, line, column }
    }
}
