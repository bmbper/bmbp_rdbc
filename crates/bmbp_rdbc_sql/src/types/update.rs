use crate::types::{RdbcDmlColumn, RdbcJoinTable, RdbcOrderColumn, RdbcTable, RdbcWhereFilter};

pub struct RdbcUpdate {
    pub(crate) table: Vec<RdbcTable>,
    pub(crate) join_table: Vec<RdbcJoinTable>,
    pub(crate) dml_column: Vec<RdbcDmlColumn>,
    pub(crate) where_: Option<RdbcWhereFilter>,
    pub(crate) order_by: Vec<RdbcOrderColumn>,
    pub(crate) limit: Option<u64>,
    pub(crate) offset: Option<u64>,
}
