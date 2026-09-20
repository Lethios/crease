use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    Lex(LexerError),
    Parse(ParserError),
    Runtime(RuntimeError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Lex(lex) => write!(
                f,
                "Lexer error at line {}, column {}: {}",
                lex.line, lex.column, lex
            ),
            Error::Parse(parse) => write!(
                f,
                "Parser error at line {}, column {}: {}",
                parse.line, parse.column, parse
            ),
            Error::Runtime(r) => write!(f, "Runtime error: {}", r),
        }
    }
}

impl From<LexerError> for Error {
    fn from(value: LexerError) -> Self {
        Error::Lex(value)
    }
}

impl From<ParserError> for Error {
    fn from(value: ParserError) -> Self {
        Error::Parse(value)
    }
}

impl From<RuntimeError> for Error {
    fn from(value: RuntimeError) -> Self {
        Error::Runtime(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LexerErrorKind {
    UnidentifiedCharacter,
    UnexpectedCharacter,
    InvalidNumber,
}

#[derive(Debug, Clone, Copy)]
pub struct LexerError {
    pub kind: LexerErrorKind,
    pub line: usize,
    pub column: usize,
}

impl LexerError {
    pub fn new(kind: LexerErrorKind, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            LexerErrorKind::UnidentifiedCharacter => write!(f, "unidentified character"),
            LexerErrorKind::UnexpectedCharacter => write!(f, "unexpected character"),
            LexerErrorKind::InvalidNumber => write!(f, "invalid number"),
        }
    }
}

impl std::error::Error for LexerError {}

#[derive(Debug, Clone, Copy)]
pub enum ParserErrorKind {
    MissingDelimiter,
    UnidentifiedToken,
    ExpectedStatement,
    UnexpectedToken,
    MissingSemicolon,
}

#[derive(Debug, Clone, Copy)]
pub struct ParserError {
    pub kind: ParserErrorKind,
    pub line: usize,
    pub column: usize,
}

impl ParserError {
    pub fn new(kind: ParserErrorKind, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ParserErrorKind::MissingDelimiter => write!(f, "missing delimiter"),
            ParserErrorKind::UnidentifiedToken => write!(f, "unidentified token"),
            ParserErrorKind::ExpectedStatement => write!(f, "expected statement"),
            ParserErrorKind::UnexpectedToken => write!(f, "unexpected token"),
            ParserErrorKind::MissingSemicolon => write!(f, "missing semicolon"),
        }
    }
}

impl std::error::Error for ParserError {}

#[derive(Debug, Clone, Copy)]
pub enum RuntimeErrorKind {
    UndefinedVariable,
}

#[derive(Debug, Clone, Copy)]
pub struct RuntimeError {
    pub kind: RuntimeErrorKind,
}

impl RuntimeError {
    pub fn new(kind: RuntimeErrorKind) -> Self {
        Self { kind }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            RuntimeErrorKind::UndefinedVariable => write!(f, "undefined variable"),
        }
    }
}
