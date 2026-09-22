use crease::error::Error;
use crease::interpreter::Interpreter;
use crease::lexer::Lexer;
use crease::parser::Parser;

fn main() {
    let src = r#"
set x = 4

if 3 == 3:
    set x = 2
    set y = 3
else:
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
