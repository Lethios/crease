use crease::error::Error;
use crease::interpreter::Interpreter;
use crease::lexer::Lexer;
use crease::parser::Parser;

fn main() {
    let src = r#"
in x

if x == "abc":
    out 0
else:
    out 1
endif
"#;

    let result = || -> Result<(), Error> {
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse()?;
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program)
    };

    if let Err(e) = result() {
        eprintln!("{e}");
    }
}
