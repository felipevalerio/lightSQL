pub struct Parser {
	tokens: Vec<TokenType>,
	position: usize,
}

impl Parser {

	pub fn parse(&mut self) -> Result<Statement, String> {
		self.parse_select_statement()
	}

	
	fn parse_select_statement(&mut self) -> Result<Statement, String> {

		self.expect(Token::SELECT)?; // Verifica se o primeiro token é SELECT
		let columns = self.parse_columns()?; // Parseia a lista de colunas (ou *)
		self.expect(Token::From)?; // Verifica se o próximo token é FROM

		// Parseia o nome da tabela
		let table = match self.next_token() {
			Some(Token::Identifier(name)) => name,
			_ => return Err("Expected table name after FROM".to_string()),
		};

		Ok(Statement::Select(SelectStatement { columns, table })) // Retorna a estrutura montada
	}
	
}


// Recebe os tokens do lexer

// Percorre os tokens sequencialmente

// Verifica se os tokens estão na ordem correta (ex: SELECT deve vir antes de FROM)

// Constrói os nós da AST conforme reconhece partes da query