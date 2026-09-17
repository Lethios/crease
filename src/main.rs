use crate::lexer::*;
use crate::token::*;

pub mod lexer;
pub mod token;

fn main() {
    let src = " 34+2+-40+20";
    let mut lexer = Lexer::new(src);

    loop {
        let token = lexer.next_token();

        if token.kind == TokenKind::EOF {
            break;
        }

        println!("{:?}", token);
    }
}
