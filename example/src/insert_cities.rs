
#[cfg(test)]
mod insert_cities {

    use edgedb_query_derive::{InsertQuery, DeleteQuery};
    use edgedb_query::{*, models::query_result::BasicResult};
    use rstest::*;
    use edgedb_query::models::edge_query::{EdgeQuery, ToEdgeQuery};

    #[derive(InsertQuery)]
    pub struct InsertCity {
        #[meta(table="City")]
        __meta__: (),

        pub name: String,
        pub modern_name: Option<String>
    }

    #[derive(DeleteQuery)]
    pub struct DeleteCities {
        #[meta(table="City")]
        __meta__: (),
    }

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

        let del_query: EdgeQuery = DeleteCities {
            __meta__: ()

        }.to_edge_query();


        let _ = client.query_json(
            del_query.query.as_str(),
            &del_query.args.unwrap()
        ).await.unwrap();

        let cities = vec![
            InsertCity {
                __meta__: (),
                name: "Munich".to_owned(),
                modern_name: None
            },
            InsertCity {
                __meta__: (),
                name: "Buda-Pesth".to_owned(),
                modern_name: Some("Budapest".to_owned())
            },
            InsertCity {
                __meta__: (),
                name: "Bistritz".to_owned(),
                modern_name: Some("Bistrița".to_owned())
            },
        ];

        for city in cities {
            let edge_query: EdgeQuery = city.to_edge_query();

            let args = &edge_query.args.unwrap();
            println!("{:#?}", args);

            let query = edge_query.query.as_str();

            if let Some(json) = client.query_single_json(query, args).await.unwrap() {
                let result = serde_json::from_str::<BasicResult>(json.as_ref());
                if let Ok(b_result) = result {
                    assert_ne!(b_result.id, String::default());
                } else {
                    assert!(false);
                }
            } else {
                assert!(false);
            }
        }


        let count = client.query_required_single_json("select count((select City))", &()).await.unwrap();

        let result_count = serde_json::from_str::<u32>(count.as_ref());

        if let Ok(c) = result_count {
            assert_eq!(c, 3);
        } else {
            assert!(false);
        }

    }
}