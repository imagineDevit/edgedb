
#[cfg(test)]
mod tests {
    use edgedb_protocol::client_message::Cardinality;
    use edgedb_query_derive::{query_result, edgedb_sets, edgedb_filters, update_query, select_query};
    use edgedb_query::{*, models::query_result::BasicResult, queries::set::Sets};
    use rstest::*;
    use edgedb_query::models::edge_query::{EdgeQuery, ToEdgeQuery};

    #[query_result]
    pub struct City {
        pub id: uuid::Uuid,
        pub name: String
    }

    #[edgedb_sets]
    pub struct MySet {
        pub name: String,
    }

    #[edgedb_filters]
    pub struct MyFilter {
        #[field( column_name="modern_name",)]
        #[filter(operator="=", wrapper_fn="str_lower")]
        pub city_name: String,
    }


    #[update_query(table="City")]
    pub struct UpdateCity {

        #[sets]
        pub set: MySet,

        #[filters]
        pub filter: MyFilter,
    }

    #[select_query(table="City", result="City")]
    pub struct SelectCity {
        #[filters]
        name_filter: MyFilter,
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

        let new_name = "BUDA-PESTH";

        let update_query: EdgeQuery = UpdateCity {
            set: MySet {
                name: new_name.to_owned()
            },
            filter: MyFilter {
                city_name: "budapest".to_owned()
            }
        }.to_edge_query_with_cardinality(Cardinality::One);


        let query_str = update_query.query.as_str();

        let args = &update_query.args.unwrap();

        let result = client.query_required_single::<BasicResult, _>(query_str, args).await;


        if let Ok(r) = result {
            assert_ne!(r.id.to_string(), String::default());
        } else {
            unreachable!()
        }

        let select_city: EdgeQuery = SelectCity {
            name_filter: MyFilter {
                city_name: "budapest".to_owned()
            }
        }.to_edge_query();

        if let Ok(cities) = client.query(select_city.query.as_str(), &select_city.args.unwrap()).await {
            let city: &City = cities.get(0).unwrap();
            assert_eq!(new_name, city.name);
        } else {
            unreachable!()
        };
    }
}