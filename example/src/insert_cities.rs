
#[cfg(test)]
mod tests {

    use rstest::*;
    use edgedb_query::BasicResult;
    use edgedb_query_derive::{delete_query, insert_query};
    use edgedb_query::models::edge_query::{EdgeQuery, ToEdgeQuery};


    #[insert_query(table="City")]
    pub struct InsertCity {
        pub name: String,
        pub modern_name: Option<String>
    }

    #[delete_query(table="City")]
    pub struct DeleteCities {}

    #[fixture]
    async fn edgedb_client() -> edgedb_tokio::Client {
         edgedb_tokio::create_client().await.unwrap()
    }

    #[rstest]
    async fn create_cities(
        #[future]
        edgedb_client: edgedb_tokio::Client
    ) {

        let client: edgedb_tokio::Client = edgedb_client.await;

        let del_query: EdgeQuery = DeleteCities {}.to_edge_query();


        let _ = client.query_json(
            del_query.query.as_str(),
            &del_query.args.unwrap()
        ).await.unwrap();

        let cities = vec![
            InsertCity {
                name: "Munich".to_owned(),
                modern_name: None
            },
            InsertCity {
                name: "Buda-Pesth".to_owned(),
                modern_name: Some("Budapest".to_owned())
            },
            InsertCity {
                name: "Bistritz".to_owned(),
                modern_name: Some("Bistrița".to_owned())
            },
        ];

        for city in cities {
            let edge_query: EdgeQuery = city.to_edge_query();

            let args = &edge_query.args.unwrap();

            let query = edge_query.query.as_str();

            if let Some(json) = client.query_single_json(query, args).await.unwrap() {
                let result = serde_json::from_str::<BasicResult>(json.as_ref());
                if let Ok(b_result) = result {
                    assert_ne!(b_result.id, String::default());
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        }


        let count = client.query_required_single_json("select count((select City))", &()).await.unwrap();

        let result_count = serde_json::from_str::<u32>(count.as_ref());

        if let Ok(c) = result_count {
            assert_eq!(c, 3);
        } else {
            unreachable!()
        }

    }
}