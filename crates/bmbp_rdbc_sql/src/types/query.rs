use crate::types::{RdbcJoinTable, RdbcOrderColumn, RdbcSelectColumn, RdbcTable, RdbcWhereFilter};


pub struct RdbcQuery {
   pub(crate) select: Vec<RdbcSelectColumn>,
    pub(crate) table: Vec<RdbcTable>,
    pub(crate) join_table: Vec<RdbcJoinTable>,
    pub(crate)  where_: Option<RdbcWhereFilter>,
    pub(crate) order_by: Vec<RdbcOrderColumn>,
    pub(crate) group_by: Vec<RdbcSelectColumn>,
    pub(crate) having: Option<RdbcWhereFilter>,
    pub(crate)  limit: Option<u64>,
    pub(crate) offset: Option<u64>,
    pub(crate) union: Vec<RdbcQuery>,
    pub(crate) union_all: Vec<RdbcQuery>,
}
