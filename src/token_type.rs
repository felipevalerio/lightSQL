struct TokenType {
    type_: String,
    value: String,
}


impl TokenType {
	
    fn new(type_: String, value: String) -> Self {
        TokenType { type_, value }
    }

    fn repr(&self) -> String {
        format!("Token => {} => {}", self.type_, self.value)
    }
}