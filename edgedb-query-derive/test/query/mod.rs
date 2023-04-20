#[cfg(test)]
pub mod query_value {
    use edgedb_protocol::value::Value;
    use edgedb_query::EdgeQuery;
    use edgedb_query_derive::{ query };
    use edgedb_query::models::edge_query::ToEdgeQuery;

    #[query(value=r#"
        insert users::User {
            name := <str>$user_name,
            age := <int16>$age,
            friend := (
                select users::User {
                    name,
                    age,
                }
                filter .name = <str>$friend_name
            )
        }
    "#)]
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


        let expected = r#"
            insert users::User {
                name := <str>$user_name,
                age := <int16>$age,
                friend := (
                    select users::User {
                        name,
                        age,
                    }
                    filter .name = <str>$friend_name
                )
            }
        "#;

        let query: EdgeQuery = user.to_edge_query();

        assert_eq!(query.query.replace(' ', ""), expected.replace(' ', ""));

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