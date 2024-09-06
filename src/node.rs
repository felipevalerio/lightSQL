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

	if(tokenized_query.key == "KEYWORD" && tokenized_query.value == "SELECT") {

		let mut columns = Vec::new();

		if(tokenized_query.)
	}
	else {
		eprintln!("Expected for SELECT keyword");
		process::exit(1);
	}

}