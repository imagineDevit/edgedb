## Edgedb query
[![minimum rustc 1.31](https://img.shields.io/badge/rustc-1.59+-red.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
[![GitHub](https://img.shields.io/github/license/imagineDevit/edgedb?style=flat)](https://github.com/imagineDevit/edgedb/blob/main/License)
[![GitHub contributors](https://badgen.net/github/contributors/imagineDevit/edgedb)](https://github.com/imagineDevit/edgedb/graphs/contributors)
[![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/imagineDevit/edgedb/github%20pages/main?style=flat)](https://github.com/imagineDevit/edgedb/runs/7468742405?check_suite_focus=true)


[**Edgedb-query-derive**](https://github.com/imagineDevit/edgedb) is a rust crate project that aims to provide a bunch procedural macros in order to facilitate writing of [_edgeql_](https://www.edgedb.com/tutorial) queries when
using [edgedb-rust](https://github.com/edgedb/edgedb-rust) crate.

___

**Documentation** _available_ [_here_ 👉 ](https://imaginedevit.github.io/edgedb/)


---

## Example

[**Edgedb-query-derive**](https://github.com/imagineDevit/edgedb) allows you to go from this 👇

````rust

use edgedb_protocol::value::Value;
use edgedb_protocol::codec::ObjectShape;
use edgedb_protocol::codec::ShapeElement;
use edgedb_protocol::common::Cardinality;
use edgedb_protocol::codec::EnumValue;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    let client = edgedb_tokio::create_client().await?;

    let query = r#"
        select (
            insert users::User {
                name := (select <str>$name),
                surname := (select <str>$surname),
                age := (select <int32>$age),
                major := (select <bool>$major),
                vs := (select <array<str>>$vs),
                gender := (select <users::Gender>$gender),
                wallet := (
                    insert users::Wallet {
                            money := (select <int16>$money)
                       })
                   }
        ) {
            id,
            name : {
                name
            }
         }
    "#;


    let elements = vec![
        ShapeElement {
            flag_implicit: false,
            flag_link_property: false,
            flag_link: false,
            cardinality: Some(
                Cardinality::One,
            ),
            name: "name".to_string(),
        },
        ShapeElement {
            flag_implicit: false,
            flag_link_property: false,
            flag_link: false,
            cardinality: Some(
                Cardinality::One,
            ),
            name: "surname".to_string(),
        },
        ShapeElement {
            flag_implicit: false,
            flag_link_property: false,
            flag_link: false,
            cardinality: Some(
                Cardinality::One,
            ),
            name: "age".to_string(),
        },
        ShapeElement {
            flag_implicit: false,
            flag_link_property: false,
            flag_link: false,
            cardinality: Some(
                Cardinality::One,
            ),
            name: "major".to_string(),
        },
        ShapeElement {
            flag_implicit: false,
            flag_link_property: false,
            flag_link: false,
            cardinality: Some(
                Cardinality::One,
            ),
            name: "vs".to_string(),
        },
        ShapeElement {
            flag_implicit: false,
            flag_link_property: false,
            flag_link: false,
            cardinality: Some(
                Cardinality::One,
            ),
            name: "gender".to_string(),
        },
        ShapeElement {
            flag_implicit: false,
            flag_link_property: false,
            flag_link: false,
            cardinality: Some(
                Cardinality::One,
            ),
            name: "wallet".to_string(),
        },
    ];

    let args =  Value::Object {
        shape: ObjectShape::new(elements),
        fields: vec![
            Some(
                Value::Str(
                    "Joe".to_string(),
                ),
            ),
            Some(
                Value::Str(
                    "Henri".to_string(),
                ),
            ),
            Some(
                Value::Int32(
                    35,
                ),
            ),
            Some(
                Value::Bool(
                    true,
                ),
            ),
            Some(
                Value::Array(
                    vec![
                        Value::Str(
                            "vs1".to_string(),
                        ),
                    ],
                ),
            ),
            Some(
                Value::Enum(
                    EnumValue::from("male"),
                ),
            ),
            Some(
                Value::Object {
                    shape: ObjectShape::new(
                        vec![
                            ShapeElement {
                                flag_implicit: false,
                                flag_link_property: false,
                                flag_link: false,
                                cardinality: Some(
                                    Cardinality::One,
                                ),
                                name: "money".to_string(),
                            },
                        ]
                    ),
                    fields: vec![
                        Some(
                            Value::Int16(
                                0,
                            ),
                        ),
                    ],
                },
            ),
        ],
    };


    let _result = client.query_json(query, &args).await?;
    
    
    Ok(())
}

````
to this 👇

```rust
    use edgedb_query_derive::{insert_query, EdgedbEnum, EdgedbResult};
    use edgedb_query::{*, ToEdgeShape, models::{ edge_query::EdgeQuery, query_result::BasicResult}};

    #[insert_query(module ="users", table="User", result="UserResult")]
    pub struct InsertUser {
        #[field(param="first_name")]
        pub name: String,
        pub surname: Option<String>,
        pub age: i32,
        pub major: bool,
        pub vs: Vec<String>,
        #[field(scalar = "<users::Gender>")]
        pub gender: Sex,
        #[nested_query]
        pub wallet: Wallet,
        #[unless_conflict]
        pub find_user: UnlessConflictElse<FindUser>
    }

    #[query_result]
    pub struct UserResult {
        pub id: String,
        pub name: NameResult,
    }

    #[query_result]
    pub struct NameResult {
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


    #[tokio::main]
    async fn main() -> anyhow::Result<()> {
        let client = edgedb_tokio::create_client().await?;
        let insert_user: EdgeQuery = InsertUser {
            name: "Joe".to_string(),
            surname: Some("sj".to_string()),
            age: 35,
            major: true,
            vs: vec!["vs1".to_string()],
            gender: Sex::Male,
            wallet: Wallet {
                __meta__: (),
                money: 0 }
        }.to_edge_query();

        let _result = client.query_json(insert_user.query, &insert_user.args).await?;
        
        OK(())
    }
```