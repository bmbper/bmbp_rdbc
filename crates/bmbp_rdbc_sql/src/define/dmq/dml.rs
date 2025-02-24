
use serde::{Deserialize, Serialize};
use bmbp_rdbc_type::RdbcValue;
use crate::define::dmq::{RdbcColumn, RdbcFunc, RdbcJoinTable, RdbcOrderColumn, RdbcQuery, RdbcTable, RdbcWhereFilter};

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct RdbcInsert {
    pub(crate) table: Vec<RdbcTable>,
    pub(crate) column: Vec<String>,
    pub(crate) values: Vec<RdbcDmlValue>,
    pub(crate) column_value: Vec<RdbcDmlColumn>,
    pub(crate) query: Option<RdbcQuery>,
}
#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct RdbcUpdate {
    pub(crate) table: Vec<RdbcTable>,
    pub(crate) join_table: Vec<RdbcJoinTable>,
    pub(crate) column_value: Vec<RdbcDmlColumn>,
    pub(crate) where_: Option<RdbcWhereFilter>,
    pub(crate) order_by: Vec<RdbcOrderColumn>,
    pub(crate) limit: Option<u64>,
    pub(crate) offset: Option<u64>,
}
#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct RdbcDelete {
    pub(crate) table: Vec<RdbcTable>,
    pub(crate) join_table: Vec<RdbcJoinTable>,
    pub(crate) where_: Option<RdbcWhereFilter>,
    pub(crate) order_by: Vec<RdbcOrderColumn>,
    pub(crate) limit: Option<u64>,
    pub(crate) offset: Option<u64>,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct RdbcDmlColumn {
    pub column: RdbcColumn,
    pub value: RdbcDmlValue,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RdbcDmlValue {
    VALUE(RdbcValue),
    COLUMN(RdbcColumn),
    FUNC(RdbcFunc),
}

