use crate::{ToEdgeQl, ToEdgeShape};
use serde::Deserialize;
const STRUCT_ID: &'static str = "{ id }";

/// BasicResult represents the default edgeDB query result
#[derive(Default, Deserialize)]
pub struct BasicResult {
    pub id: String,
}

impl ToEdgeShape for BasicResult {
    fn shape() -> String {
        String::default()
    }
}

impl ToEdgeQl for BasicResult {
    fn to_edgeql(&self) -> String {
        STRUCT_ID.to_owned()
    }
}
