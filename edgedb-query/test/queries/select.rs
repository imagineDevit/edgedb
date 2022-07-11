
#[cfg(test)]
mod tests {
    use edgedb_query::queries::select::{OrderDir, OrderOptions, PageOptions, parse_options, SelectOptions};

    #[test]
    fn parse_with_no_options() {

        let options = SelectOptions {
            table_name: "User",
            module: None,
            order_options: None,
            page_options: None
        };

        let stmt = parse_options(&options);

        assert_eq!(String::default(), stmt);
    }

    #[test]
    fn parse_with_no_module_specified_and_order_by_options() {

        let options = SelectOptions {
            table_name: "User",
            module: None,
            order_options: Some(OrderOptions {
                order_by: "name".to_string(),
                order_direction: None
            }),
            page_options: None
        };

        let stmt = parse_options(&options);

        assert_eq!(String::from(" order by default::User.name asc"), stmt);
    }

    #[test]
    fn parse_with_no_module_specified_and_order_options() {

        let options = SelectOptions {
            table_name: "User",
            module: None,
            order_options: Some(OrderOptions {
                order_by: "name".to_string(),
                order_direction: Some(OrderDir::Desc)
            }),
            page_options: None
        };

        let stmt = parse_options(&options);

        assert_eq!(String::from(" order by default::User.name desc"), stmt);
    }

    #[test]
    fn parse_with_module_specified_and_order_options() {

        let options = SelectOptions {
            table_name: "User",
            module: Some("users"),
            order_options: Some(OrderOptions {
                order_by: "name".to_string(),
                order_direction: Some(OrderDir::Desc)
            }),
            page_options: None
        };

        let stmt = parse_options(&options);

        assert_eq!(String::from(" order by users::User.name desc"), stmt);
    }

    #[test]
    fn parse_with_module_specified_and_order_options_and_limit_options() {

        let options = SelectOptions {
            table_name: "User",
            module: Some("users"),
            order_options: Some(OrderOptions {
                order_by: "name".to_string(),
                order_direction: Some(OrderDir::Desc)
            }),
            page_options: Some(PageOptions {
                limit: 10,
                offset: None
            })
        };

        let stmt = parse_options(&options);

        assert_eq!(String::from(" order by users::User.name desc limit 10"), stmt);
    }

    #[test]
    fn parse_with_module_specified_and_order_options_and_page_options() {

        let options = SelectOptions {
            table_name: "User",
            module: Some("users"),
            order_options: Some(OrderOptions {
                order_by: "name".to_string(),
                order_direction: Some(OrderDir::Desc)
            }),
            page_options: Some(PageOptions {
                limit: 10,
                offset: Some(1)
            })
        };

        let stmt = parse_options(&options);

        assert_eq!(String::from(" order by users::User.name desc limit 10 offset 1"), stmt);
    }
}