use edgedb_protocol::value::Value;
use crate::{EdgeQl, ToEdgeQl, ToEdgeQuery, ToEdgeValue};


const UNLESS_CONFLICT: &str = " unless conflict ";
const ON: &str = "on ";
const OPEN_PARENTHESIS: &str = "( ";
const CLOSE_PARENTHESIS: &str = " ) ";
const COMMA: &str = ", ";
const ELSE: &str = "else ( ";

/// Conflict trait represents an 'unless conflict' statement in an edgeDB query
pub trait Conflict<T: ToEdgeQuery + Clone> {
    fn else_query(&self) -> Option<T>;
}

/// InsertConflict struct
#[derive(Debug, Clone)]
pub struct UnlessConflictElse<T: ToEdgeQuery> {
    pub else_query: T
}

impl<T: ToEdgeQuery + Clone> Conflict<T> for UnlessConflictElse<T> {
    fn else_query(&self) -> Option<T> {
        Some(self.else_query.clone())
    }
}


/// DefaultInsertConflict struct
#[derive(Debug, Clone)]
pub struct UnlessConflict;

#[derive(Clone)]
pub struct EmptyQuery;

impl ToEdgeQl for EmptyQuery {
    fn to_edgeql(&self) -> EdgeQl {
        EdgeQl::default()
    }
}

impl ToEdgeValue for EmptyQuery {
    fn to_edge_value(&self) -> Value {
        Value::Nothing
    }
}

impl ToEdgeQuery for EmptyQuery{}

impl Conflict<EmptyQuery> for UnlessConflict {
    fn else_query(&self) -> Option<EmptyQuery> {
        None
    }
}


/// parse a conflict into a string statement
///
/// ## Examples
///
/// ```
///use edgedb_protocol::value::Value;
///use edgedb_query::queries::conflict::{UnlessConflictElse, Conflict, parse_conflict};
///use edgedb_query::{ToEdgeQl, ToEdgeQuery, ToEdgeValue};
///
///#[derive(Clone)]
///pub struct FindUser {}
///
///impl ToEdgeQl for FindUser {
///   fn to_edgeql(&self) -> String {
///       format!("select users")
///   }
///}
///
///impl ToEdgeValue for FindUser {
///   fn to_edge_value(&self) -> Value {
///       Value::Nothing
///   }
///}
///
///impl ToEdgeQuery for FindUser  {}
///
///fn main() {
///   let insert_conflict = UnlessConflictElse {
///       else_query: Some(FindUser{}),
///   };
///
///   let stmt = parse_conflict(&insert_conflict, vec!["name", "age"]);
///
///    assert_eq!(stmt, " unless conflict on ( .name, .age ) else ( select users )");
/// }
/// ```
pub fn parse_conflict<T: ToEdgeQuery + Clone, R: Conflict<T>>(conflict: &R, on_fields: Vec<&str>) -> String {
    let mut stmt = UNLESS_CONFLICT.to_owned();

    if !on_fields.is_empty() {

        stmt.push_str(ON);

        if on_fields.len() > 1 {
            stmt.push_str(OPEN_PARENTHESIS);
        }

        stmt.push_str(on_fields
            .iter()
            .map(|s| format!(".{s}"))
            .collect::<Vec<String>>()
            .join(COMMA).as_str()
        );

        if on_fields.len() > 1 {
            stmt.push_str(CLOSE_PARENTHESIS);
        } else {
            stmt.push(' ');
        }
    }

    if let Some(else_query)= conflict.else_query() {
        stmt.push_str(ELSE);
        stmt.push_str(else_query.to_edgeql().to_string().as_str());
        stmt.push_str(CLOSE_PARENTHESIS);
    }

    stmt
}

