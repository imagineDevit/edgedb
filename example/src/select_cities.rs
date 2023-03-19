
#[cfg(test)]
mod tests {
    use edgedb_query_derive::{query_result, select_query};
    use rstest::*;
    use serde::Deserialize;
    use edgedb_query::models::edge_query::{EdgeQuery, ToEdgeQuery};

    #[derive(Deserialize)]
    #[query_result]
    pub struct City {
        pub name: String
    }

    #[select_query(table="City", result="City")]
    pub struct SelectCity {}


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

        let select_query: EdgeQuery = SelectCity {}.to_edge_query();


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