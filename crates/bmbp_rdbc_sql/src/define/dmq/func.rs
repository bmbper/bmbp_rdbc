use serde::{Deserialize, Serialize};
use bmbp_rdbc_type::RdbcValue;
use crate::RdbcColumn;

#[derive(Debug, Clone,Serialize,Deserialize)]
pub enum RdbcFunc {
    DISTINCT(RdbcDistinctFunc),
    ABS(RdbcAbsFunc),
    SUM(RdbcSumFunc),
}
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcDistinctFunc {
    columns: Vec<RdbcColumn>,
}


#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcSumFunc {
    pub column: Vec<RdbcSumFuncColumn>,
}
#[derive(Debug, Clone,Serialize,Deserialize)]
pub enum RdbcSumFuncColumn {
    Column(RdbcColumn),
    Value(RdbcValue),
    Raw(String),
}
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcAbsFunc {
    column: Vec<RdbcColumn>,
}
