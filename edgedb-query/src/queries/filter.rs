
/// Filter trait represents a filter statment in a edgeDB select or update query
pub trait Filter {
    /// build the filter statment
    /// __table_name__ : the edgedb table name
    fn to_edgeql(&self, table_name: &str) -> String;

    /// build the args object
    fn to_edge_value(&self) -> edgedb_protocol::value::Value;
}