
#[cfg(test)]
mod update {
    use edgedb_protocol::value::Value;
    use edgedb_query_derive::{EdgedbSet, UpdateQuery, EdgedbFilters};
    use edgedb_query::{ToEdgeScalar, ToEdgeQl, ToEdgeValue, queries::filter::Filter, models::edge_query::{ToEdgeQuery}};
    use crate::test_utils::check_shape;

    #[derive(EdgedbSet)]
    pub struct MySet {
        pub name: String,
    }

    #[derive(EdgedbFilters)]
    pub struct MyFilter {
        #[filter(operator="=", column_name="identity.first_name", wrapper_fn="str_lower")]
        pub first_name: String,
        #[filter(operator=">=",  conjunctive="And")]
        pub age: i8,
    }

    #[derive(UpdateQuery)]
    pub struct UpdateUserName {
        #[meta(module = "users", table="User")]
        __meta__: (),
        #[set]
        pub set: MySet,
        #[filters]
        pub filter: MyFilter,
    }

    #[test]
    pub fn test() {
        let q = UpdateUserName {
            __meta__: (),
            set: MySet {
                name: "Joe".to_string()
            },
            filter: MyFilter {
                first_name :"Henri".to_string(),
                age : 18
            }
        };

        let eq = q.to_edge_query();

        let expected_query = r#"
            update users::User
            filter str_lower(users::User.identity.first_name) = (select <str>$first_name)
            and users::User.age >= (select <int16>$age)
            set {
                name := (select <str>$name)
            }
        "#.to_owned().replace("\n", "");

        assert_eq!(eq.query.replace(" ", ""), expected_query.replace(" ", ""));

        if let Some(Value::Object { shape, fields}) = eq.args {
            check_shape(&shape, vec!["first_name", "age", "name"]);

            assert_eq!(fields, vec![
                Some(Value::Str(q.filter.first_name)),
                Some(Value::Int16(q.filter.age as i16)),
                Some(Value::Str(q.set.name)),
            ]);
        }

    }
}