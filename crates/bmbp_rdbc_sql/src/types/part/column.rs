use bmbp_rdbc_type::RdbcValue;
use crate::types::{RdbcFunc, RdbcQuery, RdbcTable};

pub enum RdbcColumn {
    SimpleColumn(RdbcSimpleColumn),
    TableColumn(RdbcTableColumn),
    QueryColumn(RdbcQueryColumn),
    ValueColumn(RdbcValueColumn),
    FuncColumn(RdbcFuncColumn),
    RawColumn(RdbcRawColumn)
}

pub struct RdbcSimpleColumn {
    pub column: String,
}
pub struct RdbcTableColumn {
    pub table: RdbcTable,
    pub column: String,
}
pub struct RdbcQueryColumn {
    pub column: RdbcQuery,
}
pub struct RdbcValueColumn {
    pub column: RdbcValue,
}
pub struct RdbcFuncColumn {
    pub func: RdbcFunc,
}
pub struct RdbcRawColumn {
    pub column: String,
}


pub struct RdbcDmlColumn {
    pub column: RdbcColumn,
    pub value: RdbcValue,
}
pub struct RdbcSelectColumn {
    pub column: RdbcColumn,
    pub alias: String,
}
pub struct QueryFilterColumn {
    pub column: RdbcColumn,
}

pub struct RdbcOrderColumn {
    pub column: RdbcColumn,
    pub order_type : OrderType
}
pub enum OrderType {
    Asc,
    Desc
}