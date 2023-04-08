use crate::EdgeQl;

pub trait Sets {
    fn to_edgeql(&self) -> String;
    fn nested_edgeqls(&self) -> Vec<EdgeQl>;
    fn to_edge_value(&self) -> edgedb_protocol::value::Value;
}
