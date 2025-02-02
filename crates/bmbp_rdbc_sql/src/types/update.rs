use crate::types::{RdbcDmlColumn, RdbcJoinTable, RdbcOrderColumn, RdbcTable, RdbcWhereFilter};

#[derive(Debug, Clone)]
pub struct RdbcUpdate {
    pub(crate) table: Vec<RdbcTable>,
    pub(crate) join_table: Vec<RdbcJoinTable>,
    pub(crate) dml_column: Vec<RdbcDmlColumn>,
    pub(crate) where_: Option<RdbcWhereFilter>,
    pub(crate) order_by: Vec<RdbcOrderColumn>,
    pub(crate) limit: Option<u64>,
    pub(crate) offset: Option<u64>,
}
impl RdbcUpdate {
    pub fn new() -> Self {
        RdbcUpdate {
            table: vec![],
            join_table: vec![],
            dml_column: vec![],
            where_: None,
            order_by: vec![],
            limit: None,
            offset: None,
        }
    }
}
