use crease::error::Error;
use crease::interpreter::Interpreter;
use crease::lexer::Lexer;
use crease::parser::Parser;

fn main() -> Result<(), Error> {
    let src = "(2 * (6 - (2 + 2))";
    let lexer = Lexer::new(src);

    let mut parser = Parser::new(lexer)?;
    let ast = parser.parse()?;

    let interpreter = Interpreter::new();
    let res = interpreter.interpret(&ast);

    println!("{src} = {}", res);

    Ok(())
}
