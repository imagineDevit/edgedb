pub trait ToScalar {
    fn to_scalar(&self) -> String;
}

impl ToScalar for () {
    fn to_scalar(&self) -> String {
        "".to_owned()
    }
}
impl ToScalar for String {
    fn to_scalar(&self) -> String {
        "<str>".to_owned()
    }
}
impl ToScalar for u8 {
    fn to_scalar(&self) -> String {
        "<int16>".to_owned()
    }
}
impl ToScalar for u16 {
    fn to_scalar(&self) -> String {
        "<int16>".to_owned()
    }
}
impl ToScalar for u32 {
    fn to_scalar(&self) -> String {
        "<int32>".to_owned()
    }
}
impl ToScalar for u64 {
    fn to_scalar(&self) -> String {
        "<int64>".to_owned()
    }
}
impl ToScalar for i8 {
    fn to_scalar(&self) -> String {
        "<int16>".to_owned()
    }
}
impl ToScalar for i16 {
    fn to_scalar(&self) -> String {
        "<int16>".to_owned()
    }
}
impl ToScalar for i32 {
    fn to_scalar(&self) -> String {
        "<int32>".to_owned()
    }
}
impl ToScalar for i64 {
    fn to_scalar(&self) -> String {
        "<int64>".to_owned()
    }
}
impl ToScalar for f32 {
    fn to_scalar(&self) -> String {
        "<float32>".to_owned()
    }
}
impl ToScalar for f64 {
    fn to_scalar(&self) -> String {
        "<float64>".to_owned()
    }
}
impl ToScalar for bool {
    fn to_scalar(&self) -> String {
        "<bool>".to_owned()
    }
}
impl ToScalar for serde_json::Value {
    fn to_scalar(&self) -> String {
        "<json>".to_owned()
    }
}
impl ToScalar for uuid::Uuid {
    fn to_scalar(&self) -> String {
        "<uuid>".to_owned()
    }
}

impl<T: ToScalar + Default> ToScalar for Vec<T> {
    fn to_scalar(&self) -> String {
        format!("<array{}>", T::default().to_scalar())
    }
}
