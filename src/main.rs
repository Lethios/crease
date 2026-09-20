use crease::error::Error;
use crease::interpreter::Interpreter;
use crease::lexer::Lexer;
use crease::parser::Parser;

fn main() {
    let src = r#"
set x = 5;
set y = 3;
set z = x % y;
out z;
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
