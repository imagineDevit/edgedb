
pub mod queries;
pub mod models;

//----- ToEdgeQL----

///  ## ToEdgeQl
pub trait ToEdgeQl {

    /// Transform a struct into a edgeDB query language statement
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
impl<T: ToEdgeQl> ToEdgeQl for Vec<T> {
    fn to_edgeql(&self) -> String {
        let s = self
            .iter()
            .map(|s| s.to_edgeql())
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

//----- ToEdgeScalar----

/// ## ToEdgeScalar
pub trait ToEdgeScalar {
    /// returns the cast expression corresponding to the self struct
    ///
    /// <br>
    ///
    /// ## Examples
    ///
    /// ``` html
    ///     |  string  | <str>   |
    ///     ----------------------
    ///     |   u8     | <int16> |
    ///     ----------------------
    ///     |   bool   | <bool>  |
    /// ```
    fn to_edge_scalar(&self) -> String;
}

impl ToEdgeScalar for () {
    fn to_edge_scalar(&self) -> String {
        "".to_owned()
    }
}
impl ToEdgeScalar for String {
    fn to_edge_scalar(&self) -> String {
        "<str>".to_owned()
    }
}
impl ToEdgeScalar for u8 {
    fn to_edge_scalar(&self) -> String {
        "<int16>".to_owned()
    }
}
impl ToEdgeScalar for u16 {
    fn to_edge_scalar(&self) -> String {
        "<int16>".to_owned()
    }
}
impl ToEdgeScalar for u32 {
    fn to_edge_scalar(&self) -> String {
        "<int32>".to_owned()
    }
}
impl ToEdgeScalar for u64 {
    fn to_edge_scalar(&self) -> String {
        "<int64>".to_owned()
    }
}
impl ToEdgeScalar for i8 {
    fn to_edge_scalar(&self) -> String {
        "<int16>".to_owned()
    }
}
impl ToEdgeScalar for i16 {
    fn to_edge_scalar(&self) -> String {
        "<int16>".to_owned()
    }
}
impl ToEdgeScalar for i32 {
    fn to_edge_scalar(&self) -> String {
        "<int32>".to_owned()
    }
}
impl ToEdgeScalar for i64 {
    fn to_edge_scalar(&self) -> String {
        "<int64>".to_owned()
    }
}
impl ToEdgeScalar for f32 {
    fn to_edge_scalar(&self) -> String {
        "<float32>".to_owned()
    }
}
impl ToEdgeScalar for f64 {
    fn to_edge_scalar(&self) -> String {
        "<float64>".to_owned()
    }
}
impl ToEdgeScalar for bool {
    fn to_edge_scalar(&self) -> String {
        "<bool>".to_owned()
    }
}
impl ToEdgeScalar for serde_json::Value {
    fn to_edge_scalar(&self) -> String {
        "<json>".to_owned()
    }
}
impl ToEdgeScalar for uuid::Uuid {
    fn to_edge_scalar(&self) -> String {
        "<uuid>".to_owned()
    }
}
impl<T: ToEdgeScalar + Default> ToEdgeScalar for Vec<T> {
    fn to_edge_scalar(&self) -> String {
        format!("<array{}>", T::default().to_edge_scalar())
    }
}

//----- ToEdgeValue----
use edgedb_protocol::model::Uuid;
use edgedb_protocol::value::Value;

pub trait ToEdgeValue {
    /// Transform a struct data into a edgedb_protocol::value::Value
    fn to_edge_value(&self) -> Value;
}

// Implementations
impl ToEdgeValue for () {
    fn to_edge_value(&self) -> Value {
        Value::Str("".to_string())
    }
}
impl ToEdgeValue for String {
    fn to_edge_value(&self) -> Value {
        Value::Str(self.to_string())
    }
}
impl ToEdgeValue for u8 {
    fn to_edge_value(&self) -> Value {
        Value::Int16(*self as i16)
    }
}
impl ToEdgeValue for u16 {
    fn to_edge_value(&self) -> Value {
        Value::Int16(*self as i16)
    }
}
impl ToEdgeValue for u32 {
    fn to_edge_value(&self) -> Value {
        Value::Int32(*self as i32)
    }
}
impl ToEdgeValue for u64 {
    fn to_edge_value(&self) -> Value {
        Value::Int64(*self as i64)
    }
}
impl ToEdgeValue for i8 {
    fn to_edge_value(&self) -> Value {
        Value::Int16(*self as i16)
    }
}
impl ToEdgeValue for i16 {
    fn to_edge_value(&self) -> Value {
        Value::Int16(*self)
    }
}
impl ToEdgeValue for i32 {
    fn to_edge_value(&self) -> Value {
        Value::Int32(*self)
    }
}
impl ToEdgeValue for i64 {
    fn to_edge_value(&self) -> Value {
        Value::Int64(*self)
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
impl ToEdgeValue for uuid::Uuid {
    fn to_edge_value(&self) -> Value {
        Value::Uuid(Uuid::from_u128(self.as_u128()))
    }
}
