
#[cfg(test)]
mod tests {
    use edgedb_protocol::codec::EnumValue;
    use edgedb_protocol::value::Value;
    use edgedb_query::{ToEdgeValue, ToEdgeQl};
    use edgedb_query_derive::{EdgedbSet, EdgedbEnum, SelectQuery};
    use crate::test_utils::check_shape;

    #[derive(EdgedbSet)]
    pub struct MySet {
        #[field(column_name="first_name", assignment = "Concat")]
        #[scalar(type="str")]
        #[param("user_name")]
        pub name: String,
        #[scalar(type="enum", name="State", module="default")]
        pub status: Status,
        #[nested_query]
        pub users: FindUsers
    }

    #[derive(EdgedbEnum)]
    pub enum Status {
        Open, _Closed
    }

    #[derive(SelectQuery)]
    pub struct FindUsers {
        #[meta(module = "users", table = "User")]
        __meta__: (),
        #[filter(operator = "Is")]
        pub name: String
    }

    #[test]
    pub fn test_set() {
        let set = MySet {
            name: "Joe".to_owned(),
            status: Status::Open,
            users: FindUsers { __meta__: (), name: "Joe".to_owned() }
        };

        assert_eq!("set { first_name := .first_name ++ (select <str>$user_name), status := (select <default::State>$status), users := (select users::User  filter users::User.name = (select <str>$name))}", set.to_edgeql());


        if let Value::Object { shape, fields} = set.to_edge_value() {
            check_shape(&shape, vec!["user_name", "status", "name"]);

            assert_eq!(fields, vec![
                Some(Value::Str(set.name)),
                Some(Value::Enum(EnumValue::from("Open"))),
                Some(Value::Str(set.users.name))
            ])
        }

    }

}