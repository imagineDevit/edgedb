use crate::to_edge_ql::ToEdgeQl;

#[derive(Default)]
pub struct BasicResult {
    pub id: String,
}

impl ToEdgeQl for BasicResult {
    fn to_edgeql(&self) -> String {
        "{ id }".to_owned()
    }
}
