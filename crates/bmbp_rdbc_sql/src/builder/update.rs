use crate::builder::part::{RdbcJoinTableBuilder, RdbcTableBuilder, RdbcWhereFilterBuilder};
use crate::builder::query::RdbcQueryBuilder;
use crate::types::RdbcUpdate;

pub struct RdbcUpdateBuilder {
    update: RdbcUpdate,
}

impl RdbcUpdateBuilder {
    pub fn build(&self) -> &RdbcUpdate {
        &self.update
    }
}
impl RdbcTableBuilder for RdbcUpdateBuilder {}
impl RdbcJoinTableBuilder for RdbcUpdateBuilder {}
impl RdbcWhereFilterBuilder for RdbcUpdateBuilder {}
