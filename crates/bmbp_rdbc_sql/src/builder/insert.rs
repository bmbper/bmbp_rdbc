use crate::builder::part::RdbcTableBuilder;
use crate::types::{RdbcInsert, RdbcQuery};

pub struct RdbcInsertBuilder {
    insert: RdbcInsert
}
impl RdbcInsertBuilder {
    pub fn build(&self) -> &RdbcInsert {
        &self.insert
    }
}
impl RdbcTableBuilder for RdbcInsertBuilder {
}

