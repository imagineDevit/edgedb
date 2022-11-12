#[cfg(test)]
mod select {
    use edgedb_protocol::value::Value;
    use edgedb_query_derive::{SelectQuery, EdgedbResult, EdgedbFilters};
    use edgedb_query::{
        *,
        ToEdgeShape,
        models::edge_query::ToEdgeQuery

    };
    use edgedb_query::models::edge_query::EdgeQuery;
    use edgedb_query::queries::{select::{OrderDir, OrderOptions, SelectOptions} , filter::Filter};

    #[derive(Default, EdgedbResult)]
    pub struct UserResult {
        pub id: String,
        pub name: String,
        pub age: i8,
    }

    #[derive(SelectQuery)]
    pub struct FindUsers {
        #[meta(module = "users", table = "User")]
        #[result(type = "UserResult")]
        __meta__: (),
    }

    #[test]
    pub fn find_users_test() {

        let q = FindUsers {
            __meta__ : ()
        };

        let edge_query : EdgeQuery = q.to_edge_query();

        let expected_query = "select users::User {id,name,age}";

        assert_eq!(edge_query.query, expected_query);

        if let Some(Value::Nothing) = edge_query.args {
            assert!(true)
        } else {
            assert!(false)
        }
    }

    #[derive(SelectQuery)]
    pub struct FindUsersByNameExists {
        #[meta(module = "users", table = "User")]
        #[result(type = "UserResult")]
        __meta__: (),

        #[filter(operator = "Exists")]
        pub name: (),
    }

    #[test]
    pub fn filter_exists_test() {

        let q = FindUsersByNameExists {
            __meta__ : (),
            name: (),
        };

        let edge_query : EdgeQuery = q.to_edge_query();

        let expected_query = "select users::User {id,name,age} filter exists users::User.name";

        assert_eq!(edge_query.query, expected_query);
    }

    #[derive(SelectQuery)]
    pub struct FindUsersByNameNotExists {
        #[meta(module = "users", table = "User")]
        #[result(type = "UserResult")]
        __meta__: (),

        #[filter(operator = "NotExists")]
        pub name: (),
    }

    #[test]
    pub fn filter_not_exists_test() {

        let q = FindUsersByNameNotExists {
            __meta__ : (),
            name: (),
        };

        let edge_query : EdgeQuery = q.to_edge_query();

        let expected_query = "select users::User {id,name,age} filter not exists users::User.name";

        assert_eq!(edge_query.query, expected_query);
    }

    #[derive(SelectQuery)]
    pub struct FindUsersByNameIs {
        #[meta(module = "users", table = "User")]
        #[result(type = "UserResult")]
        __meta__: (),

        #[filter(operator = "Is")]
        pub name: String,
    }

    #[test]
    pub fn filter_is_test() {

        let q = FindUsersByNameIs {
            __meta__ : (),
            name: String::from("Joe"),
        };

        let edge_query : EdgeQuery = q.to_edge_query();

        do_test_filter("=",  edge_query, vec!["name"], vec![
            Some(Value::Str(q.name))
        ], "<str>");
    }

    #[derive(SelectQuery)]
    pub struct FindUsersByNameIsNot {
        #[meta(module = "users", table = "User")]
        #[result(type = "UserResult")]
        __meta__: (),

        #[filter(operator = "IsNot")]
        pub name: String,
    }

    #[test]
    pub fn filter_is_not_test() {

        let q = FindUsersByNameIsNot {
            __meta__ : (),
            name: String::from("Joe"),
        };

        let edge_query : EdgeQuery = q.to_edge_query();

        do_test_filter("!=",  edge_query, vec!["name"], vec![
            Some(Value::Str(q.name))
        ], "<str>");
    }

    #[derive(SelectQuery)]
    pub struct FindUsersByNameLike {
        #[meta(module = "users", table = "User")]
        #[result(type = "UserResult")]
        __meta__: (),

