use crate::to_edge_ql::ToEdgeQl;
use crate::to_edge_value::ToEdgeValue;
use edgedb_protocol::value::Value;

/// <h2> EdgeQuery </h2>
///
/// EgdeQuery represents a sql query string associated to the query arguments
///
/// __query__ : the string query
///
/// __args__ : the query arguments
///
///
///<br>
/// <h3 style="text-decoration: underline"> Usage : </h3>
///
///``` rust
///     use edgedb_protocol::codec::ObjectShape;
///     use edgedb_protocol::descriptors::{ShapeElement,  TypePos};
///     use edgedb_protocol::value::Value;
///     use edgedb_protocol::common::Cardinality;
///     use edgedb_query::edge_query::EdgeQuery;
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

/// <h2> ToEdgeQuery </h2>
///
/// ToEdgeQuery trait
pub trait ToEdgeQuery: ToEdgeQl + ToEdgeValue {
    ///<h3>to_edge_query</h3>
    /// Transform the self object into a EgdeQuery object
    fn to_edge_query(&self) -> EdgeQuery {
        EdgeQuery {
            query: self.to_edgeql(),
            args: Some(self.to_edge_value()),
        }
    }
}
