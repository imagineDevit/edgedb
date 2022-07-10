/// <h2> ToEdgeQl </h2>
///
/// ToEdgeQl trait
pub trait ToEdgeQl {
    fn to_edgeql(&self) -> String;
}

// Implementations
impl ToEdgeQl for String {
    fn to_edgeql(&self) -> String {
        self.to_string()
    }
}
impl ToEdgeQl for u8 {
    fn to_edgeql(&self) -> String {
        self.to_string()
    }
}
impl ToEdgeQl for u16 {
    fn to_edgeql(&self) -> String {
        self.to_string()
    }
}
impl ToEdgeQl for u32 {
    fn to_edgeql(&self) -> String {
        self.to_string()
    }
}
impl ToEdgeQl for u64 {
    fn to_edgeql(&self) -> String {
        self.to_string()
    }
}
impl ToEdgeQl for i8 {
    fn to_edgeql(&self) -> String {
        self.to_string()
    }
}
impl ToEdgeQl for i16 {
    fn to_edgeql(&self) -> String {
        self.to_string()
    }
}
impl ToEdgeQl for i32 {
    fn to_edgeql(&self) -> String {
        self.to_string()
    }
}
impl ToEdgeQl for i64 {
    fn to_edgeql(&self) -> String {
        self.to_string()
    }
}
impl ToEdgeQl for f32 {
    fn to_edgeql(&self) -> String {
        self.to_string()
    }
}
impl ToEdgeQl for f64 {
    fn to_edgeql(&self) -> String {
        self.to_string()
    }
}
impl ToEdgeQl for bool {
    fn to_edgeql(&self) -> String {
        self.to_string()
    }
}
impl<T: ToString> ToEdgeQl for Vec<T> {
    fn to_edgeql(&self) -> String {
        let s = self
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(",");
        format!("[{}]", s)
    }
}
impl ToEdgeQl for serde_json::Value {
    fn to_edgeql(&self) -> String {
        self.to_string()
    }
}
impl ToEdgeQl for uuid::Uuid {
    fn to_edgeql(&self) -> String {
        self.to_string()
    }
}
