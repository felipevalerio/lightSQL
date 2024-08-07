mod lexer;
mod token_type;

use std::io;
use lexer::Lexer;

fn main() {
    
    let mut token;
    let mut query: String = String::new();

    io::stdin().read_line(&mut query).expect("Erro na leitura");
    let mut lexer = Lexer::new(query);


    loop {
        token = lexer.next_token();

        if token.token_type == "EOF" {
            break;
        }
        println!("{}", token.repr());
    }
}