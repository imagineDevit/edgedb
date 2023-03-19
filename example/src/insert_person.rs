
#[cfg(test)]
mod insert_person {
    use edgedb_query_derive::{EdgeValue, FromFileQuery, InsertQuery};
    use edgedb_query::{*, models::query_result::BasicResult};
    use rstest::*;


    #[derive(FromFileQuery)]
    pub struct InsertPerson {
        #[src("example/src/add_person.edgeql")]
        __meta__: (),
        name: String,
        #[param("city_name")]
        city: String
    }

    #[derive(EdgeValue)]
    pub struct City {
        #[param("city_name")]
        pub name: String,
    }

    #[derive(InsertQuery)]
    pub struct InsertPerson2 {
        #[meta(table="Person")]
        __meta__: (),
        name: String,
        #[nested_query]
        places_visited: InsertCity
    }

    #[derive(InsertQuery)]
    pub struct InsertCity {
        #[meta(table="City")]
        __meta__: (),
        #[param("city_name")]
        pub name: String,
        pub modern_name: Option<String>
    }

    #[fixture]
    async fn edgedb_client() -> edgedb_tokio::Client {
         edgedb_tokio::create_client().await.unwrap()
    }

    #[rstest]
    async fn create_persons(
        #[future]
        edgedb_client: edgedb_tokio::Client
    ) {
        let client: edgedb_tokio::Client = edgedb_client.await;

        let q: EdgeQuery = InsertPerson2 {
            __meta__: (),
            name: "Karl".to_owned(),
            places_visited: InsertCity {
                __meta__: (),
                name: "Amsterdam".to_owned(),
                modern_name: None
            }
        }.to_edge_query();

        println!("{:#?}", q.args);

        let r = client.query_json(
            q.query.as_str(),
            &q.args.unwrap()
        ).await;

        match r {
            Err(e) => println!("{:#?}", e),
            _ => {}
        }
    }
}