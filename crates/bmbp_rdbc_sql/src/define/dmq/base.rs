use serde::{Deserialize, Serialize};
use bmbp_rdbc_type::RdbcValue;
use crate::define::dmq::{RdbcFunc, RdbcQuery, RdbcTable};

#[derive(Debug, Clone,Serialize,Deserialize)]
pub enum RdbcColumn {
    SimpleColumn(RdbcSimpleColumn),
    TableColumn(RdbcTableColumn),
    QueryColumn(RdbcQueryColumn),
    ValueColumn(RdbcValueColumn),
    FuncColumn(RdbcFuncColumn),
    RawColumn(RdbcRawColumn),
}
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcSimpleColumn {
    pub column: String,
}
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcTableColumn {
    pub table: RdbcTable,
    pub column: String,
}
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcQueryColumn {
    pub column: RdbcQuery,
}
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcValueColumn {
    pub column: RdbcValue,
}

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcFuncColumn {
    pub func: RdbcFunc,
}
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcRawColumn {
    pub column: String,
}

