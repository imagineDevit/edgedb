#[cfg(test)]
mod select {
    use edgedb_protocol::value::Value;
    use edgedb_query_derive::{edgedb_filters, query_result, select_query};
    use edgedb_query::models::edge_query::{ToEdgeQuery, EdgeQuery};
    use edgedb_query::queries::select::{OrderDir, OrderOptions, SelectOptions};


    #[query_result]
    pub struct UserResult {
        pub id: String,
        pub name: String,
        pub age: i8,
    }

    #[select_query(module = "users", table = "User", result = "UserResult")]
    pub struct FindUsers {}

    #[test]
    pub fn find_users_test() {

        let q = FindUsers {};

        let edge_query : EdgeQuery = q.to_edge_query();

        let expected_query = "select users::User {id,name,age}";

        assert_eq!(edge_query.query, expected_query);

        if let Some(Value::Nothing) = edge_query.args {
            assert!(true)
        } else {
            assert!(false)
        }
    }

    #[select_query(module = "users", table = "User", result = "UserResult")]
    pub struct FindUsersByNameExists {
        #[filter(operator = "Exists")]
        pub name: (),
    }

    #[test]
    pub fn filter_exists_test() {

        let q = FindUsersByNameExists {
            name: (),
        };

        let edge_query : EdgeQuery = q.to_edge_query();

        let expected_query = "select users::User {id,name,age} filter exists users::User.name";

        assert_eq!(edge_query.query, expected_query);
    }

    #[select_query(module = "users", table = "User", result = "UserResult")]
    pub struct FindUsersByNameNotExists {
        #[filter(operator = "NotExists")]
        pub name: (),
    }

    #[test]
    pub fn filter_not_exists_test() {

        let q = FindUsersByNameNotExists {
            name: (),
        };

        let edge_query : EdgeQuery = q.to_edge_query();

        let expected_query = "select users::User {id,name,age} filter not exists users::User.name";

        assert_eq!(edge_query.query, expected_query);
    }

    #[select_query(module = "users", table = "User", result = "UserResult")]
    pub struct FindUsersByNameIs {
        #[filter(operator = "Is")]
        pub name: String,
    }

    #[test]
    pub fn filter_is_test() {

        let q = FindUsersByNameIs {
            name: String::from("Joe"),
        };

        let edge_query : EdgeQuery = q.to_edge_query();

        do_test_filter("=",  edge_query, vec!["name"], vec![
            Some(Value::Str(q.name))
        ], "<str>");
    }

    #[select_query(module = "users", table = "User", result = "UserResult")]
    pub struct FindUsersByNameIsNot {
        #[filter(operator = "IsNot")]
        pub name: String,
    }

    #[test]
    pub fn filter_is_not_test() {

        let q = FindUsersByNameIsNot {
            name: String::from("Joe"),
        };

        let edge_query : EdgeQuery = q.to_edge_query();

        do_test_filter("!=",  edge_query, vec!["name"], vec![
            Some(Value::Str(q.name))
        ], "<str>");
    }

    #[select_query(module = "users", table = "User", result = "UserResult")]
    pub struct FindUsersByNameLike {
        #[filter(operator = "Like")]
        pub name: String,
    }

    #[test]
    pub fn filter_like_test() {

        let q = FindUsersByNameLike {
            name: String::from("Joe"),
        };

        let edge_query : EdgeQuery = q.to_edge_query();

        do_test_filter("like",  edge_query, vec!["name"], vec![
            Some(Value::Str(q.name))
        ], "<str>");
    }

    #[select_query(module = "users", table = "User", result = "UserResult")]
    pub struct FindUsersByNameILike {

        #[filter(operator = "ILike")]
        pub name: String,
    }

    #[test]
    pub fn filter_ilike_test() {

        let q = FindUsersByNameILike {
            name: String::from("Joe"),
        };

        let edge_query : EdgeQuery = q.to_edge_query();

        do_test_filter("ilike",  edge_query, vec!["name"], vec![
            Some(Value::Str(q.name))
        ], "<str>");
    }

    #[select_query(module = "users", table = "User", result = "UserResult")]
    pub struct FindUsersByNameIn {
        #[filter(operator = "In")]
        pub name: Vec<String>
    }

