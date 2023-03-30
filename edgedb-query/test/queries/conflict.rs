
#[cfg(test)]
mod conflict_test {
    use edgedb_protocol::value::Value;
    use edgedb_query::queries::conflict::{UnlessConflict, UnlessConflictElse, parse_conflict};
    use edgedb_query::{ToEdgeQl, ToEdgeQuery, ToEdgeValue};

    #[derive(Clone)]
    pub struct FindUser {}

    impl ToEdgeQl for FindUser {
        fn to_edgeql(&self) -> String {
            "select users".to_string()
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
            else_query: FindUser{},
        };

        let stmt = parse_conflict(&insert_conflict, vec!["name", "age"]);

        assert_eq!(stmt, " unless conflict on ( .name, .age ) else ( select users ) ");
    }

    #[test]
    fn parse_conflict_with_one_on_and_else_fn() {

        let insert_conflict = UnlessConflictElse {
            else_query: FindUser{},
        };

        let stmt = parse_conflict(&insert_conflict, vec!["name"]);

        assert_eq!(stmt, " unless conflict on .name else ( select users ) ");
    }

    #[test]
    fn parse_conflict_with_on() {

        let insert_conflict = UnlessConflict {};

        let stmt = parse_conflict(&insert_conflict, vec!["name", "age"]);

        assert_eq!(stmt, " unless conflict on ( .name, .age ) ");
    }


    #[test]
    fn parse_conflict_with_else_fn() {

        let insert_conflict = UnlessConflictElse {
            else_query: FindUser{},
        };

        let stmt = parse_conflict(&insert_conflict, vec![]);

        assert_eq!(stmt, " unless conflict else ( select users ) ");
    }
    #[test]
    fn parse_conflict_with_no_on_no_else_fn() {

        let insert_conflict: UnlessConflict = UnlessConflict{};

        let stmt = parse_conflict(&insert_conflict, vec![]);

        assert_eq!(stmt, " unless conflict ");
    }
}