use crease::lexer::Lexer;
use crease::parser::Parser;

fn main() {
    let src = "14 + 2 * 3 - 6 / 2";

    let lexer = Lexer::new(src);
    let mut parser = Parser::new(lexer);

    let result = parser.expr();
    println!("{src} = {result}");
}
