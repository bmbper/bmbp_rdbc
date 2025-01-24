use crate::types::{RdbcInsert};
pub struct RdbcInsertBuilder {
    insert: RdbcInsert
}
impl RdbcInsertBuilder {
    pub fn build(&self) -> &RdbcInsert {
        &self.insert
    }
}

