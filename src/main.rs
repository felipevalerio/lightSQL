mod lexer;
mod token_type;

use std::io;
use lexer::Lexer;

fn main() {
    
    let mut token;
    let mut query: String = String::new();

    io::stdin().read_line(&mut query).expect("Erro na leitura");
    query = query.trim().to_uppercase();
    let mut lexer = Lexer::new(query);

    lexer.check_grammar();

    loop {
        token = lexer.next_token();

        if token.token_type == "EOF" {
            break;
        }
        println!("{}", token.repr());
    }
}