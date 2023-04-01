# SelectQuery

        #[select_query(module, table, result)] {
            #[field]
            #[filter]
            #[and_filter]
            #[or_filter]
            #[filters]
            #[options]
        }

**_select_query_** attribute macro indicates that the struct represents an edgeDB select query.


---

### Usage

Consider the following edgeDB schema 👇

````sql
    module models {
           type Person {
                required property user_name -> str;
                required property age -> int16;
                required property gender -> Gender;
                link address -> Address;
           }
    }
````

To perform a select query using edgedb-tokio we can write code as follows 👇

````rust
    #[select_query(module="models", table="Person", result="Person")]
    pub struct SelectPerson {
       
        #[field(column_name="user_name")]
        #[filter(operator = "Like")]
        pub name: String,
    
        #[and_filter(operator = "<=")]
        pub age: u16,
    
        #[options]
        options: SelectOptions
    }
    
    #[query_result]
    pub struct Person {
        pub user_name: String,
        pub age: i8
    }
    
    #[tokio::main]
    async fn main() -> Result<()> {
        let client = edgedb_tokio::create_client().await?;
        
        let select_person = SelectPerson {
            name: "%oe".to_owned(),
            age: 18,
            options: SelectOptions {
                table_name: "Person",
                module: Some("models"),
                order_options: Some(OrderOptions {
                    order_by: "name".to_string(),
                    order_direction: Some(OrderDir::Desc)
                }),
                page_options: None
            }
        };
    
        let edge_query: EdgeQuery = select_person.to_edge_query();
    
        let args = &edge_query.args.unwrap();
    
        let query = edge_query.query.as_str();
    
        if let Some(json) = client.query_json(query, args).await? {
            if let Ok(persons) = serde_json: from_str::<Vec<Person>>(json.as_ref()) {
                assert!(persons.len() > 0 );
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

````
