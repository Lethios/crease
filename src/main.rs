use crease::error::Error;
use crease::interpreter::Interpreter;
use crease::lexer::Lexer;
use crease::parser::Parser;

fn main() {
    let src = "2 + *";
    let lexer = Lexer::new(src);

    let result = || -> Result<f64, Error> {
        let mut parser = Parser::new(lexer)?;
        let ast = parser.parse()?;
        let interpreter = Interpreter::new();
        let res = interpreter.interpret(&ast);

        Ok(res)
    };

    let res = result();

    match res {
        Ok(res) => println!("{src} = {res}"),
        Err(e) => eprintln!("{e}"),
    }
}
