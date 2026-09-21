use crease::error::Error;
use crease::interpreter::Interpreter;
use crease::lexer::Lexer;
use crease::parser::Parser;

fn main() {
    let src = r#"
if 2 == 2:
    set x = true
else:
    set x = false
endif

out x
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
