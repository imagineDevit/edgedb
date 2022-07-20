
#[cfg(test)]
mod result {
    use edgedb_query_derive::{EdgedbResult};
    use edgedb_query::{ToEdgeShape, ToEdgeScalar};

    #[derive(EdgedbResult)]
    pub struct Identity {
        pub name: String,
        pub age: u8
    }

    #[derive(EdgedbResult)]
    pub struct Friend {
        pub surname: String
    }

    #[derive(EdgedbResult)]
    pub struct User {
        pub login: String,
        pub identity: Identity,
    }

    #[derive(EdgedbResult)]
    pub struct UserWithFriends {
        pub login: String,
        pub identity: Identity,
        #[query_shape(
            module="users",
            source_table="User",
            target_table="Friend",
            target_column="friend",
            result="Friend"
        )]
        pub friends: Vec<Friend>,
    }

    #[derive(EdgedbResult)]
    pub struct UserWithFriend {
        pub login: String,
        pub identity: Identity,
        #[query_shape(
        module="users",
        source_table="User",
        target_table="Friend",
        target_column="friend",
        result="Friend"
        )]
        pub friend: Friend,
    }

    #[test]
    pub fn test_shape() {
        let shape = Identity::shape();

        assert_eq!(shape, "{name,age}")
    }

    #[test]
    pub fn test_nested_shape() {
        let shape = User::shape();

        assert_eq!(shape, "{login,identity : {name,age}}")
    }

    #[test]
    pub fn test_nested_query_shape_vec() {
        let shape = UserWithFriends::shape();
        assert_eq!(shape, "{login,identity : {name,age},friends := (select users::User.<friend[is users::Friend]{surname})}")
    }

    #[test]
    pub fn test_nested_query_shape() {
        let shape = UserWithFriend::shape();
        assert_eq!(shape, "{login,identity : {name,age},friend := (select users::User.<friend[is users::Friend]{surname} limit 1)}")
    }


    #[derive(EdgedbResult)]
    pub struct UserWithFriendAndWrapperFn {
        #[field(wrapper_fn="str_upper")]
        pub login: String,
        pub identity: Identity,
        #[query_shape(
        module="users",
        source_table="User",
        target_table="Friend",
        target_column="friend",
        result="Friend"
        )]
        pub friend: Friend,
    }

    #[test]
    pub fn test_nested_query_shape_wrapperfn() {
        let shape = UserWithFriendAndWrapperFn::shape();
        assert_eq!(shape, "{login := (select <str>str_upper(.login)),identity : {name,age},friend := (select users::User.<friend[is users::Friend]{surname} limit 1)}")
    }

    #[derive(EdgedbResult)]
    pub struct UserWithFriendAndField {
        #[field(column_name="pseudo")]
        pub login: String,
        pub identity: Identity,
        #[query_shape(
        module="users",
        source_table="User",
        target_table="Friend",
        target_column="friend",
        result="Friend"
        )]
        pub friend: Friend,
    }

    #[test]
    pub fn test_nested_query_shape_field() {
        let shape = UserWithFriendAndField::shape();
        assert_eq!(shape, "{login := .pseudo,identity : {name,age},friend := (select users::User.<friend[is users::Friend]{surname} limit 1)}")
    }

    #[derive(EdgedbResult)]
    pub struct UserWithFriendAndFieldAndWrapperFn {
        #[field(column_name="pseudo", wrapper_fn="str_upper")]
        pub login: String,
        pub identity: Identity,
        #[query_shape(
        module="users",
        source_table="User",
        target_table="Friend",
        target_column="friend",
        result="Friend"
        )]
        pub friend: Friend,
    }

    #[test]
    pub fn test_nested_query_shape_field_and_wrapper() {
        let shape = UserWithFriendAndFieldAndWrapperFn::shape();
        assert_eq!(shape, "{login := (select <str>str_upper(.pseudo)),identity : {name,age},friend := (select users::User.<friend[is users::Friend]{surname} limit 1)}")
    }

}