
pub enum TokenType {
    Select,
    From,
    Asterisk,
    Identifier(String), // nome da tabela
    EOF,
}
