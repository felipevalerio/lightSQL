
pub enum Node {

}

pub struct SelectStatement {

	columns: Vec<String>,
	table: Option<TableName>,
	where_clause: Option<WhereClause>
}

struct TableName {
	name: String
}

struct WhereClause {
	condition: String
}