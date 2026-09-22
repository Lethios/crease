use std::fmt;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct LexerError {
    pub kind: LexerErrorKind,
    pub msg: String,
    pub line: usize,
    pub column: usize,
}

impl LexerError {
    pub fn new(kind: LexerErrorKind, msg: String, line: usize, column: usize) -> Self {
        Self {
            kind,
            msg,
            line,
            column,
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            LexerErrorKind::UnidentifiedCharacter => write!(f, "{}", self.msg),
            LexerErrorKind::UnexpectedCharacter => write!(f, "{}", self.msg),
            LexerErrorKind::InvalidNumber => write!(f, "{}", self.msg),
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

#[derive(Debug, Clone)]
pub struct ParserError {
    pub kind: ParserErrorKind,
    pub msg: String,
    pub line: usize,
    pub column: usize,
}

impl ParserError {
    pub fn new(kind: ParserErrorKind, msg: String, line: usize, column: usize) -> Self {
        Self {
            kind,
            msg,
            line,
            column,
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ParserErrorKind::MissingDelimiter => write!(f, "{}", self.msg),
            ParserErrorKind::UnidentifiedToken => write!(f, "{}", self.msg),
            ParserErrorKind::ExpectedStatement => write!(f, "{}", self.msg),
            ParserErrorKind::UnexpectedToken => write!(f, "{}", self.msg),
            ParserErrorKind::MissingSemicolon => write!(f, "{}", self.msg),
        }
    }
}

impl std::error::Error for ParserError {}

#[derive(Debug, Clone, Copy)]
pub enum RuntimeErrorKind {
    UndefinedVariable,
    TypeMismatch,
    DivisionByZero,
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub kind: RuntimeErrorKind,
    pub msg: String,
}

impl RuntimeError {
    pub fn new(kind: RuntimeErrorKind, msg: String) -> Self {
        Self { kind, msg }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            RuntimeErrorKind::UndefinedVariable => write!(f, "{}", self.msg),
            RuntimeErrorKind::TypeMismatch => write!(f, "{}", self.msg),
            RuntimeErrorKind::DivisionByZero => write!(f, "{}", self.msg),
        }
    }
}
