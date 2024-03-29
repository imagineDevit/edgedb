
#[cfg(test)]
mod tests {
    use edgedb_protocol::codec::EnumValue;
    use edgedb_protocol::value::Value;
    use edgedb_query::queries::set::Sets;
    use edgedb_query_derive::{select_query, edgedb_enum, edgedb_sets};
    use crate::test_utils::check_shape;


    #[edgedb_sets]
    pub struct MySet {
        #[field(column_name="first_name", param = "user_name", scalar="<str>")]
        #[set(option="Concat")]
        pub name: String,
        #[field(scalar="default::State")]
        pub status: Status,
        #[nested_query]
        #[set(option=":=")]
        #[field(column_name="u")]
        pub users: FindUsers
    }


    #[edgedb_enum]
    pub enum Status {
        Open, _Closed
    }

    #[select_query(module = "users", table = "User")]
    pub struct FindUsers {
        #[filter(operator = "Is")]
        pub name: String
    }

    #[test]
    pub fn test_set() {

        let set = MySet {
            name: "Joe".to_owned(),
            status: Status::Open,
            users: FindUsers { name: "Joe".to_owned() }
        };

        assert_eq!("set { first_name := .first_name ++ (select <str>$user_name), status := (select <default::State>$status), u := (select users::User filter users::User.name = (select <str>$name)) }", set.to_edgeql().to_string());


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