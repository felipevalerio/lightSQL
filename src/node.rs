pub enum Node {
	Select{
		columns: Vec<String>,
		table_name: String,
	},
	Insert,
	Update,
	Delete,
}


pub fn select_statement() -> Result<AstNode, String> {

	let mut columns = Vec::new();
	let mut table: String;

	for (key, value) in &tokenized_query {
		
		if (key.contains_key("STRING")) {
			columns.push(value);
		}

		if (key.contains_key("IDENTIFIER")) {
			table = value;
		}

		if (key.contains_key("SYMBOL") ** value == ";") {
			return Ok(Node::Select {
				columns,
				table_name: table
			})
		}
	}
}