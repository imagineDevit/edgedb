use edgedb_protocol::common::Cardinality;
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
/// __cardinality__ : the query cardinality, MANY by default
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
///     let edge_query = EdgeQuery { query, args, cardinality: Cardinality:: One };
///
/// ```
///
#[derive(Debug)]
pub struct EdgeQuery {
    pub query: String,
    pub args: Option<Value>,
    pub cardinality: Cardinality
}

impl EdgeQuery {
    fn with_cardinality(self, cardinality: Cardinality) -> Self {
        let query = match cardinality {
            Cardinality::AtMostOne | Cardinality::One => {
                 format!("SELECT ({}) limit 1", self.query)
            }
            _ => {
                self.query
            }
        };

        Self {
            query,
            args: self.args,
            cardinality
        }
    }
}


/// ToEdgeQuery trait
pub trait ToEdgeQuery: ToEdgeQl + ToEdgeValue {

    /// Convert a given struct into a EdgeQuery struct
    fn to_edge_query(&self) -> EdgeQuery {
        EdgeQuery {
            query: self.to_edgeql(),
            args: Some(self.to_edge_value()),
            cardinality: Cardinality::Many
        }
    }

    fn to_edge_query_with_cardinality(&self, cardinality: Cardinality) -> EdgeQuery {
        self.to_edge_query().with_cardinality(cardinality)
    }
}
