use crease::lexer::Lexer;
use crease::parser::Parser;

fn main() {
    let src = "-((((((2 + 3) * 4) - 10) / 5) + 7) * (3 + (2 * (4 - 1)))) - 20";

    let lexer = Lexer::new(src);
    let mut parser = Parser::new(lexer);

    let result = parser.expr();
    println!("{src} = {result}");
}
