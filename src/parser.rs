pub struct Parser {
	tokens: Vec<TokenType>,
	position: usize,
}

impl Parser {

	pub fn parse(&mut self) -> Result<Statement, String> {
		self.parse_select_statement()
	}

	
	
}


// Recebe os tokens do lexer

// Percorre os tokens sequencialmente

// Verifica se os tokens estão na ordem correta (ex: SELECT deve vir antes de FROM)

// Constrói os nós da AST conforme reconhece partes da query