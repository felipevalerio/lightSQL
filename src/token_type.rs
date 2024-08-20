use std::fmt::{self, write};

pub struct TokenType {
    pub token_type: String,
    pub value: String,
}


impl TokenType {
	
    pub fn new(token_type: &str, value: &str) -> Self {
        TokenType { token_type: token_type.to_string(), value: value.to_string() }
    }

    pub fn repr(&self) -> String {
        format!("Token => {} => {}", self.token_type, self.value)
    }
}


impl fmt::Display for TokenType {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(f,"{}", self.value)
    }
}