use crate::types::{RdbcJoinTable, RdbcOrderColumn, RdbcTable, RdbcWhereFilter};

pub struct RdbcDelete {
    table: Vec<RdbcTable>,
    join_table: Vec<RdbcJoinTable>,
    where_: Option<RdbcWhereFilter>,
    order_by: Vec<RdbcOrderColumn>,
    limit: Option<u64>,
    offset: Option<u64>,
}