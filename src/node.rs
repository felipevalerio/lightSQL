pub enum Node {
	Select{
		columns: Vec<String>,
		table_name: String,
	},
	Insert,
	Update,
	Delete,
}


pub fn select_statement() {

	for (key, value) in &tokenized_query {

		if(key.contains_key("KEYWORD") && value == "SELECT") {

			let mut columns = Vec::new();

			// if(key.)
		}
		else {
			eprintln!("Expected for SELECT keyword");
			process::exit(1);
		}
	}
	
}