mod lexer;
mod token_type;

use lexer::Lexer;

fn main() {
    
    let query: String = String::from("SELECT * FROM users WHERE name = 'Felipe'");
    let mut lexer = Lexer::new(query);
    let token;

    loop {
        token = lexer.next_token();

        if token.token_type == "EOF" {
            break;
        }
        println!("{}", token.repr());
        break;
    }
}