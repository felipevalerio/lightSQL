use crate::token_type::TokenType;

struct Lexer {
	input: String,
	position: usize,
	read_position: usize,
	current_char: Option<char>
}



impl Lexer {

	fn new(input: String) -> Self {

        let mut lexer = Lexer { input, position: 0, read_position: 0, current_char: None };
		lexer.read_char(); // inicializa o primeiro caractere
		lexer // retorna a instância da "classe"

    }


	fn next_token(&mut self) -> TokenType {
		
		self.skip_whitespace();

		match self.current_char {
			
			None => TokenType::new("EOF", ""),

			Some(c) if c.is_alphabetic() => {
				let input = self.read_identifier();
				self.lookup_keyword(&input)
			},

			Some(c) if c.is_digit(10) => TokenType::new("NUMBER", self.read_number()),

			Some('\'') => TokenType::new("STRING", self.read_string()),

			Some(c) if "+-*/%=".contains(c) => {
				let operator = c.to_string();
				self.read_char();
				TokenType::new("STRING", &operator)
			},

			Some(c) if "(),;".contains(c) => {
				let symbol = c.to_string();
				self.read_char();
				TokenType::new("SYMBOL", &symbol)
			}

			_ => {
                self.read_char();
                TokenType::new("UNKNOWN", "")
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

	fn read_number() {
		unimplemented!();
	}

	fn lookup_keyword() {
		unimplemented!();
	}


	fn read_identifier() {
		unimplemented!();
	}

	fn read_string() {
		unimplemented!();
	}
}