const DEFAULT: &'static str = "default";
const ORDER_BY: &'static str = "order by";
const LIMIT: &'static str = "limit";
const OFFSET: &'static str = "offset";
const ASC: &'static str = " asc";
const DESC: &'static str = " desc";

/// # ToSelectOtions
///
/// Trait representing a select query options
///
pub trait Options {
    fn module(&self) -> Option<&str>;
    fn table_name(&self) -> &str;
    fn order_options(&self) -> Option<OrderOptions>;
    fn page_options(&self) -> Option<PageOptions>;
}

/// Parse the select query options
///
/// returns the select options statement
pub fn parse_options<T: Options>(options: T) -> String {
    let table_name = options
        .module()
        .or_else(|| Some(DEFAULT))
        .map(|module| format!("{}::{}", module, options.table_name()))
        .unwrap();

    let mut stmt = String::default();

    if let Some(OrderOptions {
                    order_by,
                    order_direction,
                }) = options.order_options().clone()
    {
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

/// ## Order direction
#[derive(Clone)]
pub enum OrderDir {
    Asc,
    Desc,
}

/// ## Select query Order options
///
///
#[derive(Clone)]
pub struct OrderOptions {
    pub order_by: String,
    pub order_direction: Option<OrderDir>,
}

/// Select query Page Options
#[derive(Clone)]
pub struct PageOptions {
    pub limit: u32,
    pub offset: Option<u32>,
}

/// Select Options structure
pub struct SelectOptions<'a> {
    pub table_name: &'a str,
    pub module: Option<&'a str>,
    pub order_options: Option<OrderOptions>,
    pub page_options: Option<PageOptions>,
}

impl<'a> Options for SelectOptions<'a> {
    fn module(&self) -> Option<&str> {
        self.module
    }

    fn table_name(&self) -> &str {
        self.table_name
    }

    fn order_options(&self) -> Option<OrderOptions> {
        self.order_options.clone()
    }

    fn page_options(&self) -> Option<PageOptions> {
        self.page_options.clone()
    }
}
