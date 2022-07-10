const DEFAULT: &'static str = "default";
const ORDER_BY: &'static str = "order by";
const LIMIT: &'static str = "limit";
const OFFSET: &'static str = "offset";
const ASC: &'static str = " asc";
const DESC: &'static str = " desc";


pub trait ToSelectOptions {
    fn table_name(&self) -> &str;
    fn module(&self) -> Option<&str>;
    fn order_options(&self) -> Option<OrderOptions>;
    fn page_options(&self) -> Option<PageOptions>;

    fn to_edgeql(&self) -> String {
        let t_name = self
            .module()
            .or_else(|| Some(DEFAULT))
            .map(|m| format!("{}::{}", m, self.table_name()))
            .unwrap();

        let mut ql = String::default();

        if let Some(OrderOptions {
            order_by,
            order_direction,
        }) = self.order_options().clone()
        {
            ql.push_str(format!(" {} {}.{}", ORDER_BY, t_name, order_by).as_str());

            if let Some(OrderDir::Desc) = order_direction {
                ql.push_str(DESC)
            } else {
                ql.push_str(ASC)
            }
        }

        if let Some(PageOptions { limit, offset }) = self.page_options().clone() {
            ql.push_str(format!(" {} {}", LIMIT, limit).as_str());

            if let Some(off) = offset {
                ql.push_str(format!(" {} {}", OFFSET, off).as_str());
            }
        }

        ql
    }
}

#[derive(Clone)]
pub enum OrderDir {
    Asc,
    Desc,
}

#[derive(Clone)]
pub struct OrderOptions {
    pub order_by: String,
    pub order_direction: Option<OrderDir>,
}

#[derive(Clone)]
pub struct PageOptions {
    pub limit: u32,
    pub offset: Option<u32>,
}

pub struct SelectOptions<'a> {
    pub table_name: &'a str,
    pub module: Option<&'a str>,
    pub order_options: Option<OrderOptions>,
    pub page_options: Option<PageOptions>,
}

impl<'a> ToSelectOptions for SelectOptions<'a> {
    fn table_name(&self) -> &str {
        self.table_name
    }

    fn module(&self) -> Option<&str> {
        self.module
    }

    fn order_options(&self) -> Option<OrderOptions> {
        self.order_options.clone()
    }

    fn page_options(&self) -> Option<PageOptions> {
        self.page_options.clone()
    }
}
