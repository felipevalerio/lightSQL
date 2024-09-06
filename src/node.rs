pub enum Node {
	Select {
		columns: Vec<String>,
		from_clause: String,
		where_clause: Option<Box<Node>>,
	},
	Insert {
		columns: Vec<String>,
		into_clause: String,
		
	}
}