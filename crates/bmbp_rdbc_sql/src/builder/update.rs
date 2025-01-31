use crate::builder::part::{RdbcJoinTableBuilder, RdbcTableBuilder, RdbcFilterBuilder};
use crate::{RdbcJoinTable, RdbcTable, RdbcWhereFilter};
use crate::types::RdbcUpdate;

pub struct RdbcUpdateBuilder {
    update: RdbcUpdate,
}

impl RdbcUpdateBuilder {
    pub fn build(&self) -> &RdbcUpdate {
        &self.update
    }
}
impl RdbcTableBuilder for RdbcUpdateBuilder {
    fn table_mut(&mut self) -> &mut Vec<RdbcTable> {
        self.update.table.as_mut()
    }
}
impl RdbcJoinTableBuilder for RdbcUpdateBuilder {
    fn table_join_mut(&mut self) -> &mut Vec<RdbcJoinTable> {
        self.update.join_table.as_mut()
    }
}
impl RdbcFilterBuilder for RdbcUpdateBuilder {
    fn filter_mut(&mut self) -> &mut RdbcWhereFilter {
        self.update.where_.get_or_insert(RdbcWhereFilter::new())
    }
    fn filter_take(&mut self) -> Option<RdbcWhereFilter> {
        self.update.where_.take()
    }

}
