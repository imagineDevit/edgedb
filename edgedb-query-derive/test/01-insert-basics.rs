use edgedb_query::BasicResult;
use edgedb_query::ToEdgeql;
use edgedb_query::ToScalar;
use edgedb_query_derive::InsertQuery;

#[derive(InsertQuery)]
pub struct Product {
    #[edgedb(module = "default", table = "Product")]
    pub t: (),
    pub name: String,
    pub quantity: u8,
    pub price: f64,
    pub bio: bool,
    pub origin: Option<String>,
}

fn main() {
    let u = Product {
        t: (),
        name: "Joe".to_string(),
        quantity: 35,
        price: 10.5,
        bio: true,
        origin: None,
    };

    let query = u.to_edgeql();

    assert_eq!(
        query,
        "insert default::Product { name := (select <str>\"Joe\"), quantity := (select <int16>\"35\"), price := (select <float64>\"10.5\"), bio := (select <bool>\"true\"), }"
    );

    let u2 = Product {
        t: (),
        name: "Joe".to_string(),
        quantity: 35,
        price: 10.5,
        bio: true,
        origin: Some("France".to_string()),
    };

    let query2 = u2.to_edgeql();

    assert_eq!(
        query2,
        "insert default::Product { name := (select <str>\"Joe\"), quantity := (select <int16>\"35\"), price := (select <float64>\"10.5\"), bio := (select <bool>\"true\"), origin := (select <str>\"France\"), }"
    )
}
