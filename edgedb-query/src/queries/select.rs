
const ORDER_BY: &str = "order by";
const LIMIT: &str = "limit";
const OFFSET: &str = "offset";
const ASC: &str = " asc";
const DESC: &str = " desc";

/// Options Trait represents an EdgeDB select query options :
/// * order options
/// * pagination options

pub trait Options {

    /// returns the query's order options
    fn order_options(&self) -> Option<OrderOptions>;

    /// returns the query's pagination options
    fn page_options(&self) -> Option<PageOptions>;
}

/// Parse the select query options
///
/// __returns__ : the select options statment
///
/// ## Examples
///
/// ```
/// use edgedb_query::queries::select::{OrderOptions, parse_options, SelectOptions, OrderDir, PageOptions};
///
/// let options = SelectOptions {
///          order_options: Some(OrderOptions {
///              order_by: String::from("name"),
///              order_direction: Some(OrderDir::Desc),
///          }),
///          page_options: Some(PageOptions {
///              limit: 10,
///              offset: None
///          })
///      };
///  let stmt = parse_options(&options, "users::User", vec!["name"]);
///
///  assert_eq!(" order by users::User.name desc limit 10".to_owned(), stmt)
///
/// ```
pub fn parse_options<T: Options>(options: &T, table_name: impl Into<String>, result_fields: Vec<&str>) -> String {

    let mut stmt = String::default();

    let table_name = table_name.into();

    if let Some(OrderOptions {
                    order_by,
                    order_direction,
                }) = options.order_options().clone()
    {
        if !result_fields.contains(&order_by.as_str()) {
            panic!("'order by' value must be one of {:#?}", result_fields)
        }

        stmt.push_str(format!(" {} {}.{}", ORDER_BY, table_name, order_by).as_str());

        if let Some(OrderDir::Desc) = order_direction {
            stmt.push_str(DESC)
        } else {
            stmt.push_str(ASC)
        }
    }

    if let Some(PageOptions { limit, offset }) = options.page_options().clone() {
        stmt.push_str(format!(" {} {}", LIMIT, limit).as_str());

        if let Some(off) = offset {
            stmt.push_str(format!(" {} {}", OFFSET, off).as_str());
        }
    }

    stmt
}

/// Select query Order direction
#[derive(Debug, Clone)]
pub enum OrderDir {
    Asc,
    Desc,
}

/// Select query Order options
#[derive(Debug, Clone)]
pub struct OrderOptions {
    pub order_by: String,
    pub order_direction: Option<OrderDir>,
}

/// Select query Page Options
#[derive(Debug, Clone)]
pub struct PageOptions {
    pub limit: u32,
    pub offset: Option<u32>,
}

/// Select Options struct
#[derive(Debug, Clone)]
pub struct SelectOptions {
    pub order_options: Option<OrderOptions>,
    pub page_options: Option<PageOptions>,
}

impl Options for SelectOptions {

    fn order_options(&self) -> Option<OrderOptions> {
        self.order_options.clone()
    }

    fn page_options(&self) -> Option<PageOptions> {
        self.page_options.clone()
    }
}