        #[filter(operator = "Like")]
        pub name: String,
    }

    #[test]
    pub fn filter_like_test() {

        let q = FindUsersByNameLike {
            __meta__ : (),
            name: String::from("Joe"),
        };

        let edge_query : EdgeQuery = q.to_edge_query();

        do_test_filter("like",  edge_query, vec!["name"], vec![
            Some(Value::Str(q.name))
        ], "<str>");
    }

    #[derive(SelectQuery)]
    pub struct FindUsersByNameILike {
        #[meta(module = "users", table = "User")]
        #[result(type = "UserResult")]
        __meta__: (),

        #[filter(operator = "ILike")]
        pub name: String,
    }

    #[test]
    pub fn filter_ilike_test() {

        let q = FindUsersByNameILike {
            __meta__ : (),
            name: String::from("Joe"),
        };

        let edge_query : EdgeQuery = q.to_edge_query();

        do_test_filter("ilike",  edge_query, vec!["name"], vec![
            Some(Value::Str(q.name))
        ], "<str>");
    }

    #[derive(SelectQuery)]
    pub struct FindUsersByNameIn {
        #[meta(module = "users", table = "User")]
        #[result(type = "UserResult")]
        __meta__: (),

        #[filter(operator = "In")]
        pub name: Vec<String>,
    }

    #[test]
    pub fn filter_in_test() {

        let q = FindUsersByNameIn {
            __meta__ : (),
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

    #[derive(SelectQuery)]
    pub struct FindUsersByNameAndAgeGreaterThan {
        #[meta(module = "users", table = "User")]
        #[result(type = "UserResult")]
        __meta__: (),

        #[filter(operator = "Is")]
        pub name: String,

        #[filter(operator = "GreaterThan", conjunctive="And")]
        pub age: i8,
    }

    #[test]
    pub fn filter_is_and_test() {

        let q = FindUsersByNameAndAgeGreaterThan {
            __meta__ : (),
            name: String::from("Joe"),
            age: 25
        };

        let edge_query : EdgeQuery = q.to_edge_query();

        let expected = format!("select users::User {{id,name,age}} filter users::User.name = (select <str>$name) and users::User.age > (select <int16>$age)");

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

    #[derive(SelectQuery)]
    pub struct FindMajorUsers {
        #[meta(module = "users", table = "User")]
        #[result(type = "UserResult", order_by="name", order_dir="desc" )]
        __meta__: (),

        #[filter(operator = "GreaterThanOrEqual")]
        pub age: i8,
    }

    #[test]
    pub fn filter_options_attributes_test() {

        let q = FindMajorUsers {
            __meta__ : (),
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

    #[derive(SelectQuery)]
    pub struct FindMajorUsersWithOptions {
        #[meta(module = "users", table = "User")]
        #[result(type = "UserResult")]
        __meta__: (),

        #[options]
        options: SelectOptions<'static>,

        #[filter(operator = "GreaterThanOrEqual")]
        pub age: i8,
    }

    #[test]
    pub fn filter_select_options_attributes_test() {

        let q = FindMajorUsersWithOptions {
            __meta__ : (),
            options: SelectOptions {
                table_name: "User",
                module: Some("users"),
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

    #[derive(SelectQuery)]
    pub struct FindMajorUsersWithFilters {
        #[meta(module = "users", table = "User")]
        #[result(type = "UserResult")]
        __meta__: (),

        #[options]
        options: SelectOptions<'static>,

        #[filters]
        filters: AgeFilter,
    }

    #[derive(EdgedbFilters)]
    pub struct AgeFilter {
        #[filter(operator="GreaterThanOrEqual")]
        pub age: i8
    }

    #[test]
    pub fn filters_select_options_attributes_test() {

        let q = FindMajorUsersWithFilters {
            __meta__ : (),
            options: SelectOptions {
                table_name: "User",
                module: Some("users"),
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