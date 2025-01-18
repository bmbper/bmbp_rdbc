use crate::types::{RdbcDmlColumn, RdbcJoinTable, RdbcOrderColumn, RdbcTable, RdbcWhereFilter};

pub struct RdbcUpdate {
    table: Vec<RdbcTable>,
    join_table: Vec<RdbcJoinTable>,
    dml_column: Vec<RdbcDmlColumn>,
    where_: Option<RdbcWhereFilter>,
    order_by: Vec<RdbcOrderColumn>,
    limit: Option<u64>,
    offset: Option<u64>,
}


