use crease::error::Error;
use crease::lexer::Lexer;
use crease::parser::Parser;

fn main() -> Result<(), Error> {
    let src = "2 +3)";

    let lexer = Lexer::new(src);
    let mut parser = Parser::new(lexer)?;

    let result = parser.parse()?;
    println!("{src} = {:?}", result);

    Ok(())
}
