use crate::ToEdgeQl;
use crate::ToEdgeValue;
use edgedb_protocol::value::Value;

/// EgdeQuery represents a edgeDB query.
///
/// It combines the string query and its query arguments
///
/// __query__ : the string query
///
/// __args__ : the query arguments
///
///<br>
///
/// ## Examples
///
///``` rust
///     use edgedb_protocol::codec::ObjectShape;
///     use edgedb_protocol::descriptors::{ShapeElement,  TypePos};
///     use edgedb_protocol::value::Value;
///     use edgedb_protocol::common::Cardinality;
///     use edgedb_query::models::edge_query::EdgeQuery;
///
///     let shape: &[ShapeElement] = &[ShapeElement {
///         flag_implicit: false,
///         flag_link_property: false,
///         flag_link: false,
///         cardinality: Some(Cardinality::One),
///         name: "var".to_string(),
///         type_pos: TypePos(0),
///     }];
///     
///     let args = Some(Value::Object {
///         shape: ObjectShape::from(shape),
///         fields: vec![Some(Value::Str(String::from("Rust")))],
///     });
///
///     let query = "Select 'I love ' ++ <str>$var".to_owned();
///
///     let edge_query = EdgeQuery { query, args };
///
/// ```
///
#[derive(Debug)]
pub struct EdgeQuery {
    pub query: String,
    pub args: Option<Value>,
}


/// ToEdgeQuery trait
pub trait ToEdgeQuery: ToEdgeQl + ToEdgeValue {

    /// Convert a given struct into a EdgeQuery struct
    fn to_edge_query(&self) -> EdgeQuery {
        EdgeQuery {
            query: self.to_edgeql(),
            args: Some(self.to_edge_value()),
        }
    }
}
