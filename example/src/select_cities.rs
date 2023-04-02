
#[cfg(test)]
mod tests {
    use edgedb_query_derive::{query_result, select_query};
    use rstest::*;
    use serde::Deserialize;
    use edgedb_query::models::edge_query::{EdgeQuery, ToEdgeQuery};

    #[query_result]
    pub struct City {
        pub name: String,
        pub id: uuid::Uuid,
    }

    #[select_query(table="City", result="City")]
    pub struct SelectCity {}


    #[fixture]
    async fn edgedb_client() -> edgedb_tokio::Client {
        edgedb_tokio::create_client().await.unwrap()
    }

    #[rstest]
    async fn select_cities(
        #[future]
        edgedb_client: edgedb_tokio::Client
    ) {
        let client: edgedb_tokio::Client = edgedb_client.await;

        let select_query: EdgeQuery = SelectCity {}.to_edge_query();

        let query_str = select_query.query.as_str();

        let args = &select_query.args.unwrap();

        println!("{query_str:#?}");

        let cities: Vec<City> = client.query(query_str, args).await.unwrap();

        assert_eq!(3, cities.len());
    }
}