use serde::{Deserialize, Serialize};
use crate::define::dmq::{RdbcGroupColumn, RdbcJoinTable, RdbcOrderColumn, RdbcSelectColumn, RdbcTable, RdbcWhereFilter};
use crate::define::dmq::having::RdbcHaving;

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcQuery {
    pub(crate) select: Vec<RdbcSelectColumn>,
    pub(crate) table: Vec<RdbcTable>,
    pub(crate) join_table: Vec<RdbcJoinTable>,
    pub(crate) where_: Option<RdbcWhereFilter>,
    pub(crate) order_by: Vec<RdbcOrderColumn>,
    pub(crate) group_by: Vec<RdbcGroupColumn>,
    pub(crate) having: Option<RdbcHaving>,
    pub(crate) limit: Option<u64>,
    pub(crate) offset: Option<u64>,
    pub(crate) union: Vec<RdbcQuery>,
    pub(crate) union_all: Vec<RdbcQuery>,
}
