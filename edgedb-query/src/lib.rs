//! Edgedb-query crate aims to provide a bunch of traits or structs used
//! by Edgedb-query-derive crate

pub mod queries;
pub mod models;

use std::fmt::{Display, Formatter};
pub use models::edge_query::EdgeQuery;
pub use models::edge_query::ToEdgeQuery;
pub use models::query_result::BasicResult;
pub use queries::filter::Filter;
pub use queries::select::Options;
pub use queries::select::SelectOptions;
pub use queries::select::OrderDir;
pub use queries::select::OrderOptions;
pub use queries::select::PageOptions;

use edgedb_protocol::model::Uuid;
use edgedb_protocol::value::Value;
use crate::QueryType::{Delete, Insert, Select, Update};

macro_rules! _to_edgeql_and_to_edge_scalar_impls {
    ($($ty: ty => { scalar: $scalar: expr }),* $(,)?) => {
        $(
            impl ToEdgeQl for $ty {
                fn to_edgeql(&self) -> EdgeQl {
                    EdgeQl::new(self.to_string(), false)
                }
            }

            impl ToEdgeScalar for $ty {
                fn scalar() -> String {
                    $scalar.to_owned()
                }
            }

            impl ToEdgeShape for $ty {
                fn shape() -> String {
                    String::default()
                }
            }
        )*
    }
}


#[derive(Clone, Debug)]
pub enum QueryType {
    Insert,
    Select,
    Update,
    Delete,
    None,
}

impl Default for QueryType {
    fn default() -> Self {
        Self::None
    }
}

impl From<String> for QueryType {
    fn from(value: String) -> Self {
       match value.to_lowercase().trim() {
           "insert" => Insert,
           "select" => Select,
           "update" => Update,
           "delete" => Delete,
           _ => QueryType::None
       }
    }
}

impl Display for QueryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Insert => write!(f, "insert"),
            Select => write!(f, "select"),
            Update => write!(f, "update"),
            Delete => write!(f, "delete"),
            QueryType::None => write!(f, ""),
        }

    }
}

#[derive(Default, Debug)]
pub struct EdgeQl {
    pub table_name: String,
    pub query_type: QueryType,
    pub content: String,
    pub has_result: bool,
}

impl ToString for EdgeQl {
    fn to_string(&self) -> String {
        let s = match (self.content.clone().is_empty(), self.query_type.clone()) {
            (true, QueryType::None) => String::default(),
            (true, _) => format!("{} {}", self.query_type, self.table_name),
            (false, QueryType::None) => self.content.clone(),
            (false, _) => format!("{} {} {}", self.query_type, self.table_name, self.content),
        };

        if self.has_result {
            format!("select ( {s}")
        } else {
            s
        }
    }
}

impl EdgeQl {
    pub fn new(content: String, has_result: bool) -> Self {
        Self {
            table_name: String::default(),
            query_type: QueryType::None,
            content,
            has_result,
        }
    }

    pub fn detached(&mut self) -> Self {
        Self {
            table_name: format!("detached {}", self.table_name),
            query_type: self.query_type.clone(),
            content: self.content.clone(),
            has_result: self.has_result,
        }
    }
}


pub trait ToEdgeQl {
    /// Transform a struct into a edgeDB query language statement
    fn to_edgeql(&self) -> EdgeQl;
}

pub trait ToEdgeScalar {
    /// returns the cast expression corresponding to the self struct
    fn scalar() -> String;
}

pub trait ToEdgeShape {
    fn shape() -> String;
}

pub trait ToEdgeValue {
    /// Transform a struct data into a edgedb_protocol::value::Value
    fn to_edge_value(&self) -> Value;
}

pub trait EdgeResult {
    fn returning_fields() -> Vec<&'static str>;
}

