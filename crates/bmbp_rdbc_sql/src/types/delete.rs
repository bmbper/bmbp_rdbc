use crate::types::{RdbcJoinTable, RdbcOrderColumn, RdbcTable, RdbcWhereFilter};

#[derive(Debug, Clone)]
pub struct RdbcDelete {
    pub(crate) table: Vec<RdbcTable>,
    pub(crate) join_table: Vec<RdbcJoinTable>,
    pub(crate) where_: Option<RdbcWhereFilter>,
    pub(crate) order_by: Vec<RdbcOrderColumn>,
    pub(crate) limit: Option<u64>,
    pub(crate) offset: Option<u64>,
}
impl RdbcDelete {
    pub fn new() -> Self {
        RdbcDelete {
            table: vec![],
            join_table: vec![],
            where_: None,
            order_by: vec![],
            limit: None,
            offset: None,
        }
    }
}