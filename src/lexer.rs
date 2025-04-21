use crate::token_type::TokenType;

pub struct Lexer {
	input: String,
	position: usize
}



impl Lexer {

	pub fn new(input: String) -> Self {

        let mut lexer = Lexer { input, position: 0 };
		lexer.read_char(); // inicializa o primeiro caractere
		lexer // retorna a instância da "classe"

    }


	pub fn tokenize(&mut self) -> Vec<TokenType>{

		let mut tokens = Vec::new();

		//enquanto houver tokens, adiciona no vetor
		while let Some(token) = self.next_token() {
			tokens.push(token);
		}

		tokens // retorna todos os vetores encontrados
	}

	// Pula espaços: skip_whitespace() avança sobre espaços, tabs, etc.
    // Verifica fim da string: Se chegamos ao final, retorna None
    // Pega o caractere atual
    // Decide o tipo de token:
    // 	Se for * → Retorna Token::Asterisk
    // 	Se for uma letra → Lê um identificador completo
    // 		Verifica se é uma palavra-chave ("SELECT", "FROM")
    // 		Se não for, trata como identificador comum
	// 		Se for outro caractere (não suportado), ignora e continua
	pub fn next_token(&mut self) -> Option<TokenType> {
		
		self.skip_whitespace();

		if self.position >= self.input.len() {
			return None; // Fim da string
		}

		let current_char = self.input.chars().nth(self.position)?;

		match current_char {

			'*' => {
				self.position += 1;
				Some(TokenType::Asterisk)
			}
			_ if current_char.is_alphabetic() => {

				let identifier = self.read_identifier();
				
				match identifier.to_uppercase().as_str() {
					"SELECT" => Some(TokenType::Select),
					"FROM" => Some(TokenType::From),
					_ => Some(TokenType::Identifier(identifier)),
				}
			}
       		_ => {
				self.position += 1;
				self.next_token()
			}
		}
	}


	fn read_identifier(&mut self) -> String {

		let start = self.position;

		while self.position < self.input.len() {

			let current_char = self.input.chars().nth(self.position).unwrap();
			if !current_char.is_alphabetic() {
				break;
			}
			self.position += 1;
		}
		self.input[start..self.position].to_string()
	}


	fn skip_whitespace(&mut self) {
		while self.position < self.input.len() {

			let current_char = self.input.chars().nth(self.position).unwrap();
			if !current_char.is_whitespace() {
				break;
			}
			self.position += 1;
    }
}


	// fn read_number(&mut self) -> String {
		
	// 	let start = self.position;

	// 	while let Some(c) = self.current_char {
    //         if c.is_digit(10) {
    //             self.read_char();
    //         } else { break;}
    //     }

	// 	self.input[start..self.position].to_string()
	// }


	pub fn check_grammar(&mut self) {

		let words: Vec<&str> = self.input.split_whitespace().collect();

		if words.is_empty() {
			print!("Empty query"); // do a format in TokenType (maybe) to return a proper display
		}

		match words[0] {

			"SELECT" => {

				if !words.contains(&"FROM") {
					eprintln!("Expected the keyword 'FROM' in a select like query.");
					process::exit(1);
				}
			},
			"UPDATE" => {

				if !words.contains(&"FROM") {
					eprintln!("Expected the keyword 'FROM' in a select like query.");
					process::exit(1);
				}
			},
			"INSERT" => {

				if !words.contains(&"INTO") {
					eprintln!("Expected the keyword 'INTO' in a insert like query");
					process::exit(1);
				}
			},
			"DELETE" => {

				if !words.contains(&"FROM") {
					eprintln!("Expected the keyword 'FROM' in a select like query.");
					process::exit(1);
				}
			},
			_ => {
					eprintln!("Unknow keyword: {}", words[0]);
					process::exit(1);
			}
		}

	}

}

// Funcionamento do lexer
// 1 Começa na posição 0

// 2 Analisa o caractere atual

// 3 Decide que tipo de token ele representa

// 4 Avança a posição

// 5 Repete até o final da string