    #[test]
    pub fn filter_in_test() {

        let q = FindUsersByNameIn {
            name: vec![String::from("Joe")],
        };

        let edge_query : EdgeQuery = q.to_edge_query();


        do_test_filter("in",  edge_query, vec!["name"], vec![
            Some(Value::Array(vec![Value::Str(q.name[0].clone())]))
        ],"<array<str>>");
    }

    fn do_test_filter(symbol: &str, edge_query: EdgeQuery, query_args: Vec<&str>, args_values: Vec<Option<Value>>, scalar: &str) {

        let expected = format!("select users::User {{id,name,age}} filter users::User.name {symbol} (select {scalar}$name)");

        assert_eq!(edge_query.query, expected);

        if let Some(Value::Object { shape, fields }) = edge_query.args {
            crate::test_utils::check_shape(&shape, query_args);
            assert_eq!(fields, args_values)
        } else {
            assert!(false)
        }
    }

    #[select_query(module = "users", table = "User", result = "UserResult")]
    pub struct FindUsersByNameAndAgeGreaterThan {
        #[filter(operator = "Is")]
        pub name: String,

        #[and_filter(operator = "GreaterThan")]
        pub age: i8,
    }

    #[test]
    pub fn filter_is_and_test() {

        let q = FindUsersByNameAndAgeGreaterThan {
            name: String::from("Joe"),
            age: 25
        };

        let edge_query : EdgeQuery = q.to_edge_query();

        let expected = "select users::User {id,name,age} filter users::User.name = (select <str>$name) and users::User.age > (select <int16>$age)".to_string();

        assert_eq!(edge_query.query, expected);

        if let Some(Value::Object { shape, fields }) = edge_query.args {
            crate::test_utils::check_shape(&shape, vec!["name", "age"]);
            assert_eq!(fields, vec![
                Some(Value::Str(q.name)),
                Some(Value::Int16(q.age as i16))
            ])
        } else {
            assert!(false)
        }
    }


    #[select_query(module = "users", table = "User", result = "UserResult")]
    pub struct FindMajorUsersWithOptions {

        #[options]
        options: SelectOptions,

        #[filter(operator = "GreaterThanOrEqual")]
        pub age: i8,
    }

    #[test]
    pub fn filter_select_options_attributes_test() {

        let q = FindMajorUsersWithOptions {
            options: SelectOptions {
                order_options: Some(OrderOptions {
                    order_by: "name".to_string(),
                    order_direction: Some(OrderDir::Desc)
                }),
                page_options: None
            },
            age: 18
        };

        let edge_query : EdgeQuery = q.to_edge_query();

        let expected = "select users::User {id,name,age} filter users::User.age >= (select <int16>$age) order by users::User.name desc";

        assert_eq!(edge_query.query, expected);

        if let Some(Value::Object { shape, fields }) = edge_query.args {
            crate::test_utils::check_shape(&shape, vec!["age"]);
            assert_eq!(fields, vec![
                Some(Value::Int16(q.age as i16))
            ])
        } else {
            assert!(false)
        }
    }

    #[select_query(module = "users", table = "User", result = "UserResult")]
    pub struct FindMajorUsersWithFilters {

        #[options]
        options: SelectOptions,

        #[filters]
        filters: AgeFilter,
    }

    #[edgedb_filters]
    pub struct AgeFilter {
        #[filter(operator="GreaterThanOrEqual")]
        pub age: i8
    }

    #[test]
    pub fn filters_select_options_attributes_test() {

        let q = FindMajorUsersWithFilters {
            options: SelectOptions {
                order_options: Some(OrderOptions {
                    order_by: "name".to_string(),
                    order_direction: Some(OrderDir::Desc)
                }),
                page_options: None
            },
            filters: AgeFilter {
                age: 18
            }
        };

        let edge_query : EdgeQuery = q.to_edge_query();

        let expected = "select users::User {id,name,age} filter users::User.age >= (select <int16>$age) order by users::User.name desc";

        assert_eq!(edge_query.query, expected);

        if let Some(Value::Object { shape, fields }) = edge_query.args {
            crate::test_utils::check_shape(&shape, vec!["age"]);
            assert_eq!(fields, vec![
                Some(Value::Int16(q.filters.age as i16))
            ])
        } else {
            assert!(false)
        }
    }


}