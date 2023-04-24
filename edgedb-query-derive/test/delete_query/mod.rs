
#[cfg(test)]
mod delete {
    use edgedb_protocol::value::Value;
    use edgedb_query_derive::{delete_query, edgedb_filters};
    use edgedb_query::models::{edge_query::{ToEdgeQuery, EdgeQuery}};

    #[delete_query(module ="users", table="User")]
    pub struct DeleteUsers;

    #[test]
    pub fn delete_users_test() {
        let del_users = DeleteUsers{};
        let edge_query: EdgeQuery = del_users.to_edge_query();
        assert_eq!(edge_query.query, "delete users::User");
    }

    #[delete_query(module ="users", table="User")]
    pub struct DeleteUsersByName {
        #[filters]
        pub filters: NameFilter
    }

    #[edgedb_filters]
    pub struct NameFilter {
        #[filter(operator="=")]
        pub name: String
    }

    #[test]
    pub fn delete_users_by_name_test() {
        let del_users = DeleteUsersByName {
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

    #[delete_query(module ="users", table="User")]
    pub struct DeleteUsersByAge {

        #[filter(operator="=")]
        pub age: i16
    }

    #[test]
    pub fn delete_users_by_age_test() {
        let del_users = DeleteUsersByAge{
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