use edgedb_query::models::edge_query::*;

use edgedb_query::queries::select::{
    OrderDir, OrderOptions, PageOptions, SelectOptions
};
use edgedb_query::models::query_result::*;
use edgedb_query::*;
use edgedb_query_derive::{EdgedbEnum, EdgedbResult, InsertQuery, SelectQuery};

#[derive(InsertQuery)]
pub struct User {
    #[edgedb(module = "users", table = "User")]
    #[query(result = "UserResult")]
    __meta__: (),

    pub name: String,
    pub surname: Option<String>,
    pub age: u32,
    pub major: bool,
    pub vs: Vec<String>,
    #[edgedb(type = "enum", module = "users", name = "Gender")]
    pub gender: Gender,
    pub wallet: Wallet,
}

#[derive(Default, EdgedbResult)]
pub struct UserResult {
    pub id: String,
    pub name: String,
}

#[derive(EdgedbEnum)]
pub enum Gender {
    #[value("male")]
    Male,
    #[value("female")]
    Female,
}

#[derive(InsertQuery)]
pub struct Wallet {
    #[edgedb(module = "users", table = "Wallet")]
    __meta__: (),
    pub money: u16,
}

#[derive(SelectQuery)]
pub struct FindUserByName {
    #[edgedb(module = "users", table = "User")]
    #[query(result = "UserResult", order_by = "name", order_dir = "asc", limit = 3)]
    __meta__: (),

    #[options]
    pub options: SelectOptions<'static>,

    #[filter(Is)]
    pub name: String,
}

#[test]
fn query() {
    //let f = User {
    //    __meta__: (),
    //    name: "Joe".to_string(),
    //    vs: vec!["1".to_string(), "2".to_string()],
    //    age: 35,
    //    major: true,
    //    gender: Gender::Female,
    //    surname: None,
    //    wallet: Wallet {
    //        __meta__: (),
    //        money: 15,
    //    },
    //};

    let f = FindUserByName {
        __meta__: (),
        name: String::from("Joe"),
        options: SelectOptions {
            module: Some("users"),
            table_name: "User",
            order_options: Some(OrderOptions {
                order_by: "name".to_string(),
                order_direction: Some(OrderDir::Desc),
            }),
            page_options: Some(PageOptions {
                limit: 10,
                offset: Some(0),
            }),
        },
    };
    println!("{:#?}", f.to_edge_query().query);

    //let t = trybuild::TestCases::new();
    //t.pass("test/01-insert-basics.rs");
    //t.pass("test/02-insert-with-enum.rs");
    //t.pass("test/03-insert-with-result.rs");
}
