
#[cfg(test)]
mod result {
    use edgedb_query_derive::query_result;
    use edgedb_query::{EdgeResult, ToEdgeShape};


    #[query_result]
    pub struct Identity {
        pub name: String,
        pub age: i8
    }

    #[query_result]
    pub struct Friend {
        pub surname: String
    }

    #[query_result]
    pub struct User {
        pub login: String,
        pub identity: Identity,
    }

    #[query_result]
    pub struct UserWithFriends {
        pub login: String,
        pub identity: Identity,
        #[back_link(
            module="users",
            source_table="User",
            target_table="Friend",
            target_column="friend",
        )]
        pub friends: Vec<Friend>,
    }

    #[query_result]
    pub struct UserWithFriend {
        pub login: String,
        pub identity: Identity,
        #[back_link(
        module="users",
        source_table="User",
        target_table="Friend",
        target_column="friend",
        )]
        pub friend: Friend,
    }

    #[test]
    pub fn test_shape() {
        let shape = Identity::shape();
        let fields = Identity::returning_fields();
        assert_eq!(shape, "{name,age}");
        assert_eq!(fields, vec!["name", "age"]);
    }

    #[test]
    pub fn test_nested_shape() {
        let shape = User::shape();
        let fields = User::returning_fields();

        assert_eq!(shape, "{login,identity : {name,age}}");
        assert_eq!(fields, vec!["login", "identity"]);
    }

    #[test]
    pub fn test_nested_query_shape_vec() {
        let shape = UserWithFriends::shape();
        let fields = UserWithFriends::returning_fields();
        assert_eq!(shape, "{login,identity : {name,age},friends := (select users::User.<friend[is users::Friend]{surname})}");
        assert_eq!(fields, vec!["login", "identity", "friends"]);
    }

    #[test]
    pub fn test_nested_query_shape() {
        let shape = UserWithFriend::shape();
        assert_eq!(shape, "{login,identity : {name,age},friend := (select users::User.<friend[is users::Friend]{surname} limit 1)}")
    }


    #[query_result]
    pub struct UserWithFriendAndWrapperFn {
        #[field(wrapper_fn="str_upper")]
        pub login: String,
        pub identity: Identity,
        #[back_link(
        module="users",
        source_table="User",
        target_table="Friend",
        target_column="friend",

        )]
        pub friend: Friend,
    }

    #[test]
    pub fn test_nested_query_shape_wrapperfn() {
        let shape = UserWithFriendAndWrapperFn::shape();
        assert_eq!(shape, "{login := (select <str>str_upper(.login)),identity : {name,age},friend := (select users::User.<friend[is users::Friend]{surname} limit 1)}")
    }

    #[query_result]
    pub struct UserWithFriendAndField {
        #[field(column_name="pseudo")]
        pub login: String,
        pub identity: Identity,
        #[back_link(
        module="users",
        source_table="User",
        target_table="Friend",
        target_column="friend",
        )]
        pub friend: Friend,
    }

    #[test]
    pub fn test_nested_query_shape_field() {
        let shape = UserWithFriendAndField::shape();
        assert_eq!(shape, "{login := .pseudo,identity : {name,age},friend := (select users::User.<friend[is users::Friend]{surname} limit 1)}")
    }

    #[query_result]
    pub struct UserWithFriendAndFieldAndWrapperFn {
        #[field(column_name="pseudo", wrapper_fn="str_upper")]
        pub login: String,
        pub identity: Identity,
        #[back_link(
        module="users",
        source_table="User",
        target_table="Friend",
        target_column="friend"
        )]
        pub friend: Friend,
    }

    #[test]
    pub fn test_nested_query_shape_field_and_wrapper() {
        let shape = UserWithFriendAndFieldAndWrapperFn::shape();
        assert_eq!(shape, "{login := (select <str>str_upper(.pseudo)),identity : {name,age},friend := (select users::User.<friend[is users::Friend]{surname} limit 1)}")
    }

    #[query_result]
    pub struct UserWithDefault {
        #[field(column_name="pseudo", wrapper_fn="str_upper", default_value="john")]
        pub login: String,
    }

    #[test]
    pub fn test_with_default() {
        let shape = UserWithDefault::shape();
        assert_eq!(shape, "{login := (select <str>str_upper(.pseudo)) ?? (select <str>'john')}")
    }
}