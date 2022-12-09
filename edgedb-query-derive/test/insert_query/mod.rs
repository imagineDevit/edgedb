#[cfg(test)]
mod insert {
    use edgedb_protocol::codec::EnumValue;
    use edgedb_protocol::value::Value;
    use edgedb_query::{models::query_result::BasicResult, ToEdgeShape, *};
    use edgedb_query::queries::conflict::{InsertConflict, Conflict};
    use edgedb_query_derive::{EdgedbEnum, EdgedbResult, InsertQuery, SelectQuery};

    #[derive(InsertQuery)]
    pub struct InsertEmptyUser {
        #[meta(module = "users", table = "User")]
        #[result(type = "UserResult")]
        __meta__: (),
    }

    #[test]
    fn insert_empty_user_test() {
        let insert_user = InsertEmptyUser { __meta__: () };

        let query: EdgeQuery = insert_user.to_edge_query();

        let expected = "insert users::User";

        assert_eq!(query.query, expected);
    }

    #[derive(InsertQuery)]
    pub struct InsertUser {
        #[meta(module = "users", table = "User")]
        #[result(type = "UserResult")]
        __meta__: (),

        pub name: String,
        pub surname: Option<String>,
        pub age: i32,
        pub major: bool,
        pub vs: Vec<String>,
        #[scalar(type = "enum", module = "users", name = "Gender")]
        pub gender: Sex,
        #[nested_query]
        pub wallet: Wallet,

        #[unless_conflict]
        pub find_user: InsertConflict<FindUser>
    }

    #[derive(Clone, SelectQuery)]
    pub struct FindUser {
        #[meta(module = "users", table = "User")]

        __meta__: (),
        #[filter(operator="Is", column_name="name")]
        pub user_name: String
    }

    #[derive(Default, EdgedbResult)]
    pub struct UserResult {
        pub id: String,
        pub name: NameResult,
    }

    #[derive(Default, EdgedbResult)]
    pub struct NameResult {
        pub name: String,
    }

    #[derive(EdgedbEnum)]
    pub enum Sex {
        #[value("male")]
        Male,
        #[value("female")]
        _Female,
    }

    #[derive(InsertQuery)]
    pub struct Wallet {
        #[meta(module = "users", table = "Wallet")]
        __meta__: (),
        pub money: i16,
    }

    #[test]
    fn insert_user_test() {
        let insert_user = InsertUser {
            __meta__: (),
            name: "Joe".to_string(),
            surname: Some("sj".to_string()),
            age: 35,
            major: true,
            vs: vec!["vs1".to_string()],
            gender: Sex::Male,
            wallet: Wallet {
                __meta__: (),
                money: 0,
            },
            find_user: InsertConflict {
                fields: Some(vec!["name", "surname"]),
                else_query: Some(FindUser{
                    __meta__: (),
                    user_name: "Joe".to_string(),
                }),
            }
        };

        let query: EdgeQuery = insert_user.to_edge_query();

        println!("{:#?}", query.query);

        let expected = r#"
           select (
              insert users::User {
                name := (select <str>$name),
                surname := (select <str>$surname),
                age := (select <int32>$age),
                major := (select <bool>$major),
                vs := (select <array<str>>$vs),
                gender := (select <users::Gender>$gender),

                wallet := (
                    insert users::Wallet{
                        money := (select <int16>$money),
                    }
                ),
             } unless conflict on (.name, .surname) else (
                select users::User filter users::User.name = (select<str>$user_name)
             )
         )  {
            id,
            name : { name }
        }
        "#
        .to_owned()
        .replace("\n", "");

        assert_eq!(query.query.replace(" ", ""), expected.replace(" ", ""));

        if let Some(Value::Object { shape, mut fields }) = query.args {
            crate::test_utils::check_shape(
                &shape,
                vec![
                    "name", "surname", "age", "major", "vs", "gender", "wallet", "user_name"
                ],
            );

            let find_user_field = fields.pop();
            let wallet_field = fields.pop();

            let vs_val = &insert_user.vs[0];

            assert_eq!(
                fields,
                vec![
                    Some(Value::Str(insert_user.name)),
                    Some(Value::Str(insert_user.surname.unwrap())),
                    Some(Value::Int32(insert_user.age as i32)),
                    Some(Value::Bool(insert_user.major)),
                    Some(Value::Array(vec![Value::Str(vs_val.clone())])),
                    Some(Value::Enum(EnumValue::from("male")))
                ]
            );

            if let Some(Some(Value::Object { shape, fields })) = wallet_field {
                let w_elmts = &shape.elements;
                assert_eq!(w_elmts.len(), 1);
                assert_eq!(
                    fields,
                    vec![Some(Value::Int16(insert_user.wallet.money as i16))]
                )
            }

            if let Some(Some(Value::Object { shape, fields })) = find_user_field {
                let w_elmts = &shape.elements;
                assert_eq!(w_elmts.len(), 1);
                assert_eq!(fields, vec![
                    Some(Value::Str(insert_user.find_user.else_query.unwrap().user_name))
                ])
            }
        } else {
            assert!(false)
        }
    }
}
