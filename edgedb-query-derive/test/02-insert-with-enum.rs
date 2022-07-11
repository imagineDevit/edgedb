use edgedb_query::BasicResult;
use edgedb_query::ToEdgeql;
use edgedb_query::ToScalar;
use edgedb_query_derive::{EdgedbEnum, InsertQuery};

#[derive(InsertQuery)]
pub struct Product {
    #[edgedb(module = "default", table = "Product")]
    pub t: (),
    pub name: String,
    pub quantity: u8,
    pub price: f64,
    pub bio: bool,
    #[edgedb(type = "enum", name = "ProductStatus")]
    pub status: Status,
}

#[derive(EdgedbEnum)]
pub enum Status {
    #[toString("En_Stock")]
    InStock,
    #[toString("En_Rupture")]
    InRupture,
}

fn main() {
    let u = Product {
        t: (),
        name: "Joe".to_string(),
        quantity: 35,
        price: 10.5,
        bio: true,
        status: Status::InStock,
    };

    let query = u.to_edgeql();

    assert_eq!(
        query,
        "insert default::Product { name := (select <str>\"Joe\"), quantity := (select <int16>\"35\"), price := (select <float64>\"10.5\"), bio := (select <bool>\"true\"), status := (select <ProductStatus>\"En_Stock\"), }"
    )
}
