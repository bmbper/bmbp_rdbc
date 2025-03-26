use crate::builder::dmq::filter::RdbcWhereFilterBuilder;
use crate::builder::dmq::limit::{RdbcLimitBuilder, RdbcOffsetBuilder};
use crate::builder::dmq::order::RdbcOrderBuilder;
use crate::builder::dmq::table::{RdbcJoinTableBuilder, RdbcTableBuilder};
use crate::{RdbcDelete, RdbcFilterType, RdbcInsert, RdbcJoinTable, RdbcOrderColumn,  RdbcTable, RdbcUpdate, RdbcWhereFilter};

pub struct RdbcDeleteBuilder {
    delete: RdbcDelete,
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
        self.delete.where_.get_or_insert(RdbcWhereFilter{
            type_: RdbcFilterType::And,
            conditions: vec![],
            distinct: false,
        })
    }

    fn filter_take(&mut self) -> Option<RdbcWhereFilter> {
        self.delete.where_.take()
    }
}

impl RdbcOrderBuilder for RdbcDeleteBuilder {
    fn order_mut(&mut self) -> &mut Vec<RdbcOrderColumn> {
        self.delete.order_by.as_mut()
    }
}

impl RdbcLimitBuilder for RdbcDeleteBuilder {
    fn limit(&mut self, limit: u64) -> &mut Self {
       self.delete.limit = Some(limit);
        self
    }
}
impl RdbcOffsetBuilder for RdbcDeleteBuilder {
    fn offset(&mut self, limit: u64) -> &mut Self {
        self.delete.offset = Some(limit);
        self
    }
}
pub struct RdbcUpdateBuilder {
    update: RdbcUpdate,
}

impl RdbcUpdateBuilder {
    pub fn new() -> Self {
        RdbcUpdateBuilder { 
            update: RdbcUpdate { 
                table: vec![], 
                 join_table: vec![],
                  column_value: vec![], 
                  where_: None, 
                  order_by: vec![], 
                  limit: None, 
                  offset: None
                 }
        }
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

pub struct RdbcInsertBuilder {
    insert: RdbcInsert,
}

impl RdbcInsertBuilder {
    pub fn new() -> Self {
        RdbcInsertBuilder {
            insert: RdbcInsert{
                table: vec![],
                column: vec![],
                values: vec![],
                column_value: vec![],
                query: None,
            }
        }
    }
}
impl RdbcTableBuilder for RdbcInsertBuilder {
    fn table_mut(&mut self) -> &mut Vec<RdbcTable> {
        self.insert.table.as_mut()
    }
}