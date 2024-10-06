use std::io;
use lexer::Lexer;
use std::collections::HashMap;

mod lexer;
mod token_type;



fn main() {
    
    let mut token;
    let mut query: String = String::new();

    io::stdin().read_line(&mut query).expect("Erro na leitura");
    query = query.trim().to_uppercase();
    let mut lexer = Lexer::new(query);
    let tokenized_query: HashMap<String, String> = HashMap::new();

    lexer.check_grammar();

    loop {
        token = lexer.next_token();

        if token.token_type == "EOF" {
            break;
        }
        token.add_hashset(tokenized_query.clone());
        println!("{}", token.repr());
    }
}