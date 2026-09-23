use crease::error::Error;
use crease::interpreter::Interpreter;
use crease::lexer::Lexer;
use crease::parser::Parser;

fn run() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();

    let src = std::fs::read_to_string(&args[1]).unwrap();

    let lexer = Lexer::new(&src);
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse()?;
    let mut interpreter = Interpreter::new();

    interpreter.interpret(&program)?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
    }
}
