
#[cfg(test)]
mod delete {
    use edgedb_protocol::value::Value;
    use edgedb_query_derive::{DeleteQuery, EdgedbFilters};
    use edgedb_query::{*,
                       queries::filter::Filter,
                       models::{edge_query::{ToEdgeQuery, EdgeQuery}}};

    #[derive(DeleteQuery)]
    pub struct DeleteUsers {
        #[meta(module="users", table="User")]
        __meta__: ()
    }

    #[test]
    pub fn delete_users_test() {
        let del_users = DeleteUsers {
            __meta__: ()
        };

        let edge_query: EdgeQuery = del_users.to_edge_query();

        assert_eq!(edge_query.query, "delete users::User");
    }

    #[derive(DeleteQuery)]
    pub struct DeleteUsersByName {
        #[meta(module="users", table="User")]
        __meta__: (),
        #[filters]
        pub filters: NameFilter
    }

    #[derive(EdgedbFilters)]
    pub struct NameFilter {
        #[filter(operator="=")]
        pub name: String
    }

    #[test]
    pub fn delete_users_by_name_test() {
        let del_users = DeleteUsersByName {
            __meta__: (),
            filters: NameFilter {
                name: "Joe".to_owned()
            }
        };

        let edge_query: EdgeQuery = del_users.to_edge_query();

        assert_eq!(edge_query.query, "delete users::User filter users::User.name = (select <str>$name)");

        if let Some(Value::Object { shape, fields }) = edge_query.args {
            crate::test_utils::check_shape(&shape, vec!["name"]);
            assert_eq!(fields, vec![
                Some(Value::Str(del_users.filters.name ))
            ])
        } else {
            assert!(false)
        }
    }

    #[derive(DeleteQuery)]
    pub struct DeleteUsersByAge {
        #[meta(module="users", table="User")]
        __meta__: (),

        #[filter(operator="=")]
        pub age: i16
    }

    #[test]
    pub fn delete_users_by_age_test() {
        let del_users = DeleteUsersByAge{
            __meta__: (),
            age: 25
        };

        let edge_query: EdgeQuery = del_users.to_edge_query();

        assert_eq!(edge_query.query, "delete users::User filter users::User.age = (select <int16>$age)");

        if let Some(Value::Object { shape, fields }) = edge_query.args {
            crate::test_utils::check_shape(&shape, vec!["age"]);
            assert_eq!(fields, vec![
                Some(Value::Int16(del_users.age))
            ])
        } else {
            assert!(false)
        }
    }
}