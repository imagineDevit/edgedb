
#[cfg(test)]
pub mod from_file {
    use edgedb_protocol::value::Value;
    use edgedb_query_derive::FromFileQuery;
    use edgedb_query::{ToEdgeQl, ToEdgeQuery, EdgeQuery};


    #[derive(FromFileQuery)]
    pub struct AddUser {
        #[src("edgedb-query-derive/test/from_file_query/add_user.edgeql")]
        pub __meta__: (),
        #[param("user_name")]
        pub name: String,
        pub age: i8,
    }

    #[test]
    fn test() {
        let user = AddUser {
            __meta__: (),
            name: "Joe".to_string(),
            age: 35
        };

        let ql = include_str!("add_user.edgeql");

        let query: EdgeQuery = user.to_edge_query();

        assert_eq!(ql, query.query.as_str());

        if let Some(Value::Object { shape, fields }) = query.args {
            crate::test_utils::check_shape(
                &shape,
                vec!["user_name", "age"],
            );

            assert_eq!(
                fields,
                vec![
                    Some(Value::Str(user.name)),
                    Some(Value::Int16(user.age as i16)),
                ]
            );

        } else {
            assert!(false)
        }

    }
}