_to_edgeql_and_to_edge_scalar_impls!(
    String => { scalar: "<str>" },
    i8 => { scalar: "<int16>" },
    u8 => { scalar: "<int16>" },
    i16 => { scalar: "<int16>" },
    u16 => { scalar: "<int16>" },
    i32 => { scalar: "<int32>" },
    u32 => { scalar: "<int32>" },
    i64 => { scalar: "<int64>" },
    u64 => { scalar: "<int64>" },
    f32 => { scalar: "<float32>" },
    f64 => { scalar: "<float64>" },
    bool => { scalar: "<bool>" },
    serde_json::Value => { scalar: "<json>" },
    uuid::Uuid => { scalar:"<uuid>"},
    chrono::DateTime<chrono::Utc> => { scalar: "<datetime>"},
    chrono::DateTime<chrono::Local> => { scalar: "<cal::local_datetime>"},
    chrono::Duration => { scalar : "<duration>"},
    chrono::Date<chrono::Local> => { scalar: "<cal::local_date>"},
    chrono::NaiveTime => { scalar: "<cal::local_time>"},
    chrono::NaiveDate => { scalar: "<cal::local_date>"},
);

impl ToEdgeScalar for () {
    fn scalar() -> String {
        "".to_owned()
    }
}

impl<T: ToEdgeQl> ToEdgeQl for Vec<T> {
    fn to_edgeql(&self) -> EdgeQl {
        let s = self
            .iter()
            .map(|s| s.to_edgeql().to_string())
            .collect::<Vec<String>>()
            .join(",");

        EdgeQl::new(format!("[{}]", s), false)
    }
}

impl<T: ToEdgeScalar + Default> ToEdgeScalar for Vec<T> {
    fn scalar() -> String {
        format!("<array{}>", T::scalar())
    }
}

impl<T> ToEdgeShape for Vec<T> {
    fn shape() -> String {
        String::default()
    }
}

impl ToEdgeValue for () {
    fn to_edge_value(&self) -> Value {
        Value::Nothing
    }
}

impl ToEdgeValue for String {
    fn to_edge_value(&self) -> Value {
        Value::Str(self.to_string())
    }
}

impl ToEdgeValue for i8 {
    fn to_edge_value(&self) -> Value {
        Value::Int16(*self as i16)
    }
}

impl ToEdgeValue for u8 {
    fn to_edge_value(&self) -> Value {
        Value::Int16(*self as i16)
    }
}

impl ToEdgeValue for i16 {
    fn to_edge_value(&self) -> Value {
        Value::Int16(*self)
    }
}

impl ToEdgeValue for u16 {
    fn to_edge_value(&self) -> Value {
        Value::Int16(*self as i16)
    }
}

impl ToEdgeValue for i32 {
    fn to_edge_value(&self) -> Value {
        Value::Int32(*self)
    }
}


impl ToEdgeValue for u32 {
    fn to_edge_value(&self) -> Value {
        Value::Int32(*self as i32)
    }
}

impl ToEdgeValue for i64 {
    fn to_edge_value(&self) -> Value {
        Value::Int64(*self)
    }
}


impl ToEdgeValue for u64 {
    fn to_edge_value(&self) -> Value {
        Value::Int64(*self as i64)
    }
}

impl ToEdgeValue for f32 {
    fn to_edge_value(&self) -> Value {
        Value::Float32(*self)
    }
}

impl ToEdgeValue for f64 {
    fn to_edge_value(&self) -> Value {
        Value::Float64(*self)
    }
}

impl ToEdgeValue for bool {
    fn to_edge_value(&self) -> Value {
        Value::Bool(*self)
    }
}

impl<T: ToEdgeValue> ToEdgeValue for Vec<T> {
    fn to_edge_value(&self) -> Value {
        Value::Array(
            self.iter()
                .map(|t| t.to_edge_value())
                .collect::<Vec<Value>>(),
        )
    }
}

impl ToEdgeValue for serde_json::Value {
    fn to_edge_value(&self) -> Value {
        Value::Json(self.to_string())
    }
}

impl ToEdgeValue for Uuid {
    fn to_edge_value(&self) -> Value {
        Value::Uuid(Uuid::from_u128(self.as_u128()))
    }
}
