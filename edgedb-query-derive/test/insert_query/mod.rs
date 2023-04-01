#[cfg(test)]
mod insert {
    use edgedb_protocol::codec::EnumValue;
    use edgedb_protocol::value::Value;
    use edgedb_query::{ToEdgeQuery, EdgeQuery};
    use edgedb_query::queries::conflict::{UnlessConflictElse, Conflict};
    use uuid::Uuid;
    use edgedb_query_derive::{insert_query, select_query, query_result, edgedb_enum};


    #[insert_query(module ="users", table="User", result="UserResult")]
    pub struct InsertEmptyUser;

    #[test]
    fn insert_empty_user_test() {
        let insert_user = InsertEmptyUser {};

        let query: EdgeQuery = insert_user.to_edge_query();

        let expected = "select ( insert users::User {} ){id,name : {id,name}}";

        assert_eq!(query.query, expected);
    }

    #[insert_query(module ="users", table="User", result="UserResult")]
    pub struct InsertUser {
        #[field(column_name="username", param="first_name")]
        pub name: String,
        pub surname: Option<String>,
        #[field(scalar="<int16>")]
        pub age: u8,
        pub major: bool,
        pub vs: Vec<String>,
        #[field(scalar = "<users::Gender>")]
        pub gender: Sex,
        #[nested_query]
        pub wallet: Wallet,
        #[unless_conflict(on="username, surname")]
        pub find_user: UnlessConflictElse<FindUser>
    }

    #[select_query(module = "users", table = "User")]
    pub struct FindUser {
        #[filter(operator="Is")]
        #[field(column_name="name")]
        pub user_name: String
    }

    #[query_result]
    pub struct UserResult {
        pub id: Uuid,
        pub name: NameResult,
    }
    #[query_result]
    pub struct NameResult {
        pub id: Uuid,
        pub name: String,
    }

    #[edgedb_enum]
    pub enum Sex {
        #[value("male")]
        Male,
        #[value("female")]
        _Female,
    }

    #[insert_query(module = "users", table = "Wallet")]
    pub struct Wallet {
        pub money: i16,
    }

    #[test]
    fn insert_user_test() {
        let insert_user = InsertUser {
            name: "Joe".to_string(),
            surname: Some("sj".to_string()),
            age: 35,
            major: true,
            vs: vec!["vs1".to_string()],
            gender: Sex::Male,
            wallet: Wallet {
                money: 0,
            },
            find_user: UnlessConflictElse {
                else_query: FindUser{
                    user_name: "Joe".to_string(),
                },
            }
        };

        
        let query: EdgeQuery = insert_user.to_edge_query();

        let expected = r#"
           select (
              insert users::User {
                username := (select <str>$first_name),
                surname := (select <str>$surname),
                age := (select <int16>$age),
                major := (select <bool>$major),
                vs := (select <array<str>>$vs),
                gender := (select <users::Gender>$gender),

                wallet := (
                    insert users::Wallet{
                        money := (select <int16>$money),
                    }
                ),
             } unless conflict on (.username, .surname) else (
                select users::User filter users::User.name = (select<str>$user_name)
             )
         ) {
            id,
            name : { id, name }
        }
        "#
        .to_owned()
        .replace('\n', "");

        assert_eq!(query.query.replace(' ', ""), expected.replace(' ', ""));

        if let Some(Value::Object { shape, fields }) = query.args {
            crate::test_utils::check_shape(
                &shape,
                vec![
                    "first_name", "surname", "age", "major", "vs", "gender", "money", "user_name"
                ],
            );

            let vs_val = &insert_user.vs[0];

            assert_eq!(
                fields,
                vec![
                    Some(Value::Str(insert_user.name)),
                    Some(Value::Str(insert_user.surname.unwrap())),
                    Some(Value::Int16(insert_user.age as i16)),
                    Some(Value::Bool(insert_user.major)),
                    Some(Value::Array(vec![Value::Str(vs_val.clone())])),
                    Some(Value::Enum(EnumValue::from("male"))),
                    Some(Value::Int16(insert_user.wallet.money)),
                    Some(Value::Str(insert_user.find_user.else_query.user_name))
                ]
            );

        } else {
            unreachable!()
        }
    }

}
