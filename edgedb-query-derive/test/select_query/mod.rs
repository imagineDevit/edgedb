#[cfg(test)]
mod select {

    use edgedb_query_derive::{SelectQuery, EdgedbEnum, EdgedbResult};
    use edgedb_query::{
        *,
        queries::select::SelectOptions

    };

    #[derive(Default, EdgedbResult)]
    pub struct UserResult {
        pub id: String,
        pub name: String,
        pub age: u8,
    }

    #[derive(SelectQuery)]
    pub struct FindUsersByNameLike {
        #[edgedb(module = "users", table = "User")]
        #[query(result = "UserResult", order_by = "age", order_dir = "asc", limit = 3)]
        __meta__: (),

        #[filter(Like)]
        pub name: String,
    }

    #[derive(SelectQuery)]
    pub struct FindUsersByNameIs {
        #[edgedb(module = "users", table = "User")]
        #[query(result = "UserResult")]
        __meta__: (),

        #[filter(Is)]
        pub name: String,
    }

    #[derive(SelectQuery)]
    pub struct FindUsersByAgeGreaterThan {
        #[edgedb(module = "users", table = "User")]
        #[query(result = "UserResult", order_by="name")]
        __meta__: (),

        #[filter(GreaterThan)]
        pub age: u32,
    }

    #[derive(SelectQuery)]
    pub struct FindUsersByAgeLesserThan {
        #[edgedb(module = "users", table = "User")]
        #[query(result = "UserResult", order_by="name")]
        __meta__: (),

        #[filter(LesserThan)]
        pub age: u32,
    }





}