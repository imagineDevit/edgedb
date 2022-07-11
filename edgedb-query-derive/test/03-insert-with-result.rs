use edgedb_query::BasicResult;
use edgedb_query::ToEdgeql;
use edgedb_query::ToScalar;
use edgedb_query_derive::{EdgedbResult, InsertQuery};

#[derive(InsertQuery)]
pub struct Product {
    #[edgedb(module = "default", table = "Product")]
    pub __meta__: (),
    #[query(result = "ProductResult")]
    __result__: (),
    pub name: String,
    pub quantity: u8,
    pub price: f64,
    pub bio: bool,
    pub origin: Option<String>,
}

#[derive(Default, EdgedbResult)]
pub struct ProductResult {
    pub name: String,
    pub quantity: u8,
    pub price: f64,
    pub bio: bool,
    pub origin: Option<String>,
}

fn main() {
    let u = Product {
        __meta__: (),
        __result__: (),
        name: "Joe".to_string(),
        quantity: 35,
        price: 10.5,
        bio: true,
        origin: None,
    };

    let query = u.to_edgeql();

    assert_eq!(
        query,
        "select ( insert default::Product { name := (select <str>\"Joe\"), quantity := (select <int16>\"35\"), price := (select <float64>\"10.5\"), bio := (select <bool>\"true\"), }){ name, quantity, price, bio, origin }"
    );
}
