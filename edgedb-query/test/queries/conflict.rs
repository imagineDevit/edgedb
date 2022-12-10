
#[cfg(test)]
mod conflict_test {
    use edgedb_protocol::value::Value;
    use edgedb_query::queries::conflict::{Conflict, UnlessConflict, UnlessConflictElse, parse_conflict};
    use edgedb_query::{ToEdgeQl, ToEdgeQuery, ToEdgeValue};

    #[derive(Clone)]
    pub struct FindUser {}

    impl ToEdgeQl for FindUser {
        fn to_edgeql(&self) -> String {
            format!("select users")
        }
    }

    impl ToEdgeValue for FindUser {
        fn to_edge_value(&self) -> Value {
            Value::Nothing
        }
    }

    impl ToEdgeQuery for FindUser  {}



    #[test]
    fn parse_conflict_with_on_and_else_fn() {

        let insert_conflict = UnlessConflictElse {
            fields: Some(vec!["name", "age"]),
            else_query: Some(FindUser{}),
        };

        let stmt = parse_conflict(&insert_conflict, vec!["name", "age", "surname"]);

        assert_eq!(stmt, " unless conflict on ( .name, .age ) else ( select users )");
    }

    #[test]
    fn parse_conflict_with_one_on_and_else_fn() {

        let insert_conflict = UnlessConflictElse {
            fields: Some(vec!["name"]),
            else_query: Some(FindUser{}),
        };

        let stmt = parse_conflict(&insert_conflict, vec!["name", "age", "surname"]);

        assert_eq!(stmt, " unless conflict on .name else ( select users )");
    }

    #[test]
    fn parse_conflict_with_on() {

        let insert_conflict = UnlessConflict {
            fields: Some(vec!["name", "age"]),
        };

        let stmt = parse_conflict(&insert_conflict, vec!["name", "age", "surname"]);

        assert_eq!(stmt, " unless conflict on ( .name, .age )");
    }


    #[test]
    fn parse_conflict_with_else_fn() {

        let insert_conflict = UnlessConflictElse {
            fields: None,
            else_query: Some(FindUser{}),
        };

        let stmt = parse_conflict(&insert_conflict, vec!["name", "age", "surname"]);

        assert_eq!(stmt, " unless conflict ");
    }
    #[test]
    fn parse_conflict_with_no_on_no_else_fn() {

        let insert_conflict: UnlessConflictElse<FindUser> = UnlessConflictElse {
            fields: None,
            else_query: None,
        };

        let stmt = parse_conflict(&insert_conflict, vec!["name", "age", "surname"]);

        assert_eq!(stmt, " unless conflict ");
    }
}