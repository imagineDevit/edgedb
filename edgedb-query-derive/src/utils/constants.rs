use std::collections::HashMap;

use crate::constants::{ENUM, NAME, QUERY, RESULT, TABLE, TYPE};

pub fn edgedb_attributes() -> HashMap<String, Option<(String, String)>> {
    let mut hlps = HashMap::new();

    hlps.insert(TYPE.to_owned(), Some((ENUM.to_owned(), NAME.to_owned())));

    hlps.insert(RESULT.to_owned(), None);

    hlps
}
