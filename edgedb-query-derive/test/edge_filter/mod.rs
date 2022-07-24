
mod filter {
    use edgedb_query_derive::EdgedbFilters;
    use edgedb_query::{ToEdgeScalar, ToEdgeValue, queries::filter::Filter};
    use edgedb_protocol::value::Value;

    #[derive(EdgedbFilters)]
    pub struct MyFilter {
        #[filter(operator="=", column_name="identity.first_name", wrapper_fn="str_lower")]
        pub name: String
    }

    #[test]
    pub fn test_filter() {
        let filter = MyFilter {
            name: "".to_string()
        };

        let query = filter.to_edgeql("users::User");

        let value: Value = filter.to_edge_value();

        assert_eq!(query, "filter str_lower(users::User.identity.first_name) = (select <str>$name)");

        if let Value::Object { shape, fields } = value {
            crate::test_utils::check_shape(&shape, vec!["name"]);
            assert_eq!(fields, vec![
                Some(Value::Str(filter.name))
            ])
        } else {
            assert!(false)
        }
    }

}