
#[cfg(test)]
mod update_cities {
    use edgedb_query_derive::{UpdateQuery, EdgedbSet, EdgedbFilters, EdgedbResult, SelectQuery};
    use edgedb_query::{*, queries::filter::Filter, models::query_result::BasicResult};
    use rstest::*;
    use edgedb_query::models::edge_query::{EdgeQuery, ToEdgeQuery};
    use serde::Deserialize;

    #[derive(EdgedbResult, Deserialize)]
    pub struct City {
        pub name: String
    }

    #[derive(EdgedbSet)]
    pub struct MySet {
        pub name: String,
    }

    #[derive(EdgedbFilters)]
    pub struct MyFilter {
        #[filter(operator="=", column_name="modern_name", wrapper_fn="str_lower")]
        pub city_name: String,
    }


    #[derive(UpdateQuery)]
    pub struct UpdateCity {
        #[meta(table="City")]
        __meta__: (),

        #[set]
        pub set: MySet,

        #[filters]
        pub filter: MyFilter,
    }

    #[derive(SelectQuery)]
    pub struct SelectCity {
        #[meta(table="City")]
        #[result(type="City")]
        __meta__: (),

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
            __meta__: (),
            set: MySet {
                name: new_name.to_owned()
            },
            filter: MyFilter {
                city_name: "budapest".to_owned()
            }
        }.to_edge_query();


        let query_str = update_query.query.as_str();

        let args = &update_query.args.unwrap();

        let result = client.query_json(query_str, args).await;

        let select_city: EdgeQuery = SelectCity {
            __meta__: (),
            name_filter: MyFilter {
                city_name: "budapest".to_owned()
            }
        }.to_edge_query();

        if let Ok(json) = client.query_json(select_city.query.as_str(), &select_city.args.unwrap()).await {
            let cities : Vec<City> = serde_json::from_str::<Vec<City>>(json.as_ref()).unwrap();
            let city: &City = cities.get(0).unwrap();
            assert_eq!(new_name, city.name);
        } else {
            assert!(false)
        };

        println!("{:#?}", result);

    }
}