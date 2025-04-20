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


	fn skip_whitespace(&mut self) {
		// map_or tem um valor padrão "False", mas se o current_char for diferente de None, ele atribue o valor dado "True"
        while self.current_char.map_or(false, char::is_whitespace) {
            self.read_char();
        }
    }


	fn read_char(&mut self) {

		if self.read_position >= self.input.len() {
			self.current_char = None;
		} else {
			self.current_char = self.input[self.read_position..].chars().next();
		}

		self.position = self.read_position;
		self.read_position += 1;
	}


	fn read_number(&mut self) -> String {
		
		let start = self.position;

		while let Some(c) = self.current_char {
            if c.is_digit(10) {
                self.read_char();
            } else { break;}
        }

		self.input[start..self.position].to_string()
	}

	// keywords primeiramente tem valores do tipo &str
	// depois é feita a iteração nos valores (iter)
	// transforma cada um dos valores e converte eles para String (map)
	// e depois transforma em uma collections e o _ no HashSet
	// pede para o compilador inferir o novo tipo do HashSet, que será String
	fn lookup_keyword(&mut self, input: &str) -> TokenType {

		let keywords: HashSet<_> = ["SELECT", "INSERT", "UPDATE", "DELETE", "FROM", "WHERE"]
            .iter()
            .map(|&s| s.to_string())
            .collect();

        if keywords.contains(&input.to_uppercase()) {
            TokenType::new("KEYWORD", input)
        } else {
            TokenType::new("IDENTIFIER", input)
        }
	}


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


	fn read_identifier(&mut self) -> String {
		let start = self.position;
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                self.read_char();
            } else {
                break;
            }
        }
        self.input[start..self.position].to_string()
	}


	fn read_string(&mut self) -> String {
		self.read_char();
        let start = self.position;
        while let Some(c) = self.current_char {
            if c != '\'' {
                self.read_char();
            } else {
                break;
            }
        }
        let result = self.input[start..self.position].to_string();
        self.read_char();
        result
	}

}

// Funcionamento do lexer
// 1 Começa na posição 0

// 2 Analisa o caractere atual

// 3 Decide que tipo de token ele representa

// 4 Avança a posição

// 5 Repete até o final da string