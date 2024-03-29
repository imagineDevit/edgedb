#[cfg(test)]
pub mod from_file {
    use edgedb_protocol::value::Value;
    use edgedb_query::EdgeQuery;
    use edgedb_query_derive::file_query;
    use edgedb_query::models::edge_query::ToEdgeQuery;

    #[file_query(src="edgedb-query-derive/test/from_file_query/add_user.edgeql")]
    pub struct AddUser {
        #[param("user_name")]
        pub name: String,
        pub age: i8,
        #[param("friend_name")]
        pub friend: String,
    }

    #[test]
    fn test() {
        let user = AddUser {
            name: "Joe".to_string(),
            age: 35,
            friend: "John".to_string(),
        };


        let ql = include_str!("add_user.edgeql");

        let query: EdgeQuery = user.to_edge_query();

        assert_eq!(ql, query.query.as_str());

        if let Some(Value::Object { shape, fields }) = query.args {
            crate::test_utils::check_shape(
                &shape,
                vec!["user_name", "age", "friend_name"],
            );

            assert_eq!(
                fields,
                vec![
                    Some(Value::Str(user.name)),
                    Some(Value::Int16(user.age as i16)),
                    Some(Value::Str(user.friend)),
                ]
            );
        } else {
            assert!(false)
        }
    }
}