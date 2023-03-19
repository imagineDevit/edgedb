
#[cfg(test)]
mod tests {
    use edgedb_query_derive::{file_query, insert_query};
    use edgedb_query::{*, models::query_result::BasicResult};
    use rstest::*;


    #[file_query(src="example/src/add_person.edgeql")]
    pub struct InsertPerson {
        name: String,
        #[param("city_name")]
        city: String
    }


    #[insert_query(table="Person")]
    pub struct InsertPerson2 {
        name: String,
        #[nested_query]
        places_visited: InsertCity
    }

    #[insert_query(table="City")]
    pub struct InsertCity {
        #[field(param="city_name")]
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
            name: "Karl".to_owned(),
            places_visited: InsertCity {
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