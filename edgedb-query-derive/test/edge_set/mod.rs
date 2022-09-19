
#[cfg(test)]
mod tests {
    use edgedb_protocol::codec::EnumValue;
    use edgedb_protocol::value::Value;
    use edgedb_query::{ToEdgeScalar, ToEdgeQl, ToEdgeValue};
    use edgedb_query_derive::{EdgedbSet, EdgedbEnum};
    use crate::test_utils::check_shape;

    #[derive(EdgedbSet)]
    pub struct MySet {
        #[field(column_name="first_name", assignment = "Concat")]
        #[scalar(type="str")]
        pub name: String,
        #[scalar(type="enum", name="State", module="default")]
        pub status: Status
    }

    #[derive(EdgedbEnum)]
    pub enum Status {
        Open, _Closed
    }

    #[test]
    pub fn test_set() {
        let set = MySet {
            name: "Joe".to_owned(),
            status: Status::Open
        };

        assert_eq!("set { first_name := .first_name ++ (select <str>$name), status := (select <default::State>$status)}", set.to_edgeql());

        if let Value::Object { shape, fields} = set.to_edge_value() {
            check_shape(&shape, vec!["name", "status"]);

            assert_eq!(fields, vec![
                Some(Value::Str(set.name)),
                Some(Value::Enum(EnumValue::from("Open")))
            ])
        }

    }

}