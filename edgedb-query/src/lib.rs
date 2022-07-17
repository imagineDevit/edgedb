
pub mod queries;
pub mod models;
use edgedb_protocol::model::Uuid;
use edgedb_protocol::value::Value;


//----- ToEdgeQL  ToEdgeScalar ----
macro_rules! _to_edgeql_and_to_edge_scalar_impls {
    ($($ty: ty => { scalar: $scalar: expr }),* $(,)?) => {
        $(
            impl ToEdgeQl for $ty {
                fn to_edgeql(&self) -> String {
                    self.to_string()
                }
            }
            impl ToEdgeScalar for $ty {
                fn to_edge_scalar(&self) -> String {
                    $scalar.to_owned()
                }
            }

            impl ToEdgeShape for $ty {
                fn to_edge_shape(&self) -> String {
                    String::default()
                }
            }
        )*
    }
}


///  ## ToEdgeQl
pub trait ToEdgeQl {

    /// Transform a struct into a edgeDB query language statement
    fn to_edgeql(&self) -> String;
}

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

pub trait ToEdgeShape {
    fn to_edge_shape(&self) -> String;
}

_to_edgeql_and_to_edge_scalar_impls!(
    String => { scalar: "<str>" },
    u8 => { scalar: "<int16>" },
    u16 => { scalar: "<int16>" },
    u32 => { scalar: "<int32>" },
    u64 => { scalar: "<int64>"  },
    i8 => { scalar: "<int16>" },
    i16 => { scalar: "<int16>" },
    i32 => { scalar: "<int32>" },
    i64 => { scalar: "<int64>" },
    f32 => { scalar: "<float32>" },
    f64 => { scalar: "<float64>" },
    bool => { scalar: "<bool>" },
    serde_json::Value => { scalar: "<json>" },
    uuid::Uuid => { scalar:"<uuid>"},
);

impl ToEdgeScalar for () {
    fn to_edge_scalar(&self) -> String {
        "".to_owned()
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

impl<T: ToEdgeScalar + Default> ToEdgeScalar for Vec<T> {
    fn to_edge_scalar(&self) -> String {
        format!("<array{}>", T::default().to_edge_scalar())
    }
}

impl<T> ToEdgeShape for Vec<T> {
    fn to_edge_shape(&self) -> String {
        String::default()
    }
}

//----- ToEdgeValue----

pub trait ToEdgeValue {
    /// Transform a struct data into a edgedb_protocol::value::Value
    fn to_edge_value(&self) -> Value;
}

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
