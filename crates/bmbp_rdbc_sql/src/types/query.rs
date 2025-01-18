use crate::types::{RdbcJoinTable, RdbcOrderColumn, RdbcSelectColumn, RdbcTable, RdbcWhereFilter};

pub struct RdbcQuery {
    select: Vec<RdbcSelectColumn>,
    table: Vec<RdbcTable>,
    join_table: Vec<RdbcJoinTable>,
    where_: Option<RdbcWhereFilter>,
    order_by: Vec<RdbcOrderColumn>,
    group_by: Vec<RdbcSelectColumn>,
    having: Option<RdbcWhereFilter>,
    limit: Option<u64>,
    offset: Option<u64>,
    union: Vec<RdbcQuery>,
    union_all: Vec<RdbcQuery>,
}
