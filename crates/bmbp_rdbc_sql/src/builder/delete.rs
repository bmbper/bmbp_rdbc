use crate::builder::part::{RdbcJoinTableBuilder, RdbcTableBuilder, RdbcWhereFilterBuilder};
use crate::types::{RdbcDelete, RdbcQuery};

pub struct RdbcDeleteBuilder {
    delete: RdbcDelete
}

impl RdbcDeleteBuilder{
    pub fn build(&self) -> &RdbcDelete {
        &self.delete
    }
}
impl RdbcTableBuilder for RdbcDeleteBuilder {

}
impl RdbcJoinTableBuilder for RdbcDeleteBuilder {

}
impl RdbcWhereFilterBuilder for RdbcDeleteBuilder {

}