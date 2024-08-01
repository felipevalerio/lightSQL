

struct Lexer {
	input: String,
	position: usize,
	read_position: usize,
	current_char: Option<char>
}



impl Lexer {

	fn new(input: String, position: usize, read_position: usize, current_char: Option<char>) -> Self {

        let mut lexer = Lexer { input, position: 0, read_position: 0, current_char: None };
		lexer.read_char(); // inicializa o primeiro caractere
		lexer // retorna a instância da "classe"

    }


	fn skip_whitespace(&mut self) {

		while self.current_char != None && self.current_char.is_whitespace() {
			self.read_char()
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
}