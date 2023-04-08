
#[cfg(test)]
mod tests {
    use edgedb_protocol::common::Cardinality;
    use edgedb_query_derive::{query_result, select_query};
    use rstest::*;
    use edgedb_query::models::edge_query::{EdgeQuery, ToEdgeQuery};

    #[query_result]
    pub struct City {
        pub name: String,
        pub id: uuid::Uuid,
    }

    #[select_query(table="City", result="City")]
    pub struct SelectCity {
        #[filter(operator="Is")]
        pub name: String
    }


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

        let  select_query: EdgeQuery = SelectCity {
            name: "Munich".to_owned()
        }.to_edge_query_with_cardinality(Cardinality::One);


        let query_str = select_query.query.as_str();

        let args = &select_query.args.unwrap();

        println!("{query_str:#?}");

        let cities: City = client.query_required_single(query_str, args).await.unwrap();

        //assert_eq!(3, cities.len());

        //cities.iter().for_each(|c| println!("{c:#?}"))

    }
}