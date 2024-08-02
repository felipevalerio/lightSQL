pub enum TokenType {
    EOF,
    Number(String),
    String(String),
    Operator(char),
    Symbol(char),
    Unknown
}


// impl TokenType {
	
//     fn new(type_: String, value: String) -> Self {
//         TokenType { type_, value }
//     }

//     fn repr(&self) -> String {
//         format!("Token => {} => {}", self.type_, self.value)
//     }
// }