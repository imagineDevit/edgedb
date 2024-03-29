
#[cfg(test)]
mod tests {

    use edgedb_query_derive::{delete_query};
    use rstest::*;
    use edgedb_query::models::edge_query::{EdgeQuery, ToEdgeQuery};

    #[delete_query(table="City")]
    pub struct DeleteCities {}

    #[fixture]
    async fn edgedb_client() -> edgedb_tokio::Client {
        edgedb_tokio::create_client().await.unwrap()
    }

    #[rstest]
    async fn delete_cities(
        #[future]
        edgedb_client: edgedb_tokio::Client
    ) {

        let client: edgedb_tokio::Client = edgedb_client.await;

        let del_query: EdgeQuery = DeleteCities {}.to_edge_query();

        let _ = client.query_json(
            del_query.query.as_str(),
            &del_query.args.unwrap()
        ).await.unwrap();

        let count = client.query_required_single_json("select count((select City))", &()).await.unwrap();

        let result_count = serde_json::from_str::<u32>(count.as_ref());

        if let Ok(c) = result_count {
            assert_eq!(c, 0);
        } else {
            assert!(false);
        }

    }
}