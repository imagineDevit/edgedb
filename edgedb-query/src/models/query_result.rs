use crate::ToEdgeQl;

const STRUCT_ID: &'static str = "{ id }";

/// ## Basic EdgeBD query result
///
/// When you made an edgedb query without mentioning the returning fields,
///
/// the query returns a structure containing just the id field.
#[derive(Default)]
pub struct BasicResult {
    pub id: String,
}

impl ToEdgeQl for BasicResult {
    fn to_edgeql(&self) -> String {
        STRUCT_ID.to_owned()
    }
}
