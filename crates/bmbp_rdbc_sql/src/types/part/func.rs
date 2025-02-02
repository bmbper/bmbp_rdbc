use crate::types::RdbcColumn;
use bmbp_rdbc_type::{RdbcIdent, RdbcValue};

#[derive(Debug, Clone)]
pub enum RdbcFunc {
    DISTINCT(RdbcDistinctFunc),
    ABS(RdbcAbsFunc),
    SUM(RdbcSumFunc),
}
#[derive(Debug, Clone)]
pub struct RdbcDistinctFunc {
    columns: Vec<RdbcColumn>,
}

pub fn distinct<T>(column: T) -> RdbcDistinctFunc
where
    T: RdbcIdent,
{
    RdbcDistinctFunc {
        columns: vec![RdbcColumn::from(column.name())],
    }
}
pub fn distinct_vec<T>(column: Vec<T>) -> RdbcDistinctFunc
where
    T: RdbcIdent,
{
    let columns = column.into_iter().map(|i| RdbcColumn::from(i.name())).collect();
    RdbcDistinctFunc {
        columns,
    }
}
#[derive(Debug, Clone)]
pub struct RdbcSumFunc {
    pub column: Vec<RdbcSumFuncColumn>,
}
#[derive(Debug, Clone)]
pub enum RdbcSumFuncColumn {
    Column(RdbcColumn),
    Value(RdbcValue),
    Raw(String),
}
#[derive(Debug, Clone)]
pub struct RdbcAbsFunc {
    column: Vec<RdbcColumn>,
}
