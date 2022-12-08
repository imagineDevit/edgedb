use crate::ToEdgeQuery;


const UNLESS_CONFLICT: &'static str = " unless conflict ";
const ON: &'static str = "on ";
const OPEN_PARENTHESIS: &'static str = "( ";
const CLOSE_PARENTHESIS: &'static str = " )";
const COMMA: &'static str = ", ";
const ELSE: &'static str = " else ( ";

/// Conflict trait represents an 'unless conflict' statement in an edgeDB query
pub trait Conflict<T: ToEdgeQuery + Clone> {
    fn fields(&self) -> Option<Vec<&str>>;
    fn else_query(&self) -> Option<T>;
}

/// InsertConflict struct
pub struct InsertConflict<T: ToEdgeQuery> {
    pub fields: Option<Vec<&'static str>>,
    pub else_query: Option<T>
}

impl<T: ToEdgeQuery + Clone> Conflict<T> for InsertConflict<T> {

    fn fields(&self) -> Option<Vec<&str>> {
        self.fields.clone()
    }

    fn else_query(&self) -> Option<T> {
        self.else_query.clone()
    }
}


/// parse a conflict into a string statement
///
/// ## Examples
///
/// ```
///use edgedb_protocol::value::Value;
///use edgedb_query::queries::conflict::{InsertConflict, Conflict, parse_conflict};
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
///   let insert_conflict = InsertConflict {
///       fields: Some(vec!["name", "age"]),
///       else_query: Some(FindUser{}),
///   };
///
///   let stmt = parse_conflict(&insert_conflict, vec![]);
///
///    assert_eq!(stmt, " unless conflict on ( .name, .age ) else ( select users )");
/// }
/// ```
pub fn parse_conflict<T: ToEdgeQuery + Clone, R: Conflict<T>>(conflict: &R, query_fields: Vec<&str>) -> String {
    let mut stmt = UNLESS_CONFLICT.to_owned();

    if let Some(fields) = conflict.fields() {

        if fields.iter().any(|f| !query_fields.contains(f)) {
            panic!("Unless conflict fields must be one of : {:#?}", query_fields)
        }

        stmt.push_str(ON);

        if fields.len() > 1 {
            stmt.push_str(OPEN_PARENTHESIS);
        }
        stmt.push_str(fields
            .iter()
            .map(|s| format!(".{}", s))
            .collect::<Vec<String>>()
            .join(COMMA).as_str()
        );
        if fields.len() > 1 {
            stmt.push_str(CLOSE_PARENTHESIS);
        }

        if let Some(else_query)= conflict.else_query() {
            stmt.push_str(ELSE);
            stmt.push_str(else_query.to_edgeql().as_str());
            stmt.push_str(CLOSE_PARENTHESIS);
        }
    }

    stmt
}