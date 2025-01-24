use crate::builder::part::{RdbcJoinTableBuilder, RdbcTableBuilder, RdbcWhereFilterBuilder};
use crate::types::RdbcDelete;
use crate::{RdbcJoinTable, RdbcTable, RdbcWhereFilter};

pub struct RdbcDeleteBuilder {
    delete: RdbcDelete,
}

impl RdbcDeleteBuilder {
    pub fn build(&self) -> &RdbcDelete {
        &self.delete
    }
}
impl RdbcTableBuilder for RdbcDeleteBuilder {
    fn table_mut(&mut self) -> &mut Vec<RdbcTable> {
        self.delete.table.as_mut()
    }

}
impl RdbcJoinTableBuilder for RdbcDeleteBuilder {
    fn table_join_mut(&mut self) -> &mut Vec<RdbcJoinTable> {
        self.delete.join_table.as_mut()
    }
}
impl RdbcWhereFilterBuilder for RdbcDeleteBuilder {
    fn filter_mut(&mut self) -> &mut RdbcWhereFilter {
        self.delete.where_.get_or_insert(RdbcWhereFilter::new())
    }
}
