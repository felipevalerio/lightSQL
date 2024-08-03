mod lexer;
mod token_type;

fn main() {
    
    let query: String = String::from("SELECT * FROM users WHERE name = 'Felipe'");
    let lexer = lexer::new(query);
    let token;

    while true {
        token = lexer.next_token();

        if token.token_type == 'EOF' {
            break;
        }
        print!("{}", token)
    }
}