
#[cfg(test)]
mod select_cities {
    use edgedb_query_derive::{SelectQuery, EdgedbResult};
    use edgedb_query::*;
    use rstest::*;
    use serde::Deserialize;

    use edgedb_query::models::edge_query::{EdgeQuery, ToEdgeQuery};

    #[derive(EdgedbResult, Deserialize)]
    pub struct City {
        pub name: String
    }


    #[derive(SelectQuery)]
    pub struct SelectCity {
        #[meta(table="City")]
        #[result(type="City")]
        __meta__: ()
    }


    #[fixture]
    async fn edgedb_client() -> edgedb_tokio::Client {
        edgedb_tokio::create_client().await.unwrap()
    }

    #[rstest]
    async fn update_city(
        #[future]
        edgedb_client: edgedb_tokio::Client
    ) {
        let client: edgedb_tokio::Client = edgedb_client.await;

        let select_query: EdgeQuery = SelectCity {
            __meta__: (),
        }.to_edge_query();


        let query_str = select_query.query.as_str();

        let args = &select_query.args.unwrap();

        println!("{:#?}", query_str);
        println!("{:#?}", args);


        if let Ok(json) = client.query_json(query_str, args).await {
            let cities = serde_json::from_str::<Vec<City>>(json.as_ref()).unwrap();

            assert_eq!(3, cities.len());

        } else {
            assert!(false)
        }

    }
}