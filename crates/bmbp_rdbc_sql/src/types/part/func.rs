use crate::types::RdbcColumn;
use bmbp_rdbc_type::RdbcValue;

pub enum RdbcFunc {
    SUM(RdbcSumFunc),
}
pub struct RdbcSumFunc {
    pub column: Vec<RdbcSumFuncColumn>,
}
pub enum RdbcSumFuncColumn {
    Column(RdbcColumn),
    Value(RdbcValue),
    Raw(String),
}
