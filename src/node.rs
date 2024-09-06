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

			if(key.contains_key("STRING")) {
				columns.push(value)
			} else {
				eprintln!("Expected columns after the SELECT keyword");
			}
		}
		else {
			eprintln!("Expected for the SELECT keyword");
			process::exit(1);
		}

		if (key.contains_key("KEYWORD") && value == "FROM") {
			
		}
	}
	
}