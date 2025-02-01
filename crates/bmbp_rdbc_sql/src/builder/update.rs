use crate::builder::part::{RdbcFilterBuilder, RdbcJoinTableBuilder, RdbcTableBuilder};
use crate::types::RdbcUpdate;
use crate::{
    RdbcColumn, RdbcDmlColumn, RdbcDmlValue, RdbcFunc, RdbcJoinTable, RdbcTable, RdbcWhereFilter,
};
use bmbp_rdbc_type::{RdbcIdent, RdbcValue, RdbcValueIdent};

pub struct RdbcUpdateBuilder {
    update: RdbcUpdate,
}

impl RdbcUpdateBuilder {
    pub fn build(&self) -> &RdbcUpdate {
        &self.update
    }
}

impl RdbcUpdateBuilder {
    pub fn set<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        RdbcColumn: From<C>,
        RdbcValue: From<V>,
    {
        let dml_column = RdbcDmlColumn {
            column: RdbcColumn::from(column),
            value: RdbcDmlValue::VALUE(RdbcValue::from(value)),
        };
        self.update.dml_column.push(dml_column);
        self
    }
    pub fn set_func<C>(&mut self, column: C, func: RdbcFunc) -> &mut Self
    where
        RdbcColumn: From<C>,
    {
        let dml_column = RdbcDmlColumn {
            column: RdbcColumn::from(column),
            value: RdbcDmlValue::FUNC(func),
        };
        self.update.dml_column.push(dml_column);
        self
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
