#[cfg(test)]
mod update {
    use edgedb_protocol::value::Value;
    use edgedb_query_derive::{edgedb_filters, edgedb_sets, query_result, update_query};
    use edgedb_query::models::edge_query::ToEdgeQuery;
    use crate::test_utils::check_shape;

    #[query_result]
    pub struct User {
        pub name: String
    }

    #[edgedb_sets]
    pub struct MySet {
        pub name: String,
    }

    #[edgedb_filters]
    pub struct MyFilter {
        #[field(column_name = "identity.first_name")]
        #[filter(operator = "=", wrapper_fn = "str_lower")]
        pub first_name: String,
        #[and_filter(operator = ">=")]
        pub age: i8,
    }

    #[update_query(module = "users", table = "User", result="User")]
    pub struct UpdateUserName {
        pub name: String,
        #[filters]
        pub filter: MyFilter,
    }

    #[test]
    pub fn test() {
        let q = UpdateUserName {
            name: "Joe".to_string(),
            filter: MyFilter {
                first_name: "Henri".to_string(),
                age: 18,
            },
        };

        let eq = q.to_edge_query();

        let expected_query = r#"
            select (update users::User
            filter str_lower(users::User.identity.first_name) = (select <str>$first_name)
            and users::User.age >= (select <int16>$age)
            set {
                name := (select <str>$name)
            }){ name }
        "#.to_owned().replace('\n', "");

        assert_eq!(eq.query.replace(' ', ""), expected_query.replace(' ', ""));

        if let Some(Value::Object { shape, fields }) = eq.args {
            check_shape(&shape, vec!["first_name", "age", "name"]);

            assert_eq!(fields, vec![
                Some(Value::Str(q.filter.first_name)),
                Some(Value::Int16(q.filter.age as i16)),
                Some(Value::Str(q.name)),
            ]);
        }
    }

    #[update_query(module = "users", table = "User")]
    pub struct UpdateUser {
        #[field(column_name="username")]
        pub name: String,


        #[filter(operator = "=", wrapper_fn = "str_lower")]
        #[field(column_name = "identity.first_name")]
        pub first_name: String,

        #[and_filter(operator = ">=")]
        pub age: i8,
    }

    #[test]
    pub fn test_2() {
        let q = UpdateUser {
            name: "Joe".to_string(),
            first_name: "Henri".to_string(),
            age: 18,
        };

        let eq = q.to_edge_query();

        let expected_query = r#"
            update users::User
            filter str_lower(users::User.identity.first_name) = (select <str>$first_name)
            and users::User.age >= (select <int16>$age)
            set {
                username := (select <str>$name)
            }
        "#.to_owned().replace('\n', "");

        assert_eq!(eq.query.replace(' ', ""), expected_query.replace(' ', ""));

        if let Some(Value::Object { shape, fields }) = eq.args {
            check_shape(&shape, vec!["first_name", "age", "name"]);

            assert_eq!(fields, vec![
                Some(Value::Str(q.first_name)),
                Some(Value::Int16(q.age as i16)),
                Some(Value::Str(q.name)),
            ]);
        }
    }


    #[update_query(module = "users", table = "User")]
    pub struct UpdateName {
        #[sets]
        pub set: MySet,

        #[filters]
        pub filter: MyFilter,
    }

    #[test]
    pub fn test_3() {
        let q = UpdateName {
            set: MySet {
                name: "Joe".to_string(),
            },
            filter: MyFilter {
                first_name: "Henri".to_string(),
                age: 18,
            },
        };

        let eq = q.to_edge_query();

        let expected_query = r#"
            update users::User
            filter str_lower(users::User.identity.first_name) = (select <str>$first_name)
            and users::User.age >= (select <int16>$age)
            set {
                name := (select <str>$name)
            }
        "#.to_owned().replace('\n', "");

        assert_eq!(eq.query.replace(' ', ""), expected_query.replace(' ', ""));

        if let Some(Value::Object { shape, fields }) = eq.args {
            check_shape(&shape, vec!["first_name", "age", "name"]);

            assert_eq!(fields, vec![
                Some(Value::Str(q.filter.first_name)),
                Some(Value::Int16(q.filter.age as i16)),
                Some(Value::Str(q.set.name)),
            ]);
        }
    }
}