
mod filter {
    use edgedb_query_derive::{edgedb_filters, select_query};
    use edgedb_query::{queries::filter::Filter};
    use edgedb_protocol::value::Value;


    #[edgedb_filters]
    pub struct MyFilter {
        #[field(column_name="identity.first_name", param = "first_name")]
        #[filter(operator="=",  wrapper_fn="str_lower")]
        pub name: String,
        #[and_filter(operator=">=")]
        pub age: i8,
        #[nested_query]
        #[and_filter(operator="in")]
        pub friends: FindFriends
    }

    #[select_query(table="Person")]
    pub struct FindFriends {
        #[filter(operator="like")]
        pub name: String
    }


    #[test]
    pub fn test_filter() {
        let filter = MyFilter {
            name: "Joe".to_string(),
            age: 18,
            friends: FindFriends {
                name: "%_ami".to_string()
            }
        };

        let query = filter.to_edgeql("users::User");

        let value: Value = filter.to_edge_value();

        assert_eq!(query, "filter str_lower(users::User.identity.first_name) = (select <str>$first_name) and users::User.age >= (select <int16>$age) and .friends in (select default::Person filter default::Person.name like (select <str>$name))");

        if let Value::Object { shape, fields } = value {
            crate::test_utils::check_shape(&shape, vec!["first_name", "age", "name"]);
            assert_eq!(fields, vec![
                Some(Value::Str(filter.name)),
                Some(Value::Int16(filter.age as i16)),
                Some(Value::Str(filter.friends.name))
            ])
        } else {
            unreachable!()
        }
    }

}