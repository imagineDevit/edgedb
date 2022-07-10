use edgedb_protocol::model::Uuid;
use edgedb_protocol::value::Value;

pub trait ToEdgeValue {
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
